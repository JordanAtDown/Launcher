# launcher

Orchestrateur de démarrage Windows. Au boot, il détecte le contexte (TV connectée = mode jeu, écran bureau = mode bureau) et configure la machine en conséquence : résolution, son, plan d'alimentation, services, overclocking, puis lance Steam.

---

## Comportement au démarrage

```
Windows démarre
  → Explorer
  → launcher.exe (Run key HKCU\...\Run)
      │
      ├─ monitor::resolve_mode()
      │     → mode "auto"  : TV LG connectée ?  oui → game, non → desktop
      │     → mode "game"  : force game
      │     → mode "desktop": force desktop
      │
      ├─ mode jeu ──────────────────────────────────────────────────────┐
      │   cec power_on + set_source                                     │
      │   display : 1920×1080 + scale 175%                              │
      │   sound   : device LG (HDMI TV)                                 │
      │   wsl --shutdown                                                │
      │   HAGS enable                                                   │
      │   TimerResolution 0.5ms                                         │
      │   GameBar uninstall                                             │
      │   GameMode enable                                               │
      │   Windows Update pause                                          │
      │   Notifications disable                                         │
      │   PowerPlan → Gaming                                            │
      │   killist : sc stop WSearch, SysMain, Spooler …                │
      │   disable_services : désactivation persistante                  │
      │   RTSS : limit fps 0                                            │
      │   Afterburner : profile 1                                       │
      │   startup.game : exécutables à lancer                          │
      │   Steam Big Picture                                             │
      │   cec-daemon.exe (daemon TV veille/réveil)                      │
      │                                                                 │
      └─ mode bureau ───────────────────────────────────────────────────┘
          display : 2560×1440 + scale 125%
          sound   : device Speakers
          HAGS disable
          PowerPlan → Eco
          killist restore / disable_services restore
          startup.desktop : exécutables à lancer
```

---

## Modes

| Mode | Déclenchement | Comportement |
|------|--------------|--------------|
| `game` | TV LG détectée (ou `mode = "game"`) | Optimise tout, lance Steam |
| `desktop` | TV absente (ou `mode = "desktop"`) | Restaure les réglages bureau |

---

## Modules — référence complète

Chaque module est contrôlé par une section dans `config.toml`. Supprimer une section ou laisser un champ vide désactive silencieusement le module.

### `[monitor]` — Détection du mode

| Paramètre | Valeurs | Description |
|-----------|---------|-------------|
| `name` | chaîne | Sous-chaîne du nom du moniteur à détecter (insensible à la casse) |
| `mode` | `"auto"` / `"game"` / `"desktop"` | `"auto"` = détection par moniteur connecté |

```toml
[monitor]
name = "LG"
mode = "auto"
```

---

### `[display]` — Résolution et mise à l'échelle

Change la résolution via `QRes.exe` (immédiat) et la mise à l'échelle Windows via le registre (effet au prochain démarrage). Chaque mode a sa propre résolution et son propre zoom.

La mise à l'échelle se définit en `%` — la conversion vers la valeur interne Windows (LogPixels) est automatique.

| Paramètre | Description |
|-----------|-------------|
| `exe_path` | Chemin vers `QRes.exe` (absent = résolution désactivée) |
| `game_width` / `game_height` | Résolution en mode jeu |
| `game_scale` | Zoom mode jeu en % (`100`, `125`, `150`, `175`, `200`) |
| `desktop_width` / `desktop_height` | Résolution en mode bureau |
| `desktop_scale` | Zoom mode bureau en % |
| `refresh_rate` | Hz (`0` = ne pas changer) |

> `QRes.exe` : standalone, pas d'installation.

---

### `[sound]` — Redirection audio

Redirige le périphérique audio principal selon le mode via `svcl.exe` (SoundVolumeCommandLine de NirSoft). Avec `auto_detect = true`, vérifie que le device est visible avant de switcher — si la TV est éteinte, le switch est ignoré proprement.

Pour trouver le bon nom : ouvrir `SoundVolumeView.exe` en GUI, colonne **Name**.

| Paramètre | Description |
|-----------|-------------|
| `exe_path` | Chemin vers `svcl.exe` ou `SoundVolumeView.exe` |
| `game_device` | Sous-chaîne du nom du device en mode jeu (ex: `"LG"`) |
| `desktop_device` | Sous-chaîne du nom du device en mode bureau (ex: `"Speakers"`) |
| `auto_detect` | `true` = vérifie que le device est disponible avant de switcher |

---

### `[wsl]` — Windows Subsystem for Linux

Arrête WSL en mode jeu pour libérer CPU, RAM et la VM légère Hyper-V.

| Paramètre | Description |
|-----------|-------------|
| `enabled` | `true` = exécute `wsl --shutdown` en mode jeu |

---

### `[hags]` — Hardware-Accelerated GPU Scheduling

Active HAGS en mode jeu, le désactive en mode bureau. Aucun paramètre — appliqué systématiquement. Nécessite les droits admin.

---

### `[timerresolution]` — Résolution du timer Windows

Lance `TimerResolution.exe` en mode jeu. La résolution revient à 15.625ms à la fermeture du processus.

| Paramètre | Description |
|-----------|-------------|
| `path` | Chemin vers `TimerResolution.exe` |
| `resolution` | Résolution en ms (ex: `0.5`) |

---

### `[gamebar]` — Xbox Gaming Overlay

Désinstalle `Microsoft.XboxGamingOverlay` et désactive Game DVR via le registre.

| Paramètre | Description |
|-----------|-------------|
| `uninstall` | `true` = désinstalle l'overlay Xbox et désactive Game DVR |

---

### `[gamemode]` — Game Mode Windows

Active la priorité Game Mode via le registre `HKCU`.

