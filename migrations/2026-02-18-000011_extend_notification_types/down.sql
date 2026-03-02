ALTER TABLE notifications
    DROP CONSTRAINT IF EXISTS notifications_notification_type_check;

ALTER TABLE notifications
    ADD CONSTRAINT notifications_notification_type_check CHECK (
        notification_type IN (
            'ticket_created',
            'ticket_assigned',
            'ticket_updated',
            'ticket_completed',
            'ticket_canceled',
            'urgent_ticket',
            'help_request_received'
        )
    );

ALTER TABLE notification_preferences
    ALTER COLUMN preferences SET DEFAULT '{
        "ticket_created": {"enabled": true, "sound": true},
        "ticket_assigned": {"enabled": true, "sound": true},
        "ticket_updated": {"enabled": true, "sound": false},
        "ticket_completed": {"enabled": true, "sound": false},
        "ticket_canceled": {"enabled": true, "sound": true},
        "urgent_ticket": {"enabled": true, "sound": true},
        "help_request_received": {"enabled": true, "sound": true}
    }'::jsonb;

UPDATE notification_preferences
SET preferences = COALESCE(preferences, '{}'::jsonb)
    - 'user_message'
    - 'co_partner_added'
    - 'co_partner_removed'
    - 'mission_suspended'
    - 'mission_resumed'
    - 'rdv_activated'
    - 'rdv_overdue';
