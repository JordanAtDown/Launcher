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

# update.bat : copie dans le dossier d'installation (toujours à jour)
$updateBatSrc = Join-Path $scriptDir "update.bat"
if (Test-Path $updateBatSrc) {
    Copy-Item $updateBatSrc -Destination $InstallDir -Force
    Write-Host "  [OK] update.bat"
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

# Tâche planifiée : logon, utilisateur courant, privilèges élevés (no UAC prompt au démarrage)
$installedLauncher = Join-Path $InstallDir $launcherSrc.Name
$ErrorActionPreference = "SilentlyContinue"
schtasks /delete /tn "Launcher" /f 2>&1 | Out-Null
$ErrorActionPreference = "Stop"
schtasks /create /tn "Launcher" /tr "`"$installedLauncher`"" /sc ONLOGON /rl HIGHEST /f 2>&1 | Out-Null
if ($LASTEXITCODE -ne 0) {
    Write-Warning "  [WARN] Echec creation tache planifiee (code $LASTEXITCODE)"
} else {
    Write-Host "  [OK] Tache planifiee : Launcher (logon, privileges eleves)"
}

Write-Host ""
Write-Host "Installation terminee : $InstallDir"
if (-not (Test-Path $configDst)) {
    Write-Host "Editez config.toml dans ce dossier pour configurer."
}
