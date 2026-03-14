# Steam Big Picture Launcher

Monorepo contenant deux binaires Windows :
- **`launcher.exe`** — orchestrateur de démarrage : lance les bons programmes selon le mode (jeu / bureau), configurable via `config.toml`
- **`cec-daemon.exe`** — daemon autonome : met la TV en veille/réveil via HDMI CEC sur extinction d'écran et arrêt PC (couvre Modern Standby S0 + S3 + shutdown)

## Comportement au démarrage

```
Windows démarre
    → Explorer (shell normal)
    → launcher.exe (via Run key)
        → game_mode = false → quitte (bureau normal)
        → game_mode = true  →
            → MSI Afterburner /ProfileN  (si configuré)
            → Steam Big Picture
            → quitte
```

---

## Configuration

Tous les paramètres sont dans `config.toml`, dans le même dossier que le `.exe`. Aucune valeur n'est codée en dur — tout est modifiable sans recompiler.

```toml
game_mode = true          # false = mode bureau, le launcher quitte sans rien faire

steam_path = "C:\\Program Files (x86)\\Steam\\steam.exe"
steam_args = ["-bigpicture"]

# MSI Afterburner — optionnel, supprimer ou commenter pour désactiver
afterburner_path = "C:\\Program Files (x86)\\MSI Afterburner\\MSIAfterburner.exe"
afterburner_profile = 1   # profil 1 à 5
```

### Référence des paramètres

| Paramètre | Type | Défaut | Description |
|---|---|---|---|
| `game_mode` | bool | `false` | `true` = lance Steam BP, `false` = quitte immédiatement |
| `steam_path` | string | chemin Steam par défaut | Chemin complet vers `steam.exe` |
| `steam_args` | liste | `["-bigpicture"]` | Arguments passés à Steam |
| `afterburner_path` | string | *(absent = désactivé)* | Chemin complet vers `MSIAfterburner.exe` |
| `afterburner_profile` | entier 1–5 | *(absent = pas de profil)* | Profil MSI Afterburner à charger au démarrage |

> Si `config.toml` est absent, `game_mode = false` et Steam n'est pas lancé.

---

## MSI Afterburner — Profils

MSI Afterburner accepte l'argument `/Profile1` à `/Profile5` en ligne de commande pour charger un profil d'overclocking. Le launcher passe automatiquement `/ProfileN` selon la valeur de `afterburner_profile`.

Pour désactiver MSI Afterburner sans supprimer la ligne, commenter avec `#` :
```toml
# afterburner_path = "..."
```

---

## Prérequis (build)

- WSL Ubuntu 24.04
- Rust + cible Windows GNU
- MinGW cross-compiler

### Installer les outils (une seule fois)

Dans WSL Ubuntu :

```bash
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env

# Cible Windows
rustup target add x86_64-pc-windows-gnu

# Compilateurs
sudo apt-get install -y build-essential gcc-mingw-w64-x86-64
```

---

## Build

Dans WSL Ubuntu :

```bash
cd /mnt/d/developpement.code/launcher
source ~/.cargo/env
cargo build --release
```

Les binaires sont générés dans :
```
target/x86_64-pc-windows-gnu/release/launcher.exe
target/x86_64-pc-windows-gnu/release/cec-daemon.exe
```

---

## Installation

### 1. Copier config.toml à côté du .exe

```powershell
$exe = "D:\developpement.code\launcher\target\x86_64-pc-windows-gnu\release\launcher.exe"
copy "D:\developpement.code\launcher\config.toml" (Split-Path $exe)
```

### 2. Ajouter au démarrage Windows

```powershell
$exe = "D:\developpement.code\launcher\target\x86_64-pc-windows-gnu\release\launcher.exe"
Set-ItemProperty "HKCU:\Software\Microsoft\Windows\CurrentVersion\Run" -Name "SteamLauncher" -Value $exe
```

### 3. Redémarrer le PC

---

## Désactiver

