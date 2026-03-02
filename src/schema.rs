// @generated automatically by Diesel CLI.

diesel::table! {
    audit_logs (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        user_id -> Varchar,
        #[max_length = 100]
        action -> Varchar,
        #[max_length = 50]
        entity_type -> Varchar,
        #[max_length = 255]
        entity_id -> Varchar,
        old_value -> Nullable<Jsonb>,
        new_value -> Nullable<Jsonb>,
        #[max_length = 45]
        ip_address -> Nullable<Varchar>,
        user_agent -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    help_requests (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        ticket_id -> Varchar,
        #[max_length = 255]
        requesting_porter_id -> Varchar,
        #[max_length = 255]
        requested_porter_id -> Varchar,
        #[max_length = 50]
        status -> Varchar,
        created_at -> Timestamp,
        responded_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    notification_preferences (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        user_id -> Varchar,
        preferences -> Jsonb,
        sound_enabled -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    notifications (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        user_id -> Varchar,
        #[max_length = 50]
        notification_type -> Varchar,
        #[max_length = 255]
        title -> Varchar,
        message -> Text,
        #[max_length = 20]
        priority -> Varchar,
        data -> Nullable<Jsonb>,
        is_read -> Bool,
        #[max_length = 255]
        related_ticket_id -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    patients (id) {
        #[max_length = 50]
        id -> Varchar,
        #[max_length = 100]
        first_name -> Varchar,
        #[max_length = 100]
        last_name -> Varchar,
        age -> Nullable<Int4>,
        #[max_length = 1]
        gender -> Nullable<Bpchar>,
        #[max_length = 100]
        service -> Nullable<Varchar>,
        #[max_length = 50]
        room -> Nullable<Varchar>,
        #[max_length = 50]
        building -> Nullable<Varchar>,
        #[max_length = 50]
        floor -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        date_of_birth -> Nullable<Date>,
        #[max_length = 10]
        sex -> Nullable<Varchar>,
    }
}

diesel::table! {
    porters (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        user_id -> Varchar,
        #[max_length = 50]
        status -> Varchar,
        skills -> Array<Nullable<Text>>,
        #[max_length = 255]
        current_location -> Nullable<Varchar>,
        completed_missions_today -> Int4,
        total_missions -> Int4,
        rating -> Float8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    priority_rules_config (id) {
        #[max_length = 255]
        id -> Varchar,
        rules_json -> Jsonb,
        is_active -> Bool,
        #[max_length = 255]
        updated_by -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    referential_equipment (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        label -> Varchar,
        sizes -> Nullable<Array<Nullable<Text>>>,
        required_fields -> Jsonb,
        is_active -> Bool,
        created_at -> Timestamp,
    }
}

diesel::table! {
    referential_services (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 100]
        building -> Nullable<Varchar>,
        #[max_length = 50]
        floor -> Nullable<Varchar>,
        is_active -> Bool,
        created_at -> Timestamp,
        #[max_length = 100]
        site_id -> Varchar,
        #[max_length = 255]
        site_name -> Varchar,
        #[max_length = 100]
        building_id -> Varchar,
        #[max_length = 255]
        building_name -> Varchar,
        #[max_length = 100]
        level_id -> Varchar,
        #[max_length = 255]
        level_name -> Varchar,
        #[max_length = 100]
        zone_id -> Varchar,
        #[max_length = 255]
        zone_name -> Varchar,
        #[max_length = 100]
        subzone_id -> Nullable<Varchar>,
        #[max_length = 255]
        subzone_name -> Nullable<Varchar>,
    }
}

diesel::table! {
    referential_specimens (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        label -> Varchar,
        is_active -> Bool,
        created_at -> Timestamp,
    }
}

diesel::table! {
    referential_transport_modes (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        label -> Varchar,
        is_active -> Bool,
        sort_order -> Int4,
    }
}

diesel::table! {
    services (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 100]
        building -> Varchar,
        #[max_length = 50]
        floor -> Varchar,
        #[max_length = 500]
        full_name -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    ticket_assignments (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        ticket_id -> Varchar,
        #[max_length = 255]
        porter_id -> Varchar,
        #[max_length = 20]
        role -> Varchar,
        assigned_at -> Timestamp,
        removed_at -> Nullable<Timestamp>,
        is_active -> Bool,
    }
}

diesel::table! {
    tickets (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        patient_id -> Varchar,
        #[max_length = 255]
        patient_name -> Varchar,
        #[max_length = 255]
        origin -> Varchar,
        #[max_length = 255]
        destination -> Varchar,
        priority -> Int4,
        #[max_length = 50]
        mode -> Varchar,
        #[max_length = 50]
        status -> Varchar,
        #[max_length = 255]
        porter_id -> Nullable<Varchar>,
        #[max_length = 255]
        requester_id -> Varchar,
        needs_o2 -> Bool,
        needs_perfusion -> Bool,
        isolation -> Bool,
        patient_weight -> Nullable<Int4>,
        patient_agitated -> Bool,
        patient_monitoring -> Bool,
        needs_two_porters -> Bool,
        notes -> Nullable<Text>,
        scheduled_time -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        completed_at -> Nullable<Timestamp>,
        #[max_length = 50]
        transport_type -> Varchar,
        #[max_length = 50]
        transport_subtype -> Varchar,
        #[max_length = 255]
        equipment_recipient_patient_id -> Nullable<Varchar>,
        #[max_length = 255]
        equipment_recipient_patient_name -> Nullable<Varchar>,
        #[max_length = 10]
        equipment_size -> Nullable<Varchar>,
        #[max_length = 255]
        equipment_return_service -> Nullable<Varchar>,
        equipment_delivered -> Nullable<Bool>,
        equipment_label_returned -> Nullable<Bool>,
        #[max_length = 50]
        laboratory_name -> Nullable<Varchar>,
        specimen_types -> Nullable<Array<Nullable<Text>>>,
        notes_for_reception -> Nullable<Text>,
        help_requested -> Nullable<Bool>,
        #[max_length = 255]
        help_porter_id -> Nullable<Varchar>,
        #[max_length = 50]
        help_status -> Nullable<Varchar>,
        help_requested_at -> Nullable<Timestamp>,
        is_archived -> Bool,
        archived_at -> Nullable<Timestamp>,
        #[max_length = 255]
        reservation_locked_by -> Nullable<Varchar>,
        reservation_locked_at -> Nullable<Timestamp>,
        activation_minutes_before -> Nullable<Int4>,
        is_visible_to_porters -> Bool,
        patient_contentious -> Bool,
        patient_confused -> Bool,
        patient_over_120kg -> Bool,
        patient_bariatric -> Bool,
        patient_psychiatry -> Bool,
        patient_dialysis -> Bool,
        patient_icu -> Bool,
        other_precautions -> Nullable<Text>,
        #[max_length = 255]
        patient_first_name -> Nullable<Varchar>,
        #[max_length = 255]
        patient_last_name -> Nullable<Varchar>,
        patient_dob -> Nullable<Date>,
        #[max_length = 10]
        patient_sex -> Nullable<Varchar>,
        #[max_length = 50]
        patient_ipp -> Nullable<Varchar>,
        #[max_length = 50]
        motif -> Nullable<Varchar>,
    }
}

diesel::table! {
    users (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 100]
        username -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        #[max_length = 50]
        role -> Varchar,
        #[max_length = 100]
        first_name -> Varchar,
        #[max_length = 100]
        last_name -> Varchar,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        #[max_length = 255]
        service -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_active -> Bool,
    }
}

diesel::joinable!(audit_logs -> users (user_id));
diesel::joinable!(help_requests -> tickets (ticket_id));
diesel::joinable!(notification_preferences -> users (user_id));
diesel::joinable!(notifications -> tickets (related_ticket_id));
diesel::joinable!(notifications -> users (user_id));
diesel::joinable!(porters -> users (user_id));
diesel::joinable!(ticket_assignments -> porters (porter_id));
diesel::joinable!(ticket_assignments -> tickets (ticket_id));
diesel::joinable!(tickets -> users (requester_id));

diesel::allow_tables_to_appear_in_same_query!(
    audit_logs,
    help_requests,
    notification_preferences,
    notifications,
    patients,
    porters,
    priority_rules_config,
    referential_equipment,
    referential_services,
    referential_specimens,
    referential_transport_modes,
    services,
    ticket_assignments,
    tickets,
    users,
);
