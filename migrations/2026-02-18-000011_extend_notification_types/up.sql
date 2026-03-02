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
            'help_request_received',
            'user_message',
            'co_partner_added',
            'co_partner_removed',
            'mission_suspended',
            'mission_resumed',
            'rdv_activated',
            'rdv_overdue'
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
        "help_request_received": {"enabled": true, "sound": true},
        "user_message": {"enabled": true, "sound": true},
        "co_partner_added": {"enabled": true, "sound": true},
        "co_partner_removed": {"enabled": true, "sound": true},
        "mission_suspended": {"enabled": true, "sound": true},
        "mission_resumed": {"enabled": true, "sound": false},
        "rdv_activated": {"enabled": true, "sound": true},
        "rdv_overdue": {"enabled": true, "sound": true}
    }'::jsonb;

UPDATE notification_preferences
SET preferences = COALESCE(preferences, '{}'::jsonb) || jsonb_build_object(
    'user_message', COALESCE(preferences->'user_message', '{"enabled": true, "sound": true}'::jsonb),
    'co_partner_added', COALESCE(preferences->'co_partner_added', '{"enabled": true, "sound": true}'::jsonb),
    'co_partner_removed', COALESCE(preferences->'co_partner_removed', '{"enabled": true, "sound": true}'::jsonb),
    'mission_suspended', COALESCE(preferences->'mission_suspended', '{"enabled": true, "sound": true}'::jsonb),
    'mission_resumed', COALESCE(preferences->'mission_resumed', '{"enabled": true, "sound": false}'::jsonb),
    'rdv_activated', COALESCE(preferences->'rdv_activated', '{"enabled": true, "sound": true}'::jsonb),
    'rdv_overdue', COALESCE(preferences->'rdv_overdue', '{"enabled": true, "sound": true}'::jsonb)
);