```powershell
Remove-ItemProperty "HKCU:\Software\Microsoft\Windows\CurrentVersion\Run" -Name "SteamLauncher"
```

---

## Modules

Chaque module est activé/désactivé et configuré via `config.toml`. Supprimer une section ou laisser un champ vide désactive silencieusement le module.

### `[monitor]` — Détection du mode
Détermine si le launcher s'exécute en mode jeu ou bureau.

| Paramètre | Valeurs | Description |
|-----------|---------|-------------|
| `name` | chaîne | Sous-chaîne du nom du moniteur à détecter (insensible à la casse) |
| `mode` | `"auto"` / `"game"` / `"desktop"` | `"auto"` = détection par moniteur, les deux autres forcent le mode |

### `[hags]` — Hardware-Accelerated GPU Scheduling
Active HAGS en mode jeu, le désactive en mode bureau. Pas de paramètre — toujours appliqué. Nécessite les droits admin.

### `[timerresolution]` — Résolution du timer Windows
Lance TimerResolution.exe en mode jeu. Ne fait rien en mode bureau (la résolution revient à 15.625ms à la fermeture du processus).

| Paramètre | Défaut | Description |
|-----------|--------|-------------|
| `path` | *(absent = désactivé)* | Chemin complet vers `TimerResolution.exe` |
| `resolution` | `0.5` | Résolution en ms |

### `[gamebar]` — Xbox Gaming Overlay
Désinstalle le package `Microsoft.XboxGamingOverlay` et désactive Game DVR via le registre.

| Paramètre | Défaut | Description |
|-----------|--------|-------------|
| `uninstall` | `false` | `true` = désinstalle l'overlay Xbox et désactive la capture DVR |

### `[gamemode]` — Game Mode Windows
Active la priorité Game Mode via le registre HKCU.

| Paramètre | Défaut | Description |
|-----------|--------|-------------|
| `enabled` | `false` | `true` = active `AutoGameModeEnabled` en mode jeu |

### `[updates]` — Windows Update
Arrête le service `wuauserv` en mode jeu.

| Paramètre | Défaut | Description |
|-----------|--------|-------------|
| `pause_in_game` | `false` | `true` = stoppe Windows Update au démarrage du mode jeu |

### `[notifications]` — Notifications toast
Désactive les notifications Windows pendant la session de jeu.

| Paramètre | Défaut | Description |
|-----------|--------|-------------|
| `disable_in_game` | `false` | `true` = coupe les notifications toast en mode jeu |

### `[powerplan]` — Plan d'alimentation
Change le plan d'alimentation Windows selon le mode. Lister les GUIDs : `powercfg /L` dans un terminal admin.

| Paramètre | Défaut | Description |
|-----------|--------|-------------|
| `game` | `""` (désactivé) | GUID du plan à activer en mode jeu |
| `desktop` | `""` (désactivé) | GUID du plan à activer en mode bureau |

### `[killist]` — Kill list
Arrête des services (`sc stop`) et tue des processus (`taskkill`) pour libérer des ressources en mode jeu. L'arrêt est **temporaire** : les services redémarrent normalement au prochain boot.

| Paramètre | Défaut | Description |
|-----------|--------|-------------|
| `services` | `[]` | Noms courts de services Windows (ex: `["DiagTrack"]`) |
| `processes` | `[]` | Noms d'exécutables (ex: `["OneDrive.exe"]`) |

### `[disable_services]` — Désactivation de services
Désactive des services (`sc config start= disabled`) en mode jeu et les restaure en démarrage Manuel en mode bureau. La désactivation est **persistante** : le service ne redémarre pas non plus aux boots suivants tant qu'on reste en mode jeu.

> **killist vs disable_services** — utilisez `killist` pour juste libérer des ressources pendant la session. Utilisez `disable_services` pour des services que vous ne voulez jamais voir tourner en mode jeu, même après un redémarrage (ex: Windows Search, Wacom).

