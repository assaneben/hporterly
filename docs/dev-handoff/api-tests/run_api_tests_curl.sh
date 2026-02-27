#!/usr/bin/env bash
set -u

# API tests (curl) generated from Postman collection - synthetic demo only
# Usage:
#   1) Edit variables below if needed (tokens/IDs can also be exported in the shell).
#   2) Run: bash ./run_api_tests_curl.sh
# Notes:
#   - This script does not auto-extract tokens/IDs from responses.
#   - Prefer Postman collection for fully chained execution with variable capture.

baseUrl="${baseUrl:-http://127.0.0.1:8080}"
token="${token:-}"
requester_token="${requester_token:-}"
porter_token="${porter_token:-}"
admin_token="${admin_token:-}"
requester_username="${requester_username:-marie.durand}"
requester_password="${requester_password:-password123}"
porter_username="${porter_username:-jean.martin}"
porter_password="${porter_password:-password123}"
admin_username="${admin_username:-admin}"
admin_password="${admin_password:-password123}"
patient_search="${patient_search:-ali}"
ticket_id="${ticket_id:-}"
help_request_id="${help_request_id:-}"
assigned_porter_id="${assigned_porter_id:-jean.martin}"
help_target_porter_id="${help_target_porter_id:-jean.martin}"
notification_id="${notification_id:-}"
user_id="${user_id:-}"
porter_id="${porter_id:-}"
ref_service_id="${ref_service_id:-}"
ref_equipment_id="${ref_equipment_id:-}"
ref_transport_mode_id="${ref_transport_mode_id:-}"
ref_specimen_id="${ref_specimen_id:-}"
new_user_username="${new_user_username:-demo.requester.pm}"
new_user_password="${new_user_password:-password123}"
new_porter_username="${new_porter_username:-porter.demo.pm}"
new_porter_password="${new_porter_password:-password123}"

run_curl() {
  local label="$1"; shift
  echo
  echo "================================================================"
  echo "$label"
  echo "----------------------------------------------------------------"
  curl -sS -i "$@"
  local rc=$?
  echo
  echo "[curl exit=$rc]"
  return 0
}

tmp_json() {
  local f
  f="$(mktemp)"
  cat > "$f"
  echo "$f"
}

cleanup_file() {
  local f="$1"
  if [ -n "${f:-}" ] && [ -f "$f" ]; then rm -f "$f"; fi
}

# === 00 Health ===

# 001. GET /api/health
run_curl '00 Health :: GET /api/health' -X GET "${baseUrl}/api/health"

# 002. GET /api/ready
run_curl '00 Health :: GET /api/ready' -X GET "${baseUrl}/api/ready"

# 003. GET /api/version
run_curl '00 Health :: GET /api/version' -X GET "${baseUrl}/api/version"
# === 01 Auth ===

# 004. POST /api/auth/login (demandeur)
body_file=$(tmp_json <<JSON
{
  "username": "${requester_username}",
  "password": "${requester_password}"
}
JSON
)
run_curl '01 Auth :: POST /api/auth/login (demandeur)' -X POST "${baseUrl}/api/auth/login" -H "Content-Type: application/json" --data-binary @"${body_file}"
cleanup_file "$body_file"

# 005. POST /api/auth/login (brancardier)
body_file=$(tmp_json <<JSON
{
  "username": "${porter_username}",
  "password": "${porter_password}"
}
JSON
)
run_curl '01 Auth :: POST /api/auth/login (brancardier)' -X POST "${baseUrl}/api/auth/login" -H "Content-Type: application/json" --data-binary @"${body_file}"
cleanup_file "$body_file"

# 006. POST /api/auth/login (admin)
body_file=$(tmp_json <<JSON
{
  "username": "${admin_username}",
  "password": "${admin_password}"
}
JSON
)
run_curl '01 Auth :: POST /api/auth/login (admin)' -X POST "${baseUrl}/api/auth/login" -H "Content-Type: application/json" --data-binary @"${body_file}"
cleanup_file "$body_file"

# 007. GET /api/auth/me (admin)
run_curl '01 Auth :: GET /api/auth/me (admin)' -X GET "${baseUrl}/api/auth/me" -H "Authorization: Bearer ${admin_token}"

