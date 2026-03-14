$ErrorActionPreference = "Stop"
$regPath = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Run"
$regName = "Launcher"

Write-Host "=== Launcher Uninstaller ==="

# Récupère le chemin d'installation depuis la clé registre
$regValue = (Get-ItemProperty -Path $regPath -Name $regName -ErrorAction SilentlyContinue).$regName
if (-not $regValue) {
    Write-Host "Launcher n'est pas installe (cle registre introuvable)."
    exit 0
}

# La valeur registre est de la forme "C:\...\launcher-0.2.0.exe" (avec guillemets)
$launcherExe = $regValue.Trim('"')
$InstallDir  = Split-Path -Parent $launcherExe

Write-Host "Dossier detecte : $InstallDir"
Write-Host ""

# Supprime tous les exes versionnés (launcher-*.exe et cec-daemon-*.exe)
Get-ChildItem -Path $InstallDir -Filter "launcher-*.exe"   -ErrorAction SilentlyContinue | ForEach-Object {
    Remove-Item $_.FullName -Force
    Write-Host "  [OK] Supprime $($_.Name)"
}
Get-ChildItem -Path $InstallDir -Filter "cec-daemon-*.exe" -ErrorAction SilentlyContinue | ForEach-Object {
    Remove-Item $_.FullName -Force
    Write-Host "  [OK] Supprime $($_.Name)"
}

# Supprime config.toml seulement si le dossier ne contient plus que lui
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

# Supprime la clé de registre
Remove-ItemProperty -Path $regPath -Name $regName -ErrorAction SilentlyContinue
Write-Host "  [OK] Cle registre demarrage supprimee"

Write-Host ""
Write-Host "Desinstallation terminee."
