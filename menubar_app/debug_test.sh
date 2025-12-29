#!/bin/bash
# Debug script to test menu bar app functionality

echo "🔍 Lapsus Control - Debug Test"
echo "==============================="
echo ""

# Test 1: Check lapsus_rust exists
echo "Test 1: Check lapsus_rust executable"
if [ -f "/Users/ryder/bin/lapsus/lapsus_rust" ]; then
    echo "✅ lapsus_rust found at /Users/ryder/bin/lapsus/lapsus_rust"
    ls -lh /Users/ryder/bin/lapsus/lapsus_rust
else
    echo "❌ lapsus_rust NOT found"
    exit 1
fi
echo ""

# Test 2: Check if app bundle exists
echo "Test 2: Check app bundle"
if [ -d "Lapsus Control.app" ]; then
    echo "✅ App bundle exists"
else
    echo "❌ App bundle not found"
    exit 1
fi
echo ""

# Test 3: Check binary in app bundle
echo "Test 3: Check binary in app bundle"
if [ -f "Lapsus Control.app/Contents/MacOS/lapsus-menubar" ]; then
    echo "✅ Binary exists in app bundle"
    ls -lh "Lapsus Control.app/Contents/MacOS/lapsus-menubar"
else
    echo "❌ Binary not found in app bundle"
    exit 1
fi
echo ""

# Test 4: Check icons
echo "Test 4: Check icon resources"
if [ -f "Lapsus Control.app/Contents/Resources/cursor_enabled.png" ] && \
   [ -f "Lapsus Control.app/Contents/Resources/cursor_disabled.png" ]; then
    echo "✅ Icons exist in app bundle"
    ls -lh "Lapsus Control.app/Contents/Resources/"*.png
else
    echo "❌ Icons not found in app bundle"
    exit 1
fi
echo ""

# Test 5: Check if lapsus_rust is currently running
echo "Test 5: Check if lapsus_rust is running"
if pgrep -f "lapsus_rust" > /dev/null; then
    echo "⚠️  lapsus_rust is ALREADY RUNNING"
    echo "   PID: $(pgrep -f lapsus_rust)"
    echo "   You can test Disable first"
else
    echo "✅ lapsus_rust is NOT running (expected initial state)"
    echo "   You can test Enable first"
fi
echo ""

# Test 6: Try to start lapsus_rust manually
echo "Test 6: Manual lapsus_rust start test"
echo "Starting lapsus_rust in background..."
/Users/ryder/bin/lapsus/lapsus_rust &
LAPSUS_PID=$!
sleep 1

if ps -p $LAPSUS_PID > /dev/null; then
    echo "✅ lapsus_rust can start successfully (PID: $LAPSUS_PID)"
    echo "Stopping it..."
    kill $LAPSUS_PID
    sleep 1
    if ps -p $LAPSUS_PID > /dev/null 2>&1; then
        echo "⚠️  Process didn't stop with SIGTERM, using SIGKILL"
        kill -9 $LAPSUS_PID
    fi
    echo "✅ lapsus_rust stopped"
else
    echo "❌ lapsus_rust failed to start"
fi
echo ""

# Test 7: Launch the app
echo "Test 7: Launch the menu bar app"
echo ""
echo "🚀 Launching 'Lapsus Control.app'..."
echo ""
echo "INSTRUCTIONS:"
echo "1. Look for a cursor icon in your menu bar (near the clock)"
echo "2. Click the icon to see the menu"
echo "3. Try these tests:"
echo ""
echo "   TEST A - Enable:"
echo "   • Click 'Enable lapsus_rust'"
echo "   • Check terminal: ps aux | grep lapsus_rust"
echo "   • Icon should change to black filled cursor"
echo ""
echo "   TEST B - Disable:"
echo "   • Click 'Disable lapsus_rust'"
echo "   • Check terminal: ps aux | grep lapsus_rust"
echo "   • Icon should change to white outline cursor"
echo ""
echo "   TEST C - About:"
echo "   • Click 'About'"
echo "   • Dialog should show: Lapsus Control - Version 0.1.0"
echo ""
echo "   TEST D - Start at Login:"
echo "   • Toggle the checkbox"
echo "   • Check: cat ~/.lapsus_menubar_config.json"
echo ""
echo "Press Ctrl+C in this terminal when done testing"
echo ""

# Open the app
open "Lapsus Control.app"

# Wait and monitor
sleep 2

if pgrep -f "lapsus-menubar" > /dev/null; then
    echo "✅ Menu bar app is running!"
    echo ""
    echo "Monitor process status with:"
    echo "  watch -n 1 'ps aux | grep -E \"lapsus_rust|lapsus-menubar\" | grep -v grep'"
    echo ""
else
    echo "❌ Menu bar app failed to launch"
    echo ""
    echo "Check for errors:"
    echo "  log stream --predicate 'process == \"lapsus-menubar\"' --level debug"
    exit 1
fi

# Keep script running to show instructions
echo "Waiting... (Press Ctrl+C to exit)"
while true; do
    sleep 5
    if ! pgrep -f "lapsus-menubar" > /dev/null; then
        echo ""
        echo "⚠️  Menu bar app has exited"
        break
    fi
done
