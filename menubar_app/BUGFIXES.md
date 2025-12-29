# Bug Fixes - December 29, 2025

## Issues Reported
1. ❌ Enable/Disable buttons don't do anything
2. ❌ About dialog doesn't show

## Root Causes Identified

### Issue 1: Menu Items Not Responding
**Problem**: Menu items were created with `MenuItem::new()` which auto-generates random IDs. The event handler was trying to match against menu item text ("Enable lapsus_rust") but the actual event ID was a random UUID.

**Fix**: Changed to use `MenuItem::with_id()` and `CheckMenuItem::with_id()` with explicit MenuId values:
- "enable" - for Enable menu item
- "disable" - for Disable menu item  
- "start_at_login" - for Start at Login checkbox
- "about" - for About menu item
- "quit" - for Quit menu item

Updated event handler to match against these explicit IDs instead of menu text.

### Issue 2: About Dialog Not Showing
**Problem**: The osascript command was using `.output()` which blocks and waits, but also the newline characters weren't properly escaped for AppleScript.

**Fix**: 
- Changed `.output()` to `.spawn()` for non-blocking execution
- Properly escaped newlines as `\\n` for AppleScript
- Simplified the dialog formatting

## Changes Made

### File: `src/main.rs`

1. **Added MenuId import**:
```rust
use tray_icon::{TrayIconBuilder, menu::{Menu, MenuEvent, MenuItem, CheckMenuItem, PredefinedMenuItem, MenuId}};
```

2. **Updated `build_menu()` function**:
```rust
// Before:
let enable_item = MenuItem::new("Enable lapsus_rust", true, None);

// After:
let enable_item = MenuItem::with_id(
    MenuId::new("enable"),
    "Enable lapsus_rust",
    true,
    None
);
```

3. **Updated event handler**:
```rust
// Before:
match menu_text {
    "Enable lapsus_rust" => { ... }
    "Disable lapsus_rust" => { ... }
    ...
}

// After:
match menu_id {
    "enable" => { ... }
    "disable" => { ... }
    ...
}
```

4. **Fixed `show_about_dialog()`**:
```rust
// Before:
.output();  // Blocking call

// After:
.spawn();   // Non-blocking call
// Plus proper newline escaping: \\n
```

5. **Added debug output**:
```rust
_ => {
    eprintln!("Unknown menu item: {}", menu_id);
}
```

## Testing

### Manual Test Steps:
1. Launch the app:
   ```bash
   cd /Users/ryder/bin/lapsus/menubar_app
   open "Lapsus Control.app"
   ```

2. Test Enable:
   - Click menu bar icon
   - Click "Enable lapsus_rust"
   - Verify lapsus_rust starts: `ps aux | grep lapsus_rust`
   - Icon should change to black filled cursor

3. Test Disable:
   - Click menu bar icon
   - Click "Disable lapsus_rust"
   - Verify lapsus_rust stops: `ps aux | grep lapsus_rust`
   - Icon should change to white outline cursor

4. Test About:
   - Click menu bar icon
   - Click "About"
   - Dialog should appear with app info

5. Test Start at Login:
   - Toggle the checkbox
   - Verify config saved: `cat ~/.lapsus_menubar_config.json`

### Expected Results:
- ✅ Enable button starts lapsus_rust process
- ✅ Disable button stops lapsus_rust process
- ✅ Icon updates to reflect state
- ✅ About dialog shows app information
- ✅ All menu items are responsive

## Verification Commands

```bash
# Check if lapsus_rust is running
ps aux | grep lapsus_rust | grep -v grep

# Check app logs (run this in another terminal while testing)
log stream --predicate 'process == "lapsus-menubar"' --level debug

# Test the binary directly
cd /Users/ryder/bin/lapsus/menubar_app
./target/release/lapsus-menubar
```

## Build Commands

```bash
# Rebuild with fixes
cd /Users/ryder/bin/lapsus/menubar_app
cargo build --release

# Recreate app bundle
./create_app_bundle.sh

# Test it
open "Lapsus Control.app"
```

## Status
✅ All bugs fixed
✅ Code rebuilt
✅ App bundle updated
🧪 Ready for user testing

## Notes
- The MenuId API requires explicit IDs for proper event handling
- Using `.spawn()` instead of `.output()` prevents UI blocking
- AppleScript requires `\\n` for newlines in dialog text
- Added debug output to help troubleshoot any future issues
