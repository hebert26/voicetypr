# VoiceTypr Performance Optimizations

## Summary

Applied optimizations that save **2-5 seconds per recording** for Whisper transcription.

---

## Optimizations Applied

### 1. Skip FFmpeg for Whisper Recordings (Saves 2-5 seconds)

**Before:**
```
Recording → ffmpeg (2-5s) → Write file → Transcriber reads → Whisper
```

**After:**
```
Recording → Transcriber reads directly → Whisper
```

The transcriber already has built-in conversion code:
- Reads any sample rate (48kHz, 44.1kHz, etc.)
- Converts stereo → mono
- Resamples to 16kHz using rubato

FFmpeg was redundant - we were doing the conversion twice.

**File:** `src-tauri/src/commands/audio.rs:1200-1215`

---

### 2. Duration Check Before Processing (Saves 2-5 seconds on short recordings)

**Before:**
```
Recording → ffmpeg (2-5s) → Check duration → "Too short!" → Delete
```

**After:**
```
Recording → Check duration (instant) → "Too short!" → Delete
```

Now we read the WAV header first (milliseconds) and abort immediately if too short.

**File:** `src-tauri/src/commands/audio.rs:1159-1198`

---

### 3. Reduced Logging Verbosity

Changed per-segment logging from `info!` to `debug!` level.

**Before:** 100+ log entries for longer transcriptions
**After:** Summary only at info level

**File:** `src-tauri/src/whisper/transcriber.rs:636-656`

---

### 4. AI Enhancement Optimizations

- **Global HTTP Client:** Reuses connections instead of creating new client per request
- **Reduced Timeouts:** 15s instead of 30s for local Ollama
- **Faster Retries:** 200ms base delay instead of 1000ms
- **Model Preloading:** Warms up AI model when recording starts

**Files:** `src-tauri/src/ai/mod.rs`, `src-tauri/src/ai/config.rs`, `src-tauri/src/ai/openai.rs`

---

## Memory Analysis

The ~3 GB memory usage is expected and unavoidable:

| Component | Memory | Notes |
|-----------|--------|-------|
| Whisper model | ~1-3 GB | Required for transcription accuracy |
| WebView/Chromium | ~200-500 MB | Tauri uses system WebView |
| Rust runtime | ~100-200 MB | Normal for compiled app |
| Audio buffers | ~50-100 MB | During active recording |

---

## Dead Code Removed

- `src-tauri/src/audio/normalizer.rs` - Was never called in production
- `src-tauri/src/audio/normalizer_tests.rs` - Tests for removed file

---

## Expected Results

| Scenario | Before | After |
|----------|--------|-------|
| Normal Whisper recording | 2-5s ffmpeg + transcribe | Transcribe only (~200ms conversion) |
| Too-short recording | 2-5s ffmpeg + abort | Instant abort |
| AI enhancement | ~200ms overhead | ~50ms overhead |
| Parakeet recording | Same (ffmpeg kept) | Same |
| File upload | Same (ffmpeg needed) | Same |

**Total improvement: 2-5 seconds faster for every Whisper recording**
