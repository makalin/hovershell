import React, { useState, useEffect } from 'react';
import { Palette, Check } from 'lucide-react';
import { ThemePreview } from './ThemePreview';

interface Theme {
  name: string;
  display_name: string;
  description: string;
  colors: {
    background: string;
    foreground: string;
    primary: string;
    secondary: string;
    accent: string;
    success: string;
    warning: string;
    error: string;
    border: string;
  };
}

interface ThemeSelectorProps {
  currentTheme: string;
  onThemeChange: (themeName: string) => void;
  onClose: () => void;
}

export const ThemeSelector: React.FC<ThemeSelectorProps> = ({
  currentTheme,
  onThemeChange,
  onClose,
}) => {
  const [themes, setThemes] = useState<Theme[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadThemes();
  }, []);

  const loadThemes = async () => {
    try {
      // Mock themes data - in real app this would come from the backend
      const mockThemes: Theme[] = [
        {
          name: 'tokyo-night',
          display_name: 'Tokyo Night',
          description: 'Dark theme inspired by Tokyo\'s night sky',
          colors: {
            background: '#1a1b26',
            foreground: '#a9b1d6',
            primary: '#7aa2f7',
            secondary: '#9ece6a',
            accent: '#ff9e64',
            success: '#9ece6a',
            warning: '#e0af68',
            error: '#f7768e',
            border: '#565f89',
          },
        },
        {
          name: 'dracula',
          display_name: 'Dracula',
          description: 'Dark theme with vibrant colors',
          colors: {
            background: '#282a36',
            foreground: '#f8f8f2',
            primary: '#bd93f9',
            secondary: '#50fa7b',
            accent: '#ff79c6',
            success: '#50fa7b',
            warning: '#f1fa8c',
            error: '#ff5555',
            border: '#6272a4',
          },
        },
        {
          name: 'light',
          display_name: 'Light',
          description: 'Clean light theme',
          colors: {
            background: '#ffffff',
            foreground: '#333333',
            primary: '#007acc',
            secondary: '#28a745',
            accent: '#ff6b35',
            success: '#28a745',
            warning: '#ffc107',
            error: '#dc3545',
            border: '#e0e0e0',
          },
        },
        {
          name: 'monokai',
          display_name: 'Monokai',
          description: 'Classic Monokai color scheme',
          colors: {
            background: '#272822',
            foreground: '#f8f8f2',
            primary: '#f92672',
            secondary: '#a6e22e',
            accent: '#fd971f',
            success: '#a6e22e',
            warning: '#e6db74',
            error: '#f92672',
            border: '#49483e',
          },
        },
        {
          name: 'nord',
          display_name: 'Nord',
          description: 'Arctic-inspired color palette',
          colors: {
            background: '#2e3440',
            foreground: '#d8dee9',
            primary: '#88c0d0',
            secondary: '#a3be8c',
            accent: '#ebcb8b',
            success: '#a3be8c',
            warning: '#ebcb8b',
            error: '#bf616a',
            border: '#4c566a',
          },
        },
        {
          name: 'gruvbox',
          display_name: 'Gruvbox',
          description: 'Retro groove color scheme',
          colors: {
            background: '#282828',
            foreground: '#ebdbb2',
            primary: '#fe8019',
            secondary: '#b8bb26',
            accent: '#fabd2f',
            success: '#b8bb26',
            warning: '#fabd2f',
            error: '#fb4934',
            border: '#504945',
          },
        },
        {
          name: 'one-dark',
          display_name: 'One Dark',
          description: 'Atom\'s One Dark theme',
          colors: {
            background: '#282c34',
            foreground: '#abb2bf',
            primary: '#61afef',
            secondary: '#98c379',
            accent: '#e06c75',
            success: '#98c379',
            warning: '#e5c07b',
            error: '#e06c75',
            border: '#3e4451',
          },
        },
        {
          name: 'solarized-dark',
          display_name: 'Solarized Dark',
          description: 'Solarized dark color scheme',
          colors: {
            background: '#002b36',
            foreground: '#839496',
            primary: '#268bd2',
            secondary: '#859900',
            accent: '#b58900',
            success: '#859900',
            warning: '#b58900',
            error: '#dc322f',
            border: '#073642',
          },
        },
        {
          name: 'solarized-light',
          display_name: 'Solarized Light',
          description: 'Solarized light color scheme',
          colors: {
            background: '#fdf6e3',
            foreground: '#657b83',
            primary: '#268bd2',
            secondary: '#859900',
            accent: '#b58900',
            success: '#859900',
            warning: '#b58900',
            error: '#dc322f',
            border: '#eee8d5',
          },
        },
        {
          name: 'catppuccin-mocha',
          display_name: 'Catppuccin Mocha',
          description: 'Soothing pastel theme',
          colors: {
            background: '#1e1e2e',
            foreground: '#cdd6f4',
            primary: '#89b4fa',
            secondary: '#a6e3a1',
            accent: '#f9e2af',
            success: '#a6e3a1',
            warning: '#f9e2af',
            error: '#f38ba8',
            border: '#313244',
          },
        },
        {
          name: 'catppuccin-latte',
          display_name: 'Catppuccin Latte',
          description: 'Light pastel theme',
          colors: {
            background: '#eff1f5',
            foreground: '#4c4f69',
            primary: '#1e66f5',
            secondary: '#40a02b',
            accent: '#df8e1d',
            success: '#40a02b',
            warning: '#df8e1d',
            error: '#d20f39',
            border: '#ccd0da',
          },
        },
        {
          name: 'material-dark',
          display_name: 'Material Dark',
          description: 'Google Material Design dark theme',
          colors: {
            background: '#212121',
            foreground: '#ffffff',
            primary: '#bb86fc',
            secondary: '#03dac6',
            accent: '#ff6e6e',
            success: '#03dac6',
            warning: '#ffb74d',
            error: '#cf6679',
            border: '#424242',
          },
        },
        {
          name: 'github-dark',
          display_name: 'GitHub Dark',
          description: 'GitHub\'s dark theme',
          colors: {
            background: '#0d1117',
            foreground: '#e6edf3',
            primary: '#58a6ff',
            secondary: '#3fb950',
            accent: '#f85149',
            success: '#3fb950',
            warning: '#d29922',
            error: '#f85149',
            border: '#30363d',
          },
        },
        {
          name: 'github-light',
          display_name: 'GitHub Light',
          description: 'GitHub\'s light theme',
          colors: {
            background: '#ffffff',
            foreground: '#24292f',
            primary: '#0969da',
            secondary: '#1a7f37',
            accent: '#d1242f',
            success: '#1a7f37',
            warning: '#9a6700',
            error: '#d1242f',
            border: '#d0d7de',
          },
        },
      ];
      
      setThemes(mockThemes);
      setLoading(false);
    } catch (error) {
      console.error('Failed to load themes:', error);
      setLoading(false);
    }
  };

  const handleThemeSelect = (themeName: string) => {
    onThemeChange(themeName);
  };

  if (loading) {
    return (
      <div className="theme-selector-overlay fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
        <div className="theme-selector bg-gray-900 border border-gray-700 rounded-lg shadow-xl w-full max-w-6xl mx-4 max-h-[80vh] overflow-hidden">
          <div className="p-8 text-center">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary-color mx-auto mb-4"></div>
            <p className="text-gray-300">Loading themes...</p>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="theme-selector-overlay fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="theme-selector bg-gray-900 border border-gray-700 rounded-lg shadow-xl w-full max-w-6xl mx-4 max-h-[80vh] overflow-hidden">
        {/* Header */}
        <div className="theme-selector-header p-6 border-b border-gray-700">
          <div className="flex items-center justify-between">
            <div className="flex items-center space-x-3">
              <Palette size={24} className="text-primary-color" />
              <h2 className="text-xl font-semibold text-white">Choose Theme</h2>
            </div>
            <button
              onClick={onClose}
              className="p-2 hover:bg-gray-700 rounded-lg transition-colors"
            >
              <span className="text-gray-400 hover:text-white">✕</span>
            </button>
          </div>
          <p className="text-gray-400 mt-2">
            Select a theme to customize your HoverShell experience
          </p>
        </div>

        {/* Themes Grid */}
        <div className="theme-selector-content p-6 overflow-y-auto max-h-[60vh]">
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {themes.map((theme) => (
              <ThemePreview
                key={theme.name}
                themeName={theme.name}
                displayName={theme.display_name}
                description={theme.description}
                colors={theme.colors}
                onSelect={handleThemeSelect}
                isSelected={currentTheme === theme.name}
              />
            ))}
          </div>
        </div>

        {/* Footer */}
        <div className="theme-selector-footer p-6 border-t border-gray-700">
          <div className="flex items-center justify-between">
            <div className="flex items-center space-x-2">
              <Check size={16} className="text-success-color" />
              <span className="text-sm text-gray-400">
                {themes.length} themes available
              </span>
            </div>
            <button
              onClick={onClose}
              className="px-4 py-2 bg-primary-color text-white rounded-lg hover:bg-opacity-80 transition-colors"
            >
              Done
            </button>
          </div>
        </div>
      </div>

      <style>{`
        .theme-selector {
          background-color: var(--bg-color);
          border-color: var(--border-color);
        }
        
        .theme-selector-header {
          background-color: rgba(0, 0, 0, 0.2);
        }
        
        .theme-selector-footer {
          background-color: rgba(0, 0, 0, 0.2);
        }
        
        .theme-preview {
          transition: all 0.2s ease;
        }
        
        .theme-preview:hover {
          transform: translateY(-2px);
          box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
        }
      `}</style>
    </div>
  );
};