# 008. POST /api/auth/logout
run_curl '01 Auth :: POST /api/auth/logout' -X POST "${baseUrl}/api/auth/logout" -H "Authorization: Bearer ${admin_token}"
# === 02 Core Lookup ===

# 009. GET /api/services
run_curl '02 Core Lookup :: GET /api/services' -X GET "${baseUrl}/api/services" -H "Authorization: Bearer ${requester_token}"

# 010. GET /api/patients?search
run_curl '02 Core Lookup :: GET /api/patients?search' -X GET "${baseUrl}/api/patients?search=${patient_search}&limit=10" -H "Authorization: Bearer ${requester_token}"

# 011. GET /api/porters
run_curl '02 Core Lookup :: GET /api/porters' -X GET "${baseUrl}/api/porters" -H "Authorization: Bearer ${admin_token}"

# 012. GET /api/porters/available
run_curl '02 Core Lookup :: GET /api/porters/available' -X GET "${baseUrl}/api/porters/available" -H "Authorization: Bearer ${admin_token}"
# === 03 Tickets ===

# 013. GET /api/tickets
run_curl '03 Tickets :: GET /api/tickets' -X GET "${baseUrl}/api/tickets?limit=50" -H "Authorization: Bearer ${admin_token}"

# 014. POST /api/tickets
body_file=$(tmp_json <<JSON
{
  "patient_id": "IPP-100001",
  "patient_name": "Alice Martin",
  "origin": "Unit A",
  "destination": "Imaging",
  "priority": 3,
  "mode": "brancard",
  "notes": "DEMO DATA - SYNTHETIC / NOT REAL",
  "transport_type": "PATIENT",
  "transport_subtype": "TP-BRANC"
}
JSON
)
run_curl '03 Tickets :: POST /api/tickets' -X POST "${baseUrl}/api/tickets" -H "Authorization: Bearer ${requester_token}" -H "Content-Type: application/json" --data-binary @"${body_file}"
cleanup_file "$body_file"

# 015. GET /api/tickets/{id}
run_curl '03 Tickets :: GET /api/tickets/{id}' -X GET "${baseUrl}/api/tickets/${ticket_id}" -H "Authorization: Bearer ${requester_token}"

# 016. GET recommendations
run_curl '03 Tickets :: GET recommendations' -X GET "${baseUrl}/api/tickets/${ticket_id}/recommendations" -H "Authorization: Bearer ${admin_token}"

# 017. POST assign
body_file=$(tmp_json <<JSON
{
  "porter_id": "${assigned_porter_id}"
}
JSON
)
run_curl '03 Tickets :: POST assign' -X POST "${baseUrl}/api/tickets/${ticket_id}/assign" -H "Authorization: Bearer ${admin_token}" -H "Content-Type: application/json" --data-binary @"${body_file}"
cleanup_file "$body_file"

# 018. PATCH status -> in_progress
body_file=$(tmp_json <<JSON
{
  "status": "in_progress",
  "comment": "Demarrage test"
}
JSON
)
run_curl '03 Tickets :: PATCH status -> in_progress' -X PATCH "${baseUrl}/api/tickets/${ticket_id}/status" -H "Authorization: Bearer ${admin_token}" -H "Content-Type: application/json" --data-binary @"${body_file}"
cleanup_file "$body_file"

# 019. PATCH notes
body_file=$(tmp_json <<JSON
{
  "notes": "Note operationnelle fictive"
}
JSON
)
run_curl '03 Tickets :: PATCH notes' -X PATCH "${baseUrl}/api/tickets/${ticket_id}/notes" -H "Authorization: Bearer ${admin_token}" -H "Content-Type: application/json" --data-binary @"${body_file}"
cleanup_file "$body_file"

# 020. PATCH priority
body_file=$(tmp_json <<JSON
{
  "priority": 1,
  "reason": "Escalade operationnelle fictive"
}
JSON
)
run_curl '03 Tickets :: PATCH priority' -X PATCH "${baseUrl}/api/tickets/${ticket_id}/priority" -H "Authorization: Bearer ${admin_token}" -H "Content-Type: application/json" --data-binary @"${body_file}"
cleanup_file "$body_file"

