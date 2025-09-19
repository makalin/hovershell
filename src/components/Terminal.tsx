import React, { useRef, useEffect, useState } from 'react';
import { TerminalSession } from '../stores/terminalStore';

interface TerminalProps {
  terminal: TerminalSession;
  onInput: (input: string) => void;
}

export const Terminal: React.FC<TerminalProps> = ({ terminal, onInput }) => {
  const terminalRef = useRef<HTMLDivElement>(null);
  const [input, setInput] = useState('');
  const [cursorPosition, setCursorPosition] = useState(0);
  const [commandHistory, setCommandHistory] = useState<string[]>([]);
  const [historyIndex, setHistoryIndex] = useState(-1);

  // Auto-scroll to bottom when output changes
  useEffect(() => {
    if (terminalRef.current) {
      terminalRef.current.scrollTop = terminalRef.current.scrollHeight;
    }
  }, [terminal.output]);

  // Focus terminal on mount
  useEffect(() => {
    if (terminalRef.current) {
      terminalRef.current.focus();
    }
  }, [terminal.id]);

  const handleKeyDown = (e: React.KeyboardEvent) => {
    switch (e.key) {
      case 'Enter':
        e.preventDefault();
        if (input.trim()) {
          onInput(input);
          setCommandHistory(prev => [...prev, input]);
          setHistoryIndex(-1);
          setInput('');
          setCursorPosition(0);
        }
        break;
      
      case 'ArrowUp':
        e.preventDefault();
        if (commandHistory.length > 0) {
          const newIndex = historyIndex === -1 
            ? commandHistory.length - 1 
            : Math.max(0, historyIndex - 1);
          setHistoryIndex(newIndex);
          setInput(commandHistory[newIndex]);
          setCursorPosition(commandHistory[newIndex].length);
        }
        break;
      
      case 'ArrowDown':
        e.preventDefault();
        if (historyIndex !== -1) {
          const newIndex = historyIndex + 1;
          if (newIndex >= commandHistory.length) {
            setHistoryIndex(-1);
            setInput('');
            setCursorPosition(0);
          } else {
            setHistoryIndex(newIndex);
            setInput(commandHistory[newIndex]);
            setCursorPosition(commandHistory[newIndex].length);
          }
        }
        break;
      
      case 'ArrowLeft':
        e.preventDefault();
        setCursorPosition(prev => Math.max(0, prev - 1));
        break;
      
      case 'ArrowRight':
        e.preventDefault();
        setCursorPosition(prev => Math.min(input.length, prev + 1));
        break;
      
      case 'Home':
        e.preventDefault();
        setCursorPosition(0);
        break;
      
      case 'End':
        e.preventDefault();
        setCursorPosition(input.length);
        break;
      
      case 'Backspace':
        e.preventDefault();
        if (cursorPosition > 0) {
          const newInput = input.slice(0, cursorPosition - 1) + input.slice(cursorPosition);
          setInput(newInput);
          setCursorPosition(cursorPosition - 1);
        }
        break;
      
      case 'Delete':
        e.preventDefault();
        if (cursorPosition < input.length) {
          const newInput = input.slice(0, cursorPosition) + input.slice(cursorPosition + 1);
          setInput(newInput);
        }
        break;
      
      case 'Tab':
        e.preventDefault();
        // TODO: Implement tab completion
        break;
      
      case 'c':
        if (e.metaKey || e.ctrlKey) {
          e.preventDefault();
          // TODO: Implement copy
        }
        break;
      
      case 'v':
        if (e.metaKey || e.ctrlKey) {
          e.preventDefault();
          // TODO: Implement paste
        }
        break;
      
      default:
        if (e.key.length === 1 && !e.metaKey && !e.ctrlKey && !e.altKey) {
          e.preventDefault();
          const newInput = input.slice(0, cursorPosition) + e.key + input.slice(cursorPosition);
          setInput(newInput);
          setCursorPosition(cursorPosition + 1);
        }
        break;
    }
  };

  const handleClick = () => {
    if (terminalRef.current) {
      terminalRef.current.focus();
    }
  };

  const formatOutput = (output: string) => {
    return output.split('\n').map((line, index) => (
      <div key={index} className="terminal-line">
        {line}
      </div>
    ));
  };

  const renderCursor = () => {
    if (input.length === 0) {
      return <span className="cursor">█</span>;
    }
    
    const beforeCursor = input.slice(0, cursorPosition);
    const afterCursor = input.slice(cursorPosition);
    
    return (
      <>
        {beforeCursor}
        <span className="cursor">█</span>
        {afterCursor}
      </>
    );
  };

  return (
    <div 
      ref={terminalRef}
      className="terminal flex-1 p-4 overflow-y-auto focus:outline-none"
      tabIndex={0}
      onKeyDown={handleKeyDown}
      onClick={handleClick}
    >
      {/* Terminal output */}
      <div className="terminal-output mb-2">
        {formatOutput(terminal.output)}
      </div>
      
      {/* Current working directory */}
      <div className="terminal-prompt mb-2">
        <span className="text-primary-color">user@host</span>
        <span className="text-secondary-color">:</span>
        <span className="text-accent-color">{terminal.working_directory}</span>
        <span className="text-secondary-color">$</span>
      </div>
      
      {/* Input line */}
      <div className="terminal-input">
        {renderCursor()}
      </div>
      
      <style>{`
        .terminal {
          font-family: 'JetBrainsMono Nerd Font', Monaco, Consolas, monospace;
          font-size: 14px;
          line-height: 1.4;
          background-color: transparent;
          color: var(--fg-color);
        }
        
        .terminal-line {
          white-space: pre-wrap;
          word-break: break-all;
        }
        
        .terminal-prompt {
          font-weight: bold;
        }
        
        .terminal-input {
          display: inline-block;
          min-width: 1ch;
        }
        
        .cursor {
          background-color: var(--fg-color);
          color: var(--bg-color);
          animation: blink 1s infinite;
        }
        
        @keyframes blink {
          0%, 50% { opacity: 1; }
          51%, 100% { opacity: 0; }
        }
        
        .terminal:focus .cursor {
          animation: blink 1s infinite;
        }
      `}</style>
    </div>
  );
};