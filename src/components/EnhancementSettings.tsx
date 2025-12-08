import { Button } from "@/components/ui/button";
import { Textarea } from "@/components/ui/textarea";
import { FileText, Sparkles } from "lucide-react";

interface EnhancementSettingsProps {
  settings: {
    preset: "Default" | "Prompts";
    customVocabulary: string[];
    customInstructions?: string;
  };
  onSettingsChange: (settings: any) => void;
  disabled?: boolean;
}

export function EnhancementSettings({ settings, onSettingsChange, disabled = false }: EnhancementSettingsProps) {
  const presets = [
    {
      id: "Default",
      label: "Default",
      icon: FileText,
      description: "Clean text"
    },
    {
      id: "Prompts",
      label: "Prompts",
      icon: Sparkles,
      description: "AI prompts"
    }
  ];

  const handlePresetChange = (preset: string) => {
    if (["Default", "Prompts"].includes(preset)) {
      onSettingsChange({
        ...settings,
        preset: preset as "Default" | "Prompts"
      });
    }
  };

  const handleCustomInstructionsChange = (value: string) => {
    onSettingsChange({
      ...settings,
      customInstructions: value
    });
  };


  return (
    <div className={`space-y-6 ${disabled ? 'opacity-50' : ''}`}>
      {/* Enhancement Mode */}
      <div className="space-y-3">
        <div className="flex flex-wrap gap-2">
          {presets.map((preset) => {
            const Icon = preset.icon;
            const isSelected = settings.preset === preset.id;
            
            return (
              <Button
                key={preset.id}
                variant={isSelected ? "default" : "outline"}
                size="sm"
                className={`gap-2 ${disabled ? 'cursor-not-allowed' : ''}`}
                onClick={() => !disabled && handlePresetChange(preset.id)}
                disabled={disabled}
              >
                <Icon className="h-4 w-4" />
                {preset.label}
              </Button>
            );
          })}
        </div>
        
        {/* Mode description */}
        <p className="text-sm text-muted-foreground">
          {settings.preset === "Default" && "Format transcription with grammar, spelling, punctuation, and semantic corrections"}
          {settings.preset === "Prompts" && "Transform speech into clear, actionable AI prompts"}
        </p>
      </div>

      {/* Custom Instructions */}
      <div className="space-y-2">
        <label className="text-sm font-medium">Custom Instructions (optional)</label>
        <Textarea
          placeholder="Add custom instructions, e.g., 'Make sure the text makes sense, fix grammar, use formal tone'"
          value={settings.customInstructions || ""}
          onChange={(e) => !disabled && handleCustomInstructionsChange(e.target.value)}
          disabled={disabled}
          className="min-h-[80px] resize-none"
        />
        <p className="text-xs text-muted-foreground">
          These instructions will be added to the AI prompt along with your selected preset.
        </p>
      </div>
    </div>
  );
}