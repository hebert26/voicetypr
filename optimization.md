# Memory Optimization Plan for VoiceTypr

## Problem Summary
- **Verity (VoiceTypr)**: 2.98 GB runtime memory
- **rust-analyzer**: 4.29 GB IDE memory

## Root Causes

### App Memory (2.98 GB)
1. **Whisper model in memory**: ~1-3 GB (by design, limited to 1 model)
2. **Audio processing allocations**: Per-frame heap allocations in hot loop
3. **Normalizer**: Loads entire audio file into memory multiple times

### rust-analyzer (4.29 GB)
- IDE language server indexing all dependencies
- No VS Code settings configured for optimization

---

## Implementation Plan

### Phase 1: Quick Wins

#### 1.1 Create rust-analyzer config (NEW FILE)
**File**: `.vscode/settings.json`

```json
{
  "rust-analyzer.cargo.buildScripts.enable": true,
  "rust-analyzer.procMacro.enable": true,
  "rust-analyzer.procMacro.attributes.enable": false,
  "rust-analyzer.checkOnSave.extraArgs": ["--target-dir", "target/check"],
  "rust-analyzer.lens.enable": false,
  "rust-analyzer.inlayHints.enable": false,
  "rust-analyzer.hover.actions.enable": false
}
```

**Impact**: Reduce rust-analyzer from 4.29 GB to ~1-2 GB

#### 1.2 Pre-allocate recorder buffers
**File**: `src-tauri/src/audio/recorder.rs` (lines 268-321)

**Problem**: Per-frame `Vec::collect()` allocations in audio callbacks cause ~400MB/sec allocation churn.

**Solution**: Pre-allocate reusable buffers outside the hot loop:
- Create shared buffer structs before `build_input_stream`
- Use `Arc<Mutex<Vec<_>>>` for conversion buffers
- Resize and reuse instead of allocating new vectors

---

### Phase 2: Streaming Audio Pipeline (Major Refactor)

#### 2.1 Streaming normalizer
**File**: `src-tauri/src/audio/normalizer.rs`

**Problem** (lines 40-82): Loads entire audio into memory 5+ times:
- `samples_i16: Vec<i16>` - full file
- `samples_f32: Vec<f32>` - full file converted
- `mono: Vec<f32>` - downmixed
- `resampled: Vec<f32>` - resampled output
- `normalized: Vec<f32>` - final output

For 10-min recording: **1.1+ GB peak memory**

**Solution**: Two-pass streaming architecture:
1. **Pass 1**: Stream through file to find peak value (no storage)
2. **Pass 2**: Stream chunks through pipeline, writing directly to output

```
Input WAV → [Read Chunk] → [Convert i16→f32] → [Downmix] → [Resample] → [Normalize] → Output WAV
                ↑                                                              ↓
              Chunk 1 ─────────────────────────────────────────────────────> Write
              Chunk 2 ─────────────────────────────────────────────────────> Write
              ...
```

**Chunk size**: ~192KB (1 second of 48kHz stereo)
**Peak memory**: <1 MB instead of 1.1+ GB

#### 2.2 Streaming resampler wrapper
**File**: `src-tauri/src/audio/resampler.rs`

Create `StreamingResampler` struct that:
- Pre-allocates fixed input/output buffers
- Processes chunks through rubato resampler
- Handles flush for final samples

---

## Files to Modify

| File | Change | Priority |
|------|--------|----------|
| `.vscode/settings.json` | Create with rust-analyzer config | P0 |
| `src-tauri/src/audio/recorder.rs` | Pre-allocate conversion buffers (lines 268-321) | P0 |
| `src-tauri/src/audio/normalizer.rs` | Rewrite to streaming architecture | P1 |
| `src-tauri/src/audio/resampler.rs` | Add StreamingResampler wrapper | P1 |

---

## Expected Results

| Component | Before | After |
|-----------|--------|-------|
| rust-analyzer | 4.29 GB | ~1-2 GB |
| App (idle with model) | ~2.5 GB | ~2 GB |
| App (during long recording) | 3+ GB peak | ~2.1 GB flat |
| Normalizing 10-min audio | 1.1+ GB peak | <50 MB peak |

---

## Testing Strategy

1. **Baseline**: Record current memory during 5-min recording
2. **After Phase 1**: Verify allocation churn eliminated (flat memory line)
3. **After Phase 2**: Process 10-min recording, verify peak <50 MB
4. **Regression**: Ensure transcription quality unchanged
