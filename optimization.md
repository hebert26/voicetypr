# Memory Analysis for VoiceTypr

## Summary

After thorough code analysis, the app's memory usage (~3 GB) is **expected and unavoidable** given the Whisper model requirements.

## Memory Breakdown

| Component | Memory | Notes |
|-----------|--------|-------|
| Whisper model | ~1-3 GB | Required for transcription accuracy |
| WebView/Chromium | ~200-500 MB | Tauri uses system WebView |
| Rust runtime + deps | ~100-200 MB | Normal for compiled app |
| Audio buffers | ~50-100 MB | During active recording |

## Key Findings

### 1. Dead Code Removed

**File deleted**: `src-tauri/src/audio/normalizer.rs`

The Rust-based normalizer was never used in production. Actual normalization uses ffmpeg:

```
Recording Flow:
recorder.rs (writes WAV) → ffmpeg (normalizes to 16kHz mono) → transcriber.rs
```

FFmpeg runs as an external process and handles streaming internally.

### 2. Transcriber Memory is Reasonable

The transcriber receives already-normalized files (16kHz mono s16) from ffmpeg:

```rust
// For a 10-minute recording at 16kHz mono:
// 10 min × 60 sec × 16,000 samples = 9.6M samples
let samples_i16: Vec<i16> = ...;  // ~18 MB
let audio: Vec<f32> = ...;         // ~38 MB
// Total: ~56 MB (not 1.1 GB as originally estimated)
```

### 3. Recorder Allocations are Efficient

The audio callbacks use `collect()` on `ExactSizeIterator`, which pre-allocates the correct capacity:

```rust
// This already allocates efficiently - no reallocations
let i16_samples: Vec<i16> = data.iter().map(...).collect();
```

Further optimization would require thread-local storage or mutex-protected buffers, adding complexity for minimal gain.

## Conclusion

The ~3 GB memory usage is primarily the Whisper model, which is necessary for accurate offline transcription. No further memory optimizations are feasible without:

1. Using smaller (less accurate) models
2. Streaming transcription (not supported by Whisper)
3. Offloading to cloud services (defeats offline purpose)

The app's memory profile is appropriate for its functionality.
