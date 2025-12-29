# Lapsus Menu Bar App

A macOS menu bar application to control [lapsus_rust](https://github.com/margooey/lapsus_rust) - a tool that adds iPad-style inertial movement to your trackpad cursor.

## Features

- 🎯 **Simple Controls** - Enable/Disable Lapsus with one click
- 🔵 **Visual Status** - Filled circle (●) when running, outline (○) when stopped
- 🚀 **Start at Login** - Optional automatic startup
- 🎨 **Gradient Icon** - Pink-to-blue app icon
- 🔌 **LaunchAgent Integration** - Properly manages the macOS service
- 📦 **Fully Portable** - Bundles lapsus_rust executable

## What is Lapsus?

Lapsus adds smooth, iPad-style inertial movement to your macOS trackpad cursor. When you move the cursor, it continues with momentum like iOS/iPadOS scrolling, creating a more fluid and natural cursor experience.

## Screenshots

Menu bar showing status:
- **●** = Lapsus is active (cursor has inertial movement)
- **○** = Lapsus is stopped (cursor behaves normally)

## Installation

### Quick Install

```bash
cd menubar_app
./install.sh
```

### Manual Installation

```bash
cd menubar_app

# Build
cargo build --release

# Create app bundle
./create_app_bundle.sh

# Install to Applications
cp -r "Lapsus Control.app" /Applications/

# Launch
open "/Applications/Lapsus Control.app"
```

## Usage

1. **Launch the app** - Look for the circle icon in your menu bar
2. **Enable Lapsus** - Click the menu bar icon → "Enable Lapsus"
3. **Disable Lapsus** - Click the menu bar icon → "Disable Lapsus"
4. **Start at Login** - Check the option in the menu to auto-start
5. **Quit** - Automatically stops Lapsus before exiting

## Requirements

- macOS 10.14 or later
- Rust 1.70+ (for building from source)

## Building from Source

```bash
cd menubar_app
cargo build --release
```

The binary will be at: `target/release/lapsus-menubar`

## Configuration

Settings are stored in `~/.lapsus_menubar_config.json`:

```json
{
  "start_at_login": false,
  "lapsus_rust_path": "/custom/path/to/lapsus_rust"  // optional
}
```

## LaunchAgent Integration

If you have a LaunchAgent at `~/Library/LaunchAgents/com.lapsus.rust.plist`, the app will use `launchctl` to start/stop the service properly.

Otherwise, it falls back to direct process management.

## Project Structure

```
lapsus/
├── lapsus_rust              # Main lapsus executable
└── menubar_app/
    ├── src/main.rs          # Application code
    ├── icons/               # Menu bar status icons
    ├── icon/                # App icon resources
    ├── Cargo.toml           # Dependencies
    └── create_app_bundle.sh # Builds .app bundle
```

## Technical Details

- **Language**: Rust
- **UI Framework**: tao (event loop) + tray-icon (menu bar)
- **Process Management**: sysinfo + launchctl
- **Auto-Launch**: auto-launch crate
- **Bundle Size**: ~5 MB (includes lapsus_rust)

## Dependencies

- `tao` 0.30 - Event loop and window management
- `tray-icon` 0.19 - System tray icon
- `sysinfo` 0.32 - Process monitoring
- `auto-launch` 0.5 - Login items
- `image` 0.25 - Icon loading

## Documentation

- [QUICKSTART.md](menubar_app/QUICKSTART.md) - Quick setup guide
- [PATH_FIX.md](menubar_app/PATH_FIX.md) - Path configuration details
- [LAUNCHAGENT_INTEGRATION.md](menubar_app/LAUNCHAGENT_INTEGRATION.md) - LaunchAgent details
- [ICON_INTEGRATION.md](menubar_app/ICON_INTEGRATION.md) - Icon creation process

## Credits

- Menu bar app created with Claude
- Original lapsus_rust by [margooey](https://github.com/margooey/lapsus_rust)

## License

Same as lapsus_rust

## Contributing

Issues and pull requests welcome!
