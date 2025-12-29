#!/bin/bash
# Quick test script for Lapsus Control menubar app

echo "🧪 Lapsus Control - Quick Test"
echo "=============================="
echo ""

# Check if lapsus_rust exists
if [ -f "../lapsus_rust" ]; then
    echo "✅ lapsus_rust found"
else
    echo "❌ lapsus_rust NOT found at ../lapsus_rust"
    echo "   Please ensure lapsus_rust is in the parent directory"
    exit 1
fi

# Check if binary exists
if [ -f "target/release/lapsus-menubar" ]; then
    echo "✅ Binary built"
else
    echo "❌ Binary not found. Building now..."
    cargo build --release
fi

# Check if app bundle exists
if [ -d "Lapsus Control.app" ]; then
    echo "✅ App bundle exists"
else
    echo "⚠️  App bundle not found. Creating now..."
    ./create_app_bundle.sh
fi

echo ""
echo "🚀 Launching Lapsus Control..."
echo ""
echo "The app should appear in your menu bar."
echo "Look for a cursor icon near the clock."
echo ""
echo "To test:"
echo "  1. Click the menu bar icon"
echo "  2. Click 'Enable lapsus_rust'"
echo "  3. Verify lapsus_rust is running: ps aux | grep lapsus_rust"
echo "  4. Click 'Disable lapsus_rust'"
echo "  5. Verify it stopped"
echo ""
echo "Press Ctrl+C to stop this test"
echo ""

# Launch the app
open "Lapsus Control.app"

# Wait a moment for it to launch
sleep 2

# Check if app is running
if pgrep -f "lapsus-menubar" > /dev/null; then
    echo "✅ App is running!"
    echo ""
    echo "Menu bar app is now active."
    echo "Use the menu bar icon to control lapsus_rust."
else
    echo "❌ App failed to launch"
    echo "   Check Console.app for errors"
    exit 1
fi
