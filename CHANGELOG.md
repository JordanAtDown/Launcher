# Changelog

## v1.0.0 — 2026-04-15

### Nouveautés

- Add notifications::restore to re-enable toast notifications in desktop mode

## v0.3.13 — 2026-04-05

### Corrections

- Hide PowerShell window during monitor detection (CREATE_NO_WINDOW)

## v0.3.12 — 2026-04-03

### Corrections

- Treat wuauserv exit code 2 as success (service already stopped)

### Nouveautés

- Add updates::restore to restart wuauserv in desktop mode

## v0.3.11 — 2026-04-03

### Corrections

- Revert SW_HIDE to SW_SHOWMINIMIZED, SW_HIDE breaks apps like Afterburner

## v0.3.10 — 2026-04-03

### Corrections

- Use WmiMonitorID EDID names instead of Win32_DesktopMonitor for monitor detection

## v0.3.9 — 2026-04-03

### Corrections

- Use SW_HIDE instead of SW_SHOWMINIMIZED to reliably hide app windows on launch

## v0.3.8 — 2026-04-03

### Corrections

- Use cliff step output for release body instead of body_path

## v0.3.7 — 2026-04-03

### Documentation

- Regenerate CHANGELOG from full git history via git-cliff

### Refactoring

- Replace QRes with NirCmd and remove scale config from display module

## v0.3.6 — 2026-04-03

### Corrections

- Remove GITHUB_REPO from git-cliff action to disable PR metadata fetch

### Maintenance

- Simplify pre-commit hook, remove WSL logic, use dynamic repo root

## v0.3.5 — 2026-04-03

### Documentation

- Update sound config and improve sound module documentation

### Maintenance

- Add conventional changelog with git-cliff

### Nouveautés

- Add bat wrappers for turnkey install, update, and uninstall

## v0.3.4 — 2026-03-14

### Corrections

- Launch afterburner and startup apps minimized + use local time in logs

### Maintenance

- Abort release if worktree has uncommitted changes

## v0.3.2 — 2026-03-14

### Documentation

- Update README with install script usage and execution policy fix

## v0.3.1 — 2026-03-14

### Corrections

- Suppress schtasks delete error when task does not exist yet

## v0.3.0 — 2026-03-14

### Nouveautés

- Configurable module pipeline per mode (order + selection)

## v0.2.1 — 2026-03-14

### Nouveautés

- Embed requireAdministrator UAC manifest in launcher.exe
- Use Task Scheduler for elevated auto-start

## v0.2.0 — 2026-03-14

### Documentation

- Enforce one commit+push per feature/fix in CLAUDE.md
- Update for monorepo structure + cec-daemon
- Split README into root + launcher + cec-daemon

### Maintenance

- Add pre-commit hook — aborts commit if cargo build fails
- Clarify pre-commit hook builds launcher + cec-daemon
- Unify version via workspace inheritance
- Add release automation script

### Nouveautés

- Add HDMI CEC module (Pulse-Eight USB-CEC adapter)
- Implement cec-daemon (standby/wake on all sleep types + shutdown)
- Launcher spawns cec-daemon in game mode
- Add --log <path> option for configurable log file
- Embed Win32 version info metadata for Task Manager
- Embed Win32 version info metadata for Task Manager
- Add Windows installation script with startup registration
- Add uninstall script + include in release zip
- Versioned exe names in release zip and install scripts

### Performance

- Replace sleep in wnd_proc with SetTimer/WM_TIMER

### Refactoring

- Restructure as Cargo workspace (launcher + cec-daemon)
- Split cec-daemon into focused modules

## v0.1.0 — 2026-03-14