| Paramètre | Description |
|-----------|-------------|
| `enabled` | `true` = active `AutoGameModeEnabled` en mode jeu |

---

### `[updates]` — Windows Update

Stoppe le service `wuauserv` en mode jeu.

| Paramètre | Description |
|-----------|-------------|
| `pause_in_game` | `true` = stoppe Windows Update au démarrage du mode jeu |

---

### `[notifications]` — Notifications toast

Désactive les notifications Windows pendant la session de jeu.

| Paramètre | Description |
|-----------|-------------|
| `disable_in_game` | `true` = coupe les notifications toast en mode jeu |

---

### `[powerplan]` — Plan d'alimentation

Change le plan d'alimentation Windows. Lister les GUIDs : `powercfg /L` (terminal admin).

| Paramètre | Description |
|-----------|-------------|
| `game` | GUID du plan à activer en mode jeu (`""` = désactivé) |
| `desktop` | GUID du plan à activer en mode bureau (`""` = désactivé) |

---

### `[killist]` — Kill list (temporaire)

Arrête des services (`sc stop`) et tue des processus (`taskkill`) pour libérer des ressources en mode jeu. L'arrêt est **temporaire** : les services redémarrent normalement au prochain boot.

| Paramètre | Description |
|-----------|-------------|
| `services` | Noms courts de services Windows (ex: `["WSearch", "SysMain"]`) |
| `processes` | Noms d'exécutables (ex: `["OneDrive.exe"]`) |

> Trouver le nom court : `services.msc` → double-clic → onglet **Général** → **Nom du service**

---

### `[disable_services]` — Désactivation persistante

Désactive des services (`sc config start= disabled`) en mode jeu et les restaure en démarrage **Manuel** en mode bureau. La désactivation **persiste** aux boots suivants.

| Paramètre | Description |
|-----------|-------------|
| `services` | Noms courts des services à désactiver (ex: `["WSearch", "WacomProfessional"]`) |

> **killist vs disable_services** : utilisez `killist` pour libérer des ressources pendant la session. Utilisez `disable_services` pour des services que vous ne voulez jamais actifs en mode jeu, même après un redémarrage.

---

### `[rtss]` — RivaTuner Statistics Server

Modifie la limite de framerate dans le profil Global de RTSS, puis lance RTSS.

| Paramètre | Description |
|-----------|-------------|
| `path` | Chemin vers `RTSS.exe` |
| `profile_path` | Chemin vers le fichier profil Global de RTSS |
| `game_limit` | Limite fps en mode jeu (`0` = illimité) |
| `desktop_limit` | Limite fps en mode bureau |

---

### `[afterburner]` — MSI Afterburner

Lance MSI Afterburner avec un profil d'overclocking différent selon le mode (argument `/ProfileN`).

| Paramètre | Description |
|-----------|-------------|
| `path` | Chemin vers `MSIAfterburner.exe` |
| `game_profile` | Profil à charger en mode jeu (1 à 5) |
| `desktop_profile` | Profil à charger en mode bureau (1 à 5, optionnel) |

---

### `[startup]` — Démarrage d'applications

Lance une liste d'exécutables selon le mode.

| Paramètre | Description |
|-----------|-------------|
| `desktop` | Chemins complets des exécutables à lancer en mode bureau |
| `game` | Chemins complets des exécutables à lancer en mode jeu |

```toml
[startup]
desktop = ["C:\\Program Files\\Synology\\SynologyDrive\\bin\\launcher.exe"]
game    = []
```

---

### `[steam]` — Steam

Lance Steam en mode jeu uniquement.

| Paramètre | Description |
|-----------|-------------|
| `path` | Chemin vers `steam.exe` |
| `args` | Arguments passés à Steam (ex: `["-bigpicture"]`) |

---

### `[cec]` — Contrôle TV HDMI CEC

Allume la TV et bascule sur l'entrée HDMI au démarrage du mode jeu. Optionnellement lance `cec-daemon.exe` pour la veille/réveil automatique.

Requiert l'adaptateur USB-CEC [Pulse-Eight](https://www.pulse-eight.com/p/104/usb-hdmi-cec-adapter).

| Paramètre | Description |
|-----------|-------------|
| `enabled` | `true` = active le contrôle CEC au démarrage du mode jeu |
| `client_path` | Chemin vers `cec-client.exe` (fourni par le driver Pulse-Eight) |
| `daemon` | `true` = lance `cec-daemon.exe` en fin de mode jeu |

```toml
[cec]
enabled = true
client_path = 'C:\Program Files (x86)\Pulse-Eight\USB-CEC Adapter\cec-client.exe'
daemon = true
```

> Pour la documentation complète du daemon : [cec-daemon/README.md](../cec-daemon/README.md)

---

### `[logging]` — Fichier de log

| Paramètre | Description |
|-----------|-------------|
| `path` | Chemin du fichier de log (défaut : `launcher.log` à côté du `.exe`) |

---

## Diagnostics

Le launcher écrit `launcher.log` dans le même dossier que le `.exe` (configurable).

### Format des entrées

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
| `xxx ... FAIL` | Étape échouée — lire le `[WARN]` juste au-dessus |
| `path not found: C:\...` | Chemin incorrect dans `config.toml` |
| `spawn error: ...` | Erreur OS au lancement (permissions, fichier manquant…) |
| `spawned pid=XXXX` | Processus démarré — croiser avec le Gestionnaire des tâches |
| `exit=Some(1)` | Commande terminée avec code d'erreur |
| `killist: sc stop X exit=...` | Service déjà arrêté (normal) ou droits insuffisants |

Le fichier est en **append** — chaque démarrage ajoute des lignes sans effacer les précédentes.

---

## Rebuild après modification

```bash
cd /mnt/d/developpement.code/launcher
source ~/.cargo/env
cargo build --release
```
