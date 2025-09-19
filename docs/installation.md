# Installation

This guide will help you install HoverShell on your macOS system.

## System Requirements

- **macOS**: 12.0 (Monterey) or later
- **Architecture**: Intel (x86_64) or Apple Silicon (aarch64)
- **Memory**: 4GB RAM minimum, 8GB recommended
- **Storage**: 100MB free space

## Installation Methods

### Method 1: Homebrew (Recommended)

```bash
# Add the HoverShell tap
brew tap hovershell/tap

# Install HoverShell
brew install hovershell
```

### Method 2: Direct Download

1. Go to the [Releases page](https://github.com/makalin/hovershell/releases)
2. Download the latest `.dmg` file for your architecture
3. Open the downloaded `.dmg` file
4. Drag HoverShell to your Applications folder
5. Launch HoverShell from Applications

### Method 3: Build from Source

If you want to build HoverShell from source or contribute to development:

#### Prerequisites

- **Xcode Command Line Tools**: `xcode-select --install`
- **Rust**: Install from [rustup.rs](https://rustup.rs/)
- **Node.js**: Version 18 or later
- **pnpm**: `npm install -g pnpm`

#### Build Steps

```bash
# Clone the repository
git clone https://github.com/makalin/hovershell.git
cd hovershell

# Add Rust targets for macOS
rustup target add x86_64-apple-darwin aarch64-apple-darwin

# Install dependencies
pnpm install

# Build the application
pnpm tauri build
```

The built application will be in `src-tauri/target/release/bundle/macos/`.

## First Launch

1. **Launch HoverShell** from Applications or Spotlight
2. **Grant Permissions** when prompted:
   - Accessibility permissions (for global hotkeys)
   - Screen recording permissions (for edge detection)
   - Network permissions (for AI providers)

3. **Initial Setup**:
   - HoverShell will create its configuration directory at `~/.hovershell/`
   - A default configuration file will be created
   - The app icon will appear in your menu bar

## Verification

To verify your installation:

1. **Check Menu Bar**: Look for the HoverShell icon in your menu bar
2. **Test Hotkey**: Press `⌥` (Option) + `` ` `` (backtick) to toggle the terminal
3. **Test Edge Reveal**: Move your mouse to the top edge of the screen and hold for 500ms
4. **Check Version**: Right-click the menu bar icon and select "About"

## Troubleshooting

### Common Issues

#### "HoverShell cannot be opened because it is from an unidentified developer"

**Solution**: 
1. Right-click the HoverShell app in Applications
2. Select "Open" from the context menu
3. Click "Open" in the dialog that appears
4. Alternatively, go to System Preferences > Security & Privacy > General and click "Open Anyway"

#### Hotkeys Not Working

**Solution**:
1. Go to System Preferences > Security & Privacy > Privacy > Accessibility
2. Add HoverShell to the list of allowed applications
3. Restart HoverShell

#### Edge Detection Not Working

**Solution**:
1. Go to System Preferences > Security & Privacy > Privacy > Screen Recording
2. Add HoverShell to the list of allowed applications
3. Restart HoverShell

#### App Won't Launch

**Solution**:
1. Check Console.app for error messages
2. Try launching from Terminal: `/Applications/HoverShell.app/Contents/MacOS/HoverShell`
3. Check if you have the required macOS version
4. Verify you have sufficient disk space

### Getting Help

If you encounter issues:

1. **Check the Logs**: HoverShell logs are stored in `~/.hovershell/logs/`
2. **GitHub Issues**: Report bugs at [github.com/makalin/hovershell/issues](https://github.com/makalin/hovershell/issues)
3. **Discord**: Join our Discord server for real-time help
4. **Documentation**: Check the [Troubleshooting Guide](advanced/troubleshooting.md)

## Uninstallation

### Homebrew Installation

```bash
brew uninstall hovershell
brew untap hovershell/tap
```

### Manual Installation

1. Quit HoverShell if it's running
2. Delete the app from Applications folder
3. Remove configuration files (optional):
   ```bash
   rm -rf ~/.hovershell
   ```

## Next Steps

After installation:

1. [Configure HoverShell](configuration.md) to your preferences
2. [Set up AI providers](user-guide/ai-integration.md) for enhanced functionality
3. [Explore plugins](developer-guide/plugin-development.md) to extend capabilities
4. [Customize themes](user-guide/themes.md) for your preferred look

---

*Ready to get started? Check out the [Quick Start Guide](quick-start.md) next!*