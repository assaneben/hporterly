param(
  [switch]$ShowOnly
)

$ErrorActionPreference = 'Stop'

# API tests (curl.exe) generated from Postman collection - synthetic demo only
# This script does not auto-extract tokens/IDs from responses.

function Get-ValueOrDefault {
  param([string]$Name, [string]$Default)
  $v = [Environment]::GetEnvironmentVariable($Name)
  if ([string]::IsNullOrEmpty($v)) { return $Default }
  return $v
}

function Resolve-Template {
  param([string]$Text)
  if ($null -eq $Text) { return $Text }
  return ([regex]::Replace($Text, '\{\{([A-Za-z0-9_]+)\}\}', {
    param($m)
    $name = $m.Groups[1].Value
    if ($script:Vars.ContainsKey($name)) { return [string]$script:Vars[$name] }
    return $m.Value
  }))
}

function Invoke-HCurl {
  param(
    [string]$Label,
    [string]$Method,
    [string]$Url,
    [string[]]$Headers = @(),
    [string]$Body = $null
  )
  Write-Host ''
  Write-Host ('=' * 72)
  Write-Host $Label
  Write-Host ('-' * 72)
  if ($ShowOnly) {
    Write-Host ("$Method $Url")
    foreach ($h in $Headers) { Write-Host ("H: $h") }
    if ($null -ne $Body) { Write-Host $Body }
    return
  }
  $args = @('-sS', '-i', '-X', $Method, $Url)
  foreach ($h in $Headers) { $args += @('-H', $h) }
  if ($null -ne $Body) { $args += @('--data-raw', $Body) }
  & curl.exe @args
  Write-Host ''
}

$script:Vars = [ordered]@{}
$script:Vars['baseUrl'] = Get-ValueOrDefault -Name 'baseUrl' -Default 'http://127.0.0.1:8080'
$script:Vars['token'] = Get-ValueOrDefault -Name 'token' -Default ''
$script:Vars['requester_token'] = Get-ValueOrDefault -Name 'requester_token' -Default ''
$script:Vars['porter_token'] = Get-ValueOrDefault -Name 'porter_token' -Default ''
$script:Vars['admin_token'] = Get-ValueOrDefault -Name 'admin_token' -Default ''
$script:Vars['requester_username'] = Get-ValueOrDefault -Name 'requester_username' -Default 'marie.durand'
$script:Vars['requester_password'] = Get-ValueOrDefault -Name 'requester_password' -Default 'password123'
$script:Vars['porter_username'] = Get-ValueOrDefault -Name 'porter_username' -Default 'jean.martin'
$script:Vars['porter_password'] = Get-ValueOrDefault -Name 'porter_password' -Default 'password123'
$script:Vars['admin_username'] = Get-ValueOrDefault -Name 'admin_username' -Default 'admin'
$script:Vars['admin_password'] = Get-ValueOrDefault -Name 'admin_password' -Default 'password123'
$script:Vars['patient_search'] = Get-ValueOrDefault -Name 'patient_search' -Default 'ali'
$script:Vars['ticket_id'] = Get-ValueOrDefault -Name 'ticket_id' -Default ''
$script:Vars['help_request_id'] = Get-ValueOrDefault -Name 'help_request_id' -Default ''
$script:Vars['assigned_porter_id'] = Get-ValueOrDefault -Name 'assigned_porter_id' -Default 'jean.martin'
$script:Vars['help_target_porter_id'] = Get-ValueOrDefault -Name 'help_target_porter_id' -Default 'jean.martin'
$script:Vars['notification_id'] = Get-ValueOrDefault -Name 'notification_id' -Default ''
$script:Vars['user_id'] = Get-ValueOrDefault -Name 'user_id' -Default ''
$script:Vars['porter_id'] = Get-ValueOrDefault -Name 'porter_id' -Default ''
$script:Vars['ref_service_id'] = Get-ValueOrDefault -Name 'ref_service_id' -Default ''
$script:Vars['ref_equipment_id'] = Get-ValueOrDefault -Name 'ref_equipment_id' -Default ''
$script:Vars['ref_transport_mode_id'] = Get-ValueOrDefault -Name 'ref_transport_mode_id' -Default ''
$script:Vars['ref_specimen_id'] = Get-ValueOrDefault -Name 'ref_specimen_id' -Default ''
$script:Vars['new_user_username'] = Get-ValueOrDefault -Name 'new_user_username' -Default 'demo.requester.pm'
$script:Vars['new_user_password'] = Get-ValueOrDefault -Name 'new_user_password' -Default 'password123'
$script:Vars['new_porter_username'] = Get-ValueOrDefault -Name 'new_porter_username' -Default 'porter.demo.pm'
$script:Vars['new_porter_password'] = Get-ValueOrDefault -Name 'new_porter_password' -Default 'password123'