# 021. PATCH equipment-status
body_file=$(tmp_json <<JSON
{
  "delivered": true,
  "label_returned": false
}
JSON
)
run_curl '03 Tickets :: PATCH equipment-status' -X PATCH "${baseUrl}/api/tickets/${ticket_id}/equipment-status" -H "Authorization: Bearer ${admin_token}" -H "Content-Type: application/json" --data-binary @"${body_file}"
cleanup_file "$body_file"

# 022. GET available-porters-for-help
run_curl '03 Tickets :: GET available-porters-for-help' -X GET "${baseUrl}/api/tickets/${ticket_id}/available-porters-for-help" -H "Authorization: Bearer ${admin_token}"

# 023. POST request-help
body_file=$(tmp_json <<JSON
{
  "requested_porter_id": "${help_target_porter_id}"
}
JSON
)
run_curl '03 Tickets :: POST request-help' -X POST "${baseUrl}/api/tickets/${ticket_id}/request-help" -H "Authorization: Bearer ${porter_token}" -H "Content-Type: application/json" --data-binary @"${body_file}"
cleanup_file "$body_file"

# 024. GET my help requests
run_curl '03 Tickets :: GET my help requests' -X GET "${baseUrl}/api/porters/me/help-requests" -H "Authorization: Bearer ${porter_token}"

# 025. POST respond help
body_file=$(tmp_json <<JSON
{
  "accepted": true
}
JSON
)
run_curl '03 Tickets :: POST respond help' -X POST "${baseUrl}/api/help-requests/${help_request_id}/respond" -H "Authorization: Bearer ${porter_token}" -H "Content-Type: application/json" --data-binary @"${body_file}"
cleanup_file "$body_file"

# 026. POST pause
run_curl '03 Tickets :: POST pause' -X POST "${baseUrl}/api/tickets/${ticket_id}/pause" -H "Authorization: Bearer ${admin_token}"

# 027. POST cancel
run_curl '03 Tickets :: POST cancel' -X POST "${baseUrl}/api/tickets/${ticket_id}/cancel" -H "Authorization: Bearer ${admin_token}"

# 028. POST hard-delete
body_file=$(tmp_json <<JSON
{
  "reason": "Suppression de donnee de demo"
}
JSON
)
run_curl '03 Tickets :: POST hard-delete' -X POST "${baseUrl}/api/tickets/${ticket_id}/hard-delete" -H "Authorization: Bearer ${admin_token}" -H "Content-Type: application/json" --data-binary @"${body_file}"
cleanup_file "$body_file"
# === 04 Admin Users & Porters ===

# 029. GET /api/users
run_curl '04 Admin Users & Porters :: GET /api/users' -X GET "${baseUrl}/api/users" -H "Authorization: Bearer ${admin_token}"

# 030. POST /api/users
body_file=$(tmp_json <<JSON
{
  "username": "${new_user_username}",
  "password": "${new_user_password}",
  "role": "demandeur",
  "first_name": "Nora",
  "last_name": "Lefevre",
  "email": null,
  "service": "Unit A"
}
JSON
)
run_curl '04 Admin Users & Porters :: POST /api/users' -X POST "${baseUrl}/api/users" -H "Authorization: Bearer ${admin_token}" -H "Content-Type: application/json" --data-binary @"${body_file}"
cleanup_file "$body_file"

# 031. PUT /api/users/{id}
body_file=$(tmp_json <<JSON
{
  "first_name": "Nora",
  "last_name": "Martin",
  "service": "Unit B",
  "is_active": true
}
JSON
)
run_curl '04 Admin Users & Porters :: PUT /api/users/{id}' -X PUT "${baseUrl}/api/users/${user_id}" -H "Authorization: Bearer ${admin_token}" -H "Content-Type: application/json" --data-binary @"${body_file}"
cleanup_file "$body_file"

# 032. DELETE /api/users/{id}
run_curl '04 Admin Users & Porters :: DELETE /api/users/{id}' -X DELETE "${baseUrl}/api/users/${user_id}" -H "Authorization: Bearer ${admin_token}"

# 033. POST /api/porters
body_file=$(tmp_json <<JSON
{
  "username": "${new_porter_username}",
  "password": "${new_porter_password}",
  "first_name": "Alex",
  "last_name": "Rossi",
  "skills": [
    "O2",
    "LIT"
  ]
}
JSON
)
run_curl '04 Admin Users & Porters :: POST /api/porters' -X POST "${baseUrl}/api/porters" -H "Authorization: Bearer ${admin_token}" -H "Content-Type: application/json" --data-binary @"${body_file}"
cleanup_file "$body_file"

