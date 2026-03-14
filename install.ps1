param(
    [string]$InstallDir = "$env:LOCALAPPDATA\Programs\Launcher"
)

$ErrorActionPreference = "Stop"
$files = @("launcher.exe", "cec-daemon.exe", "config.toml")
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path

Write-Host "=== Launcher Installer ==="
Write-Host "Dossier cible : $InstallDir"
Write-Host ""

New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null

foreach ($f in $files) {
    $src = Join-Path $scriptDir $f
    if (Test-Path $src) {
        Copy-Item $src -Destination $InstallDir -Force
        Write-Host "  [OK] $f"
    } else {
        Write-Warning "  [SKIP] $f introuvable"
    }
}

# Démarrage Windows : HKCU\Software\Microsoft\Windows\CurrentVersion\Run
$launcherExe = Join-Path $InstallDir "launcher.exe"
$regPath = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Run"
Set-ItemProperty -Path $regPath -Name "Launcher" -Value "`"$launcherExe`""
Write-Host "  [OK] Démarrage Windows enregistré"

Write-Host ""
Write-Host "Installation terminée : $InstallDir"
Write-Host "Editez config.toml dans ce dossier pour configurer."