# === 00 Health ===
# 001. GET /api/health
$url = Resolve-Template '{{baseUrl}}/api/health'
$headers = @()
Invoke-HCurl -Label '00 Health :: GET /api/health' -Method 'GET' -Url $url -Headers $headers

# 002. GET /api/ready
$url = Resolve-Template '{{baseUrl}}/api/ready'
$headers = @()
Invoke-HCurl -Label '00 Health :: GET /api/ready' -Method 'GET' -Url $url -Headers $headers

# 003. GET /api/version
$url = Resolve-Template '{{baseUrl}}/api/version'
$headers = @()
Invoke-HCurl -Label '00 Health :: GET /api/version' -Method 'GET' -Url $url -Headers $headers

# === 01 Auth ===
# 004. POST /api/auth/login (demandeur)
$url = Resolve-Template '{{baseUrl}}/api/auth/login'
$headers = @()
$headers += (Resolve-Template 'Content-Type: application/json')
$body = @'
{
  "username": "{{requester_username}}",
  "password": "{{requester_password}}"
}
'@
$body = Resolve-Template $body
Invoke-HCurl -Label '01 Auth :: POST /api/auth/login (demandeur)' -Method 'POST' -Url $url -Headers $headers -Body $body

# 005. POST /api/auth/login (brancardier)
$url = Resolve-Template '{{baseUrl}}/api/auth/login'
$headers = @()
$headers += (Resolve-Template 'Content-Type: application/json')
$body = @'
{
  "username": "{{porter_username}}",
  "password": "{{porter_password}}"
}
'@
$body = Resolve-Template $body
Invoke-HCurl -Label '01 Auth :: POST /api/auth/login (brancardier)' -Method 'POST' -Url $url -Headers $headers -Body $body

# 006. POST /api/auth/login (admin)
$url = Resolve-Template '{{baseUrl}}/api/auth/login'
$headers = @()
$headers += (Resolve-Template 'Content-Type: application/json')
$body = @'
{
  "username": "{{admin_username}}",
  "password": "{{admin_password}}"
}
'@
$body = Resolve-Template $body
Invoke-HCurl -Label '01 Auth :: POST /api/auth/login (admin)' -Method 'POST' -Url $url -Headers $headers -Body $body

# 007. GET /api/auth/me (admin)
$url = Resolve-Template '{{baseUrl}}/api/auth/me'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
Invoke-HCurl -Label '01 Auth :: GET /api/auth/me (admin)' -Method 'GET' -Url $url -Headers $headers

# 008. POST /api/auth/logout
$url = Resolve-Template '{{baseUrl}}/api/auth/logout'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
Invoke-HCurl -Label '01 Auth :: POST /api/auth/logout' -Method 'POST' -Url $url -Headers $headers

# === 02 Core Lookup ===
# 009. GET /api/services
$url = Resolve-Template '{{baseUrl}}/api/services'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{requester_token}}')
Invoke-HCurl -Label '02 Core Lookup :: GET /api/services' -Method 'GET' -Url $url -Headers $headers

# 010. GET /api/patients?search
$url = Resolve-Template '{{baseUrl}}/api/patients?search={{patient_search}}&limit=10'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{requester_token}}')
Invoke-HCurl -Label '02 Core Lookup :: GET /api/patients?search' -Method 'GET' -Url $url -Headers $headers

# 011. GET /api/porters
$url = Resolve-Template '{{baseUrl}}/api/porters'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
Invoke-HCurl -Label '02 Core Lookup :: GET /api/porters' -Method 'GET' -Url $url -Headers $headers

# 012. GET /api/porters/available
$url = Resolve-Template '{{baseUrl}}/api/porters/available'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
Invoke-HCurl -Label '02 Core Lookup :: GET /api/porters/available' -Method 'GET' -Url $url -Headers $headers

