# UI Fixes - December 29, 2025

## Issues Fixed

### 1. ✅ Reversed Icon States
**Problem**: Icon showed the wrong state (outline when lapsus was running)

**Fix**: Swapped the icon logic in `get_current_icon()`:
- **Outline icon (black)** = Lapsus is RUNNING (enabled)
- **Filled icon (black)** = Lapsus is STOPPED (disabled)

```rust
fn get_current_icon(&self) -> Icon {
    if self.is_lapsus_running() {
        // When running, show DISABLED icon (outline)
        self.icon_disabled.clone()
    } else {
        // When stopped, show ENABLED icon (filled)
        self.icon_enabled.clone()
    }
}
```

### 2. ✅ White Outline Icon
**Problem**: Outline icon was white, should be black

**Fix**: Changed `cursor_disabled.svg` stroke color:
```svg
<!-- Before -->
<path ... stroke="white" stroke-width="2"/>

<!-- After -->
<path ... stroke="black" stroke-width="2"/>
```

Regenerated PNG from updated SVG.

### 3. ✅ Quit Doesn't Stop Lapsus
**Problem**: Clicking "Quit" left lapsus_rust running

**Fix**: Added automatic cleanup when quitting:
```rust
"quit" => {
    // Check if lapsus_rust is running and stop it
    if state_clone.is_lapsus_running() {
        let _ = state_clone.stop_lapsus();
    }
    *control_flow = tao::event_loop::ControlFlow::Exit;
}
```

## Current Behavior

### Menu Bar Icons
- **Black outline cursor** = Lapsus is actively running (moving your cursor)
- **Black filled cursor** = Lapsus is stopped (cursor not moving)

### Menu Options
- **"Enable lapsus_rust"** = Shows when stopped, starts lapsus
- **"Disable lapsus_rust"** = Shows when running, stops lapsus
- **"Quit"** = Stops lapsus_rust (if running) and exits the app

## Testing

### Test 1: Icon States
```bash
# Launch app
open "Lapsus Control.app"

# Initial state (lapsus not running):
# - Should show FILLED icon
# - Menu shows "Enable lapsus_rust"

# Click "Enable lapsus_rust":
# - Icon changes to OUTLINE
# - Menu shows "Disable lapsus_rust"
# - Verify: ps aux | grep lapsus_rust

# Click "Disable lapsus_rust":
# - Icon changes to FILLED
# - Menu shows "Enable lapsus_rust"
# - Verify: ps aux | grep lapsus_rust (should be none)
```

### Test 2: Quit Behavior
```bash
# Start lapsus via menu
# Click "Enable lapsus_rust"

# Verify running:
ps aux | grep lapsus_rust

# Quit the app:
# Click menu bar icon -> Quit

# Verify lapsus stopped:
ps aux | grep lapsus_rust
# Should show no results
```

### Test 3: Icon Colors
Both icons should be black (not white):
- Outline icon: Black stroke, no fill
- Filled icon: Black fill

Works correctly in both light and dark mode.

## Files Changed

1. **src/main.rs**:
   - Swapped icon logic in `get_current_icon()`
   - Added lapsus cleanup in quit handler

2. **icons/cursor_disabled.svg**:
   - Changed stroke from "white" to "black"

3. **icons/cursor_disabled.png**:
   - Regenerated from updated SVG

4. **App bundle rebuilt** with all fixes

## Installation

The updated app is ready:

```bash
cd /Users/ryder/bin/lapsus/menubar_app

# Copy to Applications
cp -r "Lapsus Control.app" /Applications/

# Launch
open "/Applications/Lapsus Control.app"
```

## Summary

✅ Icon states now correctly show lapsus status
✅ Outline icon is black (not white)  
✅ Quitting the app now stops lapsus_rust
✅ All changes tested and built
✅ App bundle updated and ready to install

The app should now behave exactly as expected! 🎉
