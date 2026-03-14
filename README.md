# Steam Big Picture Launcher

Au démarrage Windows, lance Steam en mode Big Picture (et optionnellement MSI Afterburner avec un profil). Tout le comportement est configurable via `config.toml`.

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

Le binaire est généré dans :
```
target/x86_64-pc-windows-gnu/release/launcher.exe
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