# === 03 Tickets ===
# 013. GET /api/tickets
$url = Resolve-Template '{{baseUrl}}/api/tickets?limit=50'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
Invoke-HCurl -Label '03 Tickets :: GET /api/tickets' -Method 'GET' -Url $url -Headers $headers

# 014. POST /api/tickets
$url = Resolve-Template '{{baseUrl}}/api/tickets'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{requester_token}}')
$headers += (Resolve-Template 'Content-Type: application/json')
$body = @'
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
'@
$body = Resolve-Template $body
Invoke-HCurl -Label '03 Tickets :: POST /api/tickets' -Method 'POST' -Url $url -Headers $headers -Body $body

# 015. GET /api/tickets/{id}
$url = Resolve-Template '{{baseUrl}}/api/tickets/{{ticket_id}}'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{requester_token}}')
Invoke-HCurl -Label '03 Tickets :: GET /api/tickets/{id}' -Method 'GET' -Url $url -Headers $headers

# 016. GET recommendations
$url = Resolve-Template '{{baseUrl}}/api/tickets/{{ticket_id}}/recommendations'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
Invoke-HCurl -Label '03 Tickets :: GET recommendations' -Method 'GET' -Url $url -Headers $headers

# 017. POST assign
$url = Resolve-Template '{{baseUrl}}/api/tickets/{{ticket_id}}/assign'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
$headers += (Resolve-Template 'Content-Type: application/json')
$body = @'
{
  "porter_id": "{{assigned_porter_id}}"
}
'@
$body = Resolve-Template $body
Invoke-HCurl -Label '03 Tickets :: POST assign' -Method 'POST' -Url $url -Headers $headers -Body $body

# 018. PATCH status -> in_progress
$url = Resolve-Template '{{baseUrl}}/api/tickets/{{ticket_id}}/status'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
$headers += (Resolve-Template 'Content-Type: application/json')
$body = @'
{
  "status": "in_progress",
  "comment": "Demarrage test"
}
'@
$body = Resolve-Template $body
Invoke-HCurl -Label '03 Tickets :: PATCH status -> in_progress' -Method 'PATCH' -Url $url -Headers $headers -Body $body

# 019. PATCH notes
$url = Resolve-Template '{{baseUrl}}/api/tickets/{{ticket_id}}/notes'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
$headers += (Resolve-Template 'Content-Type: application/json')
$body = @'
{
  "notes": "Note operationnelle fictive"
}
'@
$body = Resolve-Template $body
Invoke-HCurl -Label '03 Tickets :: PATCH notes' -Method 'PATCH' -Url $url -Headers $headers -Body $body

# 020. PATCH priority
$url = Resolve-Template '{{baseUrl}}/api/tickets/{{ticket_id}}/priority'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
$headers += (Resolve-Template 'Content-Type: application/json')
$body = @'
{
  "priority": 1,
  "reason": "Escalade operationnelle fictive"
}
'@
$body = Resolve-Template $body
Invoke-HCurl -Label '03 Tickets :: PATCH priority' -Method 'PATCH' -Url $url -Headers $headers -Body $body

# 021. PATCH equipment-status
$url = Resolve-Template '{{baseUrl}}/api/tickets/{{ticket_id}}/equipment-status'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
$headers += (Resolve-Template 'Content-Type: application/json')
$body = @'
{
  "delivered": true,
  "label_returned": false
}
'@
$body = Resolve-Template $body
Invoke-HCurl -Label '03 Tickets :: PATCH equipment-status' -Method 'PATCH' -Url $url -Headers $headers -Body $body

# 022. GET available-porters-for-help
$url = Resolve-Template '{{baseUrl}}/api/tickets/{{ticket_id}}/available-porters-for-help'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
Invoke-HCurl -Label '03 Tickets :: GET available-porters-for-help' -Method 'GET' -Url $url -Headers $headers

# 023. POST request-help
$url = Resolve-Template '{{baseUrl}}/api/tickets/{{ticket_id}}/request-help'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{porter_token}}')
$headers += (Resolve-Template 'Content-Type: application/json')
$body = @'
{
  "requested_porter_id": "{{help_target_porter_id}}"
}
'@
$body = Resolve-Template $body
Invoke-HCurl -Label '03 Tickets :: POST request-help' -Method 'POST' -Url $url -Headers $headers -Body $body

