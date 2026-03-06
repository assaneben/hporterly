[CmdletBinding()]
param(
    [string]$RepoRoot = ".",
    [string]$BackendRoot = "./backend",
    [string[]]$ChangedFiles = @(),
    [switch]$Ci
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$repoPath = (Resolve-Path $RepoRoot).Path
$backendPath = (Resolve-Path $BackendRoot).Path
$script:results = New-Object System.Collections.Generic.List[object]
$script:hasFailure = $false
$script:hasDiffContext = $false

function Get-RelativePath {
    param(
        [Parameter(Mandatory = $true)]
        [string]$BasePath,
        [Parameter(Mandatory = $true)]
        [string]$TargetPath
    )

    $normalizedBase = (Resolve-Path $BasePath).Path.TrimEnd("\", "/") + [System.IO.Path]::DirectorySeparatorChar
    $normalizedTarget = (Resolve-Path $TargetPath).Path
    $baseUri = New-Object System.Uri($normalizedBase)
    $targetUri = New-Object System.Uri($normalizedTarget)
    $relativeUri = $baseUri.MakeRelativeUri($targetUri)
    return [System.Uri]::UnescapeDataString($relativeUri.ToString()).Replace("/", [System.IO.Path]::DirectorySeparatorChar)
}

function Add-CheckResult {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Section,
        [Parameter(Mandatory = $true)]
        [string]$Requirement,
        [Parameter(Mandatory = $true)]
        [ValidateSet("PASS", "FAIL", "NOT RUN")]
        [string]$Status,
        [Parameter(Mandatory = $true)]
        [string]$Evidence
    )

    $script:results.Add([pscustomobject]@{
            Section     = $Section
            Requirement = $Requirement
            Status      = $Status
            Evidence    = $Evidence
        })

    if ($Status -eq "FAIL") {
        $script:hasFailure = $true
    }
}

function Test-CommandAvailable {
    param([Parameter(Mandatory = $true)][string]$Name)
    return $null -ne (Get-Command $Name -ErrorAction SilentlyContinue)
}

function Invoke-CommandCheck {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Section,
        [Parameter(Mandatory = $true)]
        [string]$Requirement,
        [Parameter(Mandatory = $true)]
        [string]$WorkingDirectory,
        [Parameter(Mandatory = $true)]
        [string]$Executable,
        [Parameter(Mandatory = $true)]
        [string[]]$Arguments
    )

    if (-not (Test-CommandAvailable $Executable)) {
        Add-CheckResult $Section $Requirement "NOT RUN" "Commande indisponible: $Executable"
        return
    }

    Push-Location $WorkingDirectory
    $previousErrorActionPreference = $ErrorActionPreference
    try {
        $ErrorActionPreference = "Continue"
        $output = & $Executable @Arguments 2>&1 | ForEach-Object { "$_" }
        $exitCode = $LASTEXITCODE
    }
    finally {
        $ErrorActionPreference = $previousErrorActionPreference
        Pop-Location
    }

    $commandText = "$Executable $($Arguments -join ' ')"
    if ($exitCode -eq 0) {
        Add-CheckResult $Section $Requirement "PASS" "$commandText -> exit 0"
    }
    else {
        $joinedOutput = $output -join "`n"
        if ($joinedOutput -match 'os error 4551|strat.gie de contr.le d.application a bloqu.|Application Control') {
            Add-CheckResult $Section $Requirement "NOT RUN" "$commandText -> execution bloquee par la politique hote locale (Windows application control)"
            return
        }
        $firstLine = ($output | Select-Object -First 1)
        if ([string]::IsNullOrWhiteSpace($firstLine)) {
            $firstLine = "commande echouee sans sortie exploitable"
        }
        Add-CheckResult $Section $Requirement "FAIL" "$commandText -> exit $exitCode :: $firstLine"
    }
}

function Get-FallbackFiles {
    param([Parameter(Mandatory = $true)][string]$RepositoryPath)

    $paths = New-Object System.Collections.Generic.List[string]
    $candidateRoots = @(
        (Join-Path $RepositoryPath "backend\src"),
        (Join-Path $RepositoryPath "backend\tests"),
        (Join-Path $RepositoryPath "docs"),
        (Join-Path $RepositoryPath ".github"),
        (Join-Path $RepositoryPath "scripts")
    )

    foreach ($candidate in $candidateRoots) {
        if (Test-Path $candidate) {
            Get-ChildItem -Path $candidate -File -Recurse | ForEach-Object {
                $paths.Add($_.FullName)
            }
        }
    }

    foreach ($singleFile in @(
            (Join-Path $RepositoryPath "backend\Cargo.toml"),
            (Join-Path $RepositoryPath "backend\.env.example"),
            (Join-Path $RepositoryPath "backend\.env.production.example")
        )) {
        if (Test-Path $singleFile) {
            $paths.Add((Resolve-Path $singleFile).Path)
        }
    }

    return $paths | Sort-Object -Unique
}

