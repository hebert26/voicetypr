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

# Stop the app if running
echo "🛑 Stopping VoiceTypr if running..."
pkill -9 voicetypr 2>/dev/null || true
pkill -9 VoiceTypr 2>/dev/null || true
sleep 1

# Remove old version from Applications folder
if [ -d "/Applications/VoiceTypr.app" ]; then
    echo "🗑️  Removing old version from /Applications..."
    rm -rf "/Applications/VoiceTypr.app"
fi

# Navigate to project root
cd "$(dirname "$0")/.."

# Clean up root-owned build directories (from previous sudo runs)
DIRS_TO_CHECK=("sidecar/parakeet-swift/.build" "dist" "src-tauri/target")
for DIR in "${DIRS_TO_CHECK[@]}"; do
    if [ -d "$DIR" ]; then
        OWNER=$(stat -f '%Su' "$DIR" 2>/dev/null || echo "unknown")
        if [ "$OWNER" = "root" ]; then
            echo "🧹 Cleaning root-owned directory: $DIR"
            sudo rm -rf "$DIR"
        fi
    fi
done

# Also check for root-owned files in dist (can happen with partial builds)
if [ -d "dist" ]; then
    ROOT_FILES=$(find dist -user root 2>/dev/null | head -1)
    if [ -n "$ROOT_FILES" ]; then
        echo "🧹 Cleaning root-owned files in dist..."
        sudo rm -rf dist
    fi
fi

# Use Apple Development certificate for consistent identity across builds
# This prevents permission issues when updating the app
export APPLE_SIGNING_IDENTITY="Apple Development: heberth26@gmail.com (R8728DNM66)"

# Build the release - only create .app bundle, skip DMG
echo "📦 Running Tauri build (signed with Apple Development certificate)..."
pnpm tauri build --bundles app

echo ""
echo "✅ Build complete!"
echo ""

# Copy to Applications folder
echo "📦 Installing to /Applications..."
cp -R src-tauri/target/release/bundle/macos/VoiceTypr.app /Applications/

echo "🚀 VoiceTypr installed to /Applications"
echo ""
echo "📍 Starting VoiceTypr..."
open /Applications/VoiceTypr.app
