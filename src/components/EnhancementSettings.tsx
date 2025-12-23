import { Textarea } from "@/components/ui/textarea";
import { FileText } from "lucide-react";

interface EnhancementSettingsProps {
  settings: {
    preset: "Default" | "Prompts";
    customVocabulary: string[];
    customInstructions?: string;
  };
  onSettingsChange: (settings: any) => void;
  disabled?: boolean;
}

export function EnhancementSettings({
  settings,
  onSettingsChange,
  disabled = false,
}: EnhancementSettingsProps) {
  const handleCustomInstructionsChange = (value: string) => {
    onSettingsChange({
      ...settings,
      customInstructions: value,
    });
  };

  const handleVocabularyChange = (value: string) => {
    // Split by newlines and filter empty lines
    const entries = value.split("\n").filter((line) => line.trim() !== "");
    onSettingsChange({
      ...settings,
      customVocabulary: entries,
    });
  };

  // Convert array back to newline-separated string for display
  const vocabularyText = settings.customVocabulary?.join("\n") || "";

  return (
    <div className={`space-y-6 ${disabled ? "opacity-50" : ""}`}>
      <div className="flex items-center gap-2 text-sm text-muted-foreground">
        <FileText className="h-4 w-4" />
        <span>Formatting is always meaning-preserving cleanup.</span>
      </div>

      {/* Custom Instructions */}
      <div className="space-y-2">
        <label className="text-sm font-medium">
          Custom Instructions (optional)
        </label>
        <Textarea
          placeholder="Add custom instructions, e.g., 'Make sure the text makes sense, fix grammar, use formal tone'"
          value={settings.customInstructions || ""}
          onChange={(e) =>
            !disabled && handleCustomInstructionsChange(e.target.value)
          }
          disabled={disabled}
          className="min-h-[80px] resize-none"
        />
        <p className="text-xs text-muted-foreground">
          These instructions will be added to the AI prompt along with your
          selected preset.
        </p>
      </div>

      {/* Vocabulary Corrections */}
      <div className="space-y-2">
        <label className="text-sm font-medium">
          Vocabulary Corrections (optional)
        </label>
        <Textarea
          placeholder={`Add pronunciation corrections, one per line:\nko → code\nlama → llama\njavasript → JavaScript`}
          value={vocabularyText}
          onChange={(e) => !disabled && handleVocabularyChange(e.target.value)}
          disabled={disabled}
          className="min-h-[80px] resize-none font-mono text-sm"
        />
        <p className="text-xs text-muted-foreground">
          Helps correct common mispronunciations. Use format: wrong → right
        </p>
      </div>
    </div>
  );
}