# 024. GET my help requests
$url = Resolve-Template '{{baseUrl}}/api/porters/me/help-requests'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{porter_token}}')
Invoke-HCurl -Label '03 Tickets :: GET my help requests' -Method 'GET' -Url $url -Headers $headers

# 025. POST respond help
$url = Resolve-Template '{{baseUrl}}/api/help-requests/{{help_request_id}}/respond'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{porter_token}}')
$headers += (Resolve-Template 'Content-Type: application/json')
$body = @'
{
  "accepted": true
}
'@
$body = Resolve-Template $body
Invoke-HCurl -Label '03 Tickets :: POST respond help' -Method 'POST' -Url $url -Headers $headers -Body $body

# 026. POST pause
$url = Resolve-Template '{{baseUrl}}/api/tickets/{{ticket_id}}/pause'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
Invoke-HCurl -Label '03 Tickets :: POST pause' -Method 'POST' -Url $url -Headers $headers

# 027. POST cancel
$url = Resolve-Template '{{baseUrl}}/api/tickets/{{ticket_id}}/cancel'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
Invoke-HCurl -Label '03 Tickets :: POST cancel' -Method 'POST' -Url $url -Headers $headers

# 028. POST hard-delete
$url = Resolve-Template '{{baseUrl}}/api/tickets/{{ticket_id}}/hard-delete'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
$headers += (Resolve-Template 'Content-Type: application/json')
$body = @'
{
  "reason": "Suppression de donnee de demo"
}
'@
$body = Resolve-Template $body
Invoke-HCurl -Label '03 Tickets :: POST hard-delete' -Method 'POST' -Url $url -Headers $headers -Body $body

# === 04 Admin Users & Porters ===
# 029. GET /api/users
$url = Resolve-Template '{{baseUrl}}/api/users'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
Invoke-HCurl -Label '04 Admin Users & Porters :: GET /api/users' -Method 'GET' -Url $url -Headers $headers

# 030. POST /api/users
$url = Resolve-Template '{{baseUrl}}/api/users'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
$headers += (Resolve-Template 'Content-Type: application/json')
$body = @'
{
  "username": "{{new_user_username}}",
  "password": "{{new_user_password}}",
  "role": "demandeur",
  "first_name": "Nora",
  "last_name": "Lefevre",
  "email": null,
  "service": "Unit A"
}
'@
$body = Resolve-Template $body
Invoke-HCurl -Label '04 Admin Users & Porters :: POST /api/users' -Method 'POST' -Url $url -Headers $headers -Body $body

# 031. PUT /api/users/{id}
$url = Resolve-Template '{{baseUrl}}/api/users/{{user_id}}'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
$headers += (Resolve-Template 'Content-Type: application/json')
$body = @'
{
  "first_name": "Nora",
  "last_name": "Martin",
  "service": "Unit B",
  "is_active": true
}
'@
$body = Resolve-Template $body
Invoke-HCurl -Label '04 Admin Users & Porters :: PUT /api/users/{id}' -Method 'PUT' -Url $url -Headers $headers -Body $body

# 032. DELETE /api/users/{id}
$url = Resolve-Template '{{baseUrl}}/api/users/{{user_id}}'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
Invoke-HCurl -Label '04 Admin Users & Porters :: DELETE /api/users/{id}' -Method 'DELETE' -Url $url -Headers $headers

# 033. POST /api/porters
$url = Resolve-Template '{{baseUrl}}/api/porters'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
$headers += (Resolve-Template 'Content-Type: application/json')
$body = @'
{
  "username": "{{new_porter_username}}",
  "password": "{{new_porter_password}}",
  "first_name": "Alex",
  "last_name": "Rossi",
  "skills": [
    "O2",
    "LIT"
  ]
}
'@
$body = Resolve-Template $body
Invoke-HCurl -Label '04 Admin Users & Porters :: POST /api/porters' -Method 'POST' -Url $url -Headers $headers -Body $body

# 034. PATCH /api/porters/{id}/skills
$url = Resolve-Template '{{baseUrl}}/api/porters/{{porter_id}}/skills'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
$headers += (Resolve-Template 'Content-Type: application/json')
$body = @'
{
  "skills": [
    "O2",
    "LIT",
    "URG"
  ]
}
'@
$body = Resolve-Template $body
Invoke-HCurl -Label '04 Admin Users & Porters :: PATCH /api/porters/{id}/skills' -Method 'PATCH' -Url $url -Headers $headers -Body $body

