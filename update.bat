@echo off
setlocal

net session >nul 2>&1
if %errorLevel% neq 0 (
    powershell -NoProfile -Command "Start-Process -FilePath '%~f0' -Verb RunAs"
    exit /b
)

set "PS1=%TEMP%\launcher_update.ps1"

echo $ErrorActionPreference = 'Stop'                                                                         > "%PS1%"
echo $repo = 'JordanAtDown/Launcher'                                                                        >> "%PS1%"
echo $tmp  = Join-Path $env:TEMP 'launcher-update'                                                          >> "%PS1%"
echo if (Test-Path $tmp) { Remove-Item $tmp -Recurse -Force }                                               >> "%PS1%"
echo New-Item -ItemType Directory -Path $tmp ^| Out-Null                                                    >> "%PS1%"
echo Write-Host '=== Launcher Updater ==='                                                                  >> "%PS1%"
echo Write-Host 'Recherche de la derniere release...'                                                       >> "%PS1%"
echo $api = Invoke-RestMethod -Uri "https://api.github.com/repos/$repo/releases/latest" -UseBasicParsing    >> "%PS1%"
echo $ver = $api.tag_name                                                                                   >> "%PS1%"
echo Write-Host "Version disponible : $ver"                                                                 >> "%PS1%"
echo $asset = $api.assets ^| Where-Object { $_.name -like 'launcher-*.zip' } ^| Select-Object -First 1     >> "%PS1%"
echo if (-not $asset) { Write-Error 'Asset zip introuvable dans la release GitHub'; exit 1 }               >> "%PS1%"
echo Write-Host "Telechargement $($asset.name)..."                                                          >> "%PS1%"
echo $zip = Join-Path $tmp $asset.name                                                                      >> "%PS1%"
echo Invoke-WebRequest -Uri $asset.browser_download_url -OutFile $zip -UseBasicParsing                      >> "%PS1%"
echo Write-Host 'Extraction...'                                                                             >> "%PS1%"
echo Expand-Archive -Path $zip -DestinationPath $tmp -Force                                                 >> "%PS1%"
echo $ps1 = Get-ChildItem -Path $tmp -Filter 'install.ps1' -Recurse ^| Select-Object -First 1              >> "%PS1%"
echo if (-not $ps1) { Write-Error 'install.ps1 introuvable dans le zip'; exit 1 }                          >> "%PS1%"
echo Write-Host 'Installation...'                                                                           >> "%PS1%"
echo ^& $ps1.FullName                                                                                       >> "%PS1%"
echo Remove-Item $tmp -Recurse -Force -ErrorAction SilentlyContinue                                        >> "%PS1%"
echo Write-Host ''                                                                                          >> "%PS1%"
echo Write-Host 'Mise a jour terminee !'                                                                    >> "%PS1%"

powershell -NoProfile -ExecutionPolicy Bypass -File "%PS1%"
del "%PS1%" 2>nul

echo.
pause