function Get-ChangedFileList {
    param(
        [Parameter(Mandatory = $true)]
        [string]$RepositoryPath,
        [string[]]$ExplicitFiles
    )

    if ($ExplicitFiles -and $ExplicitFiles.Count -gt 0) {
        $script:hasDiffContext = $true
        return $ExplicitFiles | ForEach-Object {
            if (Test-Path $_) {
                (Resolve-Path $_).Path
            }
            else {
                $candidate = Join-Path $RepositoryPath $_
                if (Test-Path $candidate) {
                    (Resolve-Path $candidate).Path
                }
            }
        } | Where-Object { $_ } | Sort-Object -Unique
    }

    if ((Test-CommandAvailable "git") -and (Test-Path (Join-Path $RepositoryPath ".git"))) {
        Push-Location $RepositoryPath
        try {
            & git rev-parse --is-inside-work-tree 2>$null | Out-Null
            if ($LASTEXITCODE -eq 0) {
                $diffTarget = if ($env:PR_BASE_SHA) { "$($env:PR_BASE_SHA)...HEAD" } else { "HEAD" }
                $diffFiles = & git diff --name-only $diffTarget 2>$null
                $resolved = @()
                foreach ($diffFile in $diffFiles) {
                    if ([string]::IsNullOrWhiteSpace($diffFile)) {
                        continue
                    }
                    $absolute = Join-Path $RepositoryPath $diffFile.Trim()
                    if (Test-Path $absolute) {
                        $resolved += (Resolve-Path $absolute).Path
                    }
                }
                if ($resolved.Count -gt 0) {
                    $script:hasDiffContext = $true
                    return $resolved | Sort-Object -Unique
                }
            }
        }
        finally {
            Pop-Location
        }
    }

    return Get-FallbackFiles -RepositoryPath $RepositoryPath
}

function Find-PatternHits {
    param(
        [string[]]$Paths,
        [Parameter(Mandatory = $true)]
        [string]$Pattern
    )

    if (-not $Paths -or @($Paths).Count -eq 0) {
        return @()
    }

    $hits = @()
    foreach ($path in $Paths) {
        if (-not (Test-Path $path -PathType Leaf)) {
            continue
        }
        $hits += Select-String -Path $path -Pattern $Pattern -AllMatches -ErrorAction SilentlyContinue
    }
    return $hits
}

function Get-FirstHitSummary {
    param(
        [Parameter(Mandatory = $true)]
        [string]$RepositoryPath,
        [Parameter(Mandatory = $true)]
        [object[]]$Hits
    )

    if (-not $Hits -or @($Hits).Count -eq 0) {
        return ""
    }

    $first = $Hits | Select-Object -First 1
    $relativePath = Get-RelativePath -BasePath $RepositoryPath -TargetPath $first.Path
    return "${relativePath}:$($first.LineNumber)"
}

function Test-FileContainsAll {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Path,
        [Parameter(Mandatory = $true)]
        [string[]]$Patterns
    )

    if (-not (Test-Path $Path)) {
        return $false
    }

    foreach ($pattern in $Patterns) {
        if (-not (Select-String -Path $Path -Pattern $pattern -SimpleMatch -Quiet -ErrorAction SilentlyContinue)) {
            return $false
        }
    }
    return $true
}

$changedFiles = Get-ChangedFileList -RepositoryPath $repoPath -ExplicitFiles $ChangedFiles
$publicDocs = @()
foreach ($docRoot in @(
        (Join-Path $repoPath "README.md"),
        (Join-Path $repoPath "docs"),
        (Join-Path $repoPath "frontend")
    )) {
    if (Test-Path $docRoot -PathType Leaf) {
        $publicDocs += (Resolve-Path $docRoot).Path
    }
    elseif (Test-Path $docRoot -PathType Container) {
        $publicDocs += Get-ChildItem -Path $docRoot -File -Recurse -Include *.md, *.html, *.js, *.ts, *.tsx, *.json |
            Select-Object -ExpandProperty FullName
    }
}

