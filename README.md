# HoverShell

**A macOS menu-bar terminal that’s always on, edge-activated, fast, AI-aware, and plugin-extensible.**
Quake-style drop-down, global hotkeys, “mouse-to-edge” reveal, colored themes, programmable function keys, and first-class control over multiple terminal AIs (OpenAI Codex, Amazon Q, Ollama, Cursor Agents, etc.).

![demo](docs/demo.gif)

---

## Why HoverShell?

* Keep a terminal **always ready** without cluttering desktops.
* **Gesture or hotkey** to summon; auto-hide when you’re done.
* Treat AIs like shells: route, prompt, and script them uniformly.
* **Plugin everything**: commands, panels, key bindings, providers.

---

## Features

* **Menu-bar resident** app (lives in macOS status bar).
* **Triggers:** global hotkeys, **edge reveal** (push cursor to top/side), wheel-roll at edge, or menu-bar click.
* **Layouts:** drop-down, side-panel, floating tile; per-display awareness.
* **Profiles & Themes:** JSON/YAML themes, opacity/blur, powerline fonts, ligatures.
* **AI Control Hub:** add/remove/list AI providers; route shells to providers; per-project context.
* **Programmable F-keys:** map F1..F12 (and chords) to commands/macros.
* **Command Plugins:** `ls+` (file/folder boards), `git+` (status/branch UI), `db+`, `http+`, `note+`, etc.
* **Quick Actions:** copy path, open in editor, run in background, stash output.
* **Workspace rules:** auto-switch profile by app/folder/repo.
* **Security:** Keychain-stored secrets, provider sandboxes, minimal scopes.
* **Fast:** Rust core + GPU-accelerated renderer; zero-copy scrollback.

### 🛠️ Built-in Tools & Functions

#### **File Operations**
* **File Management:** Copy, move, delete files and directories
* **Search & Find:** Find files by name pattern, search text content across files
* **Directory Operations:** List directories, get statistics, create folders
* **File Content:** Read, write, create files with content management

#### **Git Integration**
* **Repository Status:** View staged/unstaged changes, branch information
* **Branch Management:** Create, checkout, list branches (local & remote)
* **Commit Operations:** Add files, commit changes, view commit history
* **Diff Viewing:** See file changes, staged differences, commit comparisons
* **Remote Operations:** Pull, push with upstream tracking

#### **System Monitoring**
* **Process Management:** List, monitor, kill processes by PID
* **System Resources:** CPU usage, memory consumption, disk space
* **Network Monitoring:** Interface status, active connections
* **Performance Metrics:** Top processes by CPU/memory usage

#### **Text Processing**
* **Pattern Matching:** Grep with regex support, case-insensitive options
* **Text Transformation:** Sort, sed replacements, awk field processing
* **Text Analysis:** Word count, character counting, line operations
* **Text Manipulation:** Case conversion, truncation, trimming, deduplication

#### **Network Tools**
* **Connectivity Testing:** Ping hosts, check reachability
* **Port Scanning:** Scan open ports on remote hosts
* **HTTP Operations:** Make web requests, download files
* **DNS & Routing:** DNS lookups, traceroute network paths
* **Network Discovery:** Find local IP addresses, network interfaces

#### **Database Tools**
* **Multi-Database Support:** PostgreSQL, MySQL, SQLite, MongoDB, Redis, SQL Server
* **Connection Management:** Add, test, remove database connections
* **Query Execution:** Run SQL queries, get structured results
* **Schema Inspection:** View tables, columns, relationships, constraints
* **Database Metadata:** Get database information, table statistics

#### **Docker Integration**
* **Container Management:** Start, stop, remove containers
* **Image Operations:** List, pull, remove Docker images
* **Volume & Network:** Manage Docker volumes and networks
* **Docker Compose:** Up, down, status for compose projects
* **System Monitoring:** Docker system info, resource usage
* **Command Execution:** Run commands inside running containers

#### **Package Management**
* **Multi-Manager Support:** NPM, Yarn, PNPM, Pip, Cargo, Brew, Apt, Yum, Pacman
* **Package Operations:** Install, uninstall, update packages
* **Search & Discovery:** Find packages, get detailed information
* **Project Management:** Initialize new projects with package managers
* **Dependency Management:** Check outdated packages, manage dependencies

---

## Install

### Homebrew (planned)

```bash
brew tap hovershell/tap
brew install hovershell
```

### DMG (planned)

Download from Releases and drag to **Applications**.

### Build from source

**Requirements:** macOS 12+, Xcode CLTs, Rust stable, Node 18+, pnpm

```bash
# Rust core
git clone https://github.com/makalin/hovershell
cd hovershell
rustup target add x86_64-apple-darwin aarch64-apple-darwin
pnpm i
pnpm tauri build
```

---

## Quickstart

