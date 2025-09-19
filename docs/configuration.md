# Configuration

HoverShell uses YAML configuration files to customize behavior, appearance, and functionality.

## Configuration Location

- **Main Config**: `~/.hovershell/config.yaml`
- **Workspace Config**: `./.hovershell.json` (per-project)
- **Plugin Configs**: `~/.hovershell/plugins/`

## Configuration Structure

```yaml
# UI Configuration
ui:
  position: "top"              # Terminal position: top, bottom, left, right
  height: "45vh"              # Terminal height (CSS units)
  blur: 18                    # Background blur amount (0-50)
  opacity: 0.92              # Window opacity (0.0-1.0)
  font: "JetBrainsMono Nerd Font"  # Font family
  theme: "tokyo-night"       # Theme name
  font_size: 14              # Font size in pixels
  line_height: 1.4           # Line height multiplier
  padding: 16                # Terminal padding
  border_radius: 8           # Corner radius
  shadow: true               # Enable drop shadow
  animations: true            # Enable animations

# Trigger Configuration
triggers:
  hotkeys:
    toggle: "alt+`"          # Main toggle hotkey
    paste_run: "cmd+enter"   # Paste and run
    quick_hide: "esc"        # Quick hide
    new_tab: "cmd+t"         # New terminal tab
    close_tab: "cmd+w"       # Close tab
    next_tab: "cmd+shift+]"  # Next tab
    prev_tab: "cmd+shift+["  # Previous tab
  edges:
    reveal: true             # Enable edge reveal
    dwell_ms: 450            # Edge dwell time
    scroll_resize: true      # Scroll to resize
    sensitivity: 1.0         # Edge sensitivity
  wheel_reveal: true         # Enable wheel reveal
  menu_bar_click: true       # Enable menu bar click

# AI Providers
providers:
  - id: "openai-main"
    name: "OpenAI GPT-4"
    provider_type: "openai"
    base_url: "https://api.openai.com/v1"
    model: "gpt-4"
    api_key: "${OPENAI_API_KEY}"  # Environment variable
    default: true
    enabled: true
    config:
      temperature: 0.7
      max_tokens: 2000
      timeout: 30000

  - id: "ollama-local"
    name: "Ollama Local"
    provider_type: "ollama"
    base_url: "http://127.0.0.1:11434"
    model: "llama3.1:8b"
    default: false
    enabled: true
    config:
      timeout: 60000

# Terminal Configuration
terminal:
  shell: "/bin/zsh"          # Default shell
  working_directory: null     # Default working directory
  environment:               # Environment variables
    EDITOR: "code"
    PAGER: "less"
  scrollback_lines: 10000    # Scrollback buffer size
  cursor_blink: true         # Cursor blinking
  cursor_style: "block"      # Cursor style: block, underline, bar
  bell_sound: true           # Terminal bell sound
  auto_close: false          # Auto-close on exit

# Plugin Configuration
plugins:
  file-manager:
    enabled: true
    auto_load: true
    config:
      show_hidden: false
      default_view: "list"
      sort_by: "name"

  git-integration:
    enabled: true
    auto_load: true
    config:
      show_branch: true
      show_status: true
      auto_fetch: false

# Workspace Rules
workspace_rules:
  - name: "Node.js Projects"
    pattern: "**/package.json"
    profile: "nodejs"
    auto_switch: true
  - name: "Python Projects"
    pattern: "**/requirements.txt"
    profile: "python"
    auto_switch: true
  - name: "Rust Projects"
    pattern: "**/Cargo.toml"
    profile: "rust"
    auto_switch: true

# Security Configuration
security:
  keychain_storage: true     # Store secrets in keychain
  sandbox_providers: true    # Sandbox AI providers
  minimal_scopes: true       # Use minimal permissions
  auto_lock: false          # Auto-lock after inactivity
  lock_timeout: 300         # Lock timeout in seconds
```

## UI Configuration

### Position Options

```yaml
ui:
  position: "top"    # Terminal drops down from top
  # position: "bottom"  # Terminal slides up from bottom
  # position: "left"    # Terminal slides in from left
  # position: "right"   # Terminal slides in from right
```

### Size Configuration

```yaml
ui:
  height: "45vh"           # 45% of viewport height
  # height: "600px"        # Fixed pixel height
  # height: "50%"          # Percentage of screen height
  # height: "auto"         # Auto-size based on content
```

### Visual Effects

```yaml
ui:
  blur: 18                 # Background blur (0-50)
  opacity: 0.92           # Window opacity (0.0-1.0)
  shadow: true            # Drop shadow
  border_radius: 8        # Corner radius
  animations: true        # Smooth animations
