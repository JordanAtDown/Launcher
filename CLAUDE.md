# Architecture du projet

Ce projet est un **monorepo Cargo workspace** contenant deux binaires Windows :

| Binaire | Rôle |
|---------|------|
| `launcher.exe` | Orchestrateur de démarrage : lance les bons programmes selon le mode (jeu / bureau) |
| `cec-daemon.exe` | Daemon autonome : veille/réveil de la TV HDMI CEC sur extinction écran et arrêt PC |

## Structure workspace

```
Cargo.toml                   ← [workspace] members = ["launcher", "cec-daemon"]
config.toml                  ← configuration unique partagée
.github/workflows/release.yml
.githooks/pre-commit
launcher/
  Cargo.toml
  src/
    main.rs              ← orchestrateur : charge la config, dispatche vers le mode actif
    config.rs            ← structs de configuration (une sous-struct par module)
    modes/
      mod.rs
      game.rs            ← mode jeu : CEC TV → display → sound → … → afterburner → steam → daemon
      desktop.rs         ← mode bureau : no-op (extensible)
    modules/
      mod.rs
      cec.rs             ← allume TV + source HDMI + lance cec-daemon (Pulse-Eight)
      steam.rs           ← lance Steam
      afterburner.rs     ← lance MSI Afterburner
      ...
cec-daemon/
  Cargo.toml
  src/main.rs            ← daemon Windows : écoute extinctions écran + arrêt PC → CEC standby/wake
```

## cec-daemon — fonctionnement

`cec-daemon.exe --path <cec-client.exe>` :
- Spawn `cec-client.exe -s` avec stdin pipe (connexion CEC ouverte, réponse <100ms)
- `SetConsoleCtrlHandler` : `CTRL_SHUTDOWN_EVENT` → `standby 0`
- Fenêtre cachée + `RegisterPowerSettingNotification(GUID_CONSOLE_DISPLAY_STATE)` :
  - Écran OFF (valeur 0) → `standby 0`  ← couvre S3 + Modern Standby S0 + inactivité
  - Écran ON  (valeur 1) → `on 0` + `as`  ← allume TV + bascule source HDMI au réveil

**Pourquoi `GUID_CONSOLE_DISPLAY_STATE` et non `PBT_APMSUSPEND` :**
Modern Standby (S0) est le mode veille par défaut depuis Windows 10 — le CPU reste actif
à très basse consommation (comme un smartphone). `PBT_APMSUSPEND` n'est pas déclenché en S0.
`GUID_CONSOLE_DISPLAY_STATE` se déclenche pour tous les types de veille car l'écran s'éteint
toujours, quelle que soit la méthode.

**Pourquoi stdin-pipe et non libcec-sys :**
libcec-sys utilise des `.lib` MSVC incompatibles avec la toolchain MinGW-w64 (cross-compilation WSL).

## Types de modules (launcher)

### Module registre
Écrit une ou plusieurs valeurs dans le registre Windows. Retourne `bool` directement depuis `set_value().is_ok()`. Pas de vérification de chemin, pas de PID.
Exemples : `gamemode.rs`, `notifications.rs`, `hags.rs`

```rust
pub fn enable(cfg: &FooConfig) -> bool {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok(key) = hkcu.open_subkey_with_flags(r"SOFTWARE\...", KEY_SET_VALUE) {
        return key.set_value("SomeKey", &1u32).is_ok();
    }
    false
}
```

### Module spawn (app longue durée)
Lance un processus qui reste actif (Steam, Afterburner, TimerResolution…).
- Vérifier `Path::new(path).exists()` avant de spawner — couvre 90% des échecs réels
- Logger le PID en cas de succès (`log::info!`), l'erreur OS en cas d'échec (`log::warn!`)
- Retourner `bool`

```rust
pub fn launch(cfg: &FooConfig) -> bool {
    let path = ...; // extraire depuis cfg, retourner false si absent/vide
    if !std::path::Path::new(path).exists() {
        log::warn!("foo: path not found: {}", path);
        return false;
    }
    match Command::new(path).spawn() {
        Ok(child) => { log::info!("foo spawned pid={}", child.id()); true }
        Err(e)    => { log::warn!("foo spawn error: {}", e); false }
    }
}
```

### Module commande courte
Exécute une commande système qui se termine rapidement (`net`, `powercfg`, `sc`, `taskkill`…).
- Utiliser `.status()` (bloquant ~100ms max) pour obtenir le vrai code de sortie
- Logger le code de sortie si échec (`log::warn!`)

```rust
pub fn apply(...) -> bool {
    match Command::new("powercfg").args([...]).status() {
        Ok(s) if s.success() => true,
        Ok(s)  => { log::warn!("foo: exit={:?}", s.code()); false }
        Err(e) => { log::warn!("foo: error: {}", e); false }
    }
}
```

