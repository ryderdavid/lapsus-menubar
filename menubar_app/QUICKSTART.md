# 🚀 Quick Start Guide - Lapsus Control

## Installation (2 minutes)

### Option 1: Run from Terminal
```bash
cd /Users/ryder/bin/lapsus/menubar_app
./target/release/lapsus-menubar
```

### Option 2: Use the App Bundle (Recommended)
```bash
cd /Users/ryder/bin/lapsus/menubar_app
open "Lapsus Control.app"
```

### Option 3: Install to Applications
```bash
cd /Users/ryder/bin/lapsus/menubar_app
cp -r "Lapsus Control.app" /Applications/
# Then launch from Spotlight: Cmd+Space, type "Lapsus"
```

## Usage

### First Time Setup
1. Launch the app - it appears in your menu bar (near the clock)
2. Look for a **cursor icon** in the menu bar
3. Click the icon to see the menu
4. Click **"Enable lapsus_rust"** to start

### Daily Use
- **Enable**: Click menu bar icon → "Enable lapsus_rust"
- **Disable**: Click menu bar icon → "Disable lapsus_rust"
- **Auto-start**: Check "Start at Login" to launch on login

### Icon Meanings
- 🖱️ **White outline cursor** = lapsus_rust is OFF
- 🖱️ **Black filled cursor** = lapsus_rust is ON

## Quick Test

Run the automated test:
```bash
cd /Users/ryder/bin/lapsus/menubar_app
./test.sh
```

Or test manually:
1. Launch the app
2. Click menu bar icon
3. Click "Enable lapsus_rust"
4. Verify in terminal: `ps aux | grep lapsus_rust`
5. Click "Disable lapsus_rust"
6. Verify it stopped

## Troubleshooting

### App won't start?
```bash
# Check if lapsus_rust exists
ls -l /Users/ryder/bin/lapsus/lapsus_rust

# Rebuild if needed
cd /Users/ryder/bin/lapsus/menubar_app
cargo build --release
./create_app_bundle.sh
```

### Can't enable lapsus_rust?
- Check macOS **System Preferences** → **Security & Privacy** → **Accessibility**
- Make sure lapsus_rust has permissions

### Icon not showing?
- Check if another menu bar app is hiding it
- Try clicking the menu bar icon area
- Restart the app

## Files Created

- `/Users/ryder/bin/lapsus/menubar_app/` - All source code
- `~/.lapsus_menubar_config.json` - Your settings
- `/Applications/Lapsus Control.app` - (if installed)

## Next Steps

1. ✅ **Launch the app** - `open "Lapsus Control.app"`
2. ✅ **Test Enable/Disable** - Use menu bar icon
3. ✅ **Set "Start at Login"** - If you want auto-start
4. ✅ **Install to Applications** - For permanent use

## Need Help?

- **README**: `cat /Users/ryder/bin/lapsus/menubar_app/README.md`
- **Summary**: `cat /Users/ryder/bin/lapsus/menubar_app/PROJECT_SUMMARY.md`
- **Logs**: Check macOS Console.app for "lapsus"

---

**That's it! Your menu bar app is ready to use. 🎉**