# 034. PATCH /api/porters/{id}/skills
body_file=$(tmp_json <<JSON
{
  "skills": [
    "O2",
    "LIT",
    "URG"
  ]
}
JSON
)
run_curl '04 Admin Users & Porters :: PATCH /api/porters/{id}/skills' -X PATCH "${baseUrl}/api/porters/${porter_id}/skills" -H "Authorization: Bearer ${admin_token}" -H "Content-Type: application/json" --data-binary @"${body_file}"
cleanup_file "$body_file"

# 035. PATCH /api/porters/{id}/status
body_file=$(tmp_json <<JSON
{
  "status": "available",
  "location": "Unit A"
}
JSON
)
run_curl '04 Admin Users & Porters :: PATCH /api/porters/{id}/status' -X PATCH "${baseUrl}/api/porters/${porter_id}/status" -H "Authorization: Bearer ${admin_token}" -H "Content-Type: application/json" --data-binary @"${body_file}"
cleanup_file "$body_file"
# === 05 Notifications ===

# 036. GET /api/notifications
run_curl '05 Notifications :: GET /api/notifications' -X GET "${baseUrl}/api/notifications?limit=20" -H "Authorization: Bearer ${admin_token}"

# 037. GET unread-count
run_curl '05 Notifications :: GET unread-count' -X GET "${baseUrl}/api/notifications/unread-count" -H "Authorization: Bearer ${admin_token}"

# 038. POST mark-read
body_file=$(tmp_json <<JSON
{
  "notification_ids": [
    "${notification_id}"
  ]
}
JSON
)
run_curl '05 Notifications :: POST mark-read' -X POST "${baseUrl}/api/notifications/mark-read" -H "Authorization: Bearer ${admin_token}" -H "Content-Type: application/json" --data-binary @"${body_file}"
cleanup_file "$body_file"

# 039. POST mark-all-read
run_curl '05 Notifications :: POST mark-all-read' -X POST "${baseUrl}/api/notifications/mark-all-read" -H "Authorization: Bearer ${admin_token}"

# 040. GET preferences
run_curl '05 Notifications :: GET preferences' -X GET "${baseUrl}/api/notifications/preferences" -H "Authorization: Bearer ${admin_token}"

# 041. PATCH preferences
body_file=$(tmp_json <<JSON
{
  "sound_enabled": true,
  "preferences": {
    "ticket_created": {
      "enabled": true,
      "sound": true
    },
    "ticket_assigned": {
      "enabled": true,
      "sound": true
    },
    "ticket_updated": {
      "enabled": true,
      "sound": false
    },
    "user_message": {
      "enabled": true,
      "sound": true
    }
  }
}
JSON
)
run_curl '05 Notifications :: PATCH preferences' -X PATCH "${baseUrl}/api/notifications/preferences" -H "Authorization: Bearer ${admin_token}" -H "Content-Type: application/json" --data-binary @"${body_file}"
cleanup_file "$body_file"

# 042. GET message-recipients
run_curl '05 Notifications :: GET message-recipients' -X GET "${baseUrl}/api/notifications/message-recipients" -H "Authorization: Bearer ${admin_token}"

# 043. POST send-message (admins)
body_file=$(tmp_json <<JSON
{
  "message": "Message de test synthetique",
  "channel": "general",
  "target_type": "admins"
}
JSON
)
run_curl '05 Notifications :: POST send-message (admins)' -X POST "${baseUrl}/api/notifications/send-message" -H "Authorization: Bearer ${admin_token}" -H "Content-Type: application/json" --data-binary @"${body_file}"
cleanup_file "$body_file"

# 044. DELETE notification
run_curl '05 Notifications :: DELETE notification' -X DELETE "${baseUrl}/api/notifications/${notification_id}" -H "Authorization: Bearer ${admin_token}"
# === 06 Referentials - Services ===

# 045. GET /api/referentials/services
run_curl '06 Referentials - Services :: GET /api/referentials/services' -X GET "${baseUrl}/api/referentials/services" -H "Authorization: Bearer ${admin_token}"

