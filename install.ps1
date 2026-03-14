param(
    [string]$InstallDir = "$env:LOCALAPPDATA\Programs\Launcher"
)

$ErrorActionPreference = "Stop"
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path

Write-Host "=== Launcher Installer ==="
Write-Host "Dossier cible : $InstallDir"
Write-Host ""

# Detecte les exe versionnés dans le dossier source (ex: launcher-0.2.0.exe)
$launcherSrc  = Get-ChildItem -Path $scriptDir -Filter "launcher-*.exe"  | Select-Object -First 1
$cecDaemonSrc = Get-ChildItem -Path $scriptDir -Filter "cec-daemon-*.exe" | Select-Object -First 1

if (-not $launcherSrc) {
    Write-Error "launcher-*.exe introuvable dans $scriptDir"
    exit 1
}

New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null

# Supprime les anciennes versions avant de copier (évite l'accumulation)
Get-ChildItem -Path $InstallDir -Filter "launcher-*.exe"   -ErrorAction SilentlyContinue | Remove-Item -Force
Get-ChildItem -Path $InstallDir -Filter "cec-daemon-*.exe" -ErrorAction SilentlyContinue | Remove-Item -Force

# Copie les nouveaux exes versionnés
Copy-Item $launcherSrc.FullName -Destination $InstallDir -Force
Write-Host "  [OK] $($launcherSrc.Name)"

if ($cecDaemonSrc) {
    Copy-Item $cecDaemonSrc.FullName -Destination $InstallDir -Force
    Write-Host "  [OK] $($cecDaemonSrc.Name)"
}

# config.toml : copie uniquement à la première installation (préservé lors des mises à jour)
$configDst = Join-Path $InstallDir "config.toml"
if (-not (Test-Path $configDst)) {
    $configSrc = Join-Path $scriptDir "config.toml"
    if (Test-Path $configSrc) {
        Copy-Item $configSrc -Destination $InstallDir -Force
        Write-Host "  [OK] config.toml"
    }
} else {
    Write-Host "  [INFO] config.toml conserve (mise a jour)"
}

# Met à jour la clé de démarrage Windows avec le nouveau nom versionné
$installedLauncher = Join-Path $InstallDir $launcherSrc.Name
$regPath = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Run"
Set-ItemProperty -Path $regPath -Name "Launcher" -Value "`"$installedLauncher`""
Write-Host "  [OK] Demarrage Windows -> $($launcherSrc.Name)"

Write-Host ""
Write-Host "Installation terminee : $InstallDir"
if (-not (Test-Path $configDst)) {
    Write-Host "Editez config.toml dans ce dossier pour configurer."
}
