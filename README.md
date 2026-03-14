# win-setup

Monorepo contenant deux binaires Windows pour une station de jeu TV :

| Binaire | Rôle |
|---------|------|
| [`launcher.exe`](launcher/README.md) | Orchestrateur de démarrage : bascule le PC en mode jeu ou bureau au démarrage Windows |
| [`cec-daemon.exe`](cec-daemon/README.md) | Daemon HDMI CEC : met la TV en veille/réveil automatiquement sur extinction d'écran et arrêt PC |

---

## Architecture

```
Windows démarre
  → launcher.exe (Run key)
      → mode bureau : restaure les services, plan eco, résolution 2560×1440 …
      → mode jeu    : optimise tout, lance Steam Big Picture + cec-daemon.exe
                            ↓
                       cec-daemon.exe (daemon persistant)
                            → extinction écran → TV standby
                            → réveil écran    → TV on + source HDMI
                            → shutdown PC     → TV standby
```

---

## Prérequis (build)

- **WSL Ubuntu 24.04** avec Rust + cible Windows GNU
- **MinGW cross-compiler**

```bash
# Une seule fois dans WSL
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env
rustup target add x86_64-pc-windows-gnu
sudo apt-get install -y build-essential gcc-mingw-w64-x86-64
```

---

## Build

```bash
cd /mnt/d/developpement.code/launcher
source ~/.cargo/env
cargo build --release
# → target/x86_64-pc-windows-gnu/release/launcher.exe
# → target/x86_64-pc-windows-gnu/release/cec-daemon.exe
```

---

## Installation

**1. Activer le hook pre-commit (une seule fois après un clone frais) :**
```bash
git config core.hooksPath .githooks
```

**2. Copier `config.toml` dans le même dossier que les `.exe` :**
```powershell
$dir = "D:\tools\launcher"
Copy-Item "D:\developpement.code\launcher\config.toml" $dir
Copy-Item "target\x86_64-pc-windows-gnu\release\launcher.exe" $dir
Copy-Item "target\x86_64-pc-windows-gnu\release\cec-daemon.exe" $dir
```

**3. Ajouter `launcher.exe` au démarrage Windows :**
```powershell
$exe = "D:\tools\launcher\launcher.exe"
Set-ItemProperty "HKCU:\Software\Microsoft\Windows\CurrentVersion\Run" `
    -Name "WinSetupLauncher" -Value $exe
```

**4. Supprimer du démarrage :**
```powershell
Remove-ItemProperty "HKCU:\Software\Microsoft\Windows\CurrentVersion\Run" `
    -Name "WinSetupLauncher"
```

> La release GitHub inclut `launcher.exe`, `cec-daemon.exe` et `config.toml` dans un zip prêt à l'emploi.

---

## Release

```bash
# 1. Bumper la version dans launcher/Cargo.toml (et cec-daemon/Cargo.toml si besoin)
git add launcher/Cargo.toml
git commit -m "chore: bump version to 0.2.0"
git push

# 2. Tagger → GitHub Actions build + publie automatiquement
git tag v0.2.0
git push origin v0.2.0
```

GitHub Actions cross-compile depuis Ubuntu + mingw-w64, package les deux binaires + `config.toml`
dans `launcher-vX.Y.Z.zip` et publie la release (~2-3 min).

Suivre l'avancement : onglet **Actions** → workflow **Release**.

---

## Documentation détaillée

- [launcher/README.md](launcher/README.md) — modes, modules, configuration complète, diagnostics
- [cec-daemon/README.md](cec-daemon/README.md) — daemon HDMI CEC, Modern Standby, utilisation standalone
