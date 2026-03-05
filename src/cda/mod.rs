use std::time::Duration;

use chrono::{DateTime, NaiveDate, Utc};
use diesel::deserialize::QueryableByName;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::{Integer, Nullable, Text, Timestamp};
use reqwest::Client;
use serde::Serialize;
use uuid::Uuid;

use crate::config::Config;
use crate::hl7::models::ins_match;
use crate::models::Ticket;
use crate::repositories::TicketRepository;
use crate::schema::{patients, porters, users};
use crate::DbPool;

const CDA_INS_OID: &str = "1.2.250.1.213.1.4.8";

#[derive(Debug, Clone)]
pub struct CdaTransportReport {
    pub transport_id: Uuid,
    pub ins: String,
    pub nom_patient: String,
    pub prenom_patient: String,
    pub date_naissance: NaiveDate,
    pub sexe: String,
    pub origin_nom: String,
    pub destination_nom: String,
    pub brancardier_nom: String,
    pub prescripteur_nom: Option<String>,
    pub debut_transport: DateTime<Utc>,
    pub fin_transport: DateTime<Utc>,
    pub precautions: Vec<String>,
    pub ins_confirme: bool,
}

#[derive(Debug, Serialize)]
struct CdaDispatchPayload {
    cda_xml: String,
    ins: String,
    transport_id: String,
}

#[derive(Debug, QueryableByName)]
struct PendingCdaRow {
    #[diesel(sql_type = Text)]
    transport_id: String,
    #[diesel(sql_type = Text)]
    ins: String,
    #[diesel(sql_type = Text)]
    cda_xml: String,
}

#[derive(Debug, QueryableByName)]
struct PatientFallbackRow {
    #[diesel(sql_type = Nullable<Timestamp>)]
    date_of_birth: Option<chrono::NaiveDateTime>,
    #[diesel(sql_type = Nullable<Text>)]
    sex: Option<String>,
}

pub fn trigger_transport_report_generation(pool: DbPool, config: Config, ticket_id: String) {
    tokio::spawn(async move {
        if let Err(err) = process_transport_completion(&pool, &config, ticket_id.as_str()).await {
            log::error!(
                "CDA generation/dispatch failed for transport {}: {}",
                ticket_id,
                err
            );
        }
    });
}

pub async fn process_transport_completion(
    pool: &DbPool,
    config: &Config,
    ticket_id: &str,
) -> Result<(), String> {
    let report = {
        let mut conn = pool
            .get()
            .map_err(|e| format!("Database connection error while loading transport: {}", e))?;
        ensure_cda_pending_table(&mut conn)?;
        let ticket = TicketRepository::find_by_id(&mut conn, ticket_id).map_err(|e| {
            format!(
                "Transport {} introuvable pour generation CDA: {}",
                ticket_id, e
            )
        })?;
        build_report_from_ticket(&mut conn, &ticket)?
    };

    let cda_xml = build_cda_xml(&report);
    let payload = CdaDispatchPayload {
        cda_xml: cda_xml.clone(),
        ins: report.ins.clone(),
        transport_id: report.transport_id.to_string(),
    };

    let send_result = send_to_mirth(config.cda_mirth_endpoint.as_str(), &payload).await;

    let mut conn = pool.get().map_err(|e| {
        format!(
            "Database connection error while persisting CDA status: {}",
            e
        )
    })?;
    ensure_cda_pending_table(&mut conn)?;

    match send_result {
        Ok(()) => {
            delete_pending_cda(&mut conn, report.transport_id.to_string().as_str())?;
            log::info!(
                "CDA sent successfully for transport {}",
                report.transport_id
            );
        }
        Err(err) => {
            upsert_pending_cda(
                &mut conn,
                report.transport_id.to_string().as_str(),
                report.ins.as_str(),
                cda_xml.as_str(),
                err.as_str(),
            )?;
            log::warn!(
                "Mirth unreachable for transport {}: {}. CDA queued in cda_pending",
                report.transport_id,
                err
            );
        }
    }

    retry_pending_cda(pool, config).await?;
    Ok(())
}