# 035. PATCH /api/porters/{id}/status
$url = Resolve-Template '{{baseUrl}}/api/porters/{{porter_id}}/status'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
$headers += (Resolve-Template 'Content-Type: application/json')
$body = @'
{
  "status": "available",
  "location": "Unit A"
}
'@
$body = Resolve-Template $body
Invoke-HCurl -Label '04 Admin Users & Porters :: PATCH /api/porters/{id}/status' -Method 'PATCH' -Url $url -Headers $headers -Body $body

# === 05 Notifications ===
# 036. GET /api/notifications
$url = Resolve-Template '{{baseUrl}}/api/notifications?limit=20'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
Invoke-HCurl -Label '05 Notifications :: GET /api/notifications' -Method 'GET' -Url $url -Headers $headers

# 037. GET unread-count
$url = Resolve-Template '{{baseUrl}}/api/notifications/unread-count'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
Invoke-HCurl -Label '05 Notifications :: GET unread-count' -Method 'GET' -Url $url -Headers $headers

# 038. POST mark-read
$url = Resolve-Template '{{baseUrl}}/api/notifications/mark-read'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
$headers += (Resolve-Template 'Content-Type: application/json')
$body = @'
{
  "notification_ids": [
    "{{notification_id}}"
  ]
}
'@
$body = Resolve-Template $body
Invoke-HCurl -Label '05 Notifications :: POST mark-read' -Method 'POST' -Url $url -Headers $headers -Body $body

# 039. POST mark-all-read
$url = Resolve-Template '{{baseUrl}}/api/notifications/mark-all-read'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
Invoke-HCurl -Label '05 Notifications :: POST mark-all-read' -Method 'POST' -Url $url -Headers $headers

# 040. GET preferences
$url = Resolve-Template '{{baseUrl}}/api/notifications/preferences'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
Invoke-HCurl -Label '05 Notifications :: GET preferences' -Method 'GET' -Url $url -Headers $headers

# 041. PATCH preferences
$url = Resolve-Template '{{baseUrl}}/api/notifications/preferences'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
$headers += (Resolve-Template 'Content-Type: application/json')
$body = @'
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
'@
$body = Resolve-Template $body
Invoke-HCurl -Label '05 Notifications :: PATCH preferences' -Method 'PATCH' -Url $url -Headers $headers -Body $body

# 042. GET message-recipients
$url = Resolve-Template '{{baseUrl}}/api/notifications/message-recipients'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
Invoke-HCurl -Label '05 Notifications :: GET message-recipients' -Method 'GET' -Url $url -Headers $headers

# 043. POST send-message (admins)
$url = Resolve-Template '{{baseUrl}}/api/notifications/send-message'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
$headers += (Resolve-Template 'Content-Type: application/json')
$body = @'
{
  "message": "Message de test synthetique",
  "channel": "general",
  "target_type": "admins"
}
'@
$body = Resolve-Template $body
Invoke-HCurl -Label '05 Notifications :: POST send-message (admins)' -Method 'POST' -Url $url -Headers $headers -Body $body

# 044. DELETE notification
$url = Resolve-Template '{{baseUrl}}/api/notifications/{{notification_id}}'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
Invoke-HCurl -Label '05 Notifications :: DELETE notification' -Method 'DELETE' -Url $url -Headers $headers

# === 06 Referentials - Services ===
# 045. GET /api/referentials/services
$url = Resolve-Template '{{baseUrl}}/api/referentials/services'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
Invoke-HCurl -Label '06 Referentials - Services :: GET /api/referentials/services' -Method 'GET' -Url $url -Headers $headers

# 046. GET /api/referentials/services/active
$url = Resolve-Template '{{baseUrl}}/api/referentials/services/active'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
Invoke-HCurl -Label '06 Referentials - Services :: GET /api/referentials/services/active' -Method 'GET' -Url $url -Headers $headers

# 047. POST /api/referentials/services
$url = Resolve-Template '{{baseUrl}}/api/referentials/services'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
$headers += (Resolve-Template 'Content-Type: application/json')
$body = @'
{
  "site_name": "Site Demo",
  "building_name": "BAT-A",
  "level_name": "Niveau 1",
  "zone_name": "Unit A",
  "subzone_name": "R-101"
}
'@
$body = Resolve-Template $body
Invoke-HCurl -Label '06 Referentials - Services :: POST /api/referentials/services' -Method 'POST' -Url $url -Headers $headers -Body $body