* Launch **HoverShell** → icon appears in the menu bar.
* Default triggers:

  * **⌥\`** (Option-Backtick) to toggle
  * **Push mouse to top edge** (500ms) to reveal
  * **Scroll at top edge** to resize height
* **⌘,** to open Settings.

---

## Configuration

`~/.hovershell/config.yaml`

```yaml
ui:
  position: top
  height: 45vh
  blur: 18
  opacity: 0.92
  font: "JetBrainsMono Nerd Font"
  theme: "tokyo-night"

triggers:
  hotkeys:
    toggle: "alt+`"
    paste_run: "cmd+enter"
    quick_hide: "esc"
  edges:
    top:
      reveal: true
      dwell_ms: 450
      scroll_resize: true

providers:
  - id: "ollama-local"
    type: "ollama"
    base_url: "http://127.0.0.1:11434"
    model: "llama3.1:8b"
    default: true
```

---

## Tool Usage Examples

### File Operations
```bash
# List directory contents
file.list /path/to/directory --recursive

# Search for files by pattern
file.find . --pattern "*.rs" --case-sensitive

# Search text content in files
file.search . --query "TODO" --file-pattern "*.rs"

# Copy files and directories
file.copy source.txt destination.txt
file.copy src/ dist/ --recursive
```

### Git Operations
```bash
# Check repository status
git.status /path/to/repo

# List all branches
git.branches /path/to/repo

# View commit history
git.commits /path/to/repo --limit 10

# See file differences
git.diff /path/to/repo --file src/main.rs
```

### System Monitoring
```bash
# List running processes
system.processes --limit 20

# Check disk usage
system.disk

# Monitor network interfaces
system.network

# Get top CPU processes
system.top-cpu --limit 10
```

### Text Processing
```bash
# Search for patterns
text.grep "function" --files ["src/*.rs"] --case-insensitive

# Sort text lines
text.sort input.txt --numeric --reverse

# Count words and lines
text.wc document.txt

# Transform text
text.sed "old" "new" --global
```

### Network Tools
```bash
# Ping a host
network.ping google.com --count 4

# Scan ports
network.scan 192.168.1.1 --ports [22,80,443]

# Download files
network.download https://example.com/file.zip ./file.zip

# Check DNS
network.dns-lookup google.com
```

### Database Operations
```bash
# Add database connection
db.add-connection --type postgresql --host localhost --port 5432 --database mydb

# Execute query
db.query connection-id "SELECT * FROM users LIMIT 10"

# List tables
db.tables connection-id

# Get table schema
db.schema connection-id --table users
```

### Docker Management
```bash
# List containers
docker.containers --all

# Start/stop containers
docker.start container-id
docker.stop container-id

# Pull images
docker.pull nginx:latest

# Run containers
docker.run nginx:latest --detached --port 8080:80
```

### Package Management
```bash
# Check available package managers
package.managers

# Install packages
package.install npm react --global
package.install pip requests

# List installed packages
package.list npm --global

# Search packages
package.search npm "react component"
```

---

## AI Integration

```bash
ai chat "Summarize staged git changes" --provider openai-like --context repo
ai provider add --type ollama --base-url http://127.0.0.1:11434 --model llama3
ai provider set-default amazon-q
```

---

## Plugins

**Example plugin (file lister):**

```ts
import { defineCommand } from "hovershell-sdk";

export default defineCommand({
  name: "files",
  description: "List files and folders with preview",
  async run({ fs, ui }) {
    const items = await fs.readDir(".");
    ui.table(items.map(i => ({ Name: i.name, Type: i.isDir ? "Dir" : "File" })));
  }
});
```

---

## Roadmap

### ✅ Completed Features
* **Comprehensive Tool Suite:** File operations, Git integration, system monitoring, text processing, network tools, database management, Docker integration, package management
* **Multi-Database Support:** PostgreSQL, MySQL, SQLite, MongoDB, Redis, SQL Server
* **Package Manager Integration:** NPM, Yarn, PNPM, Pip, Cargo, Brew, Apt, Yum, Pacman, Snap, Flatpak
* **Advanced Text Processing:** Grep, sed, awk, sort, word count, text manipulation
* **Network Diagnostics:** Ping, port scanning, HTTP requests, DNS lookup, traceroute
* **Docker Management:** Container lifecycle, image operations, compose support
* **System Monitoring:** Process management, resource monitoring, performance metrics

### 🚧 In Development
* Split-screen sessions, tmux integration
* Global scratchpad with AI autosummarize
* SSH jump-host profiles
* Context bridges (git diff, file tree, clipboard)
* Enhanced plugin system with hot-reloading

### 📋 Planned Features
* Brew + DMG releases, auto-updates
* Advanced AI context awareness
* Custom theme editor
* Multi-workspace support
* Performance profiling tools
* Advanced debugging capabilities

---

## License

MIT © 2025 Mehmet T. AKALIN

---

“Hover. Click. Command.”
