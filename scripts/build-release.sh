#!/bin/bash
# Build VoiceTypr release bundle for macOS
# This creates a .app bundle and .dmg installer

set -e  # Exit on error

echo "🔧 Building VoiceTypr release..."
echo ""

# Navigate to project root
cd "$(dirname "$0")/.."

# Clean previous builds (optional, uncomment if needed)
# echo "🧹 Cleaning previous builds..."
# rm -rf src-tauri/target/release/bundle

# Skip code signing for local builds (no Developer ID certificate needed)
# Remove this line if you have a valid signing certificate installed
export APPLE_SIGNING_IDENTITY="-"

# Build the release
echo "📦 Running Tauri build (unsigned for local use)..."
pnpm tauri build

echo ""
echo "✅ Build complete!"
echo ""
echo "📍 Your app bundle is at:"
echo "   src-tauri/target/release/bundle/macos/VoiceTypr.app"
echo ""
echo "📍 DMG installer (if generated):"
echo "   src-tauri/target/release/bundle/dmg/VoiceTypr_*.dmg"
echo ""
echo "🚀 To install, either:"
echo "   1. Drag VoiceTypr.app to /Applications"
echo "   2. Open the .dmg and drag to Applications"
echo ""

# Open the bundle folder in Finder
open src-tauri/target/release/bundle/macos/
