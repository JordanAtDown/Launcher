# cec-daemon

Daemon Windows autonome qui garde une connexion HDMI CEC ouverte et envoie des commandes standby/réveil à la TV selon l'état de l'écran et les événements système.

Lancé automatiquement par `launcher.exe` en fin de mode jeu, ou manuellement en ligne de commande.

---

## Événements gérés

| Événement Windows | Réaction TV |
|-------------------|-------------|
| Extinction d'écran (veille S3) | Standby |
| Extinction d'écran (Modern Standby S0) | Standby |
| Inactivité → écran éteint automatiquement | Standby |
| Arrêt PC (`shutdown`) | Standby |
| Rallumage d'écran (réveil souris/clavier) | Allumage + source HDMI active |

---

## Pourquoi `GUID_CONSOLE_DISPLAY_STATE` et non `PBT_APMSUSPEND`

Windows 10/11 utilise **Modern Standby (S0)** par défaut sur la majorité des machines modernes :

| | S3 (veille classique) | S0 (Modern Standby) |
|---|---|---|
| CPU | S'arrête complètement | Reste actif à très basse conso |
| Réseau | Coupé | Reste connecté |
| `PBT_APMSUSPEND` | Déclenché | **Pas déclenché** |
| Écran | S'éteint | S'éteint aussi |

`PBT_APMSUSPEND` ne se déclenche pas en S0 — il est donc inutilisable pour détecter la mise en veille sur les machines modernes.

`GUID_CONSOLE_DISPLAY_STATE` se déclenche **dans tous les cas** car l'écran s'éteint toujours, quelle que soit la méthode de veille.

---

## Architecture interne

```
cec-daemon.exe --path <cec-client.exe>
  │
  ├─ spawn cec-client.exe -s  (stdin pipe, connexion CEC maintenue ouverte)
  │
  ├─ SetConsoleCtrlHandler
  │    └─ CTRL_SHUTDOWN_EVENT → "standby 0"
  │
  └─ Fenêtre cachée + RegisterPowerSettingNotification(GUID_CONSOLE_DISPLAY_STATE)
       ├─ Écran OFF (valeur 0) → "standby 0"
       └─ Écran ON  (valeur 1) → "on 0"  (attente 200ms)  → "as"
```

**Pourquoi stdin-pipe et non libcec-sys :**
`libcec-sys` utilise des `.lib` MSVC incompatibles avec la toolchain MinGW-w64 utilisée pour la cross-compilation depuis WSL. Le pipe stdin vers `cec-client -s` maintient la connexion CEC ouverte — réponse < 100ms, suffisant pour tous les cas.

---

## Prérequis matériels

1. Adaptateur [Pulse-Eight USB-CEC](https://www.pulse-eight.com/p/104/usb-hdmi-cec-adapter) branché entre le port HDMI du PC et un port HDMI de la TV + câble USB vers le PC
2. Driver Pulse-Eight installé → fournit `cec-client.exe` dans :
   `C:\Program Files (x86)\Pulse-Eight\USB-CEC Adapter\`
3. CEC activé sur la TV (paramètre souvent appelé « SIMPLINK », « Anynet+ », « Bravia Sync » selon la marque)

---

## Tester en PowerShell avant d'activer

```powershell
$cec = 'C:\Program Files (x86)\Pulse-Eight\USB-CEC Adapter\cec-client.exe'

# Allumer la TV
& $cec -s -c "on 0"

# Basculer sur l'entrée HDMI du PC (Active Source)
& $cec -s -c "as"

# Mettre la TV en veille
& $cec -s -c "standby 0"

# Lister les appareils CEC détectés sur le bus
& $cec -l
```

> Si `cec-client.exe` n'est pas dans ce chemin :
> ```powershell
> Get-ChildItem 'C:\Program Files*' -Recurse -Filter 'cec-client.exe' -ErrorAction SilentlyContinue
> ```

---

## Configuration dans le launcher

Dans `config.toml`, activer CEC et le daemon :

```toml
[cec]
enabled     = true
client_path = 'C:\Program Files (x86)\Pulse-Eight\USB-CEC Adapter\cec-client.exe'
daemon      = true   # lance cec-daemon.exe en fin de mode jeu
```

Le launcher localise automatiquement `cec-daemon.exe` dans le même dossier que `launcher.exe`.

---

## Utilisation standalone

```
cec-daemon.exe --path "<chemin vers cec-client.exe>"
```

Exemple :
```powershell
cec-daemon.exe --path "C:\Program Files (x86)\Pulse-Eight\USB-CEC Adapter\cec-client.exe"
```

Le daemon écrit un fichier `cec-daemon.log` dans le même dossier que le `.exe`.

---

## Logs

```
[2026-03-13 18:42:05] [INFO]  cec-daemon: started, cec-client pid=7812
[2026-03-13 18:42:05] [INFO]  cec-daemon: listening for display/shutdown events
[2026-03-13 22:30:11] [INFO]  cec-daemon: display OFF → standby
[2026-03-13 22:30:11] [INFO]  cec-daemon: sent 'standby 0'
[2026-03-14 08:15:03] [INFO]  cec-daemon: display ON → on + active source
[2026-03-14 08:15:03] [INFO]  cec-daemon: sent 'on 0'
[2026-03-14 08:15:03] [INFO]  cec-daemon: sent 'as'
```
