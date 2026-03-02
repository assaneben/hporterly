-- Notifications table - Persisted notifications for all users
CREATE TABLE notifications (
    id VARCHAR(255) PRIMARY KEY,
    user_id VARCHAR(255) NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    notification_type VARCHAR(50) NOT NULL CHECK (notification_type IN (
        'ticket_created', 'ticket_assigned', 'ticket_updated', 'ticket_completed',
        'ticket_canceled', 'urgent_ticket', 'help_request_received'
    )),
    title VARCHAR(255) NOT NULL,
    message TEXT NOT NULL,
    priority VARCHAR(20) NOT NULL DEFAULT 'normal' CHECK (priority IN ('low', 'normal', 'high')),
    data JSONB DEFAULT '{}',
    is_read BOOLEAN NOT NULL DEFAULT FALSE,
    related_ticket_id VARCHAR(255) REFERENCES tickets(id) ON DELETE SET NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Notification preferences table - Per-user notification settings
CREATE TABLE notification_preferences (
    id VARCHAR(255) PRIMARY KEY,
    user_id VARCHAR(255) NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    preferences JSONB NOT NULL DEFAULT '{
        "ticket_created": {"enabled": true, "sound": true},
        "ticket_assigned": {"enabled": true, "sound": true},
        "ticket_updated": {"enabled": true, "sound": false},
        "ticket_completed": {"enabled": true, "sound": false},
        "ticket_canceled": {"enabled": true, "sound": true},
        "urgent_ticket": {"enabled": true, "sound": true},
        "help_request_received": {"enabled": true, "sound": true}
    }',
    sound_enabled BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(user_id)
);

-- Indexes for performance
CREATE INDEX idx_notifications_user_id ON notifications(user_id);
CREATE INDEX idx_notifications_user_read ON notifications(user_id, is_read);
CREATE INDEX idx_notifications_created_at ON notifications(created_at);
CREATE INDEX idx_notifications_type ON notifications(notification_type);
CREATE INDEX idx_notification_preferences_user_id ON notification_preferences(user_id);

-- Trigger for updated_at on notification_preferences
CREATE TRIGGER update_notification_preferences_updated_at BEFORE UPDATE ON notification_preferences
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
