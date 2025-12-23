# AGENTS.md — AI Agent Guide for VoiceTypr

## Purpose & Scope
Guidance for AI coding agents and contributors working in this repository. Keep changes minimal, correct, and aligned with existing patterns. For deeper context, read:
- `agent-docs/ARCHITECTURE.md`
- `agent-docs/README.md`
- `agent-reports/` (analysis reports)
- `CLAUDE.md` (coding assistant ground rules)

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
├── agent-docs/                         # Architecture documentation
├── agent-reports/                      # Analysis reports
├── CLAUDE.md                           # AI assistant instructions
├── AGENTS.md                           # This file
└── README.md                           # Project documentation
```

Path alias: `@/*` → `./src/*` (see `tsconfig.json`)

## Toolchain & Commands
- Dev: `pnpm dev` (frontend), `pnpm tauri dev` (full app)
- Build: `pnpm build`
- Quality: `pnpm lint`, `pnpm typecheck`, `pnpm format`, `pnpm quality-gate`
- Tests: `pnpm test`, `pnpm test:watch`, `pnpm test:backend` (Cargo)

## Coding Conventions
- Frontend
  - React 19 with function components + hooks; strict TypeScript (see `tsconfig.json`)
  - Tailwind CSS v4 utilities; shadcn/ui components in `src/components/ui/`
  - Keep logic in hooks/lib; small, focused components; no unnecessary comments
- Backend
  - Rust 2021+, modules under `src-tauri/src/*`; run `cargo fmt`/`clippy` locally
  - Tauri v2 commands in `src-tauri/src/commands`; audio/whisper modules encapsulate native work

## Testing Strategy
- Frontend: Vitest + React Testing Library; component tests near components (e.g. `__tests__`) and integration in `src/test/`
- Backend: Rust unit/integration tests in `src-tauri/src/tests`; run with `pnpm test:backend`

## Agent Workflow & Guardrails
1. Understand first: prefer `functions.Read`, `Grep`, `Glob`, `LS` for exploration; use absolute paths.
2. Spec-first when asked “how to approach”: propose a concise plan before edits; await approval.
3. Follow existing patterns/libraries; do not introduce new deps without necessity.
4. Before completion: run `pnpm lint`, `pnpm typecheck`, `pnpm test`, and `cd src-tauri && cargo test` unless explicitly waived.
5. Git safety: `git status` → review diffs → commit; never include secrets; don’t push unless asked.

## Commit & PR Guidelines
- Conventional Commits (e.g., `feat:`, `fix:`, `docs:`); keep scopes tight and messages concise.
- Run `pnpm quality-gate` before opening PRs; document capability changes (Tauri) in the PR.

## References
- `agent-docs/ARCHITECTURE.md`, `agent-docs/EVENT-FLOW-ANALYSIS.md`, security docs in `agent-docs/` and `agent-reports/`
- `README.md` for product overview and repo structure
- `CLAUDE.md` for assistant rules and commands