# 048. PUT /api/referentials/services/{id}
$url = Resolve-Template '{{baseUrl}}/api/referentials/services/{{ref_service_id}}'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
$headers += (Resolve-Template 'Content-Type: application/json')
$body = @'
{
  "zone_name": "Unit A Updated",
  "subzone_name": "R-102",
  "is_active": true
}
'@
$body = Resolve-Template $body
Invoke-HCurl -Label '06 Referentials - Services :: PUT /api/referentials/services/{id}' -Method 'PUT' -Url $url -Headers $headers -Body $body

# 049. DELETE /api/referentials/services/{id}
$url = Resolve-Template '{{baseUrl}}/api/referentials/services/{{ref_service_id}}'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
Invoke-HCurl -Label '06 Referentials - Services :: DELETE /api/referentials/services/{id}' -Method 'DELETE' -Url $url -Headers $headers

# 050. DELETE /api/referentials/services/{id}/hard
$url = Resolve-Template '{{baseUrl}}/api/referentials/services/{{ref_service_id}}/hard'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
Invoke-HCurl -Label '06 Referentials - Services :: DELETE /api/referentials/services/{id}/hard' -Method 'DELETE' -Url $url -Headers $headers

# === 07 Referentials - Equipment ===
# 051. GET /api/referentials/equipment
$url = Resolve-Template '{{baseUrl}}/api/referentials/equipment'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
Invoke-HCurl -Label '07 Referentials - Equipment :: GET /api/referentials/equipment' -Method 'GET' -Url $url -Headers $headers

# 052. GET /api/referentials/equipment/active
$url = Resolve-Template '{{baseUrl}}/api/referentials/equipment/active'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
Invoke-HCurl -Label '07 Referentials - Equipment :: GET /api/referentials/equipment/active' -Method 'GET' -Url $url -Headers $headers

# 053. POST /api/referentials/equipment
$url = Resolve-Template '{{baseUrl}}/api/referentials/equipment'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
$headers += (Resolve-Template 'Content-Type: application/json')
$body = @'
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
'@
$body = Resolve-Template $body
Invoke-HCurl -Label '07 Referentials - Equipment :: POST /api/referentials/equipment' -Method 'POST' -Url $url -Headers $headers -Body $body

# 054. PUT /api/referentials/equipment/{id}
$url = Resolve-Template '{{baseUrl}}/api/referentials/equipment/{{ref_equipment_id}}'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
$headers += (Resolve-Template 'Content-Type: application/json')
$body = @'
{
  "label": "Stretcher Standard Updated",
  "sizes": [
    "M",
    "L"
  ],
  "is_active": true
}
'@
$body = Resolve-Template $body
Invoke-HCurl -Label '07 Referentials - Equipment :: PUT /api/referentials/equipment/{id}' -Method 'PUT' -Url $url -Headers $headers -Body $body

# 055. DELETE /api/referentials/equipment/{id}
$url = Resolve-Template '{{baseUrl}}/api/referentials/equipment/{{ref_equipment_id}}'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
Invoke-HCurl -Label '07 Referentials - Equipment :: DELETE /api/referentials/equipment/{id}' -Method 'DELETE' -Url $url -Headers $headers

# 056. DELETE /api/referentials/equipment/{id}/hard
$url = Resolve-Template '{{baseUrl}}/api/referentials/equipment/{{ref_equipment_id}}/hard'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
Invoke-HCurl -Label '07 Referentials - Equipment :: DELETE /api/referentials/equipment/{id}/hard' -Method 'DELETE' -Url $url -Headers $headers

# === 08 Referentials - Transport Modes ===
# 057. GET /api/referentials/transport-modes
$url = Resolve-Template '{{baseUrl}}/api/referentials/transport-modes'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
Invoke-HCurl -Label '08 Referentials - Transport Modes :: GET /api/referentials/transport-modes' -Method 'GET' -Url $url -Headers $headers

# 058. GET /api/referentials/transport-modes/active
$url = Resolve-Template '{{baseUrl}}/api/referentials/transport-modes/active'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
Invoke-HCurl -Label '08 Referentials - Transport Modes :: GET /api/referentials/transport-modes/active' -Method 'GET' -Url $url -Headers $headers

