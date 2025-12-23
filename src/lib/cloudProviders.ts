// Cloud providers have been removed for offline-only operation.
// All transcription uses local Whisper or Parakeet models.

export interface CloudProviderDefinition {
  id: string;
  engine: string;
  modelName: string;
  displayName: string;
  description: string;
  providerName: string;
  addKey: (key: string) => Promise<void>;
  removeKey: () => Promise<void>;
  hasKey: () => Promise<boolean>;
  docsUrl?: string;
  setupCta?: string;
}

// No cloud providers available - app operates fully offline
export const CLOUD_PROVIDERS: Record<string, CloudProviderDefinition> = {};

export const getCloudProviderByModel = (_modelName: string): CloudProviderDefinition | undefined =>
  undefined;
