#!/bin/bash
# Build VoiceTypr release bundle for macOS
# This creates a .app bundle (skips DMG for local builds)

set -e  # Exit on error

# Warn if running as root (causes permission issues)
if [ "$EUID" -eq 0 ]; then
    echo "⚠️  WARNING: Running as root/sudo is not recommended!"
    echo "   This can cause permission issues with app directories."
    echo "   Press Ctrl+C to cancel, or wait 5 seconds to continue..."
    sleep 5
fi

echo "🔧 Building VoiceTypr release..."
echo ""

# Navigate to project root
cd "$(dirname "$0")/.."

# Skip code signing for local builds (ad-hoc signing)
export APPLE_SIGNING_IDENTITY="-"

# Build the release - only create .app bundle, skip DMG
echo "📦 Running Tauri build (ad-hoc signed for local use)..."
pnpm tauri build --bundles app

echo ""
echo "✅ Build complete!"
echo ""
echo "📍 Your app bundle is at:"
echo "   src-tauri/target/release/bundle/macos/VoiceTypr.app"
echo ""
echo "🚀 To install:"
echo "   Drag VoiceTypr.app to /Applications"
echo ""

# Open the bundle folder in Finder
open src-tauri/target/release/bundle/macos/
