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
  → launcher.exe (tâche planifiée, logon, privilèges élevés)
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

### Première installation

1. Récupérer le zip `launcher-vX.Y.Z.zip` depuis la page [Releases](../../releases) et l'extraire
2. Double-cliquer sur **`install.bat`** → UAC → installation automatique

Le script copie les binaires dans `%LOCALAPPDATA%\Programs\Launcher`, copie `config.toml` (première fois uniquement) et `update.bat`, et crée une tâche planifiée au logon avec privilèges élevés.

### Mise à jour

Double-cliquer sur **`update.bat`** (présent dans le dossier d'installation après la première install) :
- télécharge automatiquement la dernière release GitHub
- installe sans intervention manuelle
- préserve `config.toml`

Pas besoin de télécharger le zip manuellement.

### Désinstallation

Double-cliquer sur **`uninstall.bat`** depuis le dossier extrait.

### Vérifier la tâche planifiée

```powershell
Get-ScheduledTask -TaskName "Launcher" | Select-Object TaskName, State
(Get-ScheduledTask -TaskName "Launcher").Actions | Select-Object Execute
```

> La release GitHub inclut les binaires, `config.toml`, les scripts `.ps1` et les fichiers `.bat` prêts à l'emploi dans un zip.

---

## Release

```bash
# Depuis WSL, à la racine du repo
bash scripts/release.sh 0.3.1
```

Le script bumpe la version, commit, tag et push. GitHub Actions prend le relais.

GitHub Actions cross-compile depuis Ubuntu + mingw-w64, package les deux binaires + `config.toml`
dans `launcher-vX.Y.Z.zip` et publie la release (~2-3 min).

Suivre l'avancement : onglet **Actions** → workflow **Release**.

---

## Documentation détaillée

- [launcher/README.md](launcher/README.md) — modes, modules, configuration complète, diagnostics
- [cec-daemon/README.md](cec-daemon/README.md) — daemon HDMI CEC, Modern Standby, utilisation standalone
