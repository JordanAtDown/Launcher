use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Default)]
pub struct LoggingConfig {
    pub path: Option<String>, // chemin du fichier log (défaut: launcher.log à côté de l'exe)
}

#[derive(Deserialize, Default)]
pub struct MonitorConfig {
    pub name: Option<String>, // sous-chaîne du nom du moniteur à détecter
    pub mode: Option<String>, // "auto" | "game" | "desktop"
}

#[derive(Deserialize, Default)]
pub struct SteamConfig {
    pub path: String,
    #[serde(default)]
    pub args: Vec<String>,
}

#[derive(Deserialize, Default)]
pub struct AfterburnerConfig {
    pub path: Option<String>,
    pub game_profile: Option<u8>,    // 1-5
    pub desktop_profile: Option<u8>, // 1-5
}

#[derive(Deserialize, Default)]
pub struct GameModeConfig {
    #[serde(default)]
    pub enabled: bool, // activer le Game Mode Windows en mode jeu
}

#[derive(Deserialize, Default)]
pub struct GameBarConfig {
    #[serde(default)]
    pub uninstall: bool, // désinstaller Xbox Gaming Overlay + désactiver Game DVR
}

#[derive(Deserialize, Default)]
pub struct UpdatesConfig {
    #[serde(default)]
    pub pause_in_game: bool, // stopper Windows Update en mode jeu
}

#[derive(Deserialize, Default)]
pub struct NotificationsConfig {
    #[serde(default)]
    pub disable_in_game: bool, // désactiver les notifications en mode jeu
}

#[derive(Deserialize, Default)]
pub struct PowerPlanConfig {
    #[serde(default)]
    pub game: String,    // GUID du plan jeu    (powercfg /L pour lister)
    #[serde(default)]
    pub desktop: String, // GUID du plan bureau (laisser vide = ne rien changer)
}

#[derive(Deserialize, Default)]
pub struct KilllistConfig {
    #[serde(default)]
    pub services: Vec<String>, // noms courts des services (services.msc → Propriétés)
    #[serde(default)]
    pub processes: Vec<String>, // noms des .exe (Gestionnaire des tâches → Détails)
}

#[derive(Deserialize, Default)]
pub struct RtssConfig {
    pub path: Option<String>,         // chemin vers RTSS.exe
    pub profile_path: Option<String>, // chemin vers le profil Global de RTSS
    pub game_limit: Option<u32>,      // images/sec en mode jeu (0 = illimité)
}

#[derive(Deserialize, Default)]
pub struct DisplayConfig {
    pub exe_path:       Option<String>, // chemin vers nircmd.exe
    pub game_width:     Option<u32>,    // résolution jeu
    pub game_height:    Option<u32>,
    pub game_scale:     Option<u32>,    // mise à l'échelle jeu en % (100, 125, 150, 175, 200)
    pub desktop_width:  Option<u32>,    // résolution bureau
    pub desktop_height: Option<u32>,
    pub desktop_scale:  Option<u32>,    // mise à l'échelle bureau en %
    #[serde(default)]
    pub refresh_rate:   u32,            // Hz (0 = ne pas changer)
}

#[derive(Deserialize, Default)]
pub struct SoundConfig {
    pub exe_path:       Option<String>, // chemin vers svcl.exe ou SoundVolumeView.exe
    pub game_device:    Option<String>, // sous-chaîne du nom du device jeu (ex: "LG", "HDMI")
    pub desktop_device: Option<String>, // sous-chaîne du nom du device bureau (ex: "Speakers")
    #[serde(default)]
    pub auto_detect:    bool,           // si true, vérifie que le device est visible avant de switcher
}

#[derive(Deserialize, Default)]
pub struct StartupConfig {
    #[serde(default)]
    pub desktop: Vec<String>, // exécutables à lancer en mode bureau (chemins complets)
    #[serde(default)]
    pub game:    Vec<String>, // exécutables à lancer en mode jeu (chemins complets)
}

#[derive(Deserialize, Default)]
pub struct DisableServicesConfig {
    #[serde(default)]
    pub services: Vec<String>, // noms courts des services (services.msc → Propriétés → champ "Nom du service")
}

#[derive(Deserialize, Default)]
pub struct WslConfig {
    #[serde(default)]
    pub enabled: bool, // arrêter WSL en mode jeu
}

#[derive(Deserialize, Default)]
pub struct CecConfig {
    #[serde(default)]
    pub enabled: bool,                  // activer le contrôle CEC de la TV
    pub client_path: Option<String>,    // chemin vers cec-client.exe (Pulse-Eight)
    #[serde(default)]
    pub daemon: bool,                   // lancer cec-daemon.exe en fin de mode jeu
}

#[derive(Deserialize, Default)]
pub struct TimerResolutionConfig {
    pub path: Option<String>,              // chemin vers TimerResolution.exe
    #[serde(default = "default_resolution")]
    pub resolution: f32,                   // résolution en ms (défaut: 0.5)
}

fn default_resolution() -> f32 { 0.5 }

#[derive(Deserialize)]
pub struct Config {
    #[serde(default)]
    pub monitor: MonitorConfig,
    #[serde(default)]
    pub steam: SteamConfig,
    #[serde(default)]
    pub afterburner: AfterburnerConfig,
    #[serde(default)]
    pub gamemode: GameModeConfig,
    #[serde(default)]
    pub gamebar: GameBarConfig,
    #[serde(default)]
    pub updates: UpdatesConfig,
    #[serde(default)]
    pub notifications: NotificationsConfig,
    #[serde(default)]
    pub powerplan: PowerPlanConfig,
    #[serde(default)]
    pub killist: KilllistConfig,
    #[serde(default)]
    pub disable_services: DisableServicesConfig,
    #[serde(default)]
    pub rtss: RtssConfig,
    #[serde(default)]
    pub display: DisplayConfig,
    #[serde(default)]
    pub sound: SoundConfig,
    #[serde(default)]
    pub wsl: WslConfig,
    #[serde(default)]
    pub timerresolution: TimerResolutionConfig,
    #[serde(default)]
    pub startup: StartupConfig,
    #[serde(default)]
    pub logging: LoggingConfig,
    #[serde(default)]
    pub cec: CecConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            monitor: MonitorConfig::default(),
            steam: SteamConfig {
                path: r"C:\Program Files (x86)\Steam\steam.exe".to_string(),
                args: vec!["-bigpicture".to_string()],
            },
            afterburner: AfterburnerConfig::default(),
            gamemode: GameModeConfig::default(),
            gamebar: GameBarConfig::default(),
            updates: UpdatesConfig::default(),
            notifications: NotificationsConfig::default(),
            powerplan: PowerPlanConfig::default(),
            killist: KilllistConfig::default(),
            disable_services: DisableServicesConfig::default(),
            rtss: RtssConfig::default(),
            display: DisplayConfig::default(),
            sound: SoundConfig::default(),
            wsl: WslConfig::default(),
            timerresolution: TimerResolutionConfig::default(),
            startup: StartupConfig::default(),
            logging: LoggingConfig::default(),
            cec: CecConfig::default(),
        }
    }
}

pub fn load() -> Config {
    let exe_dir: PathBuf = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));
    let config_path = exe_dir.join("config.toml");
    if let Ok(content) = fs::read_to_string(&config_path) {
        toml::from_str(&content).unwrap_or_default()
    } else {
        Config::default()
    }
}
