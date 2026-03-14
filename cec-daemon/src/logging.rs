/// Configure le logger en append vers `cec-daemon.log` dans le même dossier que l'exécutable.
///
/// Appelé une seule fois au démarrage, avant toute autre opération.
/// Si le fichier ne peut pas être ouvert, le processus panique (état non récupérable).
pub fn setup_logging() {
    let log_path = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("cec-daemon.log");

    let file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .unwrap_or_else(|e| panic!("cannot open cec-daemon.log at {}: {}", log_path.display(), e));

    simplelog::WriteLogger::init(
        simplelog::LevelFilter::Info,
        simplelog::Config::default(),
        file,
    )
    .expect("logger already initialized");
}
