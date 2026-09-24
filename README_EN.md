<div align="center">

# CC Switch (Fork)

This fork is based on [farion1231/cc-switch](https://github.com/farion1231/cc-switch) `v3.20.4`, with this fork's personal changes kept.

**This is a personal-use fork focused on practicality.** Some added or modified features have not been fully tested, so **bugs or incompatibilities with upstream may exist**. If you want the safest option, use the [official version](https://github.com/farion1231/cc-switch).

[![Version](https://img.shields.io/badge/version-3.20.4--fork.1-blue.svg)](https://github.com/kongkongyo/cc-switch/releases)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg)](https://github.com/kongkongyo/cc-switch/releases)
[![Built with Tauri](https://img.shields.io/badge/built%20with-Tauri%202-orange.svg)](https://tauri.app/)
[![Downloads](https://img.shields.io/github/downloads/kongkongyo/cc-switch/total)](https://github.com/kongkongyo/cc-switch/releases/latest)

[中文](README.md) | English | [日本語](README_JA.md) | [Deutsch](README_DE.md) | [中文文档](README_ZH.md) | [Changelog](CHANGELOG.md)

</div>

## Differences From Upstream

| Usage | Official | This Fork |
|-------|----------|-----------|
| Viewing current config | Provider cards mainly show the provider name | Shows the current model next to the provider name; Claude role models are shortened when possible |
| Selecting models | Long model lists mostly rely on scrolling | After fetching models, you can search, browse by provider, and see the selected model highlighted |
| Codex regular model | Model field is less visible in non-local-routing config | In non-local-routing config, you can type or fetch the model name and write it into Codex config |
| Provider-specific upstream proxy | Only the global upstream proxy applies under the existing flow | A selected provider/channel can use its own upstream proxy; once enabled, it affects only that provider, whose upstream requests use it, and it takes priority over the global upstream proxy |
| New provider position | Added to the end | Inserted in second position when existing providers exist, making it easier to enable or reorder |
| Provider ordering | Mostly drag-and-drop | Keeps drag-and-drop and adds right-click move to top / move to bottom |
| Tray action | Left click tends to open the tray menu | Left-click tray icon shows / hides the main window |
| Connectivity check | Checks whether the provider address is reachable | Follows upstream's lightweight reachability check, without sending real model requests |
| Update flow | Uses upstream auto-update chain | Checks this fork's Releases, opens the release page, and lets the user download manually |
| Release source | Official repo Releases | Installers are published in this fork's Releases |

> Note: A provider-specific upstream proxy only applies after traffic enters CC Switch's local proxy. If that proxy is invalid or unavailable, requests fail instead of silently falling back to the global upstream proxy.

---

## Main Features

### Current Version: v3.20.4-fork.1

- Supports **Claude Code, Codex, Gemini CLI, OpenCode, and OpenClaw**
- Unified provider import, switching, sorting, duplication, import/export
- Unified MCP / Prompts / Skills / Sessions management
- Local proxy, failover, health checks, provider-specific upstream proxy, and format conversion
- WebDAV and custom config-directory sync support

### Fork Enhancements

- Current model name shown directly on provider cards
- Search models immediately after fetching model list
- New providers inserted in second position by default
- Right-click provider card to move it to top or bottom
- Left-click tray icon toggles main window visibility
- Lightweight connectivity checks
  - checks whether provider base URLs are reachable
  - avoids sending real model requests during checks
  - reduces false failures from auth, model validation, and WAF rules
- Provider-specific upstream proxy for a selected provider/channel; it does not force all providers through the global upstream proxy
- Manual update guidance
  - checks this fork's Releases
  - opens release page for manual update
  - does not rely on Tauri auto-download updater

## Screenshots

|                  Main Interface                   |                  Add Provider                  |
| :-----------------------------------------------: | :--------------------------------------------: |
| ![Main Interface](assets/screenshots/main-en.png) | ![Add Provider](assets/screenshots/add-en.png) |

## Download

### System Requirements

- **Windows**: Windows 10+
- **macOS**: macOS 12 (Monterey)+
- **Linux**: Ubuntu 22.04+ / Debian 11+ / Fedora 34+ and other mainstream distros

### Release Assets

This fork currently publishes:

- **Windows**: `CC-Switch-v{version}-Windows-Portable.zip`
- **macOS**: `CC-Switch-v{version}-macOS.zip`
- **Linux**: `CC-Switch-v{version}-Linux-x86_64.deb`

Download:

- [Fork Releases](https://github.com/kongkongyo/cc-switch/releases)

## Update Flow

- The app checks this fork repository's Releases
- When a new version is found, it prompts users to update manually from the release page
- Tauri auto-download updater is not used

## Quick Start

1. Add a provider from presets or custom config
2. Switch provider from the main window or tray
3. Restart the target CLI when needed
4. Add an official preset if you want to switch back to official login

## Data Location

- **Database**: `~/.cc-switch/cc-switch.db`
- **Local settings**: `~/.cc-switch/settings.json`
- **Backups**: `~/.cc-switch/backups/`
- **Skills**: `~/.cc-switch/skills/`
- **Skill backups**: `~/.cc-switch/skill-backups/`

## Development

### Requirements

- Node.js 18+
- pnpm 8+
- Rust 1.85+
- Tauri CLI 2.8+

### Common Commands

```bash
pnpm install
pnpm dev
pnpm typecheck
pnpm format
pnpm format:check
pnpm test:unit
pnpm build
pnpm tauri build --debug
```

### Rust

```bash
cd src-tauri
cargo fmt
cargo clippy
cargo test
cargo test --features test-hooks
```

## Tech Stack

- **Frontend**: React 18, TypeScript, Vite, TailwindCSS, TanStack Query
- **Backend**: Tauri 2, Rust, serde, tokio, thiserror
- **Testing**: vitest, MSW, @testing-library/react

## Project Structure

```text
├── src/                      # frontend
├── src-tauri/                # Rust backend
├── tests/                    # frontend tests
└── assets/                   # screenshots and resources
```

## Contributing

Issues and suggestions are welcome.

Before opening a PR, please make sure:

- `pnpm typecheck`
- `pnpm format:check`
- `pnpm test:unit`

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=farion1231/cc-switch&type=Date)](https://www.star-history.com/#farion1231/cc-switch&Date)

## License

MIT © Jason Young