```

### Typography

```yaml
ui:
  font: "JetBrainsMono Nerd Font"  # Font family
  font_size: 14           # Font size in pixels
  line_height: 1.4       # Line height multiplier
  padding: 16            # Terminal padding
```

## Hotkey Configuration

### Custom Hotkeys

```yaml
triggers:
  hotkeys:
    toggle: "alt+`"       # Option + Backtick
    # toggle: "cmd+space"  # Command + Space
    # toggle: "ctrl+`"    # Control + Backtick
    
    paste_run: "cmd+enter"  # Command + Enter
    quick_hide: "esc"       # Escape key
    new_tab: "cmd+t"        # Command + T
    close_tab: "cmd+w"      # Command + W
```

### Hotkey Format

Hotkeys use the following format:
- `cmd` or `ctrl` - Command/Control key
- `alt` or `option` - Option/Alt key
- `shift` - Shift key
- `meta` or `super` - Meta/Super key
- `+` - Separator between keys
- Special keys: `space`, `enter`, `tab`, `escape`, `backspace`, `delete`, `up`, `down`, `left`, `right`, `f1`-`f12`

Examples:
- `"cmd+shift+p"` - Command + Shift + P
- `"alt+f4"` - Option + F4
- `"ctrl+alt+t"` - Control + Option + T

## AI Provider Configuration

### OpenAI Provider

```yaml
providers:
  - id: "openai-gpt4"
    name: "OpenAI GPT-4"
    provider_type: "openai"
    base_url: "https://api.openai.com/v1"
    model: "gpt-4"
    api_key: "${OPENAI_API_KEY}"  # From environment
    default: true
    enabled: true
    config:
      temperature: 0.7
      max_tokens: 2000
      timeout: 30000
      retries: 3
```

### Anthropic Provider

```yaml
providers:
  - id: "anthropic-claude"
    name: "Anthropic Claude"
    provider_type: "anthropic"
    base_url: "https://api.anthropic.com"
    model: "claude-3-sonnet-20240229"
    api_key: "${ANTHROPIC_API_KEY}"
    default: false
    enabled: true
    config:
      max_tokens: 2000
      temperature: 0.7
```

### Ollama Provider (Local)

```yaml
providers:
  - id: "ollama-local"
    name: "Ollama Local"
    provider_type: "ollama"
    base_url: "http://127.0.0.1:11434"
    model: "llama3.1:8b"
    default: false
    enabled: true
    config:
      timeout: 60000
      stream: true
```

### Cohere Provider

```yaml
providers:
  - id: "cohere-command"
    name: "Cohere Command"
    provider_type: "cohere"
    base_url: "https://api.cohere.ai"
    model: "command"
    api_key: "${COHERE_API_KEY}"
    default: false
    enabled: true
    config:
      temperature: 0.7
      max_tokens: 2000
```

## Terminal Configuration

### Shell Configuration

```yaml
terminal:
  shell: "/bin/zsh"        # Default shell
  working_directory: null   # Start in home directory
  environment:             # Environment variables
    EDITOR: "code"
    PAGER: "less"
    LANG: "en_US.UTF-8"
  scrollback_lines: 10000  # Scrollback buffer
  cursor_blink: true       # Blinking cursor
  cursor_style: "block"    # Cursor appearance
  bell_sound: true        # Terminal bell
  auto_close: false       # Close on exit
```

### Environment Variables

```yaml
terminal:
  environment:
    # Editor settings
    EDITOR: "code"
    VISUAL: "code"
    
    # Pager settings
    PAGER: "less"
    LESS: "-R"
    
    # Language settings
    LANG: "en_US.UTF-8"
    LC_ALL: "en_US.UTF-8"
    
    # Custom variables
    HOVERSHELL_THEME: "tokyo-night"
    CUSTOM_VAR: "value"
```

## Plugin Configuration

### Enable/Disable Plugins

```yaml
plugins:
  file-manager:
    enabled: true
    auto_load: true
    config:
      show_hidden: false
      default_view: "list"
      sort_by: "name"
      
  git-integration:
    enabled: true
    auto_load: true
    config:
      show_branch: true
      show_status: true
      auto_fetch: false
      
  database-manager:
    enabled: false
    auto_load: false
    config:
      connections: []
```

### Plugin-Specific Settings

Each plugin can have its own configuration section:

```yaml
plugins:
  file-manager:
    config:
      show_hidden: false
      default_view: "list"      # list, grid, tree
      sort_by: "name"          # name, size, modified, type
      group_by: "type"         # type, size, modified
      show_size: true
      show_permissions: false
      
  git-integration:
    config:
      show_branch: true
      show_status: true
      auto_fetch: false
      fetch_interval: 300      # seconds
      show_commit_count: true
      show_file_changes: true
