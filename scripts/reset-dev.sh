#!/bin/bash

# Quick reset script for development testing
# Usage: ./scripts/reset-dev.sh [--clean-build]

set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

APP_ID="com.ideaplexa.voicetypr"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

echo -e "${YELLOW}Quick development reset${NC}"

# Kill app if running
pkill -x voicetypr 2>/dev/null || true

# Check for --clean-build flag
if [[ "$1" == "--clean-build" ]]; then
    echo -e "${YELLOW}→ Cleaning build directories (requires sudo)...${NC}"

    # Reset accessibility permissions
    echo "→ Resetting accessibility permissions..."
    tccutil reset Accessibility "$APP_ID" 2>/dev/null || true

    # Clean Swift sidecar build
    if [ -d "$PROJECT_DIR/sidecar/parakeet-swift/.build" ]; then
        echo "→ Removing Swift sidecar build..."
        sudo rm -rf "$PROJECT_DIR/sidecar/parakeet-swift/.build"
    fi

    # Clean Rust target directory
    if [ -d "$PROJECT_DIR/src-tauri/target" ]; then
        echo "→ Removing Rust target directory..."
        sudo rm -rf "$PROJECT_DIR/src-tauri/target"
    fi

    echo -e "${GREEN}✅ Build directories cleaned!${NC}"
    echo ""
    echo "Now run: pnpm tauri dev"
    exit 0
fi

# Just reset preferences and state, keep models
echo "→ Resetting preferences..."
defaults delete "$APP_ID" 2>/dev/null || true

echo "→ Clearing saved state..."
rm -rf "$HOME/Library/Saved Application State/${APP_ID}.savedState" 2>/dev/null || true

# Only clear app state, not models
rm -f "$HOME/Library/Application Support/${APP_ID}/state.json" 2>/dev/null || true
rm -f "$HOME/Library/Application Support/${APP_ID}/settings.json" 2>/dev/null || true

# Clear keychain
security delete-generic-password -s "${APP_ID}" 2>/dev/null || true

killall cfprefsd 2>/dev/null || true

echo -e "${GREEN}✅ Reset complete!${NC}"
echo "Models kept, onboarding will show again."
echo ""
echo "For full build cleanup, run: ./scripts/reset-dev.sh --clean-build"
