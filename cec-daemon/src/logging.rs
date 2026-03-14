/// Configure le logger en append vers le fichier de log spécifié.
///
/// Appelé une seule fois au démarrage, après le parsing des arguments CLI.
/// Si le fichier ne peut pas être ouvert, le processus panique (état non récupérable).
pub fn setup_logging(path: &std::path::Path) {
    let file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap_or_else(|e| panic!("cannot open log at {}: {}", path.display(), e));

    simplelog::WriteLogger::init(
        simplelog::LevelFilter::Info,
        simplelog::Config::default(),
        file,
    )
    .expect("logger already initialized");
}
