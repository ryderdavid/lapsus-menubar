#!/bin/bash
# Test script for LaunchAgent integration

echo "🧪 Testing LaunchAgent Integration"
echo "=================================="
echo ""

# Check initial state
echo "1. Current service status:"
if launchctl list com.lapsus.rust &>/dev/null; then
    echo "   ✅ Service is LOADED"
    launchctl list com.lapsus.rust | grep -E "PID|Label"
else
    echo "   ❌ Service is NOT LOADED"
fi
echo ""

# Check if process is running
echo "2. Process check:"
if ps aux | grep -v grep | grep lapsus_rust > /dev/null; then
    echo "   ✅ lapsus_rust process is RUNNING"
    ps aux | grep -v grep | grep lapsus_rust | awk '{print "   PID:", $2, "CMD:", $11}'
else
    echo "   ❌ lapsus_rust process is NOT RUNNING"
fi
echo ""

echo "3. LaunchAgent plist:"
if [ -f ~/Library/LaunchAgents/com.lapsus.rust.plist ]; then
    echo "   ✅ Found at ~/Library/LaunchAgents/com.lapsus.rust.plist"
else
    echo "   ❌ NOT FOUND at ~/Library/LaunchAgents/com.lapsus.rust.plist"
fi
echo ""

echo "4. Menu bar app status:"
if pgrep -f "lapsus-menubar" > /dev/null; then
    echo "   ✅ Menu bar app is RUNNING"
else
    echo "   ❌ Menu bar app is NOT RUNNING"
fi
echo ""

echo "📝 Next steps:"
echo ""
echo "To test the menu bar app:"
echo "  1. Launch: open 'Lapsus Control.app'"
echo "  2. Look for cursor icon in menu bar"
echo "  3. Click the icon"
echo ""
echo "Expected behavior:"
echo "  - If outline icon → Service is RUNNING"
echo "  - Click 'Disable' → Should unload service"
echo "  - Icon changes to filled → Service is STOPPED"
echo "  - Click 'Enable' → Should load service"
echo "  - Icon changes to outline → Service is RUNNING"
echo ""
echo "Verify with:"
echo "  watch -n 1 'launchctl list com.lapsus.rust 2>&1 | head -3'"
echo ""