```

## Workspace Rules

### Auto-Detection Rules

```yaml
workspace_rules:
  - name: "Node.js Projects"
    pattern: "**/package.json"
    profile: "nodejs"
    auto_switch: true
    
  - name: "Python Projects"
    pattern: "**/requirements.txt"
    profile: "python"
    auto_switch: true
    
  - name: "Rust Projects"
    pattern: "**/Cargo.toml"
    profile: "rust"
    auto_switch: true
    
  - name: "Go Projects"
    pattern: "**/go.mod"
    profile: "go"
    auto_switch: true
```

### Pattern Matching

Patterns use glob syntax:
- `**` - Match any directory depth
- `*` - Match any characters except `/`
- `?` - Match single character
- `[abc]` - Match any character in brackets
- `{a,b}` - Match either a or b

Examples:
- `"**/package.json"` - Any package.json file
- `"src/**/*.ts"` - All TypeScript files in src
- `"*.{js,ts,jsx,tsx}"` - JavaScript/TypeScript files
- `"**/node_modules/**"` - All node_modules directories

## Security Configuration

### Keychain Storage

```yaml
security:
  keychain_storage: true     # Store API keys in keychain
  sandbox_providers: true    # Sandbox AI providers
  minimal_scopes: true       # Use minimal permissions
  auto_lock: false          # Auto-lock after inactivity
  lock_timeout: 300         # Lock timeout in seconds
```

### Environment Variables

Use environment variables for sensitive data:

```yaml
providers:
  - id: "openai-secure"
    api_key: "${OPENAI_API_KEY}"      # From environment
    # api_key: "sk-..."               # Don't hardcode!
```

Set environment variables in your shell profile:

```bash
# ~/.zshrc or ~/.bash_profile
export OPENAI_API_KEY="sk-your-key-here"
export ANTHROPIC_API_KEY="sk-ant-your-key-here"
export COHERE_API_KEY="your-key-here"
```

## Workspace-Specific Configuration

Create `.hovershell.json` in your project root:

```json
{
  "name": "My React Project",
  "description": "A React application with TypeScript",
  "ai": {
    "defaultProvider": "openai-gpt4",
    "context": "This is a React project using TypeScript, styled-components, and React Query. The codebase follows modern React patterns with hooks and functional components.",
    "providers": {
      "openai-gpt4": {
        "temperature": 0.3,
        "max_tokens": 1500
      }
    }
  },
  "terminal": {
    "workingDirectory": "./src",
    "environment": {
      "NODE_ENV": "development",
      "REACT_APP_API_URL": "http://localhost:3001"
    }
  },
  "plugins": {
    "git-integration": {
      "auto_fetch": true,
      "show_file_changes": true
    }
  }
}
```

## Configuration Management

### Reload Configuration

```bash
# Reload config without restart
hovershell config reload

# Validate configuration
hovershell config validate

# Export current configuration
hovershell config export ~/hovershell-backup.yaml

# Import configuration
hovershell config import ~/hovershell-backup.yaml
```

### Configuration Validation

HoverShell validates your configuration on startup. Common issues:

- **Invalid hotkey format**: Use proper hotkey syntax
- **Missing required fields**: Check provider configuration
- **Invalid file paths**: Ensure paths exist and are accessible
- **YAML syntax errors**: Use a YAML validator

### Backup and Restore

```bash
# Backup configuration
cp ~/.hovershell/config.yaml ~/.hovershell/config.yaml.backup

# Restore from backup
cp ~/.hovershell/config.yaml.backup ~/.hovershell/config.yaml
```

## Advanced Configuration

### Custom Themes

```yaml
ui:
  theme: "custom-dark"
  custom_colors:
    background: "#1a1b26"
    foreground: "#a9b1d6"
    primary: "#7aa2f7"
    secondary: "#9ece6a"
    accent: "#ff9e64"
    success: "#9ece6a"
    warning: "#e0af68"
    error: "#f7768e"
    border: "#565f89"
```

### Custom Fonts

```yaml
ui:
  font: "Fira Code Nerd Font"
  font_size: 16
  line_height: 1.5
  font_features:
    ligatures: true
    monospace: true
```

### Performance Tuning

```yaml
performance:
  scrollback_lines: 5000     # Reduce for better performance
  animation_duration: 200    # Faster animations
  blur_amount: 10            # Less blur for better performance
  update_interval: 100       # UI update interval in ms
```

---

*Need help with configuration? Check out the [Configuration Examples](examples/configuration-examples.md) or [Troubleshooting Guide](advanced/troubleshooting.md).*