import React, { useState, useEffect, useRef } from 'react';
import { 
  Search, 
  Terminal, 
  Zap, 
  Settings, 
  FileText, 
  GitBranch,
  X,
  ArrowDown,
  ArrowUp,
  Folder,
  Copy,
  Move,
  Trash2,
  GitCommit,
  Activity,
  HardDrive,
  Wifi,
  ArrowUpDown,
  Hash,
  Download,
  FileDiff
} from 'lucide-react';

interface Command {
  id: string;
  title: string;
  description: string;
  category: string;
  icon: React.ComponentType<any>;
  keywords: string[];
  action: () => void;
}

interface CommandPaletteProps {
  onClose: () => void;
  onCommand: (command: string, args: string[]) => void;
}

export const CommandPalette: React.FC<CommandPaletteProps> = ({ onClose, onCommand }) => {
  const [query, setQuery] = useState('');
  const [selectedIndex, setSelectedIndex] = useState(0);
  const [commands, setCommands] = useState<Command[]>([]);
  const inputRef = useRef<HTMLInputElement>(null);

  const allCommands: Command[] = [
    // Terminal commands
    {
      id: 'terminal.new',
      title: 'New Terminal',
      description: 'Create a new terminal session',
      category: 'Terminal',
      icon: Terminal,
      keywords: ['new', 'terminal', 'create', 'tab'],
      action: () => onCommand('terminal.new', []),
    },
    {
      id: 'terminal.close',
      title: 'Close Terminal',
      description: 'Close the current terminal session',
      category: 'Terminal',
      icon: Terminal,
      keywords: ['close', 'terminal', 'tab', 'exit'],
      action: () => onCommand('terminal.close', []),
    },
    {
      id: 'terminal.clear',
      title: 'Clear Terminal',
      description: 'Clear the terminal output',
      category: 'Terminal',
      icon: Terminal,
      keywords: ['clear', 'terminal', 'clean'],
      action: () => onCommand('terminal.clear', []),
    },

    // AI commands
    {
      id: 'ai.chat',
      title: 'AI Chat',
      description: 'Start a conversation with AI',
      category: 'AI',
      icon: Zap,
      keywords: ['ai', 'chat', 'conversation', 'ask'],
      action: () => onCommand('ai.chat', []),
    },
    {
      id: 'ai.explain',
      title: 'Explain Code',
      description: 'Ask AI to explain selected code',
      category: 'AI',
      icon: Zap,
      keywords: ['ai', 'explain', 'code', 'help'],
      action: () => onCommand('ai.explain', []),
    },
    {
      id: 'ai.generate',
      title: 'Generate Code',
      description: 'Ask AI to generate code',
      category: 'AI',
      icon: Zap,
      keywords: ['ai', 'generate', 'code', 'create'],
      action: () => onCommand('ai.generate', []),
    },

    // File operations
    {
      id: 'file.list',
      title: 'List Directory',
      description: 'List files and directories',
      category: 'File Operations',
      icon: Folder,
      keywords: ['list', 'directory', 'files', 'ls'],
      action: () => onCommand('file.list', []),
    },
    {
      id: 'file.find',
      title: 'Find Files',
      description: 'Search for files by name pattern',
      category: 'File Operations',
      icon: Search,
      keywords: ['find', 'search', 'files', 'pattern'],
      action: () => onCommand('file.find', []),
    },
    {
      id: 'file.search',
      title: 'Search in Files',
      description: 'Search for text content in files',
      category: 'File Operations',
      icon: Search,
      keywords: ['search', 'text', 'content', 'grep'],
      action: () => onCommand('file.search', []),
    },
    {
      id: 'file.copy',
      title: 'Copy File/Directory',
      description: 'Copy files or directories',
      category: 'File Operations',
      icon: Copy,
      keywords: ['copy', 'file', 'directory', 'cp'],
      action: () => onCommand('file.copy', []),
    },
    {
      id: 'file.move',
      title: 'Move File/Directory',
      description: 'Move or rename files or directories',
      category: 'File Operations',
      icon: Move,
      keywords: ['move', 'rename', 'file', 'directory', 'mv'],
      action: () => onCommand('file.move', []),
    },
    {
      id: 'file.delete',
      title: 'Delete File/Directory',
      description: 'Delete files or directories',
      category: 'File Operations',
      icon: Trash2,
      keywords: ['delete', 'remove', 'file', 'directory', 'rm'],
      action: () => onCommand('file.delete', []),
    },

    // Git operations
    {
      id: 'git.status',
      title: 'Git Status',
      description: 'Show git repository status',
      category: 'Git',
      icon: GitBranch,
      keywords: ['git', 'status', 'repository'],
      action: () => onCommand('git.status', []),
    },
    {
      id: 'git.branches',
      title: 'Git Branches',
      description: 'List git branches',
      category: 'Git',
      icon: GitBranch,
      keywords: ['git', 'branches', 'list'],
      action: () => onCommand('git.branches', []),
    },
    {
      id: 'git.commit',
      title: 'Git Commit',
      description: 'Commit staged changes',
      category: 'Git',
      icon: GitCommit,
      keywords: ['git', 'commit', 'changes'],
      action: () => onCommand('git.commit', []),
    },
    {
      id: 'git.diff',
      title: 'Git Diff',
      description: 'Show git differences',
      category: 'Git',
      icon: FileDiff,
      keywords: ['git', 'diff', 'changes', 'differences'],
      action: () => onCommand('git.diff', []),
    },

    // System monitoring
    {
      id: 'system.processes',
      title: 'List Processes',
      description: 'Show running processes',
      category: 'System',
      icon: Activity,
      keywords: ['processes', 'system', 'ps', 'top'],
      action: () => onCommand('system.processes', []),
    },
    {
      id: 'system.disk',
      title: 'Disk Usage',
      description: 'Show disk usage information',
      category: 'System',
      icon: HardDrive,
      keywords: ['disk', 'usage', 'space', 'df'],
      action: () => onCommand('system.disk', []),
    },
    {
      id: 'system.network',
      title: 'Network Interfaces',
      description: 'Show network interface information',
      category: 'System',
      icon: Wifi,
      keywords: ['network', 'interfaces', 'ifconfig'],
      action: () => onCommand('system.network', []),
    },

    // Text processing
    {
      id: 'text.grep',
      title: 'Grep Text',
      description: 'Search for patterns in text',
      category: 'Text Processing',
      icon: Search,
      keywords: ['grep', 'search', 'pattern', 'text'],
      action: () => onCommand('text.grep', []),
    },
    {
      id: 'text.sort',
      title: 'Sort Text',
      description: 'Sort lines of text',
      category: 'Text Processing',
      icon: ArrowUpDown,
      keywords: ['sort', 'text', 'lines'],
      action: () => onCommand('text.sort', []),
    },
    {
      id: 'text.wc',
      title: 'Word Count',
      description: 'Count words, lines, and characters',
      category: 'Text Processing',
      icon: Hash,
      keywords: ['wc', 'count', 'words', 'lines', 'characters'],
      action: () => onCommand('text.wc', []),
    },

    // Network tools
    {
      id: 'network.ping',
      title: 'Ping Host',
      description: 'Ping a network host',
      category: 'Network',
      icon: Wifi,
      keywords: ['ping', 'network', 'host', 'connectivity'],
      action: () => onCommand('network.ping', []),
    },
    {
      id: 'network.scan',
      title: 'Port Scan',
      description: 'Scan ports on a host',
      category: 'Network',
      icon: Search,
      keywords: ['port', 'scan', 'network', 'security'],
      action: () => onCommand('network.scan', []),
    },
    {
      id: 'network.download',
      title: 'Download File',
      description: 'Download a file from URL',
      category: 'Network',
      icon: Download,
      keywords: ['download', 'file', 'url', 'wget', 'curl'],
      action: () => onCommand('network.download', []),
    },

    // Provider commands
    {
      id: 'provider.add',
      title: 'Add AI Provider',
      description: 'Add a new AI provider',
      category: 'AI',
      icon: Zap,
      keywords: ['provider', 'add', 'ai', 'new'],
      action: () => onCommand('provider.add', []),
    },
    {
      id: 'provider.remove',
      title: 'Remove AI Provider',
      description: 'Remove an AI provider',
      category: 'AI',
      icon: Zap,
      keywords: ['provider', 'remove', 'delete', 'ai'],
      action: () => onCommand('provider.remove', []),
    },
    {
      id: 'provider.set-default',
      title: 'Set Default Provider',
      description: 'Set the default AI provider',
      category: 'AI',
      icon: Zap,
      keywords: ['provider', 'default', 'set', 'ai'],
      action: () => onCommand('provider.set-default', []),
    },

    // File commands
    {
      id: 'file.open',
      title: 'Open File',
      description: 'Open a file in the editor',
      category: 'File',
      icon: FileText,
      keywords: ['file', 'open', 'edit'],
      action: () => onCommand('file.open', []),
    },
    {
      id: 'file.new',
      title: 'New File',
      description: 'Create a new file',
      category: 'File',
      icon: FileText,
      keywords: ['file', 'new', 'create'],
      action: () => onCommand('file.new', []),
    },

    // Git commands
    {
      id: 'git.status',
      title: 'Git Status',
      description: 'Show git repository status',
      category: 'Git',
      icon: GitBranch,
      keywords: ['git', 'status', 'repo'],
      action: () => onCommand('git.status', []),
    },
    {
      id: 'git.commit',
      title: 'Git Commit',
      description: 'Commit changes to git',
      category: 'Git',
      icon: GitBranch,
      keywords: ['git', 'commit', 'save'],
      action: () => onCommand('git.commit', []),
    },

    // Settings commands
    {
      id: 'config.theme',
      title: 'Change Theme',
      description: 'Change the application theme',
      category: 'Settings',
      icon: Settings,
      keywords: ['theme', 'color', 'appearance', 'config'],
      action: () => onCommand('config.theme', []),
    },
    {
      id: 'config.hotkeys',
      title: 'Configure Hotkeys',
      description: 'Configure keyboard shortcuts',
      category: 'Settings',
      icon: Settings,
      keywords: ['hotkeys', 'shortcuts', 'keys', 'config'],
      action: () => onCommand('config.hotkeys', []),
    },
  ];

  // Filter commands based on query
  useEffect(() => {
    if (!query.trim()) {
      setCommands(allCommands);
    } else {
      const filtered = allCommands.filter(command => {
        const searchText = `${command.title} ${command.description} ${command.keywords.join(' ')}`.toLowerCase();
        return searchText.includes(query.toLowerCase());
      });
      setCommands(filtered);
    }
    setSelectedIndex(0);
  }, [query]);

  // Focus input on mount
  useEffect(() => {
    if (inputRef.current) {
      inputRef.current.focus();
    }
  }, []);

  const handleKeyDown = (e: React.KeyboardEvent) => {
    switch (e.key) {
      case 'Escape':
        onClose();
        break;
      case 'ArrowDown':
        e.preventDefault();
        setSelectedIndex(prev => (prev + 1) % commands.length);
        break;
      case 'ArrowUp':
        e.preventDefault();
        setSelectedIndex(prev => prev === 0 ? commands.length - 1 : prev - 1);
        break;
      case 'Enter':
        e.preventDefault();
        if (commands[selectedIndex]) {
          commands[selectedIndex].action();
          onClose();
        }
        break;
    }
  };

  const handleCommandClick = (command: Command) => {
    command.action();
    onClose();
  };

  const groupedCommands = commands.reduce((groups, command) => {
    if (!groups[command.category]) {
      groups[command.category] = [];
    }
    groups[command.category].push(command);
    return groups;
  }, {} as Record<string, Command[]>);

  return (
    <div className="command-palette-overlay fixed inset-0 bg-black bg-opacity-50 flex items-start justify-center pt-20 z-50">
      <div className="command-palette bg-gray-900 border border-gray-700 rounded-lg shadow-xl w-full max-w-2xl mx-4">
        {/* Header */}
        <div className="command-palette-header p-4 border-b border-gray-700">
          <div className="flex items-center space-x-3">
            <Search size={20} className="text-gray-400" />
            <input
              ref={inputRef}
              type="text"
              placeholder="Type a command or search..."
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              onKeyDown={handleKeyDown}
              className="flex-1 bg-transparent text-white placeholder-gray-400 focus:outline-none"
            />
            <button
              onClick={onClose}
              className="p-1 hover:bg-gray-700 rounded"
            >
              <X size={16} className="text-gray-400" />
            </button>
          </div>
        </div>

        {/* Commands list */}
        <div className="command-palette-content max-h-96 overflow-y-auto">
          {Object.entries(groupedCommands).map(([category, categoryCommands]) => (
            <div key={category} className="command-category">
              <div className="command-category-header px-4 py-2 text-xs font-semibold text-gray-500 uppercase tracking-wider">
                {category}
              </div>
              {categoryCommands.map((command) => {
                const globalIndex = commands.findIndex(c => c.id === command.id);
                const isSelected = globalIndex === selectedIndex;
                
                return (
                  <div
                    key={command.id}
                    className={`command-item ${isSelected ? 'selected' : ''}`}
                    onClick={() => handleCommandClick(command)}
                  >
                    <div className="flex items-center space-x-3 p-3">
                      <command.icon size={16} className="text-gray-400" />
                      <div className="flex-1">
                        <div className="text-white font-medium">{command.title}</div>
                        <div className="text-gray-400 text-sm">{command.description}</div>
                      </div>
                      {isSelected && (
                        <div className="flex items-center space-x-2 text-xs text-gray-500">
                          <span>Enter to run</span>
                        </div>
                      )}
                    </div>
                  </div>
                );
              })}
            </div>
          ))}
          
          {commands.length === 0 && (
            <div className="p-8 text-center text-gray-500">
              No commands found
            </div>
          )}
        </div>

        {/* Footer */}
        <div className="command-palette-footer p-4 border-t border-gray-700 text-xs text-gray-500">
          <div className="flex items-center justify-between">
            <div className="flex items-center space-x-4">
              <div className="flex items-center space-x-1">
                <ArrowUp size={12} />
                <ArrowDown size={12} />
                <span>Navigate</span>
              </div>
              <div className="flex items-center space-x-1">
                <span>Select</span>
              </div>
              <div className="flex items-center space-x-1">
                <X size={12} />
                <span>Close</span>
              </div>
            </div>
            <div>
              {commands.length} command{commands.length !== 1 ? 's' : ''}
            </div>
          </div>
        </div>
      </div>

      <style>{`
        .command-palette {
          background-color: var(--bg-color);
          border-color: var(--border-color);
        }
        
        .command-palette-header {
          background-color: rgba(0, 0, 0, 0.2);
        }
        
        .command-category-header {
          background-color: rgba(0, 0, 0, 0.1);
        }
        
        .command-item {
          cursor: pointer;
          transition: background-color 0.2s;
        }
        
        .command-item:hover {
          background-color: rgba(255, 255, 255, 0.05);
        }
        
        .command-item.selected {
          background-color: var(--primary-color);
        }
        
        .command-item.selected:hover {
          background-color: var(--primary-color);
        }
        
        .command-palette-footer {
          background-color: rgba(0, 0, 0, 0.2);
        }
      `}</style>
    </div>
  );
};