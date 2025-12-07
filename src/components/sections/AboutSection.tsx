// About section - simplified for fully offline/private operation

import { Badge } from "@/components/ui/badge";
import { ScrollArea } from "@/components/ui/scroll-area";
import { getVersion } from '@tauri-apps/api/app';
import {
  Info,
  WifiOff
} from "lucide-react";
import { useEffect, useState } from 'react';

export function AboutSection() {
  const [appVersion, setAppVersion] = useState<string>('');

  useEffect(() => {
    const fetchVersion = async () => {
      try {
        const version = await getVersion();
        setAppVersion(version);
      } catch (error) {
        console.error('Failed to get app version:', error);
        setAppVersion('Unknown');
      }
    };

    fetchVersion();
  }, []);

  return (
    <div className="h-full flex flex-col">
      {/* Header */}
      <div className="px-6 py-4 border-b border-border/40">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-2xl font-semibold">About</h1>
            <p className="text-sm text-muted-foreground mt-1">
              App information
            </p>
          </div>
        </div>
      </div>

      <ScrollArea className="flex-1">
        <div className="p-6 space-y-6">
          {/* App Information Section */}
          <div className="space-y-4">
            <h2 className="text-base font-semibold">App Information</h2>

            <div className="rounded-lg border border-border/50 bg-card p-4 space-y-4">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-2">
                  <Info className="h-4 w-4 text-muted-foreground" />
                  <span className="text-sm text-muted-foreground">Version</span>
                </div>
                <Badge variant="secondary" className="font-mono">
                  v{appVersion || 'Loading...'}
                </Badge>
              </div>

              <div className="flex items-center justify-between">
                <div className="flex items-center gap-2">
                  <WifiOff className="h-4 w-4 text-muted-foreground" />
                  <span className="text-sm text-muted-foreground">Mode</span>
                </div>
                <Badge variant="outline" className="font-medium">
                  Offline / Private
                </Badge>
              </div>
            </div>
          </div>

          {/* Privacy Section */}
          <div className="space-y-4">
            <h2 className="text-base font-semibold">Privacy</h2>

            <div className="rounded-lg border border-border/50 bg-card p-4">
              <div className="space-y-3 text-sm text-muted-foreground">
                <p>
                  VoiceTypr runs entirely on your device. Your voice recordings and
                  transcriptions never leave your computer.
                </p>
                <p>
                  No analytics, no tracking, no external connections required.
                </p>
              </div>
            </div>
          </div>
        </div>
      </ScrollArea>
    </div>
  );
}