> Trouver le nom court : `services.msc` → double-clic → onglet **Général** → champ **Nom du service**

| Paramètre | Défaut | Description |
|-----------|--------|-------------|
| `services` | `[]` | Noms courts des services à désactiver en mode jeu (ex: `["WSearch", "WacomProfessional"]`) |

### `[rtss]` — RivaTuner Statistics Server
Modifie la limite de framerate dans le profil Global de RTSS, puis lance RTSS.

| Paramètre | Défaut | Description |
|-----------|--------|-------------|
| `path` | *(absent = désactivé)* | Chemin vers `RTSS.exe` |
| `profile_path` | *(absent)* | Chemin vers le fichier profil Global de RTSS |
| `game_limit` | *(absent)* | Limite fps en mode jeu (`0` = illimité) |
| `desktop_limit` | *(absent)* | Limite fps en mode bureau |

### `[display]` — Résolution et mise à l'échelle
Change la résolution d'écran (immédiat via `QRes.exe`) et la mise à l'échelle Windows (registre, effet au prochain démarrage). Chaque mode peut avoir sa propre résolution et son propre zoom.

La mise à l'échelle se définit en `%` uniquement — la conversion vers la valeur interne Windows (LogPixels) est faite automatiquement.

| Paramètre | Défaut | Description |
|-----------|--------|-------------|
| `exe_path` | *(absent = résolution désactivée)* | Chemin vers `QRes.exe` |
| `game_width` / `game_height` | *(absent)* | Résolution en mode jeu (ex: 1920, 1080) |
| `game_scale` | *(absent)* | Mise à l'échelle mode jeu en % (`100`, `125`, `150`, `175`, `200`) |
| `desktop_width` / `desktop_height` | *(absent)* | Résolution en mode bureau (ex: 2560, 1440) |
| `desktop_scale` | *(absent)* | Mise à l'échelle mode bureau en % |
| `refresh_rate` | `0` (inchangé) | Fréquence de rafraîchissement en Hz |

> `QRes.exe` : standalone, pas d'installation requise.

### `[sound]` — Redirection audio
Redirige le périphérique audio principal selon le mode, via `svcl.exe` (SoundVolumeCommandLine de NirSoft). En mode `auto_detect`, vérifie d'abord que le périphérique est visible par Windows — si la TV est éteinte ou débranchée, le switch est ignoré proprement.

Pour trouver le bon nom : ouvrir `SoundVolumeView.exe` en GUI, colonne **Name** de la ligne correspondant au device cible. Utiliser une sous-chaîne distinctive (ex: `"LG"` ou `"HDMI"`).

| Paramètre | Défaut | Description |
|-----------|--------|-------------|
| `exe_path` | *(absent = désactivé)* | Chemin vers `svcl.exe` ou `SoundVolumeView.exe` |
| `game_device` | *(absent)* | Sous-chaîne du nom du device en mode jeu (ex: TV via HDMI) |
| `desktop_device` | *(absent)* | Sous-chaîne du nom du device en mode bureau (ex: enceintes) |
| `auto_detect` | `false` | `true` = vérifie que le device est visible avant de switcher |

### `[wsl]` — Windows Subsystem for Linux
Arrête WSL en mode jeu pour libérer CPU, RAM et la VM légère Hyper-V. WSL redémarre automatiquement à la demande en mode bureau.

| Paramètre | Défaut | Description |
|-----------|--------|-------------|
| `enabled` | `false` | `true` = exécute `wsl --shutdown` en mode jeu |

### `[startup]` — Démarrage d'applications
Lance une liste d'exécutables selon le mode. Utile pour démarrer automatiquement Discord, Spotify, ou tout autre outil en mode bureau, et des overlays ou outils en mode jeu.

| Paramètre | Défaut | Description |
|-----------|--------|-------------|
| `desktop` | `[]` | Chemins complets des exécutables à lancer en mode bureau |
| `game` | `[]` | Chemins complets des exécutables à lancer en mode jeu |