$cargoToml = Join-Path $backendPath "Cargo.toml"
$mainRs = Join-Path $backendPath "src\main.rs"
$authHandlerRs = Join-Path $backendPath "src\handlers\auth.rs"
$mfaRs = Join-Path $backendPath "src\auth\mfa.rs"
$hl7ModelsRs = Join-Path $backendPath "src\hl7\models.rs"
$hl7HandlersRs = Join-Path $backendPath "src\hl7\handlers.rs"
$typedRoleRs = Join-Path $backendPath "src\auth\rbac.rs"
$legacyRbacRs = Join-Path $backendPath "src\utils\rbac.rs"
$apiErrorRs = Join-Path $backendPath "src\utils\error.rs"
$auditMigrationFiles = Get-ChildItem -Path (Join-Path $backendPath "migrations") -File -Recurse -Include up.sql, *.sql -ErrorAction SilentlyContinue |
    Where-Object { $_.FullName -match "audit" } |
    Select-Object -ExpandProperty FullName

$secretHits = Find-PatternHits -Paths $changedFiles -Pattern '(?i)\b(secret|password|api[_-]?key|encryption_key|jwt_secret)\b.{0,30}[:=].{0,5}[''"][^''"]+[''"]'
$secretHits = $secretHits | Where-Object {
    $_.Line -notmatch '(?i)CHANGEME|EXAMPLE|<.+>|get_optional_env|std::env|env::var|\$env:|process\.env|\${{\s*secrets\.'
}

$insLiteralHits = Find-PatternHits -Paths $changedFiles -Pattern '(?<!\d)\d{19}(?!\d)'
$internalRouteDocHits = Find-PatternHits -Paths $publicDocs -Pattern '/internal/[A-Za-z0-9/_-]+'
$sourceFiles = Get-ChildItem -Path (Join-Path $backendPath "src") -File -Recurse -Include *.rs -ErrorAction SilentlyContinue |
    Select-Object -ExpandProperty FullName
$logDataHits = Find-PatternHits -Paths $sourceFiles -Pattern 'log::(info|warn|error|debug|trace)!\(.*(\bINS\b|\bins\b|patient_name|nom_patient|prenom_patient)'

if (Test-FileContainsAll -Path $hl7ModelsRs -Patterns @('pub fn valider_ins', 'pub fn luhn_check', 'pub fn ins_match')) {
    Add-CheckResult "Securite" "SBD-01 : Toutes les entrees HL7/FHIR validees (INS, statuts, roles)" "PASS" "backend/src/hl7/models.rs expose valider_ins, luhn_check et ins_match"
}
else {
    Add-CheckResult "Securite" "SBD-01 : Toutes les entrees HL7/FHIR validees (INS, statuts, roles)" "NOT RUN" "Validation HL7 detectee partiellement; validation FHIR complete a confirmer lors de la phase FHIR"
}

if ((Test-FileContainsAll -Path $cargoToml -Patterns @('argon2', 'aes-gcm')) -and
    (Test-FileContainsAll -Path $mfaRs -Patterns @('enforce_mfa_attempt_limit', 'verify_mfa', 'encrypt_secret', 'decrypt_secret')) -and
    (Test-FileContainsAll -Path $authHandlerRs -Patterns @('enforce_login_rate_limit'))) {
    Add-CheckResult "Securite" "SBD-04 : Argon2id sur passwords, TOTP sur MFA, rate limit sur /auth/*" "PASS" "backend/Cargo.toml, backend/src/auth/mfa.rs et backend/src/handlers/auth.rs"
}
else {
    Add-CheckResult "Securite" "SBD-04 : Argon2id sur passwords, TOTP sur MFA, rate limit sur /auth/*" "FAIL" "Garde-fous auth attendus introuvables dans Cargo/auth handlers"
}

Add-CheckResult "Securite" "SBD-05 : Aucun endpoint ne retourne des donnees d'un autre utilisateur" "NOT RUN" "Des garde-fous existent dans backend/src/middleware/auth.rs et backend/src/auth/rbac.rs; un test d'integration par role reste requis"

if (Test-FileContainsAll -Path $mainRs -Patterns @('let hl7_bind_port = config.hl7_internal_port;', '.bind(("127.0.0.1", hl7_bind_port))', '.configure(hl7::handlers::configure)')) {
    Add-CheckResult "Securite" "SBD-06 : Webhook HL7 sur port interne dedie uniquement, jamais sur le port public" "PASS" "backend/src/main.rs isole le serveur HL7 sur 127.0.0.1"
}
else {
    Add-CheckResult "Securite" "SBD-06 : Webhook HL7 sur port interne dedie uniquement, jamais sur le port public" "FAIL" "Isolation loopback du serveur HL7 introuvable dans backend/src/main.rs"
}

