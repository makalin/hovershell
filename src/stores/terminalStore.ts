import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/tauri';

export interface TerminalSession {
  id: string;
  title: string;
  working_directory: string;
  is_active: boolean;
  output: string;
}

interface TerminalState {
  terminals: TerminalSession[];
  activeTerminalId: string | null;
  isLoading: boolean;
  error: string | null;
  
  // Actions
  loadTerminals: () => Promise<void>;
  createTerminal: (title?: string, workingDirectory?: string) => Promise<void>;
  closeTerminal: (id: string) => Promise<void>;
  setActiveTerminal: (id: string) => Promise<void>;
  sendInput: (terminalId: string, input: string) => Promise<void>;
  clearOutput: (terminalId: string) => Promise<void>;
  executeCommand: (terminalId: string, command: string) => Promise<void>;
  clearError: () => void;
}

export const useTerminalStore = create<TerminalState>((set, get) => ({
  terminals: [],
  activeTerminalId: null,
  isLoading: false,
  error: null,

  loadTerminals: async () => {
    set({ isLoading: true, error: null });
    try {
      const terminals = await invoke<TerminalSession[]>('get_terminal_state');
      const activeTerminal = terminals.find(t => t.is_active);
      set({ 
        terminals, 
        activeTerminalId: activeTerminal?.id || terminals[0]?.id || null,
        isLoading: false 
      });
    } catch (error) {
      set({ error: error as string, isLoading: false });
    }
  },

  createTerminal: async (title, workingDirectory) => {
    set({ isLoading: true, error: null });
    try {
      // Create a new terminal session
      const newTerminal: TerminalSession = {
        id: `terminal_${Date.now()}`,
        title: title || `Terminal ${get().terminals.length + 1}`,
        working_directory: workingDirectory || '/',
        is_active: false,
        output: '',
      };
      
      set(state => ({
        terminals: [...state.terminals, newTerminal],
        activeTerminalId: newTerminal.id,
        isLoading: false
      }));
    } catch (error) {
      set({ error: error as string, isLoading: false });
    }
  },

  closeTerminal: async (id) => {
    set({ isLoading: true, error: null });
    try {
      set(state => {
        const terminals = state.terminals.filter(t => t.id !== id);
        const activeTerminalId = state.activeTerminalId === id 
          ? terminals[0]?.id || null 
          : state.activeTerminalId;
        
        return { terminals, activeTerminalId, isLoading: false };
      });
    } catch (error) {
      set({ error: error as string, isLoading: false });
    }
  },

  setActiveTerminal: async (id) => {
    set({ isLoading: true, error: null });
    try {
      set(state => ({
        terminals: state.terminals.map(t => ({
          ...t,
          is_active: t.id === id
        })),
        activeTerminalId: id,
        isLoading: false
      }));
    } catch (error) {
      set({ error: error as string, isLoading: false });
    }
  },

  sendInput: async (terminalId, input) => {
    set({ isLoading: true, error: null });
    try {
      await invoke('send_terminal_input', { terminalId, input });
      
      // Update terminal output
      set(state => ({
        terminals: state.terminals.map(t => 
          t.id === terminalId 
            ? { ...t, output: t.output + `$ ${input}\n` }
            : t
        ),
        isLoading: false
      }));
    } catch (error) {
      set({ error: error as string, isLoading: false });
    }
  },

  clearOutput: async (terminalId) => {
    set({ isLoading: true, error: null });
    try {
      set(state => ({
        terminals: state.terminals.map(t => 
          t.id === terminalId 
            ? { ...t, output: '' }
            : t
        ),
        isLoading: false
      }));
    } catch (error) {
      set({ error: error as string, isLoading: false });
    }
  },

  executeCommand: async (terminalId, command) => {
    set({ isLoading: true, error: null });
    try {
      // TODO: Implement actual command execution
      const result = `Executing: ${command}\n`;
      
      set(state => ({
        terminals: state.terminals.map(t => 
          t.id === terminalId 
            ? { ...t, output: t.output + result }
            : t
        ),
        isLoading: false
      }));
    } catch (error) {
      set({ error: error as string, isLoading: false });
    }
  },

  clearError: () => set({ error: null }),
}));