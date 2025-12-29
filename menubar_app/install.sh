#!/bin/bash
# Installation script for Lapsus Control

set -e

echo "🚀 Lapsus Control - Installation Script"
echo "========================================"
echo ""

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Please run this script from the menubar_app directory"
    exit 1
fi

# Build the app
echo "📦 Building release version..."
cargo build --release

# Create app bundle with lapsus_rust included
echo "📦 Creating app bundle with lapsus_rust..."
BUNDLE_LAPSUS=true ./create_app_bundle.sh

echo ""
echo "✅ Build complete!"
echo ""

# Check if running as part of automated install or interactive
if [ -t 0 ]; then
    # Interactive mode
    read -p "📥 Install to /Applications? (y/n) " -n 1 -r
    echo ""
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo "Installing to /Applications..."
        cp -r "Lapsus Control.app" /Applications/
        echo "✅ Installed to /Applications/Lapsus Control.app"
        echo ""
        
        read -p "🚀 Launch now? (y/n) " -n 1 -r
        echo ""
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            open "/Applications/Lapsus Control.app"
            echo "✅ Launched! Check your menu bar for the cursor icon."
        fi
    else
        echo "ℹ️  App bundle created at: $(pwd)/Lapsus Control.app"
        echo "   You can install it manually later with:"
        echo "   cp -r 'Lapsus Control.app' /Applications/"
    fi
else
    # Non-interactive mode (just build, don't install)
    echo "ℹ️  App bundle created at: $(pwd)/Lapsus Control.app"
    echo "   Install with: cp -r 'Lapsus Control.app' /Applications/"
fi

echo ""
echo "📚 Documentation:"
echo "   - README.md - User guide"
echo "   - QUICKSTART.md - Quick setup"
echo "   - PATH_FIX.md - Path configuration"
echo ""
echo "✅ Done!"
