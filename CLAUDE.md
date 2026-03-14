# Architecture du projet

Ce projet est un **orchestrateur de démarrage Windows** qui lance les bons programmes selon le mode configuré.

## Structure

```
src/
  main.rs              ← orchestrateur : charge la config, dispatche vers le mode actif
  config.rs            ← structs de configuration (une sous-struct par module)
  modes/
    mod.rs
    game.rs            ← mode jeu : lance afterburner + steam
    desktop.rs         ← mode bureau : no-op (extensible)
  modules/
    mod.rs
    steam.rs           ← lance Steam avec les arguments configurés
    afterburner.rs     ← lance MSI Afterburner avec le profil configuré
```

## Types de modules

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

1. Créer `src/modules/foo.rs` avec une fonction `pub fn launch(cfg: &FooConfig)`
2. Ajouter `pub mod foo;` dans `src/modules/mod.rs`
3. Ajouter `FooConfig` dans `config.rs` et un champ `pub foo: FooConfig` dans `Config`
4. Ajouter la section `[foo]` dans `config.toml`
5. Appeler `modules::foo::launch(&config.foo)` depuis le(s) mode(s) concerné(s)

## Ajouter un nouveau mode

1. Créer `src/modes/bar.rs` avec une fonction `pub fn run(config: &Config)`
2. Ajouter `pub mod bar;` dans `src/modes/mod.rs`
3. Ajouter un variant au dispatch dans `main.rs`
4. Documenter le mode dans `config.toml` et `README.md`

## Build (depuis WSL Ubuntu)

```bash
cd /mnt/d/developpement.code/launcher
source ~/.cargo/env
cargo build --release
# binaire : target/x86_64-pc-windows-gnu/release/launcher.exe
```
