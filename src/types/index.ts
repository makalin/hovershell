/**
 * Type definitions for HoverShell
 */

// Basic types
export interface Point {
  x: number;
  y: number;
}

export interface Size {
  width: number;
  height: number;
}

export interface Rect {
  x: number;
  y: number;
  width: number;
  height: number;
}

// Terminal types
export interface TerminalSession {
  id: string;
  title: string;
  working_directory: string;
  is_active: boolean;
  output: string;
}

// Provider types
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

// Config types
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
  providers: Provider[];
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

// System types
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