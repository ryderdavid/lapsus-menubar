# Lapsus Menu Bar App

A macOS menu bar application to control the lapsus_rust cursor movement tool.

## Features

- **Enable/Disable lapsus_rust** - Toggle the lapsus_rust process from the menu bar
- **Visual Status** - Menu bar icon shows whether lapsus_rust is enabled or disabled
- **Start at Login** - Option to automatically start the menu bar app on system startup
- **Process Monitoring** - Automatically detects if lapsus_rust is started or stopped externally

## Installation

### Quick Start

1. Build the application:
   ```bash
   cd menubar_app
   cargo build --release
   ```

2. Run the menu bar app:
   ```bash
   ./target/release/lapsus-menubar
   ```

3. The app will appear in your menu bar with a cursor icon

### Creating a macOS App Bundle

For a cleaner installation, you can create a `.app` bundle:

```bash
cd menubar_app
./create_app_bundle.sh
```

This will create `Lapsus Control.app` which you can:
- Double-click to launch
- Move to your Applications folder
- Add to Login Items via System Preferences

## Usage

### Menu Options

- **Enable lapsus_rust** - Starts the lapsus_rust process in the background
- **Disable lapsus_rust** - Stops the lapsus_rust process
- **Start at Login** - Toggle automatic startup when you log in
- **About** - Shows app information
- **Quit** - Exits the menu bar app (does not stop lapsus_rust)

### Icons

- **White outline cursor** - lapsus_rust is disabled
- **Black filled cursor** - lapsus_rust is enabled

## Requirements

- macOS 10.14 or later
- Rust 1.70+ (for building)
- lapsus_rust executable in the parent directory

## File Structure

```
lapsus/
├── lapsus_rust              # Main lapsus executable
└── menubar_app/
    ├── Cargo.toml           # Dependencies
    ├── src/main.rs          # Application code
    ├── icons/               # Menu bar icons
    ├── build.rs             # Build script
    └── target/release/      # Built binaries
```

## Configuration

The app stores its configuration in `~/.lapsus_menubar_config.json`:

```json
{
  "start_at_login": false
}
```

## Troubleshooting

### App won't start
- Ensure lapsus_rust is in the correct location (parent directory of menubar_app)
- Check permissions: `chmod +x ../lapsus_rust`

### Can't enable lapsus_rust
- Verify lapsus_rust works standalone: `../lapsus_rust`
- Check macOS Accessibility permissions in System Preferences

### Start at Login not working
- Re-toggle the option
- Check Login Items in System Preferences > Users & Groups

## Development

### Building

```bash
cargo build --release
```

### Testing

```bash
cargo run
```

### Updating Dependencies

```bash
cargo update
cargo build --release
```

## License

Same as lapsus_rust - see https://github.com/margooey/lapsus_rust

## Credits

Built with:
- [tao](https://github.com/tauri-apps/tao) - Window library
- [tray-icon](https://github.com/tauri-apps/tray-icon) - System tray support
- [sysinfo](https://github.com/GuillaumeGomez/sysinfo) - Process management
- [auto-launch](https://github.com/zzzgydi/auto-launch) - Startup integration