async fn retry_pending_cda(pool: &DbPool, config: &Config) -> Result<(), String> {
    let pending_rows = {
        let mut conn = pool
            .get()
            .map_err(|e| format!("Database connection error while reading cda_pending: {}", e))?;
        ensure_cda_pending_table(&mut conn)?;
        list_pending_cda(&mut conn)?
    };

    if pending_rows.is_empty() {
        return Ok(());
    }

    for row in pending_rows {
        let payload = CdaDispatchPayload {
            cda_xml: row.cda_xml.clone(),
            ins: row.ins.clone(),
            transport_id: row.transport_id.clone(),
        };
        match send_to_mirth(config.cda_mirth_endpoint.as_str(), &payload).await {
            Ok(()) => {
                let mut conn = pool.get().map_err(|e| {
                    format!(
                        "Database connection error while deleting sent cda_pending row: {}",
                        e
                    )
                })?;
                delete_pending_cda(&mut conn, row.transport_id.as_str())?;
            }
            Err(err) => {
                let mut conn = pool.get().map_err(|e| {
                    format!(
                        "Database connection error while updating failed cda_pending row: {}",
                        e
                    )
                })?;
                update_pending_error(&mut conn, row.transport_id.as_str(), err.as_str())?;
            }
        }
    }

    Ok(())
}

fn build_report_from_ticket(
    conn: &mut PgConnection,
    ticket: &Ticket,
) -> Result<CdaTransportReport, String> {
    let transport_id = uuid_from_transport_key(ticket.id.as_str());
    let ins = extract_ins(ticket)?;

    let (prenom_patient, nom_patient) = extract_patient_names(ticket);
    let (date_naissance, sexe) = resolve_patient_identity_fallback(conn, ticket)?;

    let brancardier_nom = resolve_brancardier_nom(conn, ticket)?;
    let prescripteur_nom = resolve_prescripteur_nom(conn, ticket.requester_id.as_str())?;

    let debut_transport = DateTime::<Utc>::from_naive_utc_and_offset(ticket.created_at, Utc);
    let fin_transport = DateTime::<Utc>::from_naive_utc_and_offset(
        ticket
            .completed_at
            .unwrap_or_else(|| Utc::now().naive_utc()),
        Utc,
    );

    let precautions = extract_precautions(ticket);
    let ins_confirme = ticket
        .patient_ipp
        .as_deref()
        .map(str::trim)
        .map(|ipp| ins_match(ipp, ins))
        .unwrap_or(false);

    Ok(CdaTransportReport {
        transport_id,
        ins: ins.to_string(),
        nom_patient,
        prenom_patient,
        date_naissance,
        sexe,
        origin_nom: ticket.origin.trim().to_string(),
        destination_nom: ticket.destination.trim().to_string(),
        brancardier_nom,
        prescripteur_nom,
        debut_transport,
        fin_transport,
        precautions,
        ins_confirme,
    })
}

fn uuid_from_transport_key(transport_key: &str) -> Uuid {
    if let Ok(uuid) = Uuid::parse_str(transport_key) {
        return uuid;
    }
    if let Some(raw) = transport_key.strip_prefix("HL7-") {
        if let Ok(uuid) = Uuid::parse_str(raw) {
            return uuid;
        }
    }
    Uuid::new_v5(&Uuid::NAMESPACE_OID, transport_key.as_bytes())
}

fn extract_ins(ticket: &Ticket) -> Result<&str, String> {
    let ins = ticket
        .patient_ipp
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(ticket.patient_id.trim());
    if ins.is_empty() {
        return Err("INS absent pour la generation CDA".to_string());
    }
    Ok(ins)
}

fn extract_patient_names(ticket: &Ticket) -> (String, String) {
    let first_name = ticket
        .patient_first_name
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string);
    let last_name = ticket
        .patient_last_name
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string);

    if let (Some(first), Some(last)) = (first_name, last_name) {
        return (first, last);
    }

    let fallback = ticket.patient_name.trim();
    if fallback.is_empty() {
        return ("INCONNU".to_string(), "INCONNU".to_string());
    }
    let parts: Vec<&str> = fallback.split_whitespace().collect();
    if parts.len() >= 2 {
        (parts[1..].join(" "), parts[0].to_string())
    } else {
        ("INCONNU".to_string(), parts[0].to_string())
    }
}

