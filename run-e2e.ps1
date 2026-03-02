$ErrorActionPreference = 'Stop'

function Login([string]$username, [string]$password) {
  $body = @{ username = $username; password = $password } | ConvertTo-Json
  return Invoke-RestMethod -Method POST -Uri 'http://localhost:4000/api/auth/login' -Body $body -ContentType 'application/json'
}

$results = @()

$accounts = @(
  @{ username = 'admin'; password = 'password123'; role = 'administrateur' },
  @{ username = 'marie.durand'; password = 'password123'; role = 'demandeur' },
  @{ username = 'jean.martin'; password = 'password123'; role = 'brancardier' },
  @{ username = 'regulateur'; password = 'password123'; role = 'regulateur' }
)

foreach ($acc in $accounts) {
  $login = Login $acc.username $acc.password
  $token = $login.token
  $headers = @{ Authorization = "Bearer $token" }

  $entry = [ordered]@{
    username = $acc.username
    expected_role = $acc.role
    actual_role = $login.user.role
    login_ok = $true
    checks = @()
  }

  switch ($acc.role) {
    'administrateur' {
      $users = Invoke-RestMethod -Method GET -Uri 'http://localhost:4000/api/users' -Headers $headers
      $entry.checks += "GET /api/users -> $($users.Count) utilisateurs"

      $tickets = Invoke-RestMethod -Method GET -Uri 'http://localhost:4000/api/tickets' -Headers $headers
      $entry.checks += "GET /api/tickets -> $($tickets.Count) tickets"
    }
    'demandeur' {
      $ticketsBefore = Invoke-RestMethod -Method GET -Uri 'http://localhost:4000/api/tickets' -Headers $headers
      $entry.checks += "GET /api/tickets (mes tickets) -> $($ticketsBefore.Count)"

      $createBody = @{
        patient_name = 'TEST E2E DEMANDEUR'
        origin = 'Urgences'
        destination = 'Radiologie'
        transport_type = 'PATIENT'
        transport_subtype = 'TP-BRANC'
        mode = 'Brancard'
      } | ConvertTo-Json
      $created = Invoke-RestMethod -Method POST -Uri 'http://localhost:4000/api/tickets' -Headers $headers -Body $createBody -ContentType 'application/json'
      $entry.checks += "POST /api/tickets -> cree id $($created.ticket.id)"
    }
    'brancardier' {
      $tickets = Invoke-RestMethod -Method GET -Uri 'http://localhost:4000/api/tickets' -Headers $headers
      $entry.checks += "GET /api/tickets (queue) -> $($tickets.Count)"

      $unread = Invoke-RestMethod -Method GET -Uri 'http://localhost:4000/api/notifications/unread-count' -Headers $headers
      $entry.checks += "GET /api/notifications/unread-count -> $($unread.count)"
    }
    'regulateur' {
      $tickets = Invoke-RestMethod -Method GET -Uri 'http://localhost:4000/api/tickets' -Headers $headers
      $entry.checks += "GET /api/tickets -> $($tickets.Count)"

      $runtime = Invoke-RestMethod -Method GET -Uri 'http://localhost:4000/api/priority-rules/runtime' -Headers $headers
      $entry.checks += "GET /api/priority-rules/runtime -> $($runtime.rules.Count) regles"
    }
  }

  $results += [pscustomobject]$entry
}

$frontendResponse = Invoke-WebRequest -Method GET -Uri 'http://localhost:3000' -UseBasicParsing

[pscustomobject]@{
  frontend_status = $frontendResponse.StatusCode
  backend_health = (Invoke-RestMethod -Method GET -Uri 'http://localhost:4000/api/health').status
  e2e_accounts = $results
} | ConvertTo-Json -Depth 6 | Set-Content -Encoding UTF8 "e2e-report.json"

Get-Content "e2e-report.json" -Raw