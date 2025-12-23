use serde::{Deserialize, Serialize};

// Meaning-preserving prompt - strict constraints to prevent changing intent
const BASE_PROMPT: &str = r#"You are a mechanical text cleaner. Your job is to fix typos and grammar in transcribed speech WITHOUT changing what was said.

CRITICAL - DO NOT:
- Change questions into commands (keep "why is X broken?" as a question, don't say "fix X")
- Add assumed solutions (if they ask "why", don't tell them to "do Y")
- Add steps, details, examples, or clarifications they didn't say
- Remove constraints, conditions, or specifics they mentioned
- Rewrite for "better clarity" - keep their exact phrasing
- Infer or assume anything
You are asking a question.
ONLY DO (minimal mechanical cleanup):
1. Fix punctuation, capitalization
2. Remove obvious fillers: "um", "uh", "er", "ah", "like", "yeah", "yep", "so", "well", "right", "okay", "alright", "anyway", "anyways", "basically", "actually", "honestly", "literally", "obviously", "definitely", "probably", "really", "just", "you know", "I mean", "kind of", "sort of", "I guess", "to be honest", "to be fair", "at the end of the day", and trailing hedges like "if that makes sense", "you know what I mean", "or whatever", "does that make sense" (but NOT if they carry actual meaning)
3. Remove stutters and duplicates: "the the" → "the", "I I think" → "I think", "go go ahead" → "go ahead"
4. Fix clear typos and informal speech: "teh" → "the", "gonna" → "going to", "wanna" → "want to", "gotta" → "got to", "kinda" → "kind of", "sorta" → "sort of"
5. Format numbers/dates as spoken

LANGUAGE:
- ALWAYS output in English, 100% of the time
- If the user speaks in Spanish, translate to English while preserving the original meaning
- Do not mix languages in the output

OUTPUT:
- Keep original type: question stays question, command stays command
- Keep original structure and wording
- One paragraph or simple bullets only if clearly listed
- Imperative voice if it was a command
- Question format if they asked something

WHEN UNSURE: Keep the original wording exactly. Do not guess.

Now clean this transcription:"#;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnhancementPreset {
    Default,
    Prompts,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancementOptions {
    pub preset: EnhancementPreset,
    #[serde(default)]
    pub custom_vocabulary: Vec<String>,
    #[serde(default)]
    pub custom_instructions: Option<String>,
}

impl Default for EnhancementOptions {
    fn default() -> Self {
        Self {
            preset: EnhancementPreset::Default,
            custom_vocabulary: Vec::new(),
            custom_instructions: None,
        }
    }
}

pub fn build_enhancement_prompt(
    text: &str,
    context: Option<&str>,
    options: &EnhancementOptions,
) -> String {
    // Base processing applies to ALL presets
    let base_prompt = BASE_PROMPT;

    // Preset is ignored - all modes use the same meaning-preserving prompt
    let mode_transform = "";

    // Build vocabulary section if provided
    let vocabulary_section = if !options.custom_vocabulary.is_empty() {
        let vocab_entries: Vec<&str> = options
            .custom_vocabulary
            .iter()
            .map(|s| s.as_str())
            .filter(|s| !s.trim().is_empty())
            .collect();

        if !vocab_entries.is_empty() {
            log::info!(
                "[AI Prompt] Adding {} vocabulary entries",
                vocab_entries.len()
            );
            format!(
                "\n\nVocabulary corrections (use these spellings):\n{}",
                vocab_entries
                    .iter()
                    .map(|e| format!("- {}", e))
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    // Build the complete prompt with custom instructions BEFORE the text for emphasis
    let custom_instruction_section = if let Some(instructions) = &options.custom_instructions {
        if !instructions.trim().is_empty() {
            log::info!(
                "[AI Prompt] Adding custom instructions: {}",
                instructions.trim()
            );
            format!(
                "\n\nIMPORTANT - You MUST also follow these instructions:\n{}",
                instructions.trim()
            )
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    let mut prompt = if mode_transform.is_empty() {
        // Default preset: just base processing
        format!(
            "{}{}{}\n\nTranscribed text:\n{}",
            base_prompt,
            vocabulary_section,
            custom_instruction_section,
            text.trim()
        )
    } else {
        // Other presets: base + transform
        format!(
            "{}\n\n{}{}{}\n\nTranscribed text:\n{}",
            base_prompt,
            mode_transform,
            vocabulary_section,
            custom_instruction_section,
            text.trim()
        )
    };

    // Add context if provided
    if let Some(ctx) = context {
        prompt.push_str(&format!("\n\nContext: {}", ctx));
    }

    log::debug!("[AI Prompt] Full prompt built:\n{}", prompt);
    prompt
}