fn resolve_patient_identity_fallback(
    conn: &mut PgConnection,
    ticket: &Ticket,
) -> Result<(NaiveDate, String), String> {
    let fallback_row = sql_query(
        "SELECT
            CASE
                WHEN date_of_birth IS NULL THEN NULL
                ELSE date_of_birth::timestamp
            END AS date_of_birth,
            sex
         FROM patients
         WHERE id = $1
         LIMIT 1",
    )
    .bind::<Text, _>(ticket.patient_id.as_str())
    .get_result::<PatientFallbackRow>(conn)
    .optional()
    .map_err(|e| format!("Patient fallback lookup failed: {}", e))?;

    let date_naissance = ticket
        .patient_dob
        .or_else(|| {
            fallback_row
                .as_ref()
                .and_then(|row| row.date_of_birth.map(|dt| dt.date()))
        })
        .ok_or_else(|| {
            format!(
                "Date de naissance absente pour transport {}, CDA non genere",
                ticket.id
            )
        })?;

    let sexe = ticket
        .patient_sex
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
        .or_else(|| fallback_row.as_ref().and_then(|row| row.sex.clone()))
        .or_else(|| {
            patients::table
                .find(ticket.patient_id.as_str())
                .select(patients::sex)
                .first::<Option<String>>(conn)
                .ok()
                .flatten()
        })
        .unwrap_or_else(|| "U".to_string());

    Ok((date_naissance, sexe))
}

fn resolve_brancardier_nom(conn: &mut PgConnection, ticket: &Ticket) -> Result<String, String> {
    let Some(porter_id) = ticket.porter_id.as_deref() else {
        return Ok("SYSTEME_HPORTERLY".to_string());
    };

    let names = porters::table
        .inner_join(users::table.on(porters::user_id.eq(users::id)))
        .filter(porters::id.eq(porter_id))
        .select((users::first_name, users::last_name))
        .first::<(String, String)>(conn)
        .optional()
        .map_err(|e| format!("Brancardier lookup failed: {}", e))?;

    Ok(names
        .map(|(first_name, last_name)| format!("{} {}", first_name.trim(), last_name.trim()))
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "SYSTEME_HPORTERLY".to_string()))
}

fn resolve_prescripteur_nom(
    conn: &mut PgConnection,
    requester_id: &str,
) -> Result<Option<String>, String> {
    let names = users::table
        .find(requester_id)
        .select((users::first_name, users::last_name))
        .first::<(String, String)>(conn)
        .optional()
        .map_err(|e| format!("Prescripteur lookup failed: {}", e))?;

    Ok(names
        .map(|(first_name, last_name)| format!("{} {}", first_name.trim(), last_name.trim()))
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty()))
}

fn extract_precautions(ticket: &Ticket) -> Vec<String> {
    let mut items = Vec::new();
    if ticket.needs_o2 {
        items.push("OXYGENE".to_string());
    }
    if ticket.needs_perfusion {
        items.push("PERFUSION".to_string());
    }
    if ticket.isolation {
        items.push("ISOLEMENT".to_string());
    }
    if ticket.patient_monitoring {
        items.push("MONITORING".to_string());
    }
    if ticket.needs_two_porters {
        items.push("DEUX_BRANCARDIERS".to_string());
    }
    if let Some(other) = ticket
        .other_precautions
        .as_deref()
        .map(str::trim)
        .filter(|v| !v.is_empty())
    {
        items.push(other.to_string());
    }
    items
}

