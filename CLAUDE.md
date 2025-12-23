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

## Architecture

### Frontend (`src/`)
- **Framework**: React 19 + TypeScript + Tailwind CSS v4
- **UI Components**: shadcn/ui in `src/components/ui/`
- **State**: React Context API (`src/contexts/`) for License, Settings, Readiness, ModelManagement
- **Hooks**: Custom hooks in `src/hooks/` for recording, permissions, models
- **Path Alias**: `@/*` maps to `./src/*`

### Backend (`src-tauri/src/`)
- **Core Modules**:
  - `audio/` - Recording via CoreAudio/CPAL, format conversion, silence detection
  - `whisper/` - Whisper model management, transcription, caching
  - `parakeet/` - Swift sidecar for Apple Neural Engine (macOS only)
  - `ai/` - Enhancement providers (Ollama, OpenAI, Groq, Gemini)
  - `commands/` - Tauri IPC handlers (86 public functions)
  - `license/` - License validation and keychain storage
- **State**: `AppState` in `lib.rs` manages recording state machine, hotkeys, window manager
- **Security**: AES-GCM encryption (`secure_store.rs`), OS keychain for API keys

### Sidecars (`sidecar/`)
- `parakeet-swift/` - Swift binary for Parakeet transcription (Apple Neural Engine, macOS only)
- `ffmpeg/` - Audio format conversion binaries

### Key Files
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