---

## Règles d'architecture

- **main.rs** ne fait qu'une chose : lire la config et appeler le bon mode.
- **Chaque mode** orchestre un scénario (appelle des modules dans l'ordre voulu).
- **Chaque module** fait une seule chose (lancer un programme, configurer un outil, etc.).
- **Chaque module** a sa propre section dans `config.toml` et sa propre struct dans `config.rs`.

## Ajouter un nouveau module

1. Créer `launcher/src/modules/foo.rs` avec une fonction `pub fn launch(cfg: &FooConfig)`
2. Ajouter `pub mod foo;` dans `launcher/src/modules/mod.rs`
3. Ajouter `FooConfig` dans `launcher/src/config.rs` et un champ `pub foo: FooConfig` dans `Config`
4. Ajouter la section `[foo]` dans `config.toml`
5. Appeler `modules::foo::launch(&config.foo)` depuis le(s) mode(s) concerné(s)

## Ajouter un nouveau mode

1. Créer `launcher/src/modes/bar.rs` avec une fonction `pub fn run(config: &Config)`
2. Ajouter `pub mod bar;` dans `launcher/src/modes/mod.rs`
3. Ajouter un variant au dispatch dans `launcher/src/main.rs`
4. Documenter le mode dans `config.toml` et `README.md`

## Workflow de développement

**Règle absolue : chaque fonctionnalité, correction de bug ou refactoring fait l'objet d'un commit + push dédié.** Ne jamais grouper plusieurs changements non liés dans un seul commit.

```bash
git add <fichiers modifiés>
git commit -m "feat: ..."   # ou fix: / refactor: / docs: / chore:
git push
```

### Hook pre-commit (build automatique)

Le script `.githooks/pre-commit` lance `cargo build --release` avant chaque commit et annule le commit si le build échoue. Fonctionne depuis Windows (Git Bash) et depuis WSL directement.

**Activer après un clone frais (une seule fois) :**

```bash
git config core.hooksPath .githooks
```

---

## Build (depuis WSL Ubuntu)

```bash
cd /mnt/d/developpement.code/launcher
source ~/.cargo/env
cargo build --release
# binaires :
#   target/x86_64-pc-windows-gnu/release/launcher.exe
#   target/x86_64-pc-windows-gnu/release/cec-daemon.exe
```

---

## Release (GitHub Actions)

Le workflow `.github/workflows/release.yml` se déclenche sur un tag `v*.*.*` et produit automatiquement un zip `launcher-vX.Y.Z.zip` publié en release GitHub avec un changelog généré depuis les commits conventionnels.

### Changelog conventionnel (git-cliff)

Le projet utilise **[git-cliff](https://git-cliff.org)** pour générer automatiquement le changelog à partir des préfixes de commit.

- **`cliff.toml`** — configuration des groupes et du format
- **`CHANGELOG.md`** — mis à jour à chaque release par `release.sh` (si git-cliff est installé)
- **GitHub Release body** — généré par `orhun/git-cliff-action` dans le CI (toujours à jour)

**Installer git-cliff (une seule fois, optionnel en local) :**
```bash
cargo install git-cliff
```

### Préfixes de commit conventionnels

Les préfixes impactent directement le changelog généré :

| Préfixe | Section dans le changelog |
|---------|--------------------------|
| `feat:` | Nouveautés |
| `fix:` | Corrections |
| `refactor:` | Refactoring |
| `perf:` | Performance |
| `docs:` | Documentation |
| `style:` | Style |
| `test:` | Tests |
| `chore:` | Maintenance |
| `chore: bump version` | *(ignoré dans le changelog)* |

### Processus de release

**Règle absolue :** toujours commiter les modifications fonctionnelles AVANT de lancer le script de release. L'historique doit toujours avoir un commit de modification suivi d'un commit `chore: bump version`.

```bash
# 1. Commiter les modifications avec le bon préfixe
git add <fichiers modifiés>
git commit -m "fix: ..."   # ou feat: / refactor: / docs: / chore: / etc.
git push

# 2. Lancer le script de release
bash scripts/release.sh 0.3.5
# → met à jour CHANGELOG.md, bump version, commit, tag, push
# → GitHub Actions publie automatiquement (~2-3 min)
```

Le script `release.sh` abortera si des changements non commités sont détectés.

### Ce que fait le workflow

1. `ubuntu-latest` + `mingw-w64` → cross-compile `launcher.exe` + `cec-daemon.exe`
2. `orhun/git-cliff-action` → génère les notes de release depuis les commits conventionnels
3. `zip launcher-vX.Y.Z.zip` → package binaires + `config.toml` + scripts `.ps1` + `.bat`
4. `softprops/action-gh-release` → publie la release avec le changelog formaté