pub fn build_cda_xml(report: &CdaTransportReport) -> String {
    let id = report.transport_id.to_string();
    let issue_time = Utc::now().to_rfc3339();
    let start_time = report.debut_transport.to_rfc3339();
    let end_time = report.fin_transport.to_rfc3339();
    let precautions_xml = if report.precautions.is_empty() {
        "<item>AUCUNE</item>".to_string()
    } else {
        report
            .precautions
            .iter()
            .map(|item| format!("<item>{}</item>", xml_escape(item)))
            .collect::<Vec<String>>()
            .join("")
    };
    let prescripteur = report
        .prescripteur_nom
        .as_deref()
        .map(xml_escape)
        .unwrap_or_else(|| "NON_RENSEIGNE".to_string());

    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<ClinicalDocument xmlns="urn:hl7-org:v3">
  <typeId root="2.16.840.1.113883.1.3" extension="POCD_HD000040"/>
  <id root="{id}"/>
  <code code="34133-9" codeSystem="2.16.840.1.113883.6.1" displayName="Summarization of Episode Note"/>
  <title>Compte-rendu de transport brancardage</title>
  <effectiveTime value="{issue_time}"/>
  <recordTarget>
    <patientRole>
      <id root="{ins_oid}" extension="{ins}"/>
      <patient>
        <name>
          <family>{nom_patient}</family>
          <given>{prenom_patient}</given>
        </name>
        <administrativeGenderCode code="{sexe}"/>
        <birthTime value="{date_naissance}"/>
      </patient>
    </patientRole>
  </recordTarget>
  <author>
    <time value="{issue_time}"/>
    <assignedAuthor>
      <id root="2.16.840.1.113883.3.9999.1" extension="HPORTERLY_SYSTEM"/>
      <assignedPerson>
        <name>
          <family>HPORTERLY</family>
          <given>SYSTEM</given>
        </name>
      </assignedPerson>
    </assignedAuthor>
  </author>
  <component>
    <structuredBody>
      <component>
        <section>
          <code code="55112-7" codeSystem="2.16.840.1.113883.6.1" displayName="Transport summary"/>
          <title>Details du transport</title>
          <text>
            <list>
              <item>Transport ID: {transport_id}</item>
              <item>Origine: {origin_nom}</item>
              <item>Destination: {destination_nom}</item>
              <item>Brancardier: {brancardier_nom}</item>
              <item>Prescripteur: {prescripteur}</item>
              <item>Debut transport: {start_time}</item>
              <item>Fin transport: {end_time}</item>
              <item>INS confirme: {ins_confirme}</item>
              <item>Precautions:</item>
              {precautions_xml}
            </list>
          </text>
        </section>
      </component>
    </structuredBody>
  </component>
</ClinicalDocument>"#,
        id = xml_escape(id.as_str()),
        issue_time = xml_escape(issue_time.as_str()),
        ins_oid = CDA_INS_OID,
        ins = xml_escape(report.ins.as_str()),
        nom_patient = xml_escape(report.nom_patient.as_str()),
        prenom_patient = xml_escape(report.prenom_patient.as_str()),
        sexe = xml_escape(report.sexe.as_str()),
        date_naissance = report.date_naissance.format("%Y-%m-%d"),
        transport_id = xml_escape(id.as_str()),
        origin_nom = xml_escape(report.origin_nom.as_str()),
        destination_nom = xml_escape(report.destination_nom.as_str()),
        brancardier_nom = xml_escape(report.brancardier_nom.as_str()),
        prescripteur = prescripteur,
        start_time = xml_escape(start_time.as_str()),
        end_time = xml_escape(end_time.as_str()),
        ins_confirme = if report.ins_confirme { "true" } else { "false" },
        precautions_xml = precautions_xml,
    )
}

async fn send_to_mirth(endpoint: &str, payload: &CdaDispatchPayload) -> Result<(), String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .map_err(|e| format!("HTTP client init failed: {}", e))?;

    let response = client
        .post(endpoint)
        .json(payload)
        .send()
        .await
        .map_err(|e| format!("Mirth POST request failed: {}", e))?;

    if response.status().is_success() {
        Ok(())
    } else {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        Err(format!("Mirth responded {}: {}", status, body))
    }
}

