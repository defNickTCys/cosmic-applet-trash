# Cosmic Trash Plus

Advanced trash management applet for the COSMIC™ desktop environment.

![License](https://img.shields.io/badge/license-GPL--3.0-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.92%2B-orange.svg)

## Features

### Phase 0 (Current) ✅
- **Real-time trash status**: Icon automatically changes between empty (`user-trash-symbolic`) and full (`user-trash-full-symbolic`)
- **Native integration**: Opens trash folder in `cosmic-files` using `cosmic-files --trash`
- **Reactive state**: Status checked on applet init for immediate visual feedback
- **COSMIC-style UI**: Uses `menu_button` for popup actions following COSMIC design patterns

### Roadmap

- **Phase 1**: Real-time monitoring with inotify, Empty/Restore actions
- **Phase 2**: Popup with trash item list and individual actions
- **Phase 3**: Drag & Drop for disk eject (Udisks2 integration)
- **Phase 4**: Drag & Drop for app uninstall (Flatpak/PackageKit integration)

## Architecture

Following strict modularization principles with "Native by Default" philosophy:

```
src/
├── app.rs               # Orchestrator: State + Messages
├── trash_status.rs      # Backend: Trash logic (uses trash-rs)
├── dbus_file_manager.rs # D-Bus client (org.freedesktop.FileManager1)
├── ui_panel_button.rs   # Frontend: Panel icon view
├── ui_popup.rs          # Frontend: Popup content view
├── config.rs            # Configuration persistence
├── i18n.rs              # Internationalization
├── lib.rs               # Public module exports
└── main.rs              # Entry point
```

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

```bash
# Install to system
just install

# Uninstall
just uninstall
```

## Development

```bash
# Run linter
just check

# Run with backtrace
just run

# Clean build artifacts
just clean
```

## Dependencies

- **libcosmic**: UI framework (git)
- **trash-rs**: Trash management (branch: cosmic)
- **zbus**: D-Bus communication
- **tokio**: Async runtime

## License

GPL-3.0-only

## Credits

- Trash logic replicated from [cosmic-files](https://github.com/pop-os/cosmic-files)
- D-Bus patterns from [cosmic-applets](https://github.com/pop-os/cosmic-applets)
- COSMIC Desktop by [System76](https://system76.com/)
