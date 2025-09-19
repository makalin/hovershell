import React from 'react';

interface ThemePreviewProps {
  themeName: string;
  displayName: string;
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
  onSelect: (themeName: string) => void;
  isSelected: boolean;
}

export const ThemePreview: React.FC<ThemePreviewProps> = ({
  themeName,
  displayName,
  description,
  colors,
  onSelect,
  isSelected,
}) => {
  return (
    <div
      className={`theme-preview p-4 rounded-lg border-2 cursor-pointer transition-all ${
        isSelected ? 'border-primary-color ring-2 ring-primary-color' : 'border-gray-300 hover:border-gray-400'
      }`}
      onClick={() => onSelect(themeName)}
      style={{
        backgroundColor: colors.background,
        color: colors.foreground,
        borderColor: colors.border,
      }}
    >
      <div className="flex items-center justify-between mb-3">
        <h3 className="text-lg font-semibold" style={{ color: colors.foreground }}>
          {displayName}
        </h3>
        {isSelected && (
          <div className="w-4 h-4 rounded-full" style={{ backgroundColor: colors.primary }} />
        )}
      </div>
      
      <p className="text-sm mb-4 opacity-75" style={{ color: colors.foreground }}>
        {description}
      </p>
      
      {/* Color palette preview */}
      <div className="grid grid-cols-4 gap-2 mb-4">
        <div
          className="w-8 h-8 rounded border"
          style={{ backgroundColor: colors.primary }}
          title="Primary"
        />
        <div
          className="w-8 h-8 rounded border"
          style={{ backgroundColor: colors.secondary }}
          title="Secondary"
        />
        <div
          className="w-8 h-8 rounded border"
          style={{ backgroundColor: colors.accent }}
          title="Accent"
        />
        <div
          className="w-8 h-8 rounded border"
          style={{ backgroundColor: colors.success }}
          title="Success"
        />
        <div
          className="w-8 h-8 rounded border"
          style={{ backgroundColor: colors.warning }}
          title="Warning"
        />
        <div
          className="w-8 h-8 rounded border"
          style={{ backgroundColor: colors.error }}
          title="Error"
        />
        <div
          className="w-8 h-8 rounded border"
          style={{ backgroundColor: colors.border }}
          title="Border"
        />
        <div
          className="w-8 h-8 rounded border"
          style={{ backgroundColor: colors.foreground }}
          title="Foreground"
        />
      </div>
      
      {/* Terminal preview */}
      <div
        className="terminal-preview p-3 rounded font-mono text-xs"
        style={{
          backgroundColor: colors.background,
          border: `1px solid ${colors.border}`,
        }}
      >
        <div style={{ color: colors.primary }}>$ hovershell --theme {themeName}</div>
        <div style={{ color: colors.success }}>✓ Theme applied successfully</div>
        <div style={{ color: colors.accent }}>🎨 {displayName} theme active</div>
      </div>
    </div>
  );
};