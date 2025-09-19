import React, { useState } from 'react';
import { 
  Terminal, 
  Settings, 
  Zap, 
  Folder, 
  GitBranch, 
  Database, 
  FileText,
  Plus,
  X,
  ChevronRight,
  ChevronDown
} from 'lucide-react';
import { useTerminalStore } from '../stores/terminalStore';
import { useAppStore } from '../stores/appStore';

export const Sidebar: React.FC = () => {
  const [activeTab, setActiveTab] = useState<'terminals' | 'providers' | 'files' | 'git' | 'tools'>('terminals');
  const [expandedSections, setExpandedSections] = useState<Set<string>>(new Set(['terminals']));
  
  const { terminals, activeTerminalId, setActiveTerminal, createTerminal, closeTerminal } = useTerminalStore();
  const { providers, defaultProvider } = useAppStore();

  const toggleSection = (section: string) => {
    setExpandedSections(prev => {
      const newSet = new Set(prev);
      if (newSet.has(section)) {
        newSet.delete(section);
      } else {
        newSet.add(section);
      }
      return newSet;
    });
  };

  const handleCreateTerminal = () => {
    createTerminal();
  };

  const handleCloseTerminal = (id: string, e: React.MouseEvent) => {
    e.stopPropagation();
    closeTerminal(id);
  };

  const handleTerminalClick = (id: string) => {
    setActiveTerminal(id);
  };

  const renderTerminalsSection = () => {
    const isExpanded = expandedSections.has('terminals');
    
    return (
      <div className="sidebar-section">
        <div 
          className="sidebar-section-header"
          onClick={() => toggleSection('terminals')}
        >
          {isExpanded ? <ChevronDown size={16} /> : <ChevronRight size={16} />}
          <Terminal size={16} />
          <span>Terminals</span>
          <button
            className="ml-auto p-1 hover:bg-gray-700 rounded"
            onClick={(e) => {
              e.stopPropagation();
              handleCreateTerminal();
            }}
          >
            <Plus size={14} />
          </button>
        </div>
        
        {isExpanded && (
          <div className="sidebar-section-content">
            {terminals.length === 0 ? (
              <div className="text-gray-500 text-sm p-2">
                No terminals
              </div>
            ) : (
              terminals.map(terminal => (
                <div
                  key={terminal.id}
                  className={`sidebar-item ${terminal.id === activeTerminalId ? 'active' : ''}`}
                  onClick={() => handleTerminalClick(terminal.id)}
                >
                  <span className="flex-1 truncate">{terminal.title}</span>
                  <button
                    className="p-1 hover:bg-gray-700 rounded opacity-0 group-hover:opacity-100 transition-opacity"
                    onClick={(e) => handleCloseTerminal(terminal.id, e)}
                  >
                    <X size={12} />
                  </button>
                </div>
              ))
            )}
          </div>
        )}
      </div>
    );
  };

  const renderProvidersSection = () => {
    const isExpanded = expandedSections.has('providers');
    
    return (
      <div className="sidebar-section">
        <div 
          className="sidebar-section-header"
          onClick={() => toggleSection('providers')}
        >
          {isExpanded ? <ChevronDown size={16} /> : <ChevronRight size={16} />}
          <Zap size={16} />
          <span>AI Providers</span>
        </div>
        
        {isExpanded && (
          <div className="sidebar-section-content">
            {providers.length === 0 ? (
              <div className="text-gray-500 text-sm p-2">
                No providers configured
              </div>
            ) : (
              providers.map(provider => (
                <div
                  key={provider.id}
                  className={`sidebar-item ${provider.id === defaultProvider ? 'active' : ''}`}
                >
                  <span className="flex-1 truncate">{provider.name}</span>
                  {provider.id === defaultProvider && (
                    <span className="text-xs text-primary-color">default</span>
                  )}
                </div>
              ))
            )}
          </div>
        )}
      </div>
    );
  };

  const renderFilesSection = () => {
    const isExpanded = expandedSections.has('files');
    
    return (
      <div className="sidebar-section">
        <div 
          className="sidebar-section-header"
          onClick={() => toggleSection('files')}
        >
          {isExpanded ? <ChevronDown size={16} /> : <ChevronRight size={16} />}
          <Folder size={16} />
          <span>Files</span>
        </div>
        
        {isExpanded && (
          <div className="sidebar-section-content">
            <div className="text-gray-500 text-sm p-2">
              File browser coming soon
            </div>
          </div>
        )}
      </div>
    );
  };

  const renderGitSection = () => {
    const isExpanded = expandedSections.has('git');
    
    return (
      <div className="sidebar-section">
        <div 
          className="sidebar-section-header"
          onClick={() => toggleSection('git')}
        >
          {isExpanded ? <ChevronDown size={16} /> : <ChevronRight size={16} />}
          <GitBranch size={16} />
          <span>Git</span>
        </div>
        
        {isExpanded && (
          <div className="sidebar-section-content">
            <div className="text-gray-500 text-sm p-2">
              Git integration coming soon
            </div>
          </div>
        )}
      </div>
    );
  };

  const renderToolsSection = () => {
    const isExpanded = expandedSections.has('tools');
    
    return (
      <div className="sidebar-section">
        <div 
          className="sidebar-section-header"
          onClick={() => toggleSection('tools')}
        >
          {isExpanded ? <ChevronDown size={16} /> : <ChevronRight size={16} />}
          <Database size={16} />
          <span>Tools</span>
        </div>
        
        {isExpanded && (
          <div className="sidebar-section-content">
            <div className="sidebar-item">
              <Database size={14} />
              <span>Database</span>
            </div>
            <div className="sidebar-item">
              <FileText size={14} />
              <span>Notes</span>
            </div>
          </div>
        )}
      </div>
    );
  };

  return (
    <div className="sidebar w-64 bg-gray-900 border-r border-gray-700 flex flex-col">
      {/* Sidebar header */}
      <div className="sidebar-header p-4 border-b border-gray-700">
        <div className="flex items-center space-x-2">
          <div className="w-8 h-8 bg-primary-color rounded-lg flex items-center justify-center">
            <Terminal size={20} className="text-white" />
          </div>
          <div>
            <h1 className="text-lg font-semibold">HoverShell</h1>
            <p className="text-xs text-gray-500">AI Terminal</p>
          </div>
        </div>
      </div>

      {/* Sidebar tabs */}
      <div className="sidebar-tabs flex border-b border-gray-700">
        {[
          { id: 'terminals', icon: Terminal, label: 'Terminals' },
          { id: 'providers', icon: Zap, label: 'AI' },
          { id: 'files', icon: Folder, label: 'Files' },
          { id: 'git', icon: GitBranch, label: 'Git' },
          { id: 'tools', icon: Database, label: 'Tools' },
        ].map(tab => (
          <button
            key={tab.id}
            className={`sidebar-tab ${activeTab === tab.id ? 'active' : ''}`}
            onClick={() => setActiveTab(tab.id as any)}
          >
            <tab.icon size={16} />
          </button>
        ))}
      </div>

      {/* Sidebar content */}
      <div className="sidebar-content flex-1 overflow-y-auto">
        {activeTab === 'terminals' && renderTerminalsSection()}
        {activeTab === 'providers' && renderProvidersSection()}
        {activeTab === 'files' && renderFilesSection()}
        {activeTab === 'git' && renderGitSection()}
        {activeTab === 'tools' && renderToolsSection()}
      </div>

      {/* Sidebar footer */}
      <div className="sidebar-footer p-4 border-t border-gray-700">
        <button className="sidebar-footer-button w-full">
          <Settings size={16} />
          <span>Settings</span>
        </button>
      </div>

      <style>{`
        .sidebar {
          background-color: var(--bg-color);
          border-color: var(--border-color);
        }
        
        .sidebar-header {
          background-color: rgba(0, 0, 0, 0.2);
        }
        
        .sidebar-tabs {
          background-color: rgba(0, 0, 0, 0.1);
        }
        
        .sidebar-tab {
          flex: 1;
          padding: 8px;
          display: flex;
          align-items: center;
          justify-center;
          color: var(--fg-color);
          background-color: transparent;
          border: none;
          transition: background-color 0.2s;
        }
        
        .sidebar-tab:hover {
          background-color: rgba(255, 255, 255, 0.1);
        }
        
        .sidebar-tab.active {
          background-color: var(--primary-color);
          color: white;
        }
        
        .sidebar-section {
          margin: 8px;
        }
        
        .sidebar-section-header {
          display: flex;
          align-items: center;
          gap: 8px;
          padding: 8px;
          cursor: pointer;
          border-radius: 4px;
          transition: background-color 0.2s;
          color: var(--fg-color);
        }
        
        .sidebar-section-header:hover {
          background-color: rgba(255, 255, 255, 0.1);
        }
        
        .sidebar-section-content {
          margin-left: 16px;
          margin-top: 4px;
        }
        
        .sidebar-item {
          display: flex;
          align-items: center;
          gap: 8px;
          padding: 6px 8px;
          margin: 2px 0;
          border-radius: 4px;
          cursor: pointer;
          transition: background-color 0.2s;
          color: var(--fg-color);
          position: relative;
          group: true;
        }
        
        .sidebar-item:hover {
          background-color: rgba(255, 255, 255, 0.1);
        }
        
        .sidebar-item.active {
          background-color: var(--primary-color);
          color: white;
        }
        
        .sidebar-footer-button {
          display: flex;
          align-items: center;
          gap: 8px;
          padding: 8px;
          background-color: transparent;
          border: none;
          color: var(--fg-color);
          border-radius: 4px;
          transition: background-color 0.2s;
          cursor: pointer;
        }
        
        .sidebar-footer-button:hover {
          background-color: rgba(255, 255, 255, 0.1);
        }
      `}</style>
    </div>
  );
};