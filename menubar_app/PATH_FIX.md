# Path Finding Fix - December 29, 2025

## Problem
When copying the app to `/Applications`, it couldn't find `lapsus_rust` because it was only looking for it relative to the menubar_app development directory.

**Error message**:
```
lapsus_rust not found at: "/Applications/Lapsus Control.app/Contents/lapsus_rust"
Please ensure lapsus_rust is in the correct location.
```

## Solution
Implemented a multi-location search strategy with two deployment options:

### Option 1: Bundle lapsus_rust with the App (Recommended)
The app bundle now includes `lapsus_rust` in its Resources folder, making it fully portable.

### Option 2: Use System-wide Installation
The app searches multiple locations automatically.

## Changes Made

### 1. Smart Path Finding (`src/main.rs`)

Added `find_lapsus_rust()` function that searches in order:

1. **Custom config path** (if set in `~/.lapsus_menubar_config.json`)
2. **Bundled with app** (`Contents/MacOS/lapsus_rust`)
3. **App Resources** (`Contents/Resources/lapsus_rust`)
4. **Development location** (`/Users/ryder/bin/lapsus/lapsus_rust`)
5. **User bin** (`~/bin/lapsus/lapsus_rust`)
6. **System bin** (`/usr/local/bin/lapsus_rust`)

### 2. Updated Config Structure

Added optional `lapsus_rust_path` field:
```json
{
  "start_at_login": false,
  "lapsus_rust_path": "/custom/path/to/lapsus_rust"
}
```

### 3. Enhanced Bundle Script

Updated `create_app_bundle.sh` to:
- Bundle `lapsus_rust` by default
- Skip bundling with `BUNDLE_LAPSUS=false`
- Show clear feedback about bundle contents

## Usage

### Creating a Portable App Bundle

```bash
cd /Users/ryder/bin/lapsus/menubar_app

# Create bundle with lapsus_rust included (default)
./create_app_bundle.sh

# OR explicitly
BUNDLE_LAPSUS=true ./create_app_bundle.sh
```

This creates a **fully portable** app that can be:
- Copied to `/Applications`
- Moved to any location
- Shared with other users (on same architecture)

### Creating a Lightweight Bundle

```bash
cd /Users/ryder/bin/lapsus/menubar_app

# Create bundle WITHOUT lapsus_rust
BUNDLE_LAPSUS=false ./create_app_bundle.sh
```

This is smaller but requires `lapsus_rust` to be installed separately.

### Setting a Custom Path

Edit `~/.lapsus_menubar_config.json`:
```json
{
  "start_at_login": false,
  "lapsus_rust_path": "/usr/local/bin/lapsus_rust"
}
```

## Installation

### For /Applications (Recommended - Bundled)

```bash
cd /Users/ryder/bin/lapsus/menubar_app

# Create bundled app
./create_app_bundle.sh

# Install to Applications
cp -r "Lapsus Control.app" /Applications/

# Launch
open /Applications/Lapsus\ Control.app
```

### For Development Use

```bash
cd /Users/ryder/bin/lapsus/menubar_app

# Run directly (will find lapsus_rust in parent dir)
./target/release/lapsus-menubar
```

## Verification

### Check if lapsus_rust is bundled:

```bash
ls -lh "Lapsus Control.app/Contents/Resources/lapsus_rust"
```

Expected output if bundled:
```
-rwxr-xr-x@ 1 ryder  staff   1.6M Dec 29 12:33 lapsus_rust
```

### Check search paths at runtime:

The app will check these locations in order and use the first one found:

```bash
# 1. Custom path from config
cat ~/.lapsus_menubar_config.json

# 2. Bundled with app
ls "Lapsus Control.app/Contents/Resources/lapsus_rust"

# 3. Development location
ls /Users/ryder/bin/lapsus/lapsus_rust

# 4. User bin
ls ~/bin/lapsus/lapsus_rust

# 5. System bin
ls /usr/local/bin/lapsus_rust
```

## Testing

### Test 1: Bundled App in /Applications

```bash
# Create bundled app
cd /Users/ryder/bin/lapsus/menubar_app
./create_app_bundle.sh

# Copy to Applications
cp -r "Lapsus Control.app" /Applications/

# Launch
open /Applications/Lapsus\ Control.app

# Should work without any errors
```

### Test 2: Unbundled App with System Installation

```bash
# Copy lapsus_rust to system bin
sudo cp /Users/ryder/bin/lapsus/lapsus_rust /usr/local/bin/
sudo chmod +x /usr/local/bin/lapsus_rust

# Create unbundled app
cd /Users/ryder/bin/lapsus/menubar_app
BUNDLE_LAPSUS=false ./create_app_bundle.sh

# Copy to Applications
cp -r "Lapsus Control.app" /Applications/

# Launch
open /Applications/Lapsus\ Control.app

# Should find lapsus_rust in /usr/local/bin
```

### Test 3: Custom Path

```bash
# Set custom path
cat > ~/.lapsus_menubar_config.json << 'EOF'
{
  "start_at_login": false,
  "lapsus_rust_path": "/Users/ryder/bin/lapsus/lapsus_rust"
}
EOF

# Launch app
open "Lapsus Control.app"

# Should use the custom path
```

## Benefits

✅ **Portable**: Bundle includes everything needed
✅ **Flexible**: Can use system-wide installation instead
✅ **Configurable**: Custom paths via config file
✅ **Robust**: Multiple fallback locations
✅ **User-friendly**: Clear error messages

## File Sizes

- **Bundled app**: ~5 MB (includes 1.6 MB lapsus_rust)
- **Unbundled app**: ~3.3 MB (menubar app only)

## Status

✅ Path finding implemented
✅ Bundling support added
✅ Config support added
✅ App bundle rebuilt with lapsus_rust included
🎉 Ready for /Applications installation!

## Quick Commands

```bash
# Rebuild everything with bundling
cd /Users/ryder/bin/lapsus/menubar_app
cargo build --release
./create_app_bundle.sh

# Install to Applications
cp -r "Lapsus Control.app" /Applications/

# Launch
open /Applications/Lapsus\ Control.app
```
