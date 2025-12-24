import { useAccessibilityPermission } from './useAccessibilityPermission';
import { useMicrophonePermission } from './useMicrophonePermission';
import { useModelAvailability } from './useModelAvailability';

/**
 * Computed hook that combines all domain-specific hooks to provide derived readiness values.
 * This hook does NOT manage state - it only computes values based on the domain hooks.
 */
export function useAppReadiness() {
  const accessibility = useAccessibilityPermission();
  const microphone = useMicrophonePermission();
  const models = useModelAvailability();

  // Compute derived values
  const canRecord = Boolean(
    microphone.hasPermission &&
    models.hasModels &&
    models.selectedModelAvailable
  );

  const canAutoInsert = Boolean(accessibility.hasPermission);

  const isFullyReady = Boolean(
    microphone.hasPermission &&
    accessibility.hasPermission &&
    models.hasModels &&
    models.selectedModelAvailable
  );

  // Check if any hook is still loading
  const isLoading = (
    accessibility.isChecking ||
    microphone.isChecking ||
    models.isChecking ||
    accessibility.hasPermission === null ||
    microphone.hasPermission === null ||
    models.hasModels === null
  );

  return {
    // Individual states (for debugging/specific UI needs)
    hasAccessibilityPermission: accessibility.hasPermission,
    hasMicrophonePermission: microphone.hasPermission,
    hasModels: models.hasModels,
    selectedModelAvailable: models.selectedModelAvailable,
    licenseValid: true, // Always valid for offline-only operation

    // Computed values
    canRecord,
    canAutoInsert,
    isFullyReady,
    isLoading,

    // Actions from domain hooks
    requestAccessibilityPermission: accessibility.requestPermission,
    requestMicrophonePermission: microphone.requestPermission,
    checkAccessibilityPermission: accessibility.checkPermission,
    checkMicrophonePermission: microphone.checkPermission,
    checkModels: models.checkModels,
  };
}