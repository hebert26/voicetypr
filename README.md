# VoiceTypr

Offline AI voice-to-text dictation for macOS/Windows. Alternative to Wispr Flow and SuperWhisper.

## How It Works

VoiceTypr runs entirely on your machine. Press a global hotkey from any app, speak, and your transcribed text is automatically inserted at your cursor. No cloud, no subscription, no data leaving your device.

**Under the hood:**
- **Tauri v2** (Rust backend) handles audio capture, hotkeys, and system integration
- **Whisper** or **Parakeet** (macOS only, Apple Neural Engine) transcribes speech locally
- **Optional AI cleanup** via Ollama, OpenAI, Groq, or Gemini fixes grammar and removes filler words
- **React 19 + TypeScript** frontend for settings and model management


## Logs
~/Library/Logs/com.ideaplexa.voicetypr/`


## Project Structure

```
voicetypr/
├── src/                                # React Frontend (TypeScript)
│   ├── main.tsx                        # App entry point
│   ├── App.tsx                         # Root component, routing
│   ├── pill.tsx                        # Recording pill overlay window
│   ├── types.ts                        # Shared TypeScript types
│   │
│   ├── components/
│   │   ├── AppContainer.tsx            # Main layout, event coordination
│   │   ├── Sidebar.tsx                 # Navigation sidebar
│   │   ├── RecordingPill.tsx           # Floating recording indicator
│   │   ├── ModelCard.tsx               # Whisper/Parakeet model display
│   │   ├── HotkeyInput.tsx             # Hotkey capture input
│   │   ├── MicrophoneSelection.tsx     # Audio input selector
│   │   ├── LanguageSelection.tsx       # Transcription language picker
│   │   ├── EnhancementSettings.tsx     # AI enhancement config
│   │   ├── EnhancementModelCard.tsx    # Ollama model display
│   │   ├── ApiKeyModal.tsx             # API key entry dialog
│   │   ├── OllamaConfigModal.tsx       # Ollama connection settings
│   │   ├── OpenAICompatConfigModal.tsx # OpenAI-compatible API config
│   │   ├── ErrorBoundary.tsx           # React error boundary
│   │   ├── AudioWaveAnimation.tsx      # Recording waveform visual
│   │   ├── ActivityGraph.tsx           # Usage statistics chart
│   │   ├── ShareStatsModal.tsx         # Stats sharing dialog
│   │   │
│   │   ├── tabs/                       # Main app tabs
│   │   │   ├── OverviewTab.tsx         # Dashboard/home
│   │   │   ├── ModelsTab.tsx           # Model management
│   │   │   ├── EnhancementsTab.tsx     # AI enhancement settings
│   │   │   ├── RecordingsTab.tsx       # Transcription history
│   │   │   ├── SettingsTab.tsx         # General settings
│   │   │   ├── AdvancedTab.tsx         # Advanced options
│   │   │   ├── AccountTab.tsx          # License/account
│   │   │   ├── HelpTab.tsx             # Help & support
│   │   │   └── AboutTab.tsx            # App info
│   │   │
│   │   ├── sections/                   # Reusable content sections
│   │   │   ├── ModelsSection.tsx       # Model list/download UI
│   │   │   ├── EnhancementsSection.tsx # Enhancement provider UI
│   │   │   ├── GeneralSettings.tsx     # Core settings form
│   │   │   ├── RecentRecordings.tsx    # Recent transcriptions list
│   │   │   ├── AudioUploadSection.tsx  # Manual audio file upload
│   │   │   ├── AccountSection.tsx      # License status display
│   │   │   ├── HelpSection.tsx         # FAQ/troubleshooting
│   │   │   ├── AboutSection.tsx        # Version info
│   │   │   └── AdvancedSection.tsx     # Debug/advanced settings
│   │   │
│   │   ├── ui/                         # shadcn/ui components
│   │   │   └── (button, card, dialog, select, switch, etc.)
│   │   │
│   │   └── onboarding/
│   │       └── OnboardingDesktop.tsx   # First-run setup wizard
│   │
│   ├── contexts/                       # React Context providers
│   │   ├── SettingsContext.tsx         # App settings state
│   │   ├── ModelManagementContext.tsx  # Model download/status
│   │   ├── ReadinessContext.tsx        # App readiness checks
│   │   └── LicenseContext.tsx          # License validation
│   │
│   ├── hooks/                          # Custom React hooks
│   │   ├── useRecording.ts             # Recording start/stop logic
│   │   ├── useModelManagement.ts       # Model download/delete
│   │   ├── useModelAvailability.ts     # Check installed models
│   │   ├── usePermissions.ts           # Permission state
│   │   ├── useMicrophonePermission.ts  # Mic permission check
│   │   ├── useAccessibilityPermission.ts # Accessibility check
│   │   ├── useAppReadiness.ts          # Combined readiness
│   │   ├── useEventCoordinator.ts      # Backend event handling
│   │   └── useLicenseStatus.ts         # License state
│   │
│   ├── lib/                            # Utility libraries
│   │   ├── EventCoordinator.ts         # Backend event dispatcher
│   │   ├── keyboard-normalizer.ts      # Cross-platform key mapping
│   │   ├── keyboard-mapper.ts          # Hotkey parsing
│   │   ├── hotkey-utils.tsx            # Hotkey display helpers
│   │   ├── hotkey-conflicts.ts         # Hotkey conflict detection
│   │   ├── cloudProviders.ts           # AI provider definitions
│   │   └── platform.ts                 # OS detection
│   │
│   ├── services/
│   │   └── updateService.ts            # Auto-update logic
│   │
│   └── test/                           # Test setup
│       └── setup.ts                    # Vitest configuration
│
├── src-tauri/                          # Rust Backend (Tauri v2)
│   ├── Cargo.toml                      # Rust dependencies
│   ├── tauri.conf.json                 # Tauri config, permissions
│   │
│   └── src/
│       ├── main.rs                     # Entry point
│       ├── lib.rs                      # App init, tray, shortcuts
│       ├── window_manager.rs           # Main/pill window lifecycle
│       ├── state_machine.rs            # Recording state transitions
│       ├── secure_store.rs             # AES-GCM encryption
│       ├── simple_cache.rs             # In-memory caching
│       │
│       ├── commands/                   # Tauri IPC handlers (86 functions)
│       │   ├── audio.rs                # Recording, transcription flow
│       │   ├── model.rs                # Model download/delete
│       │   ├── stt.rs                  # Speech-to-text orchestration
│       │   ├── ai.rs                   # AI enhancement calls
│       │   ├── settings.rs             # Settings read/write
│       │   ├── permissions.rs          # OS permission checks
│       │   ├── window.rs               # Window management
│       │   ├── clipboard.rs            # Clipboard operations
│       │   ├── text.rs                 # Text insertion at cursor
│       │   ├── license.rs              # License validation
│       │   ├── keyring.rs              # OS keychain access
│       │   ├── device.rs               # Device info
│       │   ├── logs.rs                 # Log file access
│       │   ├── debug.rs                # Debug utilities
│       │   ├── reset.rs                # App reset functions
│       │   └── utils.rs                # Misc helpers
│       │
│       ├── audio/                      # Audio processing
│       │   ├── recorder.rs             # CoreAudio/CPAL capture
│       │   ├── converter.rs            # Format conversion
│       │   ├── resampler.rs            # Sample rate conversion
│       │   ├── normalizer.rs           # Volume normalization
│       │   ├── silence_detector.rs     # Silence/speech detection
│       │   └── level_meter.rs          # Audio level monitoring
│       │
│       ├── whisper/                    # Whisper transcription
│       │   ├── manager.rs              # Model lifecycle
│       │   ├── transcriber.rs          # Transcription execution
│       │   ├── cache.rs                # Model caching
│       │   └── languages.rs            # Language codes/names
│       │
│       ├── parakeet/                   # Parakeet (macOS Neural Engine)
│       │   ├── manager.rs              # Swift sidecar lifecycle
│       │   ├── sidecar.rs              # Sidecar communication
│       │   ├── models.rs               # Model definitions
│       │   ├── messages.rs             # IPC message types
│       │   └── error.rs                # Error types
│       │
│       ├── ai/                         # AI text enhancement
│       │   ├── mod.rs                  # Provider routing
│       │   ├── prompts.rs              # Enhancement prompts
│       │   ├── config.rs               # Provider configuration
│       │   └── openai.rs               # OpenAI-compatible client
│       │
│       ├── license/                    # License management
│       │   ├── api_client.rs           # License server API
│       │   ├── keychain.rs             # OS keychain storage
│       │   ├── device.rs               # Device fingerprinting
│       │   └── types.rs                # License types
│       │
│       ├── utils/                      # Utilities
│       │   ├── logger.rs               # Logging setup
│       │   ├── diagnostics.rs          # System diagnostics
│       │   ├── system_monitor.rs       # Resource monitoring
│       │   └── network_diagnostics.rs  # Network checks
│       │
│       ├── state/                      # App state
│       │   └── unified_state.rs        # Global state container
│       │
│       ├── ffmpeg/                     # FFmpeg integration
│       │   └── mod.rs                  # Audio conversion via ffmpeg
│       │
│       └── tests/                      # Backend tests (160+ functions)
│           └── (comprehensive unit tests)
│
├── sidecar/                            # External binaries
│   ├── parakeet-swift/                 # Parakeet Swift sidecar (macOS)
│   │   ├── Package.swift               # Swift package manifest
│   │   └── Sources/
│   │       └── main.swift              # Neural Engine transcription
│   │
│   └── ffmpeg/                         # FFmpeg binaries
│       └── dist/                       # Platform-specific builds
│
├── scripts/
│   └── build-release.sh                # Production build script
│
├── public/                             # Static assets
├── specs/                              # Feature specifications
├── CLAUDE.md                           # AI assistant instructions
└── README.md                           # This file
```

## Quick Commands

```bash
# Development (hot reload)
pnpm tauri dev