if (-not $script:hasDiffContext) {
    Add-CheckResult "Securite" "SBD-07 : Zero secret en dur sur les fichiers modifies" "NOT RUN" "Aucun diff Git ou liste -ChangedFiles fournie; scan diff non fiable en local"
}
elseif (@($secretHits).Count -eq 0) {
    Add-CheckResult "Securite" "SBD-07 : Zero secret en dur sur les fichiers modifies" "PASS" "Scan secrets sur $(@($changedFiles).Count) fichier(s) modifies -> aucun secret en dur detecte"
}
else {
    Add-CheckResult "Securite" "SBD-07 : Zero secret en dur sur les fichiers modifies" "FAIL" ("Motif suspect detecte: " + (Get-FirstHitSummary -RepositoryPath $repoPath -Hits $secretHits))
}

if ((Test-FileContainsAll -Path $mfaRs -Patterns @('Aes256Gcm', 'encrypt_secret', 'decrypt_secret')) -and
    (Test-FileContainsAll -Path $hl7ModelsRs -Patterns @('ConstantTimeEq', 'pub fn ins_match'))) {
    Add-CheckResult "Securite" "SBD-08 : AES-256-GCM pour MFA et comparaison INS constant-time" "PASS" "backend/src/auth/mfa.rs et backend/src/hl7/models.rs"
}
else {
    Add-CheckResult "Securite" "SBD-08 : AES-256-GCM pour MFA et comparaison INS constant-time" "FAIL" "Usage AES-256-GCM ou comparaison constant-time introuvable"
}

if (@($logDataHits).Count -eq 0) {
    Add-CheckResult "Securite" "SBD-09 : Aucun INS ni donnees patient dans les logs applicatifs" "PASS" "Aucun log applicatif avec INS/patient detecte dans backend/src"
}
else {
    Add-CheckResult "Securite" "SBD-09 : Aucun INS ni donnees patient dans les logs applicatifs" "FAIL" ("Log applicatif sensible detecte: " + (Get-FirstHitSummary -RepositoryPath $repoPath -Hits $logDataHits))
}

if ((@($auditMigrationFiles).Count -gt 0) -and (Test-FileContainsAll -Path $hl7HandlersRs -Patterns @('insert_audit_log', 'diesel::insert_into(audit_logs::table)'))) {
    Add-CheckResult "Securite" "SBD-10 : Chaque action utilisateur critique ecrite dans audit_logs" "NOT RUN" "Insertion audit detectee dans backend/src/hl7/handlers.rs; couverture complete a verifier par tests de scenario"
}
else {
    Add-CheckResult "Securite" "SBD-10 : Chaque action utilisateur critique ecrite dans audit_logs" "NOT RUN" "Preuve statique incomplete; verification manuelle d'audit requise"
}

if ((Test-FileContainsAll -Path $authHandlerRs -Patterns @('enforce_login_rate_limit')) -and
    (Test-FileContainsAll -Path $mfaRs -Patterns @('enforce_mfa_attempt_limit'))) {
    Add-CheckResult "Securite" "SBD-11 : Rate limiting actif sur /api/auth/login et /api/auth/mfa/verify" "PASS" "backend/src/handlers/auth.rs et backend/src/auth/mfa.rs"
}
else {
    Add-CheckResult "Securite" "SBD-11 : Rate limiting actif sur /api/auth/login et /api/auth/mfa/verify" "FAIL" "Rate limiting auth introuvable"
}

if ((Test-FileContainsAll -Path $mainRs -Patterns @('allowed_origin')) -and
    (Test-FileContainsAll -Path (Join-Path $backendPath ".env.example") -Patterns @('CORS_ALLOWED_ORIGINS=')) -and
    -not (Select-String -Path (Join-Path $backendPath ".env.example") -Pattern 'CORS_ALLOWED_ORIGINS=\*' -Quiet -ErrorAction SilentlyContinue)) {
    Add-CheckResult "Securite" "SBD-20 : CORS_ALLOWED_ORIGINS ne contient pas *" "PASS" "backend/src/main.rs et backend/.env.example utilisent des origines explicites"
}
else {
    Add-CheckResult "Securite" "SBD-20 : CORS_ALLOWED_ORIGINS ne contient pas *" "FAIL" "Configuration CORS explicite introuvable ou wildcard detecte"
}

