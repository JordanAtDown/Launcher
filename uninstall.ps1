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

# La valeur registre est de la forme "C:\...\launcher.exe" (avec guillemets)
$launcherExe = $regValue.Trim('"')
$InstallDir  = Split-Path -Parent $launcherExe

Write-Host "Dossier detecte : $InstallDir"
Write-Host ""

# Supprime les fichiers (config.toml conserve si l'utilisateur l'a modifie)
$binaries = @("launcher.exe", "cec-daemon.exe")
foreach ($f in $binaries) {
    $target = Join-Path $InstallDir $f
    if (Test-Path $target) {
        Remove-Item $target -Force
        Write-Host "  [OK] Supprime $f"
    }
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

# Supprime la cle de registre
Remove-ItemProperty -Path $regPath -Name $regName -ErrorAction SilentlyContinue
Write-Host "  [OK] Cle registre demarrage supprimee"

Write-Host ""
Write-Host "Desinstallation terminee."
