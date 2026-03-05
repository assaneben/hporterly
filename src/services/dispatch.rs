use crate::models::{Porter, Ticket};

/// Assignment scoring logic.
pub struct DispatchService;

impl DispatchService {
    /// Compute porter score between 0 and 100.
    pub fn calculate_score(porter: &Porter, ticket: &Ticket) -> i32 {
        let mut score = 100;

        // Blocking criteria.
        if !porter.is_available() {
            return 0;
        }

        if ticket.requires_o2_skill() && !Self::has_any_skill(porter, &["CERT-O2", "O2"]) {
            return 0;
        }

        if ticket.priority <= 2 && !Self::has_any_skill(porter, &["CERT-URG", "URG"]) {
            return 0;
        }

        if ticket.mode == "Lit" && !Self::has_any_skill(porter, &["CERT-LIT", "LIT"]) {
            return 0;
        }

        if ticket.mode == "Fauteuil" && !porter.has_skill("FAUTEUIL") {
            return 0;
        }

        if (ticket.patient_weight.unwrap_or(0) >= 120 || ticket.patient_bariatric)
            && !Self::has_any_skill(porter, &["CERT-BARIA", "BARIA"])
        {
            return 0;
        }

        // IMM load balancing.
        score -= porter.completed_missions_today * 10;

        // Historical performance bonus.
        let rating_bonus = ((porter.rating - 3.0) * 5.0).round() as i32;
        score += rating_bonus;

        score.clamp(0, 100)
    }

    fn has_any_skill(porter: &Porter, skills: &[&str]) -> bool {
        skills.iter().any(|skill| porter.has_skill(skill))
    }

    /// Return top 3 recommendations sorted by score.
    pub fn get_recommendations(ticket: &Ticket, porters: Vec<Porter>) -> Vec<(Porter, i32)> {
        let mut scored_porters: Vec<(Porter, i32)> = porters
            .into_iter()
            .map(|porter| {
                let score = Self::calculate_score(&porter, ticket);
                (porter, score)
            })
            .filter(|(_, score)| *score > 0)
            .collect();

        scored_porters.sort_by(|a, b| b.1.cmp(&a.1));
        scored_porters.truncate(3);
        scored_porters
    }

    /// Return best porter recommendation.
    pub fn get_best_porter(ticket: &Ticket, porters: Vec<Porter>) -> Option<(Porter, i32)> {
        Self::get_recommendations(ticket, porters)
            .into_iter()
            .next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn create_test_porter(
        id: &str,
        status: &str,
        skills: Vec<&str>,
        completed_today: i32,
        rating: f64,
    ) -> Porter {
        Porter {
            id: id.to_string(),
            user_id: format!("user-{}", id),
            status: status.to_string(),
            skills: skills.iter().map(|s| Some(s.to_string())).collect(),
            current_location: None,
            completed_missions_today: completed_today,
            total_missions: 100,
            rating,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }

    fn create_test_ticket(priority: i32, needs_o2: bool, mode: &str) -> Ticket {
        Ticket {
            id: "ticket-1".to_string(),
            patient_id: "patient-1".to_string(),
            patient_name: "Test Patient".to_string(),
            origin: "Service A".to_string(),
            destination: "Service B".to_string(),
            priority,
            mode: mode.to_string(),
            status: "pending".to_string(),
            porter_id: None,
            requester_id: "req-1".to_string(),
            needs_o2,
            needs_perfusion: false,
            isolation: false,
            patient_weight: None,
            patient_agitated: false,
            patient_monitoring: false,
            needs_two_porters: false,
            notes: None,
            scheduled_time: None,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            completed_at: None,
            transport_type: "PATIENT".to_string(),
            transport_subtype: "TP-BRANC".to_string(),
            equipment_recipient_patient_id: None,
            equipment_recipient_patient_name: None,
            equipment_size: None,
            equipment_return_service: None,
            equipment_delivered: None,
            equipment_label_returned: None,
            laboratory_name: None,
            specimen_types: None,
            notes_for_reception: None,
            help_requested: None,
            help_porter_id: None,
            help_status: None,
            help_requested_at: None,
            is_archived: false,
            archived_at: None,
            reservation_locked_by: None,
            reservation_locked_at: None,
            activation_minutes_before: None,
            is_visible_to_porters: true,
            patient_contentious: false,
            patient_confused: false,
            patient_over_120kg: false,
            patient_bariatric: false,
            patient_psychiatry: false,
            patient_dialysis: false,
            patient_icu: false,
            other_precautions: None,
            patient_first_name: None,
            patient_last_name: None,
            patient_dob: None,
            patient_sex: None,
            patient_ipp: None,
            motif: None,
        }
    }

    #[test]
    fn test_calculate_score_available_porter() {
        let porter = create_test_porter("P1", "available", vec!["O2", "URG"], 2, 4.5);
        let ticket = create_test_ticket(1, true, "Brancard");

        let score = DispatchService::calculate_score(&porter, &ticket);
        assert_eq!(score, 88);
    }

    #[test]
    fn test_calculate_score_unavailable_porter() {
        let porter = create_test_porter("P1", "busy", vec!["O2", "URG"], 0, 5.0);
        let ticket = create_test_ticket(1, true, "Brancard");

        let score = DispatchService::calculate_score(&porter, &ticket);
        assert_eq!(score, 0);
    }

    #[test]
    fn test_calculate_score_missing_skill() {
        let porter = create_test_porter("P1", "available", vec!["LIT"], 0, 5.0);
        let ticket = create_test_ticket(1, true, "Brancard");

        let score = DispatchService::calculate_score(&porter, &ticket);
        assert_eq!(score, 0);
    }

    #[test]
    fn test_get_recommendations() {
        let porters = vec![
            create_test_porter("P1", "available", vec!["O2", "URG"], 1, 4.8),
            create_test_porter("P2", "available", vec!["O2", "URG"], 3, 4.2),
            create_test_porter("P3", "busy", vec!["O2", "URG"], 0, 5.0),
            create_test_porter("P4", "available", vec!["O2", "URG"], 0, 4.5),
        ];

        let ticket = create_test_ticket(1, true, "Brancard");
        let recommendations = DispatchService::get_recommendations(&ticket, porters);

        assert_eq!(recommendations.len(), 3);
        assert!(recommendations[0].1 >= recommendations[1].1);
        assert!(recommendations[1].1 >= recommendations[2].1);
    }
}