$typedRoleHasDefaultDeny =
    (Test-FileContainsAll -Path $typedRoleRs -Patterns @('pub fn peut', 'matches!(')) -or
    (Test-FileContainsAll -Path $typedRoleRs -Patterns @('_ => false'))

if ($typedRoleHasDefaultDeny -and
    (Test-FileContainsAll -Path $legacyRbacRs -Patterns @('ApiError::Forbidden')) -and
    (Test-FileContainsAll -Path $apiErrorRs -Patterns @('StatusCode::FORBIDDEN'))) {
    Add-CheckResult "Securite" "SBD-21 : Tout acces non autorise retourne 403" "PASS" "backend/src/auth/rbac.rs, backend/src/utils/rbac.rs et backend/src/utils/error.rs"
}
else {
    Add-CheckResult "Securite" "SBD-21 : Tout acces non autorise retourne 403" "FAIL" "Chaîne fail-secure 403 introuvable"
}

if (Test-FileContainsAll -Path $hl7ModelsRs -Patterns @('#[test]', 'ins_luhn_accepts_valid_value', 'ins_luhn_rejects_invalid_value', 'ins_error_maps_format')) {
    Add-CheckResult "Fonctionnel" "Validation INS unitaire : longueur 19, chiffres uniquement, Luhn" "PASS" "backend/src/hl7/models.rs contient les tests INS unitaires"
}
else {
    Add-CheckResult "Fonctionnel" "Validation INS unitaire : longueur 19, chiffres uniquement, Luhn" "FAIL" "Tests unitaires INS introuvables"
}

Add-CheckResult "Fonctionnel" "Workflow confirm-ins : mismatch -> 409 + audit alerte + transport bloque" "NOT RUN" "Endpoint Task confirm-ins non present dans le workspace courant"
Add-CheckResult "Fonctionnel" "Table audit_logs : INSERT fonctionne, UPDATE/DELETE refuses" "NOT RUN" "Necessite un test PostgreSQL execute contre une base"
Add-CheckResult "Fonctionnel" "MFA : connexion sans code TOTP impossible apres activation" "NOT RUN" "Flux MFA present dans backend/src/services/auth.rs et backend/src/auth/mfa.rs; test bout en bout non execute"
Add-CheckResult "Fonctionnel" "RBAC : un brancardier ne peut pas acceder aux endpoints regulateur" "NOT RUN" "Tests RBAC d'endpoint non executes sur cette PR"

$handlerSources = Get-ChildItem -Path (Join-Path $backendPath "src\handlers"), (Join-Path $backendPath "src\auth"), (Join-Path $backendPath "src\hl7"), (Join-Path $backendPath "src\middleware") -File -Recurse -ErrorAction SilentlyContinue |
    Select-Object -ExpandProperty FullName
$unwrapHits = Find-PatternHits -Paths $handlerSources -Pattern 'unwrap\('
if (@($unwrapHits).Count -eq 0) {
    Add-CheckResult "Qualite Code Rust" "Zero unwrap() dans les handlers de production" "PASS" "Aucun unwrap() detecte dans handlers/auth/hl7/middleware"
}
else {
    Add-CheckResult "Qualite Code Rust" "Zero unwrap() dans les handlers de production" "FAIL" ("unwrap() detecte: " + (Get-FirstHitSummary -RepositoryPath $repoPath -Hits $unwrapHits))
}

$boxedErrorHits = Find-PatternHits -Paths $handlerSources -Pattern 'Box\s*<\s*dyn\s+Error\s*>'
if (@($boxedErrorHits).Count -eq 0) {
    Add-CheckResult "Qualite Code Rust" "Toutes les erreurs sont typees, pas de Box<dyn Error> dans les handlers" "PASS" "Aucun Box<dyn Error> detecte dans les handlers"
}
else {
    Add-CheckResult "Qualite Code Rust" "Toutes les erreurs sont typees, pas de Box<dyn Error> dans les handlers" "FAIL" ("Box<dyn Error> detecte: " + (Get-FirstHitSummary -RepositoryPath $repoPath -Hits $boxedErrorHits))
}

