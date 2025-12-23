<div align="center">
  <img src="src-tauri/icons/icon.png" alt="VoiceTypr Logo" width="128" height="128">

  # VoiceTypr

  **Open Source AI Powered voice to text dictation tool, alternative to superwhisper, wispr flow**

  [![GitHub release](https://img.shields.io/github/v/release/moinulmoin/voicetypr)](https://github.com/moinulmoin/voicetypr/releases)
  [![License](https://img.shields.io/badge/license-AGPL--3.0-blue.svg)](LICENSE.md)
  [![macOS](https://img.shields.io/badge/macOS-13.0+-black)](https://www.apple.com/macos)
  [![Windows](https://img.shields.io/badge/Windows-10%2F11-0078D6)](https://www.microsoft.com/windows)

  [Download](https://github.com/moinulmoin/voicetypr/releases/latest) | [Features](#features) | [Installation](#installation) | [Building from Source](#building-from-source)
</div>

## What is VoiceTypr?

VoiceTypr is an open source AI voice-to-text dictation tool, alternative to Wispr Flow and SuperWhisper. Available for macOS and Windows.

## Features

- **System-wide hotkey** for quick recording with automatic text insertion at cursor
- **100% offline** - your voice never leaves your device
- **Multiple AI models** - choose accuracy vs speed tradeoffs
- **99+ languages** supported out of the box
- **Hardware acceleration** - Metal on macOS, Vulkan on Windows
- **AI Enhancement** - optional text cleanup via Ollama, OpenAI, Groq, or Gemini
- **Privacy first** - no cloud, no tracking, open source

## Installation

### macOS
1. Download [VoiceTypr.dmg](https://github.com/moinulmoin/voicetypr/releases/latest)
2. Drag VoiceTypr to Applications
3. Launch and follow onboarding to download an AI model

**Requirements:** macOS 13.0+, 3-4 GB disk space, Microphone & Accessibility permissions

### Windows
1. Download [VoiceTypr installer](https://github.com/moinulmoin/voicetypr/releases/latest)
2. Run installer and launch from Start Menu
3. Follow onboarding to download an AI model

**Requirements:** Windows 10/11 (64-bit), 3-4 GB disk space

## Usage

1. Press your hotkey (default: `Cmd+Shift+Space`) to start recording
2. Speak naturally
3. Press hotkey again to stop - text appears at your cursor

**Tips:**
- Double-press `Esc` to cancel recording
- Whisper auto-detects language

## Building from Source

### Prerequisites

- [Node.js](https://nodejs.org/) v18+
- [pnpm](https://pnpm.io/) (`npm install -g pnpm`)
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [cmake](https://cmake.org/) (`brew install cmake` on macOS)

### Build

```bash
git clone https://github.com/moinulmoin/voicetypr.git
cd voicetypr
pnpm install

# Development
pnpm tauri dev

# Release build
./scripts/build-release.sh
```

### Troubleshooting

```bash
# Reset accessibility permissions (after rebuilding)
tccutil reset Accessibility com.ideaplexa.voicetypr

# Fix permission denied errors (Error 13)
# Use if app crashes on launch or model download fails
sudo chown -R $(whoami):staff ~/Library/Application\ Support/com.ideaplexa.voicetypr/
sudo chown -R $(whoami):staff ~/Library/Logs/com.ideaplexa.voicetypr/

# Clean build directories
rm -rf sidecar/parakeet-swift/.build
rm -rf src-tauri/target
```

> **Warning**: Never run `pnpm tauri dev` or `pnpm tauri build` with `sudo`. This causes permission issues.

### Model Storage

```
# macOS
~/Library/Application Support/com.ideaplexa.voicetypr/models/

# Windows
%APPDATA%\com.ideaplexa.voicetypr\models\
```

## License

[GNU Affero General Public License v3.0](LICENSE.md)
