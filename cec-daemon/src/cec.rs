/// Gestion de la connexion CEC via `cec-client.exe`.
///
/// `cec-client.exe -s` ouvre une session interactive sur stdin/stdout.
/// On garde le pipe stdin ouvert pour envoyer des commandes CEC à la demande,
/// sans relancer le processus à chaque fois (initialisation CEC < 3 s, réponse < 100 ms).
use std::io::Write;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex, OnceLock};

// ─── Global stdin ──────────────────────────────────────────────────────────────
//
// Les callbacks Windows (`ctrl_handler`, `wnd_proc`) sont des pointeurs de fonction
// C — ils ne peuvent pas capturer de contexte Rust. On stocke donc le stdin de
// cec-client dans un global thread-safe initialisé une seule fois au démarrage.

static CEC_STDIN: OnceLock<Arc<Mutex<std::process::ChildStdin>>> = OnceLock::new();

/// Initialise le global CEC_STDIN à partir du stdin d'un [`CecClient`] déjà spawné.
///
/// Doit être appelé exactement une fois, après [`CecClient::spawn`].
/// Panique si appelé plusieurs fois.
pub fn init_global_stdin(stdin: Arc<Mutex<std::process::ChildStdin>>) {
    CEC_STDIN
        .set(stdin)
        .expect("init_global_stdin: already initialized");
}

/// Envoie une commande CEC sur le stdin de `cec-client.exe`.
///
/// Sans effet (avec avertissement) si [`init_global_stdin`] n'a pas encore été appelé.
/// Erreurs d'écriture ignorées silencieusement — le daemon continuera à tourner même
/// si cec-client s'est terminé de façon inattendue.
pub fn send_cec(cmd: &str) {
    match CEC_STDIN.get() {
        None => log::warn!("cec: send_cec('{}') called before init_global_stdin", cmd),
        Some(arc) => {
            if let Ok(mut stdin) = arc.lock() {
                let _ = writeln!(stdin, "{}", cmd);
                let _ = stdin.flush();
                log::info!("cec-daemon: sent '{}'", cmd);
            }
        }
    }
}

// ─── CecClient ─────────────────────────────────────────────────────────────────

/// Handle sur le processus `cec-client.exe` en cours d'exécution.
///
/// Contient le processus enfant (pour attendre sa terminaison) et
/// un handle partageable vers son stdin (pour envoyer des commandes CEC).
pub struct CecClient {
    /// Processus `cec-client.exe`. Utilisé pour attendre la fin propre via [`CecClient::wait`].
    pub child: Child,
    /// Stdin partageable entre les callbacks Windows et le thread principal.
    pub stdin: Arc<Mutex<std::process::ChildStdin>>,
}

impl CecClient {
    /// Spawne `cec-client.exe -s` avec stdin piped.
    ///
    /// Retourne une erreur si le chemin est introuvable ou si le spawn OS échoue.
    /// stdout et stderr sont redirigés vers `/dev/null` (le daemon n'a pas besoin
    /// des réponses textuelles de cec-client).
    pub fn spawn(path: &str) -> Result<Self, String> {
        if !std::path::Path::new(path).exists() {
            return Err(format!("cec-client not found: {}", path));
        }

        log::info!("cec-daemon: spawning cec-client: {}", path);

        let mut child = Command::new(path)
            .arg("-s")
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| format!("spawn cec-client failed: {}", e))?;

        let raw_stdin = child.stdin.take().ok_or("cec-client has no stdin")?;
        let stdin = Arc::new(Mutex::new(raw_stdin));

        Ok(Self { child, stdin })
    }

    /// Envoie une commande de fin de session et attend la terminaison du processus.
    ///
    /// Envoie d'abord `q` (quit cec-client), attend 300 ms pour laisser le temps
    /// au processus de terminer proprement, puis appelle `wait()`.
    pub fn shutdown(self) {
        send_cec("q");
        std::thread::sleep(std::time::Duration::from_millis(300));
        // On consomme `child` — `wait` libère les ressources OS.
        let mut child = self.child;
        let _ = child.wait();
    }
}