fn ensure_cda_pending_table(conn: &mut PgConnection) -> Result<(), String> {
    sql_query(
        "CREATE TABLE IF NOT EXISTS cda_pending (
            transport_id VARCHAR(64) PRIMARY KEY,
            ins VARCHAR(19) NOT NULL,
            cda_xml TEXT NOT NULL,
            last_error TEXT,
            retry_count INTEGER NOT NULL DEFAULT 0,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        )",
    )
    .execute(conn)
    .map_err(|e| format!("Failed to ensure cda_pending table: {}", e))?;

    sql_query(
        "CREATE INDEX IF NOT EXISTS idx_cda_pending_updated_at
         ON cda_pending(updated_at)",
    )
    .execute(conn)
    .map_err(|e| format!("Failed to ensure cda_pending index: {}", e))?;

    Ok(())
}

fn upsert_pending_cda(
    conn: &mut PgConnection,
    transport_id: &str,
    ins: &str,
    cda_xml: &str,
    last_error: &str,
) -> Result<(), String> {
    sql_query(
        "INSERT INTO cda_pending (transport_id, ins, cda_xml, last_error, retry_count, created_at, updated_at)
         VALUES ($1, $2, $3, $4, 1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
         ON CONFLICT (transport_id) DO UPDATE
         SET ins = EXCLUDED.ins,
             cda_xml = EXCLUDED.cda_xml,
             last_error = EXCLUDED.last_error,
             retry_count = cda_pending.retry_count + 1,
             updated_at = CURRENT_TIMESTAMP",
    )
    .bind::<Text, _>(transport_id)
    .bind::<Text, _>(ins)
    .bind::<Text, _>(cda_xml)
    .bind::<Text, _>(last_error)
    .execute(conn)
    .map_err(|e| format!("Failed to queue pending CDA: {}", e))?;

    Ok(())
}

fn delete_pending_cda(conn: &mut PgConnection, transport_id: &str) -> Result<(), String> {
    sql_query("DELETE FROM cda_pending WHERE transport_id = $1")
        .bind::<Text, _>(transport_id)
        .execute(conn)
        .map_err(|e| format!("Failed to delete sent pending CDA: {}", e))?;
    Ok(())
}

fn update_pending_error(
    conn: &mut PgConnection,
    transport_id: &str,
    last_error: &str,
) -> Result<(), String> {
    sql_query(
        "UPDATE cda_pending
         SET last_error = $2,
             retry_count = retry_count + 1,
             updated_at = CURRENT_TIMESTAMP
         WHERE transport_id = $1",
    )
    .bind::<Text, _>(transport_id)
    .bind::<Text, _>(last_error)
    .execute(conn)
    .map_err(|e| format!("Failed to update pending CDA error: {}", e))?;
    Ok(())
}

fn list_pending_cda(conn: &mut PgConnection) -> Result<Vec<PendingCdaRow>, String> {
    sql_query(
        "SELECT transport_id, ins, cda_xml
         FROM cda_pending
         ORDER BY updated_at ASC
         LIMIT $1",
    )
    .bind::<Integer, _>(20)
    .load::<PendingCdaRow>(conn)
    .map_err(|e| format!("Failed to list pending CDA rows: {}", e))
}

fn xml_escape(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/*
SECURITY REVIEW (SecureByDesign v1.1.0 - REGULATED)
- Controls reviewed: SBD-01 to SBD-25.
- Verified in this file:
  - SBD-01: strict server-side report construction with explicit field normalization and XML escaping.
  - SBD-07: no secret hardcoded; endpoint driven by runtime config.
  - SBD-08: CDA payload transport supports secured channel policy when endpoint is protected by internal network/TLS ingress.
  - SBD-09/SBD-10: only necessary metadata included in transport notification payload.
  - SBD-13: fail-safe error handling with controlled logs.
  - SBD-21: if Mirth is unreachable, CDA is persisted in `cda_pending` (never dropped).
  - SBD-24: retry mechanism for pending CDA supports incident recovery.
- Not fully satisfiable in this file:
  - SBD-08 TLS 1.3 cannot be guaranteed by module code alone for `http://localhost:6661`.
    Alternative: enforce TLS/mTLS at Mirth ingress and private network policy.
  - SBD-25 compliance evidence retention/legal attestation remains operational/governance scope.
    Alternative: add release evidence checklist + immutable archive policy.
*/
