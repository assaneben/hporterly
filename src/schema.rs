diesel::table! {
    use diesel::sql_types::*;
    use diesel::sql_types::Uuid as SqlUuid;

    users (id) {
        id -> SqlUuid,
        username -> Varchar,
        display_name -> Varchar,
        role -> Varchar,
        password_hash -> Text,
        is_active -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel::sql_types::Uuid as SqlUuid;

    transfers (id) {
        id -> SqlUuid,
        origin_location -> Varchar,
        destination_location -> Varchar,
        priority -> Varchar,
        status -> Varchar,
        requested_by -> Varchar,
        assigned_to -> Nullable<Varchar>,
        metadata -> Jsonb,
        requested_at -> Timestamptz,
        updated_at -> Timestamptz,
        archived_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel::sql_types::Uuid as SqlUuid;

    transfer_events (id) {
        id -> SqlUuid,
        transfer_id -> SqlUuid,
        actor -> Varchar,
        action -> Varchar,
        details -> Text,
        created_at -> Timestamptz,
    }
}

diesel::joinable!(transfer_events -> transfers (transfer_id));
diesel::allow_tables_to_appear_in_same_query!(users, transfers, transfer_events);
