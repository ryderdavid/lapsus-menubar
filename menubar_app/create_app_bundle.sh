#!/bin/bash

# Script to create a macOS app bundle for Lapsus Control

set -e

APP_NAME="Lapsus Control"
BUNDLE_NAME="Lapsus Control.app"
VERSION="0.1.0"
IDENTIFIER="com.lapsus.menubar"
BUNDLE_LAPSUS="${BUNDLE_LAPSUS:-true}"  # Set to "false" to skip bundling lapsus_rust

echo "Building release binary..."
cargo build --release

echo "Creating app bundle structure..."
rm -rf "$BUNDLE_NAME"
mkdir -p "$BUNDLE_NAME/Contents/MacOS"
mkdir -p "$BUNDLE_NAME/Contents/Resources"

echo "Copying binary..."
cp target/release/lapsus-menubar "$BUNDLE_NAME/Contents/MacOS/"

echo "Copying icons..."
cp icons/*.png "$BUNDLE_NAME/Contents/Resources/"

echo "Copying app icon..."
if [ -f "icon/AppIcon.icns" ]; then
    cp icon/AppIcon.icns "$BUNDLE_NAME/Contents/Resources/"
    echo "✅ App icon included"
else
    echo "⚠️  Warning: AppIcon.icns not found"
fi

# Optionally bundle lapsus_rust
if [ "$BUNDLE_LAPSUS" = "true" ]; then
    if [ -f "../lapsus_rust" ]; then
        echo "Bundling lapsus_rust executable..."
        cp ../lapsus_rust "$BUNDLE_NAME/Contents/Resources/"
        chmod +x "$BUNDLE_NAME/Contents/Resources/lapsus_rust"
        echo "✅ lapsus_rust bundled with app"
    else
        echo "⚠️  Warning: ../lapsus_rust not found, skipping bundle"
        echo "   App will search for lapsus_rust in standard locations"
    fi
else
    echo "ℹ️  Skipping lapsus_rust bundling (BUNDLE_LAPSUS=false)"
    echo "   App will search for lapsus_rust in standard locations"
fi

echo "Creating Info.plist..."
cat > "$BUNDLE_NAME/Contents/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>en</string>
    <key>CFBundleExecutable</key>
    <string>lapsus-menubar</string>
    <key>CFBundleIdentifier</key>
    <string>$IDENTIFIER</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>$APP_NAME</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>$VERSION</string>
    <key>CFBundleVersion</key>
    <string>1</string>
    <key>CFBundleIconFile</key>
    <string>AppIcon</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.14</string>
    <key>LSUIElement</key>
    <true/>
    <key>NSHighResolutionCapable</key>
    <true/>
</dict>
</plist>
EOF

echo "Making binary executable..."
chmod +x "$BUNDLE_NAME/Contents/MacOS/lapsus-menubar"

echo ""
echo "✅ App bundle created successfully!"
echo ""
echo "Location: $(pwd)/$BUNDLE_NAME"
echo ""
if [ -f "$BUNDLE_NAME/Contents/Resources/lapsus_rust" ]; then
    echo "📦 Bundle includes lapsus_rust - can be copied anywhere"
else
    echo "⚠️  Bundle does NOT include lapsus_rust"
    echo "   Ensure lapsus_rust is at: /Users/ryder/bin/lapsus/lapsus_rust"
    echo "   Or set custom path in: ~/.lapsus_menubar_config.json"
fi
echo ""
echo "To run:"
echo "  open '$BUNDLE_NAME'"
echo ""
echo "To install:"
echo "  cp -r '$BUNDLE_NAME' /Applications/"
echo ""

