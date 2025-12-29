# LaunchAgent Integration - December 29, 2025

## Changes Made

### Integrated with macOS LaunchAgent

The menu bar app now properly controls the `com.lapsus.rust` LaunchAgent service instead of just killing/starting processes.

### How It Works Now

#### Start/Enable
- Checks if `~/Library/LaunchAgents/com.lapsus.rust.plist` exists
- If yes: Uses `launchctl load` to start the service
- If no: Falls back to starting the process directly

#### Stop/Disable  
- Checks if LaunchAgent exists
- If yes: Uses `launchctl unload` to stop the service
- If no: Falls back to killing the process with SIGTERM

#### Status Check
- First checks `launchctl list com.lapsus.rust` to see if service is loaded and has a PID
- Falls back to checking process list if service doesn't exist

### Benefits

✅ **Respects KeepAlive** - Won't fight with LaunchAgent trying to restart
✅ **Clean start/stop** - Uses proper macOS service management
✅ **Backwards compatible** - Still works without LaunchAgent (direct process control)
✅ **Proper status** - Shows actual service state, not just process existence

### LaunchAgent Details

Current service at `~/Library/LaunchAgents/com.lapsus.rust.plist`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.lapsus.rust</string>
    <key>ProgramArguments</key>
    <array>
        <string>/Users/ryder/bin/lapsus_rust</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardOutPath</key>
    <string>/tmp/lapsus_rust.log</string>
    <key>StandardErrorPath</key>
    <string>/tmp/lapsus_rust_error.log</string>
</dict>
</plist>
```

**Key settings:**
- `RunAtLoad: true` - Starts automatically when loaded
- `KeepAlive: true` - Restarts if crashes
- Logs to `/tmp/lapsus_rust.log` and `/tmp/lapsus_rust_error.log`

### Usage

#### Via Menu Bar App (Recommended)
```bash
# Launch the app
open "Lapsus Control.app"

# Use the menu to:
# - Enable lapsus_rust (loads service)
# - Disable lapsus_rust (unloads service)
# - Quit (unloads service and exits)
```

#### Manual LaunchAgent Control
```bash
# Load (start) the service
launchctl load ~/Library/LaunchAgents/com.lapsus.rust.plist

# Unload (stop) the service
launchctl unload ~/Library/LaunchAgents/com.lapsus.rust.plist

# Check status
launchctl list com.lapsus.rust

# View logs
tail -f /tmp/lapsus_rust.log
tail -f /tmp/lapsus_rust_error.log
```

### Testing

#### Test 1: Enable via Menu Bar App
```bash
# Launch app
open "Lapsus Control.app"

# Current state (if service was running):
launchctl list com.lapsus.rust
# Should show: PID   -15   com.lapsus.rust

# Click "Disable" in menu
# Wait 1 second

# Check status:
launchctl list com.lapsus.rust
# Should show: Could not find service "com.lapsus.rust" in domain for user

# Verify process gone:
ps aux | grep lapsus_rust | grep -v grep
# Should show nothing
```

#### Test 2: Disable via Menu Bar App
```bash
# With service stopped, click "Enable" in menu
# Wait 1 second

# Check status:
launchctl list com.lapsus.rust
# Should show: PID   -15   com.lapsus.rust

# Verify process running:
ps aux | grep lapsus_rust | grep -v grep
# Should show lapsus_rust process
```

#### Test 3: Quit App Behavior
```bash
# With service running, click "Quit" in menu

# Check status:
launchctl list com.lapsus.rust
# Should show: Could not find service

# Service should be stopped
ps aux | grep lapsus_rust | grep -v grep
# Should show nothing
```

### Icon States

- **Black outline** = Service is loaded and running (lapsus is active)
- **Black filled** = Service is unloaded (lapsus is inactive)

### Command Summary

```bash
# Rebuild app with integration
cd /Users/ryder/bin/lapsus/menubar_app
cargo build --release
./create_app_bundle.sh

# Install
cp -r "Lapsus Control.app" /Applications/

# Test
open "/Applications/Lapsus Control.app"
```

### Backwards Compatibility

If `~/Library/LaunchAgents/com.lapsus.rust.plist` doesn't exist:
- App falls back to direct process management
- Start: Spawns lapsus_rust process directly
- Stop: Sends SIGTERM to process
- Status: Checks process list

This ensures the app works even without the LaunchAgent installed.

### Status

✅ LaunchAgent detection implemented
✅ `launchctl load/unload` commands integrated  
✅ Status checking via `launchctl list`
✅ Fallback to direct process control
✅ App rebuilt and bundled
🎉 Ready to control the service properly!
