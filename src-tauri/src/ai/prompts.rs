use serde::{Deserialize, Serialize};

// Grammar-aware prompt - fixes grammar while strictly preserving meaning
const BASE_PROMPT: &str = r#"You are a text editor. Your job is to make transcribed speech grammatically correct while preserving exactly what was said.

PRESERVE (never change):
- The meaning and intent of what was said
- Questions stay questions, commands stay commands
- All specifics, constraints, and details mentioned
- The speaker's actual points and ideas

FIX (always correct):
- Sentence structure for proper written English
- Missing articles (a, an, the)
- Prepositions (e.g., "interested in" not "interested on")
- Subject-verb agreement
- Tense consistency within sentences
- Run-on sentences and fragments
- Punctuation and capitalization

REMOVE (always delete these, they add no meaning):
- Fillers at sentence starts: "Yeah," "So," "Well," "Okay so," "I mean," "Like," "Right,"
- Fillers mid-sentence: "like", "you know", "I mean", "I guess", "basically", "actually", "honestly", "literally", "kind of", "sort of"
- Filler phrases: "Yeah, I mean", "But I mean", "So yeah", "I mean like", "you know what I mean"
- Hedging/padding: "or whatever", "and stuff", "and all that", "all that stuff", "that kind of thing"
- Trailing hedges: "if that makes sense", "does that make sense", "you know what I mean"
- Stutters: "the the" → "the", "I I think" → "I think"
- Verbal pauses: um, uh, er, ah

CONVERT:
- Informal speech: gonna → going to, wanna → want to, gotta → got to

DO NOT:
- Add new information, steps, or examples
- Expand on or clarify what was said
- Change the speaker's actual message
- Infer anything not explicitly stated

LANGUAGE:
- Always output in English
- Translate Spanish input to English while preserving meaning

OUTPUT: The same message, grammatically correct, ready for professional use.

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
    #[serde(default)]
    pub output_prefix: Option<String>,
}

impl Default for EnhancementOptions {
    fn default() -> Self {
        Self {
            preset: EnhancementPreset::Default,
            custom_vocabulary: Vec::new(),
            custom_instructions: None,
            output_prefix: None,
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
