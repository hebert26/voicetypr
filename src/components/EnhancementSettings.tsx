import { useState } from "react";
import { Textarea } from "@/components/ui/textarea";
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from "@/components/ui/collapsible";
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group";
import { Label } from "@/components/ui/label";
import { MessageSquareText, Sparkles, BookOpen, ChevronDown } from "lucide-react";

// Thinking mode presets
const THINKING_PRESETS = {
  think: "",
  hard: "Think hard about this problem. Consider multiple approaches, evaluate tradeoffs, and explain your reasoning.",
  harder: "Think harder about this problem. Analyze it from multiple angles, consider edge cases and potential issues, weigh all tradeoffs, and provide comprehensive reasoning before proposing a solution.",
  ultra: "Ultrathink about this problem. Conduct an exhaustive analysis considering all dimensions, dependencies, risks, and edge cases. Explore multiple solution paths, evaluate each thoroughly, and synthesize your complete reasoning into a comprehensive response.",
} as const;

type ThinkingMode = "think" | "hard" | "harder" | "ultra" | "custom";

// Detect mode from current prefix
const detectMode = (prefix: string | undefined): ThinkingMode => {
  if (!prefix || prefix === "") return "think";
  if (prefix === THINKING_PRESETS.hard) return "hard";
  if (prefix === THINKING_PRESETS.harder) return "harder";
  if (prefix === THINKING_PRESETS.ultra) return "ultra";
  return "custom";
};

interface EnhancementSettingsProps {
  settings: {
    preset: "Default" | "Prompts";
    customVocabulary: string[];
    customInstructions?: string;
    outputPrefix?: string;
  };
  onSettingsChange: (settings: any) => void;
  disabled?: boolean;
}