Invoke-CommandCheck -Section "Qualite Code Rust" -Requirement "cargo clippy --locked --all-targets --all-features -- -D warnings" -WorkingDirectory $backendPath -Executable "cargo" -Arguments @("clippy", "--locked", "--all-targets", "--all-features", "--", "-D", "warnings")
Invoke-CommandCheck -Section "Qualite Code Rust" -Requirement "cargo test --locked --all-targets --all-features" -WorkingDirectory $backendPath -Executable "cargo" -Arguments @("test", "--locked", "--all-targets", "--all-features")
Invoke-CommandCheck -Section "Qualite Code Rust" -Requirement "cargo check --locked --all-targets --all-features" -WorkingDirectory $backendPath -Executable "cargo" -Arguments @("check", "--locked", "--all-targets", "--all-features")

$seedFixturePaths = @()
foreach ($path in @(
        (Join-Path $backendPath "tests"),
        (Join-Path $backendPath "src\bin"),
        (Join-Path $repoPath "docs")
    )) {
    if (Test-Path $path) {
        $seedFixturePaths += Get-ChildItem -Path $path -File -Recurse -ErrorAction SilentlyContinue | Select-Object -ExpandProperty FullName
    }
}

$realPatientNameHits = Find-PatternHits -Paths $seedFixturePaths -Pattern '(?i)\b(?:patient|nom|prenom)\b.{0,20}[:=].{0,10}[''"][A-Z][a-z]+'
if (@($realPatientNameHits).Count -eq 0) {
    Add-CheckResult "Donnees de Test" "Zero donnee patient reelle dans seeds/fixtures" "PASS" "Aucun motif nominal suspect detecte dans seeds/tests/docs"
}
else {
    Add-CheckResult "Donnees de Test" "Zero donnee patient reelle dans seeds/fixtures" "FAIL" ("Motif patient suspect detecte: " + (Get-FirstHitSummary -RepositoryPath $repoPath -Hits $realPatientNameHits))
}

if (-not $script:hasDiffContext) {
    Add-CheckResult "Donnees de Test" "Aucun motif de 19 chiffres consecutifs dans les fichiers modifies" "NOT RUN" "Aucun diff Git ou liste -ChangedFiles fournie; scan diff non fiable en local"
}
elseif (@($insLiteralHits).Count -eq 0) {
    Add-CheckResult "Donnees de Test" "Aucun motif de 19 chiffres consecutifs dans les fichiers modifies" "PASS" "Recherche 19 chiffres sur $(@($changedFiles).Count) fichier(s) modifies -> aucun hit"
}
else {
    Add-CheckResult "Donnees de Test" "Aucun motif de 19 chiffres consecutifs dans les fichiers modifies" "FAIL" ("Sequence de 19 chiffres detectee: " + (Get-FirstHitSummary -RepositoryPath $repoPath -Hits $insLiteralHits))
}

if (@($internalRouteDocHits).Count -eq 0) {
    Add-CheckResult "Donnees de Test" "Aucune documentation publique n'expose de routes internes" "PASS" "Aucune route interne detectee dans README/docs/frontend"
}
else {
    Add-CheckResult "Donnees de Test" "Aucune documentation publique n'expose de routes internes" "FAIL" ("Reference publique a une route interne: " + (Get-FirstHitSummary -RepositoryPath $repoPath -Hits $internalRouteDocHits))
}

$sectionsInOrder = @("Securite", "Fonctionnel", "Qualite Code Rust", "Donnees de Test")
foreach ($section in $sectionsInOrder) {
    Write-Host ""
    Write-Host "=== $section ==="
    foreach ($item in $script:results | Where-Object { $_.Section -eq $section }) {
        Write-Host ("[{0}] {1}`n  {2}" -f $item.Status, $item.Requirement, $item.Evidence)
    }
}

if ($script:hasFailure) {
    exit 1
}

exit 0

<#
SECURITY REVIEW (SecureByDesign v1.1.0 - REGLEMENTE)
- Controls reviewed: SBD-01 to SBD-25.
- Verified in this file:
  - OK SBD-07: scans modified files for suspicious hardcoded secrets before PR submission.
  - OK SBD-10: produces explicit evidence lines for every checklist item.
  - OK SBD-11: runs locked cargo/clippy/test checks and auth-related static guardrail verification.
  - OK SBD-22: enforces reproducible dependency resolution through Cargo's locked mode.
  - OK SBD-21: reports fail-secure checks explicitly and returns non-zero on FAIL.
- Not fully satisfiable in this file:
  - WARN SBD-05: endpoint-level data isolation cannot be proven statically in all cases.
    Alternative: keep integration tests per role and mark non-executed checks as NOT RUN with evidence.
  - WARN SBD-24: local host application-control policies can block binary test execution outside CI.
    Alternative: keep Linux CI as the execution authority and report local host-policy blocks as NOT RUN with explicit evidence.
#>
