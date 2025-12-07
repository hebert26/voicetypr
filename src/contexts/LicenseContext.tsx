// License context - simplified for fully offline/private operation
// Always returns "licensed" status without any network calls

import { createContext, useContext, ReactNode } from 'react';
import { LicenseStatus } from '@/types';

interface LicenseContextValue {
  status: LicenseStatus | null;
  isLoading: boolean;
  checkStatus: () => Promise<void>;
  restoreLicense: () => Promise<void>;
  activateLicense: (key: string) => Promise<void>;
  deactivateLicense: () => Promise<void>;
  openPurchasePage: () => Promise<void>;
}

const LicenseContext = createContext<LicenseContextValue | undefined>(undefined);

// Always return licensed status (offline mode)
const licensedStatus: LicenseStatus = {
  status: 'licensed',
  trial_days_left: undefined,
  license_type: 'offline',
  license_key: undefined,
  expires_at: undefined,
};

export function LicenseProvider({ children }: { children: ReactNode }) {
  // All functions are no-ops in offline mode
  const checkStatus = async () => {};
  const restoreLicense = async () => {};
  const activateLicense = async (_key: string) => {};
  const deactivateLicense = async () => {};
  const openPurchasePage = async () => {};

  const value: LicenseContextValue = {
    status: licensedStatus,
    isLoading: false,
    checkStatus,
    restoreLicense,
    activateLicense,
    deactivateLicense,
    openPurchasePage,
  };

  return (
    <LicenseContext.Provider value={value}>
      {children}
    </LicenseContext.Provider>
  );
}

export function useLicense() {
  const context = useContext(LicenseContext);
  if (!context) {
    throw new Error('useLicense must be used within a LicenseProvider');
  }
  return context;
}
