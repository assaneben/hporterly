-- CreateTable
CREATE TABLE "users" (
    "id" VARCHAR(255) NOT NULL,
    "username" VARCHAR(100) NOT NULL,
    "password_hash" VARCHAR(255) NOT NULL,
    "role" VARCHAR(50) NOT NULL,
    "first_name" VARCHAR(100) NOT NULL,
    "last_name" VARCHAR(100) NOT NULL,
    "email" VARCHAR(255),
    "service" VARCHAR(255),
    "is_active" BOOLEAN NOT NULL DEFAULT true,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "porters" (
    "id" VARCHAR(255) NOT NULL,
    "user_id" VARCHAR(255) NOT NULL,
    "status" VARCHAR(50) NOT NULL DEFAULT 'available',
    "skills" TEXT[] DEFAULT ARRAY[]::TEXT[],
    "current_location" VARCHAR(255),
    "completed_missions_today" INTEGER NOT NULL DEFAULT 0,
    "total_missions" INTEGER NOT NULL DEFAULT 0,
    "rating" DOUBLE PRECISION NOT NULL DEFAULT 3.0,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "porters_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "tickets" (
    "id" VARCHAR(255) NOT NULL,
    "patient_id" VARCHAR(255) NOT NULL,
    "patient_name" VARCHAR(255) NOT NULL,
    "origin" VARCHAR(255) NOT NULL,
    "destination" VARCHAR(255) NOT NULL,
    "priority" INTEGER NOT NULL DEFAULT 3,
    "mode" VARCHAR(50) NOT NULL DEFAULT 'brancard',
    "status" VARCHAR(50) NOT NULL DEFAULT 'pending',
    "porter_id" VARCHAR(255),
    "requester_id" VARCHAR(255) NOT NULL,
    "needs_o2" BOOLEAN NOT NULL DEFAULT false,
    "needs_perfusion" BOOLEAN NOT NULL DEFAULT false,
    "isolation" BOOLEAN NOT NULL DEFAULT false,
    "patient_weight" INTEGER,
    "patient_agitated" BOOLEAN NOT NULL DEFAULT false,
    "patient_monitoring" BOOLEAN NOT NULL DEFAULT false,
    "needs_two_porters" BOOLEAN NOT NULL DEFAULT false,
    "notes" TEXT,
    "scheduled_time" TIMESTAMP(3),
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL,
    "completed_at" TIMESTAMP(3),
    "transport_type" VARCHAR(50) NOT NULL DEFAULT 'PATIENT',
    "transport_subtype" VARCHAR(50) NOT NULL DEFAULT 'TP-BRANC',
    "equipment_recipient_patient_id" VARCHAR(255),
    "equipment_recipient_patient_name" VARCHAR(255),
    "equipment_size" VARCHAR(10),
    "equipment_return_service" VARCHAR(255),
    "equipment_delivered" BOOLEAN,
    "equipment_label_returned" BOOLEAN,
    "laboratory_name" VARCHAR(50),
    "specimen_types" TEXT[] DEFAULT ARRAY[]::TEXT[],
    "notes_for_reception" TEXT,
    "help_requested" BOOLEAN DEFAULT false,
    "help_porter_id" VARCHAR(255),
    "help_status" VARCHAR(50),
    "help_requested_at" TIMESTAMP(3),
    "is_archived" BOOLEAN NOT NULL DEFAULT false,
    "archived_at" TIMESTAMP(3),
    "reservation_locked_by" VARCHAR(255),
    "reservation_locked_at" TIMESTAMP(3),
    "activation_minutes_before" INTEGER,
    "is_visible_to_porters" BOOLEAN NOT NULL DEFAULT true,
    "patient_contentious" BOOLEAN NOT NULL DEFAULT false,
    "patient_confused" BOOLEAN NOT NULL DEFAULT false,
    "patient_over_120kg" BOOLEAN NOT NULL DEFAULT false,
    "patient_bariatric" BOOLEAN NOT NULL DEFAULT false,
    "patient_psychiatry" BOOLEAN NOT NULL DEFAULT false,
    "patient_dialysis" BOOLEAN NOT NULL DEFAULT false,
    "patient_icu" BOOLEAN NOT NULL DEFAULT false,
    "other_precautions" TEXT,
    "patient_first_name" VARCHAR(255),
    "patient_last_name" VARCHAR(255),
    "patient_dob" DATE,
    "patient_sex" VARCHAR(10),
    "patient_ipp" VARCHAR(50),
    "motif" VARCHAR(50),

    CONSTRAINT "tickets_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "ticket_assignments" (
    "id" VARCHAR(255) NOT NULL,
    "ticket_id" VARCHAR(255) NOT NULL,
    "porter_id" VARCHAR(255) NOT NULL,
    "role" VARCHAR(20) NOT NULL DEFAULT 'primary',
    "assigned_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "removed_at" TIMESTAMP(3),
    "is_active" BOOLEAN NOT NULL DEFAULT true,

    CONSTRAINT "ticket_assignments_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "notifications" (
    "id" VARCHAR(255) NOT NULL,
    "user_id" VARCHAR(255) NOT NULL,
    "notification_type" VARCHAR(50) NOT NULL,
    "title" VARCHAR(255) NOT NULL,
    "message" TEXT NOT NULL,
    "priority" VARCHAR(20) NOT NULL DEFAULT 'normal',
    "data" JSONB,
    "is_read" BOOLEAN NOT NULL DEFAULT false,
    "related_ticket_id" VARCHAR(255),
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "notifications_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "notification_preferences" (
    "id" VARCHAR(255) NOT NULL,
    "user_id" VARCHAR(255) NOT NULL,
    "preferences" JSONB NOT NULL DEFAULT '{}',
    "sound_enabled" BOOLEAN NOT NULL DEFAULT true,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "notification_preferences_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "patients" (
    "id" VARCHAR(50) NOT NULL,
    "first_name" VARCHAR(100) NOT NULL,
    "last_name" VARCHAR(100) NOT NULL,
    "age" INTEGER,
    "gender" CHAR(1),
    "service" VARCHAR(100),
    "room" VARCHAR(50),
    "building" VARCHAR(50),
    "floor" VARCHAR(50),
    "date_of_birth" DATE,
    "sex" VARCHAR(10),
    "created_at" TIMESTAMP(3) DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3),

    CONSTRAINT "patients_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "help_requests" (
    "id" VARCHAR(255) NOT NULL,
    "ticket_id" VARCHAR(255) NOT NULL,
    "requesting_porter_id" VARCHAR(255) NOT NULL,
    "requested_porter_id" VARCHAR(255) NOT NULL,
    "status" VARCHAR(50) NOT NULL DEFAULT 'pending',
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "responded_at" TIMESTAMP(3),

    CONSTRAINT "help_requests_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "audit_logs" (
    "id" VARCHAR(255) NOT NULL,
    "user_id" VARCHAR(255) NOT NULL,
    "action" VARCHAR(100) NOT NULL,
    "entity_type" VARCHAR(50) NOT NULL,
    "entity_id" VARCHAR(255) NOT NULL,
    "old_value" JSONB,
    "new_value" JSONB,
    "ip_address" VARCHAR(45),
    "user_agent" TEXT,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "audit_logs_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "priority_rules_config" (
    "id" VARCHAR(255) NOT NULL,
    "rules_json" JSONB NOT NULL,
    "is_active" BOOLEAN NOT NULL DEFAULT true,
    "updated_by" VARCHAR(255),
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "priority_rules_config_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "referential_services" (
    "id" VARCHAR(255) NOT NULL,
    "name" VARCHAR(255) NOT NULL,
    "building" VARCHAR(100),
    "floor" VARCHAR(50),
    "is_active" BOOLEAN NOT NULL DEFAULT true,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "site_id" VARCHAR(100) NOT NULL,
    "site_name" VARCHAR(255) NOT NULL,
    "building_id" VARCHAR(100) NOT NULL,
    "building_name" VARCHAR(255) NOT NULL,
    "level_id" VARCHAR(100) NOT NULL,
    "level_name" VARCHAR(255) NOT NULL,
    "zone_id" VARCHAR(100) NOT NULL,
    "zone_name" VARCHAR(255) NOT NULL,
    "subzone_id" VARCHAR(100),
    "subzone_name" VARCHAR(255),

    CONSTRAINT "referential_services_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "referential_equipment" (
    "id" VARCHAR(255) NOT NULL,
    "label" VARCHAR(255) NOT NULL,
    "sizes" TEXT[] DEFAULT ARRAY[]::TEXT[],
    "required_fields" JSONB NOT NULL,
    "is_active" BOOLEAN NOT NULL DEFAULT true,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "referential_equipment_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "referential_transport_modes" (
    "id" VARCHAR(255) NOT NULL,
    "label" VARCHAR(255) NOT NULL,
    "is_active" BOOLEAN NOT NULL DEFAULT true,
    "sort_order" INTEGER NOT NULL DEFAULT 0,

    CONSTRAINT "referential_transport_modes_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "referential_specimens" (
    "id" VARCHAR(255) NOT NULL,
    "label" VARCHAR(255) NOT NULL,
    "is_active" BOOLEAN NOT NULL DEFAULT true,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "referential_specimens_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "users_username_key" ON "users"("username");

-- CreateIndex
CREATE UNIQUE INDEX "porters_user_id_key" ON "porters"("user_id");

-- CreateIndex
CREATE INDEX "tickets_status_idx" ON "tickets"("status");

-- CreateIndex
CREATE INDEX "tickets_priority_idx" ON "tickets"("priority");

-- CreateIndex
CREATE INDEX "tickets_porter_id_idx" ON "tickets"("porter_id");

-- CreateIndex
CREATE INDEX "tickets_requester_id_idx" ON "tickets"("requester_id");

-- CreateIndex
CREATE INDEX "tickets_transport_type_idx" ON "tickets"("transport_type");

-- CreateIndex
CREATE INDEX "tickets_created_at_idx" ON "tickets"("created_at");

-- CreateIndex
CREATE INDEX "notifications_user_id_is_read_idx" ON "notifications"("user_id", "is_read");

-- CreateIndex
CREATE UNIQUE INDEX "notification_preferences_user_id_key" ON "notification_preferences"("user_id");

-- CreateIndex
CREATE INDEX "audit_logs_entity_type_entity_id_idx" ON "audit_logs"("entity_type", "entity_id");

-- CreateIndex
CREATE INDEX "audit_logs_user_id_idx" ON "audit_logs"("user_id");

-- AddForeignKey
ALTER TABLE "porters" ADD CONSTRAINT "porters_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "tickets" ADD CONSTRAINT "tickets_requester_id_fkey" FOREIGN KEY ("requester_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "ticket_assignments" ADD CONSTRAINT "ticket_assignments_ticket_id_fkey" FOREIGN KEY ("ticket_id") REFERENCES "tickets"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "ticket_assignments" ADD CONSTRAINT "ticket_assignments_porter_id_fkey" FOREIGN KEY ("porter_id") REFERENCES "porters"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "notifications" ADD CONSTRAINT "notifications_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "notifications" ADD CONSTRAINT "notifications_related_ticket_id_fkey" FOREIGN KEY ("related_ticket_id") REFERENCES "tickets"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "notification_preferences" ADD CONSTRAINT "notification_preferences_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "help_requests" ADD CONSTRAINT "help_requests_ticket_id_fkey" FOREIGN KEY ("ticket_id") REFERENCES "tickets"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "audit_logs" ADD CONSTRAINT "audit_logs_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
