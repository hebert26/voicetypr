use serde::{Deserialize, Serialize};

// 100/100 Base Prompt - deterministic last-intent processing
const BASE_PROMPT: &str = r#"You are a text post-processor. Your ONLY job is to clean up transcribed speech and output the result.

Rules:
- Resolve self-corrections (keep final intent only)
- Fix grammar, punctuation, capitalization
- Remove fillers and false starts
- Format numbers/dates as spoken

CRITICAL: Output ONLY the cleaned text. No explanations. No preamble. No "here is..." or "I have...". Just the text itself."#;

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

    // Add mode-specific transformation if not Default
    let mode_transform = match options.preset {
        EnhancementPreset::Default => "",
        EnhancementPreset::Prompts => PROMPTS_TRANSFORM,
    };

    // Build the complete prompt with custom instructions BEFORE the text for emphasis
    let custom_instruction_section = if let Some(instructions) = &options.custom_instructions {
        if !instructions.trim().is_empty() {
            log::info!("[AI Prompt] Adding custom instructions: {}", instructions.trim());
            format!("\n\nIMPORTANT - You MUST also follow these instructions:\n{}", instructions.trim())
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    let mut prompt = if mode_transform.is_empty() {
        // Default preset: just base processing
        format!("{}{}\n\nTranscribed text:\n{}", base_prompt, custom_instruction_section, text.trim())
    } else {
        // Other presets: base + transform
        format!(
            "{}\n\n{}{}\n\nTranscribed text:\n{}",
            base_prompt,
            mode_transform,
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

// Minimal transformation layer for Prompts preset
const PROMPTS_TRANSFORM: &str = r#"Now transform the cleaned text into a concise AI prompt:
- Classify as Request, Question, or Task.
- Add only essential missing what/how/why.
- Include constraints and success criteria if relevant.
- Specify output format when helpful.
- Preserve all technical details; do not invent any.
Return only the enhanced prompt."#;
