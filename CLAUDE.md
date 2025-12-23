# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

VoiceTypr is a native macOS/Windows desktop app for offline voice transcription. Built with Tauri v2 (Rust backend) and React 19 (TypeScript frontend).

**Key Features**: System-wide hotkey recording, offline Whisper transcription, auto-insert at cursor, model management (Whisper + Parakeet), AI text enhancement via Ollama/OpenAI/Groq/Gemini.

## Development Commands

```bash
# Development
pnpm tauri dev              # Full app with hot reload (recommended)
pnpm dev                    # Frontend only (Vite dev server)

# Testing
pnpm test                   # Frontend tests (Vitest)
pnpm test:watch             # Frontend tests in watch mode
pnpm test -- -t "test name" # Run specific test by name
cd src-tauri && cargo test  # Backend tests
pnpm test:all               # Run both frontend and backend tests

# Build & Quality
pnpm tauri build            # Production app bundle (.app/.dmg)
pnpm lint                   # ESLint
pnpm typecheck              # TypeScript compiler check
pnpm format                 # Prettier formatting
```

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
└── README.md                           # Project documentation
```

## Key Files

- `src-tauri/src/lib.rs` - App initialization, tray menu, global shortcuts
- `src-tauri/src/window_manager.rs` - Main/pill window lifecycle
- `src-tauri/src/commands/audio.rs` - Recording and transcription flow
- `src/components/AppContainer.tsx` - Main app layout and event coordination

## Testing

**Backend**: Comprehensive unit tests (160+ functions in `src-tauri/src/tests/`). Run with `cargo test`.

**Frontend**: User-focused integration tests. Key files:
- `App.critical.test.tsx` - Critical user paths
- `App.user.test.tsx` - Common user scenarios
- Component tests only for complex behavior

## Platform-Specific Notes

- **macOS**: Uses Metal GPU acceleration, CoreAudio, Parakeet Swift sidecar (Neural Engine)
- **Windows**: Uses Vulkan GPU acceleration, Whisper models only (no Parakeet)
- Parakeet models are filtered out on non-macOS platforms

## Common Patterns

1. **Tauri Commands**: Frontend calls Rust via `invoke()`, handlers in `src-tauri/src/commands/`
2. **Events**: Backend emits events via `app.emit()`, frontend listens with `listen()`
3. **State Transitions**: Recording uses state machine (Idle → Starting → Recording → Stopping → Transcribing)
4. **Error Boundaries**: React ErrorBoundary wraps components for graceful failure recovery

## Development Guidelines

- Check `specs/` folder for requirements before coding tasks
- Use TypeScript strictly, avoid `any`
- Prefer editing existing files over creating new ones
- Focus on readability over performance unless specified
- Leave NO todos, placeholders, or missing pieces
- Always use `pnpm` (not npm/yarn)
