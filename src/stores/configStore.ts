import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/tauri';

export interface UIConfig {
  position: string;
  height: string;
  blur: number;
  opacity: number;
  font: string;
  theme: string;
  font_size: number;
  line_height: number;
  padding: number;
  border_radius: number;
  shadow: boolean;
  animations: boolean;
}

export interface HotkeyConfig {
  toggle: string;
  paste_run: string;
  quick_hide: string;
  new_tab: string;
  close_tab: string;
  next_tab: string;
  prev_tab: string;
}

export interface EdgeConfig {
  reveal: boolean;
  dwell_ms: number;
  scroll_resize: boolean;
  sensitivity: number;
}

export interface TriggersConfig {
  hotkeys: HotkeyConfig;
  edges: EdgeConfig;
  wheel_reveal: boolean;
  menu_bar_click: boolean;
}

export interface TerminalConfig {
  shell: string;
  working_directory?: string;
  environment: Record<string, string>;
  scrollback_lines: number;
  cursor_blink: boolean;
  cursor_style: string;
  bell_sound: boolean;
  auto_close: boolean;
}

export interface SecurityConfig {
  keychain_storage: boolean;
  sandbox_providers: boolean;
  minimal_scopes: boolean;
  auto_lock: boolean;
  lock_timeout: number;
}

export interface Config {
  ui: UIConfig;
  triggers: TriggersConfig;
  providers: any[];
  terminal: TerminalConfig;
  plugins: Record<string, any>;
  workspace_rules: any[];
  security: SecurityConfig;
}

export interface ThemeInfo {
  name: string;
  display_name: string;
  description: string;
  colors: Record<string, any>;
}

interface ConfigState {
  config: Config | null;
  themes: ThemeInfo[];
  currentTheme: string;
  isLoading: boolean;
  error: string | null;
  
  // Actions
  loadConfig: () => Promise<void>;
  saveConfig: (config: Config) => Promise<void>;
  loadThemes: () => Promise<void>;
  applyTheme: (themeName: string) => Promise<void>;
  exportConfig: (filePath: string) => Promise<void>;
  importConfig: (filePath: string) => Promise<void>;
  clearError: () => void;
}

const defaultConfig: Config = {
  ui: {
    position: 'top',
    height: '45vh',
    blur: 18,
    opacity: 0.92,
    font: 'JetBrainsMono Nerd Font',
    theme: 'tokyo-night',
    font_size: 14,
    line_height: 1.4,
    padding: 16,
    border_radius: 8,
    shadow: true,
    animations: true,
  },
  triggers: {
    hotkeys: {
      toggle: 'alt+`',
      paste_run: 'cmd+enter',
      quick_hide: 'esc',
      new_tab: 'cmd+t',
      close_tab: 'cmd+w',
      next_tab: 'cmd+shift+]',
      prev_tab: 'cmd+shift+[',
    },
    edges: {
      reveal: true,
      dwell_ms: 450,
      scroll_resize: true,
      sensitivity: 1.0,
    },
    wheel_reveal: true,
    menu_bar_click: true,
  },
  providers: [],
  terminal: {
    shell: '/bin/zsh',
    working_directory: undefined,
    environment: {},
    scrollback_lines: 10000,
    cursor_blink: true,
    cursor_style: 'block',
    bell_sound: true,
    auto_close: false,
  },
  plugins: {},
  workspace_rules: [],
  security: {
    keychain_storage: true,
    sandbox_providers: true,
    minimal_scopes: true,
    auto_lock: false,
    lock_timeout: 300,
  },
};

export const useConfigStore = create<ConfigState>((set, get) => ({
  config: defaultConfig,
  themes: [],
  currentTheme: 'tokyo-night',
  isLoading: false,
  error: null,

  loadConfig: async () => {
    set({ isLoading: true, error: null });
    try {
      const config = await invoke<Config>('get_config');
      set({ 
        config, 
        currentTheme: config.ui.theme,
        isLoading: false 
      });
    } catch (error) {
      set({ 
        config: defaultConfig,
        currentTheme: 'tokyo-night',
        error: error as string, 
        isLoading: false 
      });
    }
  },

  saveConfig: async (config) => {
    set({ isLoading: true, error: null });
    try {
      await invoke('set_config', { config });
      set({ config, isLoading: false });
    } catch (error) {
      set({ error: error as string, isLoading: false });
    }
  },

  loadThemes: async () => {
    set({ isLoading: true, error: null });
    try {
      const themes = await invoke<ThemeInfo[]>('get_theme_list');
      set({ themes, isLoading: false });
    } catch (error) {
      set({ error: error as string, isLoading: false });
    }
  },

  applyTheme: async (themeName) => {
    set({ isLoading: true, error: null });
    try {
      await invoke('apply_theme', { themeName });
      set({ currentTheme: themeName, isLoading: false });
      
      // Update CSS custom properties
      const theme = get().themes.find(t => t.name === themeName);
      if (theme) {
        const root = document.documentElement;
        Object.entries(theme.colors).forEach(([key, value]) => {
          root.style.setProperty(`--${key.replace(/_/g, '-')}`, value as string);
        });
      }
    } catch (error) {
      set({ error: error as string, isLoading: false });
    }
  },

  exportConfig: async (filePath) => {
    set({ isLoading: true, error: null });
    try {
      await invoke('export_config', { filePath });
      set({ isLoading: false });
    } catch (error) {
      set({ error: error as string, isLoading: false });
    }
  },

  importConfig: async (filePath) => {
    set({ isLoading: true, error: null });
    try {
      await invoke('import_config', { filePath });
      await get().loadConfig();
    } catch (error) {
      set({ error: error as string, isLoading: false });
    }
  },

  clearError: () => set({ error: null }),
}));