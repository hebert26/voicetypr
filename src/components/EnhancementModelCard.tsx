import { ask } from "@tauri-apps/plugin-dialog";
import { Key, Trash2 } from "lucide-react";
import { Button } from "./ui/button";
import { Card } from "./ui/card";

interface AIModel {
  id: string;
  name: string;
  provider: string;
  description?: string;
}

interface OllamaConfig {
  model: string;
  port?: number;
}

interface EnhancementModelCardProps {
  model: AIModel;
  hasApiKey: boolean;
  isSelected: boolean;
  ollamaConfig?: OllamaConfig;
  onSetupApiKey: () => void;
  onSelect: () => void;
  onRemoveApiKey: () => void;
}

export function EnhancementModelCard({
  model,
  hasApiKey,
  isSelected,
  ollamaConfig,
  onSetupApiKey,
  onSelect,
  onRemoveApiKey,
}: EnhancementModelCardProps) {
  const providers: Record<string, { name: string; color: string }> = {
    groq: { name: "Groq", color: "text-orange-600" },
    gemini: { name: "Gemini", color: "text-blue-600" },
    openai: { name: "OpenAI", color: "text-green-600" },
  };

  const provider = providers[model.provider] || {
    name: model.provider,
    color: "text-gray-600",
  };

  // For Ollama, show actual configured model and port if available
  const isOllama = model.provider === "ollama";
  const ollamaConfigured = isOllama && hasApiKey && ollamaConfig?.model && ollamaConfig?.port;
  const displayName = ollamaConfigured ? `🦙 ${ollamaConfig.model}` : model.name;
  const apiUrl = ollamaConfigured ? `http://localhost:${ollamaConfig.port}` : null;

  return (
    <Card
      className={`py-2 px-4 transition-all ${
        hasApiKey ? "cursor-pointer hover:border-border" : "opacity-60"
      } ${isSelected ? "bg-primary/5" : ""}`}
      onClick={() => hasApiKey && onSelect()}
    >
      <div className="flex items-center justify-between gap-3">
        <div className="flex-1 space-y-1">
          <div className="flex items-center gap-2">
            <h3 className="font-medium">{displayName}</h3>
            {!ollamaConfigured && (
              <span
                className={`text-xs font-medium ${isOllama ? "text-amber-600" : provider.color}`}
              >
                {isOllama ? "ollama" : provider.name}
              </span>
            )}
          </div>
          {ollamaConfigured && (
            <div className="flex flex-wrap gap-x-4 gap-y-1 text-xs text-muted-foreground">
              <span>
                <span className="text-foreground/60">Model:</span>{" "}
                <span className="font-medium text-foreground">{ollamaConfig.model}</span>
              </span>
              <span>
                <span className="text-foreground/60">URL:</span>{" "}
                <span className="font-mono text-foreground">{apiUrl}</span>
              </span>
              <span>
                <span className="text-foreground/60">Port:</span>{" "}
                <span className="font-mono text-foreground">{ollamaConfig.port}</span>
              </span>
            </div>
          )}
        </div>

        {hasApiKey ? (
          <Button
            onClick={async (e) => {
              e.stopPropagation();
              const confirmed = await ask(
                `Remove API key for ${provider.name}?`,
                { title: "Remove API Key", kind: "warning" },
              );
              if (confirmed) {
                onRemoveApiKey();
              }
            }}
            variant="ghost"
            size="sm"
            className="text-muted-foreground hover:text-destructive"
          >
            <Trash2 className="w-3.5 h-3.5" />
            Remove Key
          </Button>
        ) : (
          <Button
            onClick={(e) => {
              e.stopPropagation();
              onSetupApiKey();
            }}
            variant="outline"
            size="sm"
          >
            <Key className="w-4 h-4 mr-1" />
            {model.provider === "openai" ? "Configure" : "Add Key"}
          </Button>
        )}
      </div>
    </Card>
  );
}
