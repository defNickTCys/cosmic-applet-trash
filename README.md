# Cosmic Trash

![COSMIC](https://img.shields.io/badge/COSMIC-Desktop-blueviolet?logo=system76&logoColor=white) 
![License](https://img.shields.io/badge/license-GPL--3.0-blue.svg) 
![Rust](https://img.shields.io/badge/rust-1.92%2B-orange.svg?logo=rust) 
![Vibe Coding](https://img.shields.io/badge/development-Vibe_Coding-ff69b4?logo=codeproject)

Advanced trash management applet for the COSMIC™ desktop environment.

## Features

### ✅ Phase 0: Foundation & Adaptive UI
- **Adaptive icon system**: Colored icons in Dock (`user-trash`, `user-trash-full`), symbolic in Panel (`user-trash-symbolic`, `user-trash-full-symbolic`)
- **Reactive trash status**: Icon changes automatically on applet init
- **Native integration**: Opens trash folder using `cosmic-files --trash` (standard COSMIC pattern)
- **COSMIC-style UI**: Menu button popup following official design patterns

### ✅ Phase 1: Real-Time Auto-Update
- **inotify monitoring**: `notify-debouncer-full` with 250ms debounce
- **Subscription pattern**: Non-blocking `try_send` for efficient updates
- **Automatic icon updates**: Icon and popup reflect trash status without panel restart
- **Performance optimized**: Non-recursive monitoring, debounced events

### Roadmap

- **Phase 2**: Empty Trash & Restore Items actions with confirmation dialogs
- **Phase 3**: Drag & Drop for disk eject (Udisks2 integration)
- **Phase 4**: Drag & Drop for app uninstall (Flatpak/PackageKit integration)

## Architecture

Following strict modularization with "Native by Default" philosophy. See **[ARCHITECTURE.md](./ARCHITECTURE.md)** for technical details.

```
src/
├── app.rs              # Application orchestrator (state + messages)
├── trash_status.rs     # Backend: Trash monitoring logic
├── file_manager.rs     # Native integration: cosmic-files launcher
├── ui_panel_button.rs  # Frontend: Adaptive panel icon
├── ui_popup.rs         # Frontend: Popup content
├── config.rs           # Configuration management
├── i18n.rs             # Internationalization
├── lib.rs              # Public exports
└── main.rs             # Entry point
```

**Key Modules**:
- `trash_status.rs`: Pure trash state logic (no UI dependencies)
- `file_manager.rs`: Direct `Command::spawn` following COSMIC patterns
- `ui_panel_button.rs`: Context-aware rendering (Dock vs Panel)
- `app.rs`: Reactive subscription for real-time updates

## Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Or use justfile
just build-release
```

## Installation

**Development installation** (recommended):

```bash
# From parent directory
./dev.sh dev

# Restart panel to reload
killall cosmic-panel
```

**System installation**:

```bash
just install

# Uninstall
just uninstall
```

## Development

```bash
# Lint & check
cargo clippy --all-features -- -W clippy::pedantic

# Format code
cargo fmt

# Run with backtrace
cargo run --release

# Clean build artifacts
cargo clean
```

## Design & Development Standards

### Native by Default

All integrations follow official COSMIC patterns discovered through source code analysis:

- **Process spawning**: `std::process::Command` (not D-Bus) for launching apps
- **Icon system**: COSMIC's adaptive icons (colored/symbolic automatically managed)
- **Subscriptions**: `cosmic-files` patterns for filesystem monitoring via `inotify`
- **Reactive UI**: `iced` patterns for state-driven rendering

### Code Quality

- **Zero compiler warnings** (pedantic clippy enabled)
- **GPL-3.0 headers** on all source files
- **English-only** internal documentation
- **Modular architecture** with single-responsibility modules

## Dependencies

### Core
- **libcosmic** (git) - COSMIC UI framework
- **iced** - Reactive GUI (via libcosmic)
- **trash** (cosmic branch) - Trash operations
- **notify-debouncer-full** - Filesystem monitoring
- **tokio** - Async runtime

### Development
- **clippy** - Linting (pedantic mode)
- **rustfmt** - Code formatting

## Development Environment

This project was developed using **Vibe Coding** methodology:

- **IDE**: [Antigravity](https://antigravity.dev/) by Google DeepMind
- **AI Pair Programming**: Claude Sonnet 4.5 (Anthropic)
- **Lead Developer**: [Thiago Cysneiros](https://github.com/defNickTCys)

**Vibe Coding** emphasizes:
- Real-time AI-assisted development
- Source code analysis for pattern discovery
- Iterative refinement with zero-warning targets
- Transparent collaboration between human and AI

---

<div align="center">

Made with ❤️ and 🤖 for the COSMIC Desktop community

**[COSMIC Desktop](https://system76.com/cosmic)** by **[System76](https://system76.com/)**

</div>
