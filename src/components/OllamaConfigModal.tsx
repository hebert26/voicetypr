import React, { useEffect, useState } from "react";
import { Button } from "./ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "./ui/dialog";
import { Input } from "./ui/input";
import { Label } from "./ui/label";
import { Loader2, CheckCircle2, XCircle } from "lucide-react";
import { invoke } from "@tauri-apps/api/core";

interface OllamaConfigModalProps {
  isOpen: boolean;
  defaultModel?: string;
  defaultPort?: number;
  onClose: () => void;
  onSubmit: (config: { model: string; port: number }) => void;
}

export function OllamaConfigModal({
  isOpen,
  defaultModel = "",
  defaultPort,
  onClose,
  onSubmit,
}: OllamaConfigModalProps) {
  const [model, setModel] = useState(defaultModel);
  const [port, setPort] = useState(defaultPort?.toString() ?? "");
  const [submitting, setSubmitting] = useState(false);
  const [testing, setTesting] = useState(false);
  const [testResult, setTestResult] = useState<null | {
    ok: boolean;
    message: string;
  }>(null);
  // null = not checked, true = detected, false = not detected, "unknown" = couldn't determine
  const [ollamaDetected, setOllamaDetected] = useState<
    boolean | null | "unknown"
  >(null);

  useEffect(() => {
    if (isOpen) {
      // When opening, load the saved values from props
      setModel(defaultModel);
      setPort(defaultPort?.toString() ?? "");
      setOllamaDetected("unknown");
      setTestResult(null);
    } else {
      // Reset state when modal closes
      setSubmitting(false);
      setTesting(false);
      setTestResult(null);
      setOllamaDetected(null);
    }
  }, [isOpen, defaultModel, defaultPort]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!model.trim() || !port.trim()) return;

    const portNum = parseInt(port);
    try {
      setSubmitting(true);
      onSubmit({ model: model.trim(), port: portNum });
    } finally {
      setSubmitting(false);
    }
  };

  const handleTest = async () => {
    setTestResult(null);
    setTesting(true);
    const portNum = parseInt(port);

    try {
      await invoke("test_openai_endpoint", {
        baseUrl: `http://localhost:${portNum}`,
        model: model.trim(),
        noAuth: true,
      });
      setTestResult({ ok: true, message: "Connected to Ollama successfully!" });
      setOllamaDetected(true);
    } catch (e: unknown) {
      const errorMessage = String(e);
      if (
        errorMessage.includes("Connection refused") ||
        errorMessage.includes("error sending request")
      ) {
        setTestResult({
          ok: false,
          message:
            "Cannot connect to Ollama. Make sure Ollama is running (ollama serve)",
        });
      } else if (
        errorMessage.includes("404") ||
        errorMessage.includes("not found")
      ) {
        setTestResult({
          ok: false,
          message: `Model "${model}" not found. Run: ollama pull ${model}`,
        });
      } else {
        setTestResult({ ok: false, message: errorMessage });
      }
      setOllamaDetected(false);
    } finally {
      setTesting(false);
    }
  };

  return (
    <Dialog open={isOpen} onOpenChange={onClose}>
      <DialogContent className="sm:max-w-[420px]">
        <form onSubmit={handleSubmit}>
          <DialogHeader>
            <DialogTitle className="flex items-center gap-2">
              <span>🦙</span> Configure Ollama
            </DialogTitle>
            <DialogDescription>
              Run AI locally with Ollama - no API key or internet needed.
            </DialogDescription>
          </DialogHeader>

          <div className="grid gap-3 py-3">
            {/* Ollama Status */}
            <div className="flex items-center gap-2 text-sm">
              {ollamaDetected === null ? (
                <span className="text-muted-foreground">
                  Checking Ollama status...
                </span>
              ) : ollamaDetected === true ? (
                <>
                  <CheckCircle2 className="h-4 w-4 text-green-500" />
                  <span className="text-green-600 dark:text-green-400">
                    Ollama connected successfully!
                  </span>
                </>
              ) : ollamaDetected === false ? (
                <>
                  <XCircle className="h-4 w-4 text-red-500" />
                  <span className="text-red-600 dark:text-red-400">
                    Could not connect to Ollama
                  </span>
                </>
              ) : (
                <span className="text-muted-foreground">
                  Click "Test Connection" to verify Ollama is running
                </span>
              )}
            </div>

            {/* Model Input */}
            <div className="grid gap-2">
              <Label htmlFor="ollamaModel">Model Name</Label>
              <Input
                id="ollamaModel"
                placeholder="e.g. qwen2.5:1.5b, llama3.2:1b"
                value={model}
                onChange={(e) => {
                  setModel(e.target.value);
                  setTestResult(null);
                }}
              />
            </div>

            {/* Port Input - compact inline */}
            <div className="flex items-center gap-2">
              <Label htmlFor="ollamaPort" className="text-sm whitespace-nowrap">
                Port:
              </Label>
              <Input
                id="ollamaPort"
                type="number"
                placeholder="Port number"
                value={port}
                onChange={(e) => {
                  setPort(e.target.value);
                  setTestResult(null);
                }}
                className="w-24"
              />
            </div>

            {/* Setup Instructions - only show if connection failed */}
            {ollamaDetected === false && (
              <div className="rounded-md border border-border/50 bg-muted/30 p-3 space-y-2">
                <p className="text-sm font-medium">Quick Setup</p>
                <ol className="text-xs text-muted-foreground space-y-1 list-decimal list-inside">
                  <li>
                    Install Ollama (local AI runtime)
                  </li>
                  <li>
                    Open Terminal and run:{" "}
                    <code className="bg-background px-1 rounded">
                      ollama pull {model || "qwen2.5:1.5b"}
                    </code>
                  </li>
                  <li>
                    Start Ollama (open the app or run{" "}
                    <code className="bg-background px-1 rounded">
                      ollama serve
                    </code>
                    )
                  </li>
                  <li>Click "Test Connection" below</li>
                </ol>
              </div>
            )}
          </div>

          <DialogFooter>
            <Button
              type="button"
              variant="outline"
              onClick={onClose}
              disabled={submitting || testing}
            >
              Cancel
            </Button>
            <Button
              type="button"
              variant="outline"
              onClick={handleTest}
              disabled={!model.trim() || !port.trim() || submitting || testing}
            >
              {testing ? (
                <>
                  <Loader2 className="h-4 w-4 animate-spin mr-1" />
                  Testing...
                </>
              ) : (
                "Test Connection"
              )}
            </Button>
            <Button type="submit" disabled={!model.trim() || !port.trim() || submitting}>
              {submitting ? (
                <>
                  <Loader2 className="h-4 w-4 animate-spin mr-1" />
                  Saving...
                </>
              ) : (
                "Save"
              )}
            </Button>
          </DialogFooter>

          {testResult && (
            <div
              className={`mt-3 text-sm p-2 rounded-md ${
                testResult.ok
                  ? "text-green-600 bg-green-500/10"
                  : "text-red-600 bg-red-500/10"
              }`}
            >
              {testResult.message}
            </div>
          )}
        </form>
      </DialogContent>
    </Dialog>
  );
}