# 046. GET /api/referentials/services/active
run_curl '06 Referentials - Services :: GET /api/referentials/services/active' -X GET "${baseUrl}/api/referentials/services/active" -H "Authorization: Bearer ${admin_token}"

# 047. POST /api/referentials/services
body_file=$(tmp_json <<JSON
{
  "site_name": "Site Demo",
  "building_name": "BAT-A",
  "level_name": "Niveau 1",
  "zone_name": "Unit A",
  "subzone_name": "R-101"
}
JSON
)
run_curl '06 Referentials - Services :: POST /api/referentials/services' -X POST "${baseUrl}/api/referentials/services" -H "Authorization: Bearer ${admin_token}" -H "Content-Type: application/json" --data-binary @"${body_file}"
cleanup_file "$body_file"

# 048. PUT /api/referentials/services/{id}
body_file=$(tmp_json <<JSON
{
  "zone_name": "Unit A Updated",
  "subzone_name": "R-102",
  "is_active": true
}
JSON
)
run_curl '06 Referentials - Services :: PUT /api/referentials/services/{id}' -X PUT "${baseUrl}/api/referentials/services/${ref_service_id}" -H "Authorization: Bearer ${admin_token}" -H "Content-Type: application/json" --data-binary @"${body_file}"
cleanup_file "$body_file"

# 049. DELETE /api/referentials/services/{id}
run_curl '06 Referentials - Services :: DELETE /api/referentials/services/{id}' -X DELETE "${baseUrl}/api/referentials/services/${ref_service_id}" -H "Authorization: Bearer ${admin_token}"

# 050. DELETE /api/referentials/services/{id}/hard
run_curl '06 Referentials - Services :: DELETE /api/referentials/services/{id}/hard' -X DELETE "${baseUrl}/api/referentials/services/${ref_service_id}/hard" -H "Authorization: Bearer ${admin_token}"
# === 07 Referentials - Equipment ===

# 051. GET /api/referentials/equipment
run_curl '07 Referentials - Equipment :: GET /api/referentials/equipment' -X GET "${baseUrl}/api/referentials/equipment" -H "Authorization: Bearer ${admin_token}"

# 052. GET /api/referentials/equipment/active
run_curl '07 Referentials - Equipment :: GET /api/referentials/equipment/active' -X GET "${baseUrl}/api/referentials/equipment/active" -H "Authorization: Bearer ${admin_token}"

# 053. POST /api/referentials/equipment
body_file=$(tmp_json <<JSON
{
  "label": "Stretcher Standard",
  "sizes": [
    "S",
    "M",
    "L"
  ],
  "required_fields": {
    "recipient": false,
    "priority": true,
    "origin": true,
    "destination": true,
    "note_free": false,
    "scheduled": false
  }
}
JSON
)
run_curl '07 Referentials - Equipment :: POST /api/referentials/equipment' -X POST "${baseUrl}/api/referentials/equipment" -H "Authorization: Bearer ${admin_token}" -H "Content-Type: application/json" --data-binary @"${body_file}"
cleanup_file "$body_file"

# 054. PUT /api/referentials/equipment/{id}
body_file=$(tmp_json <<JSON
{
  "label": "Stretcher Standard Updated",
  "sizes": [
    "M",
    "L"
  ],
  "is_active": true
}
JSON
)
run_curl '07 Referentials - Equipment :: PUT /api/referentials/equipment/{id}' -X PUT "${baseUrl}/api/referentials/equipment/${ref_equipment_id}" -H "Authorization: Bearer ${admin_token}" -H "Content-Type: application/json" --data-binary @"${body_file}"
cleanup_file "$body_file"

# 055. DELETE /api/referentials/equipment/{id}
run_curl '07 Referentials - Equipment :: DELETE /api/referentials/equipment/{id}' -X DELETE "${baseUrl}/api/referentials/equipment/${ref_equipment_id}" -H "Authorization: Bearer ${admin_token}"

# 056. DELETE /api/referentials/equipment/{id}/hard
run_curl '07 Referentials - Equipment :: DELETE /api/referentials/equipment/{id}/hard' -X DELETE "${baseUrl}/api/referentials/equipment/${ref_equipment_id}/hard" -H "Authorization: Bearer ${admin_token}"
# === 08 Referentials - Transport Modes ===

