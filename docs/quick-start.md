# Quick Start Guide

Get up and running with HoverShell in just a few minutes!

## Launch HoverShell

1. **Open HoverShell** from Applications or Spotlight
2. **Look for the icon** in your menu bar (terminal icon)
3. **Click the icon** to see the menu

## Basic Usage

### Opening the Terminal

**Method 1: Hotkey (Default)**
- Press `⌥` (Option) + `` ` `` (backtick) to toggle the terminal

**Method 2: Edge Reveal**
- Move your mouse to the top edge of the screen
- Hold for 500ms to reveal the terminal

**Method 3: Menu Bar**
- Click the HoverShell icon in the menu bar
- Select "Show HoverShell"

### Closing the Terminal

- Press `⌥` + `` ` `` again to hide
- Press `Esc` for quick hide
- Click outside the terminal window

## First Configuration

### Access Settings

1. **Right-click** the menu bar icon
2. **Select "Settings"** or press `⌘,`

### Basic Settings

**UI Configuration:**
- **Position**: Choose where the terminal appears (top, bottom, left, right)
- **Height**: Set the terminal height (e.g., "45vh" for 45% of viewport height)
- **Theme**: Select from built-in themes (Tokyo Night, Dracula, Light)

**Hotkeys:**
- **Toggle**: Change the main hotkey (default: `⌥` + `` ` ``)
- **Quick Hide**: Set quick hide key (default: `Esc`)

## Adding Your First AI Provider

### OpenAI (Recommended for beginners)

1. **Open Settings** (`⌘,`)
2. **Go to "AI Providers"** tab
3. **Click "Add Provider"**
4. **Select "OpenAI"**
5. **Enter your API key** (get one from [OpenAI](https://platform.openai.com/api-keys))
6. **Choose a model** (e.g., "gpt-3.5-turbo")
7. **Click "Save"**

### Test AI Integration

1. **Open the terminal** (`⌥` + `` ` ``)
2. **Type**: `ai chat "Hello, how are you?"`
3. **Press Enter** to see the AI response

## Essential Commands

### Terminal Commands

```bash
# Create new terminal tab
Cmd+T

# Close current tab
Cmd+W

# Switch between tabs
Cmd+Shift+]  # Next tab
Cmd+Shift+[  # Previous tab

# Clear terminal
Cmd+K
```

### AI Commands

```bash
# Chat with AI
ai chat "Your question here"

# Explain code
ai explain "function example() { return 'hello'; }"

# Generate code
ai generate "Create a React component for a button"

# Use specific provider
ai chat "Hello" --provider openai
```

### File Operations

```bash
# List files with enhanced view
ls+

# Git status with visual indicators
git+

# Open file manager
files
```

## Workspace Management

### Auto-Detection

HoverShell automatically detects:
- **Git repositories** and shows branch info
- **Project types** (Node.js, Python, Rust, etc.)
- **Configuration files** (.hovershell.json)

### Manual Configuration

Create a `.hovershell.json` file in your project root:

```json
{
  "name": "My Project",
  "ai": {
    "defaultProvider": "openai",
    "context": "This is a React project with TypeScript"
  },
  "terminal": {
    "workingDirectory": "./src",
    "environment": {
      "NODE_ENV": "development"
    }
  }
}
```

## Quick Tips

### 🚀 **Performance**
- Use `Cmd+K` to clear terminal output
- Close unused terminal tabs to save memory
- Enable hardware acceleration in settings

### 🎨 **Customization**
- Try different themes in Settings > Appearance
- Adjust blur and opacity for your preference
- Customize hotkeys to your workflow

### 🤖 **AI Features**
- Use `ai explain` to understand complex code
- Try `ai generate` for boilerplate code
- Set up multiple providers for different use cases

### 🔧 **Troubleshooting**
- Check the status bar for connection status
- Use `Cmd+Shift+I` to open developer tools
- Check logs in `~/.hovershell/logs/`

## Next Steps

### Explore Advanced Features

1. **Plugins**: Browse and install plugins from the plugin manager
2. **Themes**: Create custom themes or download community themes
3. **Hotkeys**: Set up custom hotkeys for your workflow
4. **Providers**: Add more AI providers (Anthropic, Ollama, etc.)

### Learn More

- [Configuration Guide](configuration.md) - Detailed configuration options
- [AI Integration](user-guide/ai-integration.md) - Advanced AI features
- [Plugin Development](developer-guide/plugin-development.md) - Create your own plugins
- [Troubleshooting](advanced/troubleshooting.md) - Common issues and solutions

## Keyboard Shortcuts Reference

| Shortcut | Action |
|----------|--------|
| `⌥` + `` ` `` | Toggle terminal |
| `Esc` | Quick hide |
| `Cmd+T` | New terminal tab |
| `Cmd+W` | Close tab |
| `Cmd+Shift+]` | Next tab |
| `Cmd+Shift+[` | Previous tab |
| `Cmd+K` | Clear terminal |
| `Cmd+,` | Open settings |
| `Cmd+K` | Command palette |

## Getting Help

- **Documentation**: Browse the full documentation
- **GitHub Issues**: Report bugs or request features
- **Discord**: Join our community for real-time help
- **Examples**: Check out configuration and plugin examples

---

*Congratulations! You're now ready to use HoverShell. Check out the [Configuration Guide](configuration.md) to customize your setup further.*