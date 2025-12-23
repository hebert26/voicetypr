// Account section - simplified for fully offline/private operation

import { Badge } from "@/components/ui/badge";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Check, Shield, WifiOff } from "lucide-react";

export function AccountSection() {
  return (
    <div className="h-full flex flex-col">
      {/* Header */}
      <div className="px-6 py-4 border-b border-border/40">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-2xl font-semibold">License</h1>
            <p className="text-sm text-muted-foreground mt-1">
              Verity License Status
            </p>
          </div>
          <div className="flex items-center gap-2 px-3 py-1.5 rounded-lg bg-green-500/10">
            <WifiOff className="h-4 w-4 text-green-600 dark:text-green-400" />
            <span className="text-sm font-medium text-green-600 dark:text-green-400">
              Offline Mode
            </span>
          </div>
        </div>
      </div>

      <ScrollArea className="flex-1">
        <div className="p-6 space-y-6">
          {/* License Status Section */}
          <div className="space-y-4">
            <div className="rounded-lg border border-border/50 bg-card p-4 space-y-4">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-2">
                  <Shield className="h-4 w-4 text-muted-foreground" />
                  <span className="text-sm text-muted-foreground">Status</span>
                </div>
                <Badge variant="default" className="font-medium">
                  Fully Licensed
                </Badge>
              </div>

              <div className="bg-green-500/10 border border-green-500/20 rounded-lg p-4 space-y-3">
                <div className="flex items-start gap-3">
                  <div className="p-1.5 rounded-md bg-green-500/10">
                    <Check className="h-4 w-4 text-green-600 dark:text-green-400" />
                  </div>
                  <div className="flex-1 space-y-1">
                    <p className="text-sm font-medium text-green-900 dark:text-green-100">
                      Verity Pro Active
                    </p>
                    <p className="text-xs text-muted-foreground mt-2">
                      All features unlocked - fully offline and private
                    </p>
                  </div>
                </div>
              </div>

              {/* Privacy Notice */}
              <div className="bg-blue-500/10 border border-blue-500/20 rounded-lg p-4">
                <div className="flex items-start gap-3">
                  <div className="p-1.5 rounded-md bg-blue-500/10">
                    <WifiOff className="h-4 w-4 text-blue-500" />
                  </div>
                  <div className="flex-1 space-y-1">
                    <p className="text-sm font-medium text-blue-900 dark:text-blue-100">
                      Privacy Mode
                    </p>
                    <p className="text-xs text-blue-800 dark:text-blue-300">
                      This app runs entirely offline. No data is sent to
                      external servers. All transcription happens locally on
                      your device.
                    </p>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </ScrollArea>
    </div>
  );
}