# 057. GET /api/referentials/transport-modes
run_curl '08 Referentials - Transport Modes :: GET /api/referentials/transport-modes' -X GET "${baseUrl}/api/referentials/transport-modes" -H "Authorization: Bearer ${admin_token}"

# 058. GET /api/referentials/transport-modes/active
run_curl '08 Referentials - Transport Modes :: GET /api/referentials/transport-modes/active' -X GET "${baseUrl}/api/referentials/transport-modes/active" -H "Authorization: Bearer ${admin_token}"

# 059. POST /api/referentials/transport-modes
body_file=$(tmp_json <<JSON
{
  "label": "Mode Demo",
  "sort_order": 99
}
JSON
)
run_curl '08 Referentials - Transport Modes :: POST /api/referentials/transport-modes' -X POST "${baseUrl}/api/referentials/transport-modes" -H "Authorization: Bearer ${admin_token}" -H "Content-Type: application/json" --data-binary @"${body_file}"
cleanup_file "$body_file"

# 060. PUT /api/referentials/transport-modes/{id}
body_file=$(tmp_json <<JSON
{
  "label": "Mode Demo Updated",
  "sort_order": 100,
  "is_active": true
}
JSON
)
run_curl '08 Referentials - Transport Modes :: PUT /api/referentials/transport-modes/{id}' -X PUT "${baseUrl}/api/referentials/transport-modes/${ref_transport_mode_id}" -H "Authorization: Bearer ${admin_token}" -H "Content-Type: application/json" --data-binary @"${body_file}"
cleanup_file "$body_file"

# 061. DELETE /api/referentials/transport-modes/{id}
run_curl '08 Referentials - Transport Modes :: DELETE /api/referentials/transport-modes/{id}' -X DELETE "${baseUrl}/api/referentials/transport-modes/${ref_transport_mode_id}" -H "Authorization: Bearer ${admin_token}"

# 062. DELETE /api/referentials/transport-modes/{id}/hard
run_curl '08 Referentials - Transport Modes :: DELETE /api/referentials/transport-modes/{id}/hard' -X DELETE "${baseUrl}/api/referentials/transport-modes/${ref_transport_mode_id}/hard" -H "Authorization: Bearer ${admin_token}"
# === 09 Referentials - Specimens ===

# 063. GET /api/referentials/specimens
run_curl '09 Referentials - Specimens :: GET /api/referentials/specimens' -X GET "${baseUrl}/api/referentials/specimens" -H "Authorization: Bearer ${admin_token}"

# 064. GET /api/referentials/specimens/active
run_curl '09 Referentials - Specimens :: GET /api/referentials/specimens/active' -X GET "${baseUrl}/api/referentials/specimens/active" -H "Authorization: Bearer ${admin_token}"

# 065. POST /api/referentials/specimens
body_file=$(tmp_json <<JSON
{
  "label": "Specimen Demo"
}
JSON
)
run_curl '09 Referentials - Specimens :: POST /api/referentials/specimens' -X POST "${baseUrl}/api/referentials/specimens" -H "Authorization: Bearer ${admin_token}" -H "Content-Type: application/json" --data-binary @"${body_file}"
cleanup_file "$body_file"

# 066. PUT /api/referentials/specimens/{id}
body_file=$(tmp_json <<JSON
{
  "label": "Specimen Demo Updated",
  "is_active": true
}
JSON
)
run_curl '09 Referentials - Specimens :: PUT /api/referentials/specimens/{id}' -X PUT "${baseUrl}/api/referentials/specimens/${ref_specimen_id}" -H "Authorization: Bearer ${admin_token}" -H "Content-Type: application/json" --data-binary @"${body_file}"
cleanup_file "$body_file"

# 067. DELETE /api/referentials/specimens/{id}
run_curl '09 Referentials - Specimens :: DELETE /api/referentials/specimens/{id}' -X DELETE "${baseUrl}/api/referentials/specimens/${ref_specimen_id}" -H "Authorization: Bearer ${admin_token}"

# 068. DELETE /api/referentials/specimens/{id}/hard
run_curl '09 Referentials - Specimens :: DELETE /api/referentials/specimens/{id}/hard' -X DELETE "${baseUrl}/api/referentials/specimens/${ref_specimen_id}/hard" -H "Authorization: Bearer ${admin_token}"
