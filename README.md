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

### Télécharger la release

Récupérer le zip `launcher-vX.Y.Z.zip` depuis la page [Releases](../../releases), extraire dans un dossier temporaire.

### Lancer le script d'installation

Le script nécessite des droits administrateur (création de tâche planifiée) et PowerShell doit être autorisé à exécuter des scripts.

**Méthode recommandée — PowerShell admin, bypass de politique :**
```powershell
# Ouvrir PowerShell en administrateur, se placer dans le dossier extrait
cd "C:\chemin\vers\launcher-vX.Y.Z"
PowerShell -ExecutionPolicy Bypass -File install.ps1
```

> **Pourquoi `-ExecutionPolicy Bypass` ?**
> Windows bloque par défaut l'exécution de scripts `.ps1` non signés.
> Ce flag s'applique uniquement à la session courante, sans modifier la politique globale.

Le script :
- copie `launcher-X.Y.Z.exe` et `cec-daemon-X.Y.Z.exe` dans `%LOCALAPPDATA%\Programs\Launcher`
- copie `config.toml` uniquement à la **première installation** (préservé lors des mises à jour)
- crée une **tâche planifiée** `Launcher` qui s'exécute au logon avec privilèges élevés (sans prompt UAC)

### Désinstaller

```powershell
PowerShell -ExecutionPolicy Bypass -File uninstall.ps1
```

### Vérifier la tâche planifiée

```powershell
Get-ScheduledTask -TaskName "Launcher" | Select-Object TaskName, State
(Get-ScheduledTask -TaskName "Launcher").Actions | Select-Object Execute
```

> La release GitHub inclut `launcher-X.Y.Z.exe`, `cec-daemon-X.Y.Z.exe`, `config.toml`, `install.ps1` et `uninstall.ps1` dans un zip prêt à l'emploi.

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
