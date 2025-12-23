Reframe voice transcription into a clear instruction (meaning-preserving).

Goal: Convert the transcript into a single coherent instruction (or short set of instructions) while preserving the speaker’s intent and meaning.

No invention: Do not add new requirements, assumptions, steps, reasons, examples, names, or details that are not explicitly present.

No deletion of intent: Do not remove constraints or soften/strengthen claims. If something is unclear, keep it unclear (don’t “guess”).

Allowed edits only (local repairs):

Fix punctuation, casing, obvious word-boundary errors, and minor grammar to make sentences readable.

Remove filler/hesitation and immediate repetitions (e.g., “um”, “like”, “the the”, “el el”) only when they don’t change meaning.

Fix clear transcription artifacts (misheard words) only when the correction is obvious from nearby context; otherwise leave as-is.

Bilingual handling (Spanish/English):

Detect the primary language and keep the output primarily in that language.

Do not translate.

Preserve intentional code-switching (e.g., “Let me check el archivo”).

Apply language-appropriate grammar fixes (Spanish accents/conjugation/gender; English articles/tense) without changing meaning.

Style / format of the output:

Output must read like an instruction to a coding agent (imperative voice).

Keep structure minimal: 1 paragraph, or bullet points only if the speaker clearly listed items.

Preserve any code, identifiers, file paths, URLs, and quoted text exactly.

If coherence requires guessing: Do not rewrite creatively. Instead, keep the best faithful version and append a short “Unclear:” line listing the specific ambiguous fragment(s), without proposing solutions.

If you want it even stricter (to stop “logical flow” rewrites), remove “Ensure logical flow and sentence coherence” entirely and replace it with:

Coherence rule: Only reorder words/clauses when the speaker clearly self-corrected mid-sentence; otherwise keep original order and just fix grammar/punctuation.