# Release build
./scripts/build-release.sh

# Tests
pnpm test                    # Frontend (Vitest)
cd src-tauri && cargo test   # Backend (Rust)

# Quality
pnpm typecheck               # TypeScript check
pnpm lint                    # ESLint
pnpm format                  # Prettier
```

## Troubleshooting

```bash
# Reset accessibility permissions (after rebuild)
tccutil reset Accessibility com.ideaplexa.voicetypr

# Fix permission errors (Error 13)
sudo chown -R $(whoami):staff ~/Library/Application\ Support/com.ideaplexa.voicetypr/
sudo chown -R $(whoami):staff ~/Library/Logs/com.ideaplexa.voicetypr/

# Clean build
rm -rf sidecar/parakeet-swift/.build src-tauri/target
```

> **Warning**: Never run `pnpm tauri dev` or `pnpm tauri build` with `sudo`.

## AI Enhancement (Ollama)

```bash
# Recommended models for text cleanup (smallest → largest)
ollama pull gemma3:1b        # Fastest, ~1GB
ollama pull gemma3:4b        # Better quality, ~3GB
ollama pull qwen2.5:1.5b     # Alternative, ~1GB
```

## Setup (First Time)

```bash
# Prerequisites: Node.js 18+, Rust, cmake
brew install cmake           # macOS
pnpm install                 # Install dependencies
```

## Model Storage

```
macOS:   ~/Library/Application Support/com.ideaplexa.voicetypr/models/
Windows: %APPDATA%\com.ideaplexa.voicetypr\models\
```

## Features

- **System-wide hotkey** - `Cmd+Shift+Space` to record, auto-insert at cursor
- **100% offline** - Whisper runs locally, voice never leaves device
- **Hardware acceleration** - Metal (macOS), Vulkan (Windows)
- **AI Enhancement** - Optional cleanup via Ollama, OpenAI, Groq, Gemini
- **99+ languages** - Auto-detected by Whisper

## Usage

1. Press hotkey → speak → press again → text inserted at cursor
2. Double-press `Esc` to cancel

## License

[GNU Affero General Public License v3.0](LICENSE.md)
