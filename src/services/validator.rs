use crate::models::CreateTicketRequest;
use crate::utils::{ApiError, ApiResult};

pub struct ValidatorService;
const MAX_TICKET_NOTES_LEN: usize = 5000;

impl ValidatorService {
    /// Validate ticket creation payload.
    pub fn validate_ticket(request: &CreateTicketRequest) -> ApiResult<()> {
        let transport_type = request.transport_type.trim().to_uppercase();
        let transport_subtype = request.transport_subtype.trim().to_uppercase();
        if !["PATIENT", "EQUIPMENT", "SPECIMEN"].contains(&transport_type.as_str()) {
            return Err(ApiError::ValidationError("Type de transport invalide".to_string()));
        }

        // Origin and destination must be different.
        if request.origin == request.destination {
            return Err(ApiError::ValidationError(
                "L'origine et la destination doivent etre differentes".to_string(),
            ));
        }

        // Priority must be between 1 and 4.
        if !(1..=4).contains(&request.priority) {
            return Err(ApiError::ValidationError(
                "La priorite doit etre entre 1 et 4".to_string(),
            ));
        }

        if transport_type == "EQUIPMENT" && request.priority == 1 {
            return Err(ApiError::ValidationError(
                "P1 interdit pour une mission materiel seule".to_string(),
            ));
        }

        if transport_subtype.contains("LAB_URG") && ![1, 2].contains(&request.priority) {
            return Err(ApiError::ValidationError(
                "LAB_URG urgent doit etre classe en P1 ou P2".to_string(),
            ));
        }

        // Mode validation only applies to patient transport.
        if transport_type == "PATIENT" {
            let mode = request.mode.as_deref().unwrap_or("").trim();
            let valid_modes = ["Lit", "Fauteuil", "Brancard", "Marche"];
            if !valid_modes.contains(&mode) {
                return Err(ApiError::ValidationError("Mode de transport invalide".to_string()));
            }

            let has_identity = !request.patient_id.trim().is_empty()
                || request.patient_ipp.as_deref().unwrap_or("").trim().len() >= 2;
            if !has_identity {
                return Err(ApiError::ValidationError(
                    "Une identite patient (IPP/INS) est obligatoire pour un transport patient"
                        .to_string(),
                ));
            }
        }

        // Notes can include a structured [META-FORM] block used by business rules.
        // Keep a protective upper bound while avoiding false rejects on complete forms.
        if let Some(notes) = &request.notes {
            if notes.len() > MAX_TICKET_NOTES_LEN {
                return Err(ApiError::ValidationError(format!(
                    "Les notes ne peuvent pas depasser {} caracteres",
                    MAX_TICKET_NOTES_LEN
                )));
            }
        }

        // Business rule: if weight >= 120kg, two porters are usually required.
        if let Some(weight) = request.patient_weight {
            if weight >= 120 && !request.needs_two_porters {
                log::warn!("Patient weight {} kg but needs_two_porters not set", weight);
            }
        }

        // Business rule: agitated patient usually needs two porters.
        if request.patient_agitated && !request.needs_two_porters {
            log::warn!("Patient agitated but needs_two_porters not set");
        }

        Ok(())
    }

    /// Auto-calculate whether two porters are required.
    pub fn auto_calculate_two_porters(request: &mut CreateTicketRequest) {
        if let Some(weight) = request.patient_weight {
            if weight >= 120 {
                request.needs_two_porters = true;
            }
        }

        if request.patient_agitated || request.patient_bariatric || request.patient_over_120kg {
            request.needs_two_porters = true;
        }

        let transport_type = request.transport_type.trim().to_uppercase();
        let transport_subtype = request.transport_subtype.trim().to_uppercase();
        if transport_type == "EQUIPMENT"
            && (transport_subtype.contains("ECHO") || transport_subtype.contains("ARCEAU"))
        {
            request.needs_two_porters = true;
        }
    }
}