export function EnhancementSettings({
  settings,
  onSettingsChange,
  disabled = false,
}: EnhancementSettingsProps) {
  // All sections collapsed by default
  const [prefixOpen, setPrefixOpen] = useState(false);
  const [instructionsOpen, setInstructionsOpen] = useState(false);
  const [vocabularyOpen, setVocabularyOpen] = useState(false);

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

  const handleOutputPrefixChange = (value: string) => {
    onSettingsChange({
      ...settings,
      outputPrefix: value,
    });
  };

  const handleThinkingModeChange = (mode: ThinkingMode) => {
    if (mode === "custom") {
      // Keep current prefix when switching to custom
      return;
    }
    const prefix = mode === "think" ? "" : THINKING_PRESETS[mode];
    handleOutputPrefixChange(prefix);
  };

  // Detect current thinking mode from prefix
  const currentMode = detectMode(settings.outputPrefix);

  // Convert array back to newline-separated string for display
  const vocabularyText = settings.customVocabulary?.join("\n") || "";

  return (
    <div className={`space-y-3 ${disabled ? "opacity-50" : ""}`}>
      {/* Output Prefix */}
      <Collapsible open={prefixOpen} onOpenChange={setPrefixOpen}>
        <CollapsibleTrigger className="flex items-center gap-2 w-full py-2 hover:bg-muted/50 rounded-md px-2 -mx-2 transition-colors">
          <MessageSquareText className="h-4 w-4 text-muted-foreground" />
          <span className="text-sm font-medium">Output Prefix</span>
          <span className="text-xs text-muted-foreground flex-1 text-left">— added before output</span>
          <ChevronDown className={`h-4 w-4 text-muted-foreground transition-transform ${prefixOpen ? "rotate-180" : ""}`} />
        </CollapsibleTrigger>
        <CollapsibleContent className="pt-2 space-y-3">
          <RadioGroup
            value={currentMode}
            onValueChange={(value) => !disabled && handleThinkingModeChange(value as ThinkingMode)}
            disabled={disabled}
            className="space-y-1"
          >
            <div className="flex items-center space-x-2">
              <RadioGroupItem value="think" id="think" />
              <Label htmlFor="think" className="text-sm font-normal cursor-pointer">
                Think <span className="text-muted-foreground">— Standard thinking</span>
              </Label>
            </div>
            <div className="flex items-center space-x-2">
              <RadioGroupItem value="hard" id="hard" />
              <Label htmlFor="hard" className="text-sm font-normal cursor-pointer">
                Think Hard <span className="text-muted-foreground">— Extended thinking</span>
              </Label>
            </div>
            <div className="flex items-center space-x-2">
              <RadioGroupItem value="harder" id="harder" />
              <Label htmlFor="harder" className="text-sm font-normal cursor-pointer">
                Think Harder <span className="text-muted-foreground">— Deeper analysis</span>
              </Label>
            </div>
            <div className="flex items-center space-x-2">
              <RadioGroupItem value="ultra" id="ultra" />
              <Label htmlFor="ultra" className="text-sm font-normal cursor-pointer">
                Ultrathink <span className="text-muted-foreground">— Maximum reasoning</span>
              </Label>
            </div>
            <div className="flex items-center space-x-2">
              <RadioGroupItem value="custom" id="custom" />
              <Label htmlFor="custom" className="text-sm font-normal cursor-pointer">
                Custom <span className="text-muted-foreground">— Your own prefix</span>
              </Label>
            </div>
          </RadioGroup>

          {currentMode === "custom" && (
            <Textarea
              placeholder="Enter your custom prefix..."
              value={settings.outputPrefix || ""}
              onChange={(e) =>
                !disabled && handleOutputPrefixChange(e.target.value)
              }
              disabled={disabled}
              className="min-h-[60px] resize-none border-border bg-background"
            />
          )}
        </CollapsibleContent>
      </Collapsible>

      <div className="h-px bg-border/50" />

      {/* AI Instructions */}
      <Collapsible open={instructionsOpen} onOpenChange={setInstructionsOpen}>
        <CollapsibleTrigger className="flex items-center gap-2 w-full py-2 hover:bg-muted/50 rounded-md px-2 -mx-2 transition-colors">
          <Sparkles className="h-4 w-4 text-muted-foreground" />
          <span className="text-sm font-medium">AI Instructions</span>
          <span className="text-xs text-muted-foreground flex-1 text-left">— guides the cleanup</span>
          <ChevronDown className={`h-4 w-4 text-muted-foreground transition-transform ${instructionsOpen ? "rotate-180" : ""}`} />
        </CollapsibleTrigger>
        <CollapsibleContent className="pt-2">
          <Textarea
            placeholder="e.g., 'Use formal tone, fix grammar'"
            value={settings.customInstructions || ""}
            onChange={(e) =>
              !disabled && handleCustomInstructionsChange(e.target.value)
            }
            disabled={disabled}
            className="min-h-[70px] resize-none border-border bg-background"
          />
        </CollapsibleContent>
      </Collapsible>

      <div className="h-px bg-border/50" />

      {/* Vocabulary */}
      <Collapsible open={vocabularyOpen} onOpenChange={setVocabularyOpen}>
        <CollapsibleTrigger className="flex items-center gap-2 w-full py-2 hover:bg-muted/50 rounded-md px-2 -mx-2 transition-colors">
          <BookOpen className="h-4 w-4 text-muted-foreground" />
          <span className="text-sm font-medium">Vocabulary</span>
          <span className="text-xs text-muted-foreground flex-1 text-left">— fix mispronunciations</span>
          <ChevronDown className={`h-4 w-4 text-muted-foreground transition-transform ${vocabularyOpen ? "rotate-180" : ""}`} />
        </CollapsibleTrigger>
        <CollapsibleContent className="pt-2">
          <Textarea
            placeholder={`ko → code\nlama → llama\njavasript → JavaScript`}
            value={vocabularyText}
            onChange={(e) => !disabled && handleVocabularyChange(e.target.value)}
            disabled={disabled}
            className="min-h-[70px] resize-none border-border bg-background font-mono text-sm"
          />
        </CollapsibleContent>
      </Collapsible>
    </div>
  );
}