```toml
[startup]
desktop = [
    "C:\\Program Files\\Discord\\Discord.exe",
    "C:\\Users\\jorda\\AppData\\Roaming\\Spotify\\Spotify.exe",
]
```

### `[cec]` — Contrôle TV HDMI CEC (Pulse-Eight)

Allume la TV et bascule sur l'entrée HDMI du PC au démarrage du mode jeu, via l'adaptateur USB-CEC [Pulse-Eight](https://www.pulse-eight.com/p/104/usb-hdmi-cec-adapter) et son outil `cec-client.exe`.

| Paramètre | Défaut | Description |
|-----------|--------|-------------|
| `enabled` | `false` | `true` = active le contrôle CEC au démarrage du mode jeu |
| `client_path` | *(absent = désactivé)* | Chemin vers `cec-client.exe` (fourni par le driver Pulse-Eight) |
| `daemon` | `false` | `true` = lance `cec-daemon.exe` en fin de mode jeu (veille/réveil automatique) |

> La source HDMI est auto-détectée : `cec-client` diffuse l'adresse physique de l'adaptateur via le bus CEC, la TV switch automatiquement sur le bon port — aucune configuration de source nécessaire.

#### Prérequis

1. Adaptateur USB-CEC Pulse-Eight branché entre le PC (HDMI) et la TV (HDMI), + câble USB vers le PC
2. Driver Pulse-Eight installé → fournit `cec-client.exe` dans `C:\Program Files (x86)\Pulse-Eight\USB-CEC Adapter\`

#### Tester en PowerShell avant d'activer

```powershell
$cec = 'C:\Program Files (x86)\Pulse-Eight\USB-CEC Adapter\cec-client.exe'

# Allumer la TV
& $cec -s -c "on 0"

# Basculer la TV sur l'entrée HDMI du PC (Active Source)
& $cec -s -c "as"

# Mettre la TV en veille
& $cec -s -c "standby 0"

# Lister les appareils CEC détectés sur le bus
& $cec -l
```

> Si `cec-client.exe` n'est pas dans ce chemin, chercher avec : `Get-ChildItem 'C:\Program Files*' -Recurse -Filter 'cec-client.exe' -ErrorAction SilentlyContinue`

#### Configuration

```toml
[cec]
enabled = true
client_path = 'C:\Program Files (x86)\Pulse-Eight\USB-CEC Adapter\cec-client.exe'
daemon = true   # lance cec-daemon.exe pour la veille/réveil automatique
```

#### `cec-daemon.exe` — veille et réveil automatiques

`cec-daemon.exe` est un daemon autonome lancé en fin de mode jeu. Il garde une connexion CEC ouverte et réagit à :

| Événement | Action TV |
|-----------|-----------|
| Extinction d'écran (veille S3, Modern Standby S0, inactivité 30 min) | Standby TV |
| Rallumage d'écran (réveil souris/clavier) | TV allumée + source HDMI active |
| Arrêt PC (shutdown) | Standby TV |

**Pourquoi `GUID_CONSOLE_DISPLAY_STATE` et non `PBT_APMSUSPEND` :**
Modern Standby (S0) est le mode veille par défaut depuis Windows 10 — `PBT_APMSUSPEND` n'est pas déclenché. `GUID_CONSOLE_DISPLAY_STATE` se déclenche pour tous les types de veille car l'écran s'éteint toujours.

Le daemon est à côté de `launcher.exe` dans la release zip. Il se lance avec :
```
cec-daemon.exe --path "C:\Program Files (x86)\Pulse-Eight\USB-CEC Adapter\cec-client.exe"
```

---

### `[afterburner]` — MSI Afterburner
Lance MSI Afterburner avec un profil d'overclocking différent selon le mode.

| Paramètre | Défaut | Description |
|-----------|--------|-------------|
| `path` | *(absent = désactivé)* | Chemin vers `MSIAfterburner.exe` |
| `game_profile` | *(absent)* | Profil à charger en mode jeu (1 à 5) |
| `desktop_profile` | *(absent)* | Profil à charger en mode bureau (1 à 5) |

### `[steam]` — Steam
Lance Steam avec les arguments configurés (mode jeu uniquement).

| Paramètre | Défaut | Description |
|-----------|--------|-------------|
| `path` | chemin Steam par défaut | Chemin complet vers `steam.exe` |
| `args` | `["-bigpicture"]` | Arguments passés à Steam au démarrage |

### `[logging]` — Fichier de log
Configure l'emplacement du fichier de log.

| Paramètre | Défaut | Description |
|-----------|--------|-------------|
| `path` | `launcher.log` à côté du `.exe` | Chemin complet vers le fichier de log |

---

## Diagnostiquer avec les logs

Le launcher écrit un fichier `launcher.log` dans le même dossier que le `.exe` (configurable dans `config.toml`).

### Format d'une entrée

```
[2026-03-13 18:42:01] [INFO]  steam spawned pid=4521
[2026-03-13 18:42:01] [INFO]  steam::launch ... ok
[2026-03-13 18:42:01] [WARN]  afterburner: path not found: C:\...\MSIAfterburner.exe
[2026-03-13 18:42:01] [WARN]  afterburner::launch ... FAIL
```

### Interpréter les lignes

| Ce que tu vois | Signification |
|----------------|---------------|
| `xxx ... ok` | Étape réussie |
| `xxx ... FAIL` | Étape échouée — lire la ligne `[WARN]` juste au-dessus pour la cause |
| `path not found: C:\...` | Le chemin dans `config.toml` est incorrect |
| `spawn error: ...` | Erreur OS au lancement (permissions, fichier corrompu…) |
| `spawned pid=XXXX` | Processus démarré — croiser avec le Gestionnaire des tâches |
| `exit=Some(1)` | Commande terminée avec code d'erreur |
| `killist: sc stop X exit=...` | Le service X n'a pas pu être arrêté (déjà arrêté = normal) |

### Localisation du log

Par défaut : `launcher.log` à côté du `.exe`.
Pour changer :
```toml
[logging]
path = "C:\\Users\\jorda\\Desktop\\launcher.log"
```

Le fichier est en **append** — chaque démarrage ajoute des lignes sans effacer les précédentes.

---

## Après modification du code

Rebuilder depuis WSL :

```bash
cd /mnt/d/developpement.code/launcher
source ~/.cargo/env
cargo build --release
```

---

## Publier une release

Le projet utilise GitHub Actions pour builder et publier automatiquement une release quand un tag est poussé.

### Processus complet

**1. Modifier le code** puis commiter :
```bash
git add .
git commit -m "feat: description de la modification"
git push
```

**2. Bumper la version** dans `launcher/Cargo.toml` (et `cec-daemon/Cargo.toml` si besoin) :
```toml
[package]
version = "0.2.0"   # ← incrémenter ici
```

**3. Commiter la nouvelle version :**
```bash
git add launcher/Cargo.toml
git commit -m "chore: bump version to 0.2.0"
git push
```

**4. Créer et pousser le tag :**
```bash
git tag v0.2.0
git push origin v0.2.0
```

GitHub Actions se déclenche automatiquement et :
- Build `launcher.exe` + `cec-daemon.exe` via cross-compilation Windows (ubuntu + mingw-w64)
- Package `launcher.exe` + `cec-daemon.exe` + `config.toml` dans `launcher-v0.2.0.zip`
- Publie la release sur GitHub avec le changelog automatique

### Suivre le build

Onglet **Actions** du dépôt GitHub → workflow **Release** → vérifier que le job passe en vert (~2-3 min).

### Télécharger la release

Onglet **Releases** → dernière release → télécharger `launcher-vX.Y.Z.zip`.

> Le zip contient `launcher.exe`, `cec-daemon.exe` et `config.toml`. Extraire les trois dans le même dossier.
