// Cloud providers have been removed for offline-only operation.
// This stub is kept for backward compatibility with code that references it.

export interface CloudProviderDefinition {
  id: string;
  name: string;
  description: string;
  providerName: string;
  modelName: string;
  displayName: string;
  setupCta: string;
  docsUrl: string;
  addKey: (apiKey: string) => Promise<void>;
  removeKey: () => Promise<void>;
}

export const getCloudProviderByModel = (
  _modelName: string
): CloudProviderDefinition | undefined => undefined;
