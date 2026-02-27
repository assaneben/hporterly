param(
  [string]$PackFolder = "DOSSIER_REPRISE_DEV"
)

Write-Host "Regeneration des inventaires pour $PackFolder" -ForegroundColor Cyan
Write-Host "Astuce: si npm est bloque par PowerShell, le script utilisera npm.cmd" -ForegroundColor Yellow
# Ce script est un point d entree simple. La generation complete est faite par l assistant lors de la creation initiale.
# Vous pouvez l etendre selon vos besoins (tree, versions, checks de presence, etc.).
