import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/tauri';

export interface Provider {
  id: string;
  name: string;
  provider_type: string;
  base_url?: string;
  model?: string;
  api_key?: string;
  default: boolean;
  enabled: boolean;
  config: Record<string, any>;
}

export interface SystemInfo {
  os: string;
  arch: string;
  version: string;
  memory: number;
  cpu_count: number;
}

export interface WorkspaceInfo {
  path: string;
  name: string;
  git_branch?: string;
  git_status?: string;
  file_count: number;
  language?: string;
}

interface AppState {
  providers: Provider[];
  defaultProvider: string | null;
  systemInfo: SystemInfo | null;
  workspaceInfo: WorkspaceInfo | null;
  isLoading: boolean;
  error: string | null;
  
  // Actions
  loadProviders: () => Promise<void>;
  addProvider: (provider: Omit<Provider, 'id'>) => Promise<void>;
  removeProvider: (id: string) => Promise<void>;
  setDefaultProvider: (id: string) => Promise<void>;
  getSystemInfo: () => Promise<void>;
  getWorkspaceInfo: (path: string) => Promise<void>;
  executeCommand: (command: string, providerId?: string) => Promise<string>;
  clearError: () => void;
}

export const useAppStore = create<AppState>((set, get) => ({
  providers: [],
  defaultProvider: null,
  systemInfo: null,
  workspaceInfo: null,
  isLoading: false,
  error: null,

  loadProviders: async () => {
    set({ isLoading: true, error: null });
    try {
      const providers = await invoke<Provider[]>('get_providers');
      const defaultProvider = providers.find(p => p.default)?.id || null;
      set({ providers, defaultProvider, isLoading: false });
    } catch (error) {
      set({ error: error as string, isLoading: false });
    }
  },

  addProvider: async (providerData) => {
    set({ isLoading: true, error: null });
    try {
      const provider: Provider = {
        ...providerData,
        id: `provider_${Date.now()}`,
      };
      await invoke('add_provider', { provider });
      await get().loadProviders();
    } catch (error) {
      set({ error: error as string, isLoading: false });
    }
  },

  removeProvider: async (id) => {
    set({ isLoading: true, error: null });
    try {
      await invoke('remove_provider', { id });
      await get().loadProviders();
    } catch (error) {
      set({ error: error as string, isLoading: false });
    }
  },

  setDefaultProvider: async (id) => {
    set({ isLoading: true, error: null });
    try {
      await invoke('set_default_provider', { id });
      set({ defaultProvider: id, isLoading: false });
    } catch (error) {
      set({ error: error as string, isLoading: false });
    }
  },

  getSystemInfo: async () => {
    set({ isLoading: true, error: null });
    try {
      const systemInfo = await invoke<SystemInfo>('get_system_info');
      set({ systemInfo, isLoading: false });
    } catch (error) {
      set({ error: error as string, isLoading: false });
    }
  },

  getWorkspaceInfo: async (path) => {
    set({ isLoading: true, error: null });
    try {
      const workspaceInfo = await invoke<WorkspaceInfo>('get_workspace_info', { workspacePath: path });
      set({ workspaceInfo, isLoading: false });
    } catch (error) {
      set({ error: error as string, isLoading: false });
    }
  },

  executeCommand: async (command, providerId) => {
    set({ isLoading: true, error: null });
    try {
      const result = await invoke<string>('execute_command', { command, providerId });
      set({ isLoading: false });
      return result;
    } catch (error) {
      set({ error: error as string, isLoading: false });
      throw error;
    }
  },

  clearError: () => set({ error: null }),
}));