$ErrorActionPreference = "Stop"
$taskName = "Launcher"

Write-Host "=== Launcher Uninstaller ==="

# Récupère le chemin d'installation depuis la tâche planifiée
$task = Get-ScheduledTask -TaskName $taskName -ErrorAction SilentlyContinue
if (-not $task) {
    Write-Host "Launcher n'est pas installe (tache '$taskName' introuvable)."
    exit 0
}

$launcherExe = $task.Actions[0].Execute.Trim('"')
$InstallDir  = Split-Path -Parent $launcherExe

Write-Host "Dossier detecte : $InstallDir"
Write-Host ""

# Supprime tous les exes versionnés
Get-ChildItem -Path $InstallDir -Filter "launcher-*.exe"   -ErrorAction SilentlyContinue | ForEach-Object {
    Remove-Item $_.FullName -Force
    Write-Host "  [OK] Supprime $($_.Name)"
}
Get-ChildItem -Path $InstallDir -Filter "cec-daemon-*.exe" -ErrorAction SilentlyContinue | ForEach-Object {
    Remove-Item $_.FullName -Force
    Write-Host "  [OK] Supprime $($_.Name)"
}

# Supprime config.toml seulement si c'est le dernier fichier restant
$remaining = @(Get-ChildItem -Path $InstallDir -ErrorAction SilentlyContinue)
if ($remaining.Count -eq 1 -and $remaining[0].Name -eq "config.toml") {
    Remove-Item (Join-Path $InstallDir "config.toml") -Force
    Write-Host "  [OK] Supprime config.toml"
} elseif ($remaining.Count -gt 0) {
    Write-Host "  [INFO] config.toml conserve (modifie ou autres fichiers presents)"
}

# Supprime le dossier s'il est vide
if (-not (Get-ChildItem -Path $InstallDir -ErrorAction SilentlyContinue)) {
    Remove-Item $InstallDir -Force
    Write-Host "  [OK] Supprime le dossier $InstallDir"
}

# Supprime la tâche planifiée
schtasks /delete /tn $taskName /f 2>&1 | Out-Null
Write-Host "  [OK] Tache planifiee '$taskName' supprimee"

Write-Host ""
Write-Host "Desinstallation terminee."
