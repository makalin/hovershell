import { useEffect, useState } from 'react';
import { listen } from '@tauri-apps/api/event';
import { Terminal } from './components/Terminal';
import { Sidebar } from './components/Sidebar';
import { StatusBar } from './components/StatusBar';
import { CommandPalette } from './components/CommandPalette';
import { useAppStore } from './stores/appStore';
import { useTerminalStore } from './stores/terminalStore';
import { useConfigStore } from './stores/configStore';
import { useHotkeys } from 'react-hotkeys-hook';

function App() {
  const [isVisible, setIsVisible] = useState(false);
  const [isCommandPaletteOpen, setIsCommandPaletteOpen] = useState(false);
  
  const { 
    loadConfig, 
    currentTheme,
    applyTheme 
  } = useConfigStore();
  
  const { 
    terminals, 
    activeTerminalId, 
    setActiveTerminal,
    createTerminal,
    closeTerminal 
  } = useTerminalStore();
  
  const { 
    loadProviders,
    removeProvider,
    setDefaultProvider 
  } = useAppStore();

  // Load initial data
  useEffect(() => {
    const initializeApp = async () => {
      try {
        await loadConfig();
        await loadProviders();
      } catch (error) {
        console.error('Failed to initialize app:', error);
      }
    };
    
    initializeApp();
  }, []);

  // Listen for hotkey events
  useEffect(() => {
    const unlisten = listen('hotkey-triggered', (event) => {
      const { callback } = event.payload as { hotkey: string; callback: string };
      handleHotkeyCallback(callback);
    });

    return () => {
      unlisten.then(fn => fn());
    };
  }, []);

  // Hotkey handlers
  useHotkeys('cmd+k', () => {
    setIsCommandPaletteOpen(true);
  });

  useHotkeys('cmd+t', () => {
    createTerminal();
  });

  useHotkeys('cmd+w', () => {
    if (activeTerminalId) {
      closeTerminal(activeTerminalId);
    }
  });

  useHotkeys('cmd+shift+]', () => {
    const currentIndex = terminals.findIndex(t => t.id === activeTerminalId);
    const nextIndex = (currentIndex + 1) % terminals.length;
    if (terminals[nextIndex]) {
      setActiveTerminal(terminals[nextIndex].id);
    }
  });

  useHotkeys('cmd+shift+[', () => {
    const currentIndex = terminals.findIndex(t => t.id === activeTerminalId);
    const prevIndex = currentIndex === 0 ? terminals.length - 1 : currentIndex - 1;
    if (terminals[prevIndex]) {
      setActiveTerminal(terminals[prevIndex].id);
    }
  });

  useHotkeys('escape', () => {
    setIsCommandPaletteOpen(false);
    setIsVisible(false);
  });

  const handleHotkeyCallback = (callback: string) => {
    switch (callback) {
      case 'toggle_window':
        setIsVisible(!isVisible);
        break;
      case 'paste_run':
        // TODO: Implement paste and run
        break;
      case 'quick_hide':
        setIsVisible(false);
        break;
      case 'new_tab':
        createTerminal();
        break;
      case 'close_tab':
        if (activeTerminalId) {
          closeTerminal(activeTerminalId);
        }
        break;
      case 'next_tab':
        const currentIndex = terminals.findIndex(t => t.id === activeTerminalId);
        const nextIndex = (currentIndex + 1) % terminals.length;
        if (terminals[nextIndex]) {
          setActiveTerminal(terminals[nextIndex].id);
        }
        break;
      case 'prev_tab':
        const currentIdx = terminals.findIndex(t => t.id === activeTerminalId);
        const prevIndex = currentIdx === 0 ? terminals.length - 1 : currentIdx - 1;
        if (terminals[prevIndex]) {
          setActiveTerminal(terminals[prevIndex].id);
        }
        break;
      default:
        console.log('Unknown hotkey callback:', callback);
    }
  };

  const handleCommandPaletteCommand = async (command: string, args: string[]) => {
    try {
      switch (command) {
        case 'ai.chat':
          // TODO: Implement AI chat
          break;
        case 'terminal.new':
          createTerminal();
          break;
        case 'terminal.close':
          if (activeTerminalId) {
            closeTerminal(activeTerminalId);
          }
          break;
        case 'config.theme':
          if (args[0]) {
            applyTheme(args[0]);
          }
          break;
        case 'provider.add':
          // TODO: Implement provider addition
          break;
        case 'provider.remove':
          if (args[0]) {
            removeProvider(args[0]);
          }
          break;
        case 'provider.set-default':
          if (args[0]) {
            setDefaultProvider(args[0]);
          }
          break;
        default:
          console.log('Unknown command:', command);
      }
    } catch (error) {
      console.error('Command execution failed:', error);
    }
  };

  if (!isVisible) {
    return null;
  }

  return (
    <div className={`theme-${currentTheme} w-full h-full flex flex-col glass`}>
      {/* Main content area */}
      <div className="flex-1 flex">
        {/* Sidebar */}
        <Sidebar />
        
        {/* Terminal area */}
        <div className="flex-1 flex flex-col">
          {terminals.length > 0 ? (
            <Terminal 
              terminal={terminals.find(t => t.id === activeTerminalId) || terminals[0]}
              onInput={(input) => {
                // TODO: Handle terminal input
                console.log('Terminal input:', input);
              }}
            />
          ) : (
            <div className="flex-1 flex items-center justify-center">
              <div className="text-center">
                <h2 className="text-xl font-semibold mb-4">No Terminal Sessions</h2>
                <p className="text-gray-500 mb-4">Press Cmd+T to create a new terminal</p>
                <button
                  onClick={() => createTerminal()}
                  className="px-4 py-2 bg-primary-color text-white rounded-lg hover:bg-opacity-80 transition-colors"
                >
                  Create Terminal
                </button>
              </div>
            </div>
          )}
        </div>
      </div>
      
      {/* Status bar */}
      <StatusBar />
      
      {/* Command palette */}
      {isCommandPaletteOpen && (
        <CommandPalette
          onClose={() => setIsCommandPaletteOpen(false)}
          onCommand={handleCommandPaletteCommand}
        />
      )}
    </div>
  );
}

export default App;