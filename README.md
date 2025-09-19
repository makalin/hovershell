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

* Split-screen sessions, tmux integration
* Global scratchpad with AI autosummarize
* SSH jump-host profiles
* Context bridges (git diff, file tree, clipboard)
* Brew + DMG releases, auto-updates

---

## License

MIT © 2025 Mehmet T. AKALIN

---

“Hover. Click. Command.”