# 059. POST /api/referentials/transport-modes
$url = Resolve-Template '{{baseUrl}}/api/referentials/transport-modes'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
$headers += (Resolve-Template 'Content-Type: application/json')
$body = @'
{
  "label": "Mode Demo",
  "sort_order": 99
}
'@
$body = Resolve-Template $body
Invoke-HCurl -Label '08 Referentials - Transport Modes :: POST /api/referentials/transport-modes' -Method 'POST' -Url $url -Headers $headers -Body $body

# 060. PUT /api/referentials/transport-modes/{id}
$url = Resolve-Template '{{baseUrl}}/api/referentials/transport-modes/{{ref_transport_mode_id}}'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
$headers += (Resolve-Template 'Content-Type: application/json')
$body = @'
{
  "label": "Mode Demo Updated",
  "sort_order": 100,
  "is_active": true
}
'@
$body = Resolve-Template $body
Invoke-HCurl -Label '08 Referentials - Transport Modes :: PUT /api/referentials/transport-modes/{id}' -Method 'PUT' -Url $url -Headers $headers -Body $body

# 061. DELETE /api/referentials/transport-modes/{id}
$url = Resolve-Template '{{baseUrl}}/api/referentials/transport-modes/{{ref_transport_mode_id}}'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
Invoke-HCurl -Label '08 Referentials - Transport Modes :: DELETE /api/referentials/transport-modes/{id}' -Method 'DELETE' -Url $url -Headers $headers

# 062. DELETE /api/referentials/transport-modes/{id}/hard
$url = Resolve-Template '{{baseUrl}}/api/referentials/transport-modes/{{ref_transport_mode_id}}/hard'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
Invoke-HCurl -Label '08 Referentials - Transport Modes :: DELETE /api/referentials/transport-modes/{id}/hard' -Method 'DELETE' -Url $url -Headers $headers

# === 09 Referentials - Specimens ===
# 063. GET /api/referentials/specimens
$url = Resolve-Template '{{baseUrl}}/api/referentials/specimens'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
Invoke-HCurl -Label '09 Referentials - Specimens :: GET /api/referentials/specimens' -Method 'GET' -Url $url -Headers $headers

# 064. GET /api/referentials/specimens/active
$url = Resolve-Template '{{baseUrl}}/api/referentials/specimens/active'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
Invoke-HCurl -Label '09 Referentials - Specimens :: GET /api/referentials/specimens/active' -Method 'GET' -Url $url -Headers $headers

# 065. POST /api/referentials/specimens
$url = Resolve-Template '{{baseUrl}}/api/referentials/specimens'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
$headers += (Resolve-Template 'Content-Type: application/json')
$body = @'
{
  "label": "Specimen Demo"
}
'@
$body = Resolve-Template $body
Invoke-HCurl -Label '09 Referentials - Specimens :: POST /api/referentials/specimens' -Method 'POST' -Url $url -Headers $headers -Body $body

# 066. PUT /api/referentials/specimens/{id}
$url = Resolve-Template '{{baseUrl}}/api/referentials/specimens/{{ref_specimen_id}}'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
$headers += (Resolve-Template 'Content-Type: application/json')
$body = @'
{
  "label": "Specimen Demo Updated",
  "is_active": true
}
'@
$body = Resolve-Template $body
Invoke-HCurl -Label '09 Referentials - Specimens :: PUT /api/referentials/specimens/{id}' -Method 'PUT' -Url $url -Headers $headers -Body $body

# 067. DELETE /api/referentials/specimens/{id}
$url = Resolve-Template '{{baseUrl}}/api/referentials/specimens/{{ref_specimen_id}}'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
Invoke-HCurl -Label '09 Referentials - Specimens :: DELETE /api/referentials/specimens/{id}' -Method 'DELETE' -Url $url -Headers $headers

# 068. DELETE /api/referentials/specimens/{id}/hard
$url = Resolve-Template '{{baseUrl}}/api/referentials/specimens/{{ref_specimen_id}}/hard'
$headers = @()
$headers += (Resolve-Template 'Authorization: Bearer {{admin_token}}')
Invoke-HCurl -Label '09 Referentials - Specimens :: DELETE /api/referentials/specimens/{id}/hard' -Method 'DELETE' -Url $url -Headers $headers
