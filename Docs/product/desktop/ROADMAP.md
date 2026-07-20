# Roadmap — FkCloud Share Desktop (progressive bisection)

French: [ROADMAP.fr.md](ROADMAP.fr.md).

Axis: **bound A** = nothing → **bound B** = v1.0 signed, distributed for
win-x64, mac-x64, mac-arm64. Minimal step: milestone ≤ 1 week, binary
verification. Each milestone = git tag.

## Bisection passes

- **Pass 1 — bounds**: A (zero desktop code) and B (v1.0 three targets).
- **Pass 2 — midpoint M**: *Tauri window that sends* — drag & drop of a
  PDF → folder → server-verified upload, on the development OS
  (mac-arm64). Splits the space: before = Rust/CLI core, after =
  multi-platform, tray, packaging.
- **Pass 3 — quarters**:
  - A–M: *validation CLI* — `fkcloud-cli` binary (login, ls, put)
    on top of the crate; proves the core without UI. H1 webview tested here
    on all 3 OSes (blank page + drop test).
  - M–B: *three packaged targets* — msi + 2 unsigned dmg, E2E CI green
    everywhere.
- **Pass 4**: tray, queue, signing — gaps ≤ 1 week, stop.

## Milestones (rough order → end)

| Tag | Milestone | Status | Binary verification |
|---|---|---|---|
| — | Bound A | Done | `apps/desktop/`: README only (initial state) |
| `desktop-m1` | Quarter A–M: `fkcloud-cli` | Done | functional `fkcloud-cli` (login, ls, put, sync); core tests ok |
| `desktop-m2` | **Midpoint M**: Tauri window drag & drop | Done | Tauri window D&D, folder tree, inline creation |
| `desktop-m3` | Quarter M–B: queue & tray icon | Done | async unit queue (F3), systray icon, menu, native dialogs (F4) |
| `desktop-v1.0` | Bound B: CI & publication | Done | settings (auto-start, context menu), active tray icon glow, About signing, CI & publication |

## Risk segments

Breakage at `m3` → test `m2` (midpoint) first; faulty half in one trial.
The `desktop-m1` CLI remains in the repo permanently: it is the fastest
bisection tool to distinguish « crate bug » from « UI bug ».

## After B (ordered backlog by version)

### Milestone v1.1: Integrations & Improvements
1. **KeePassXC integration**: Option to store/retrieve credentials via KeePassXC local protocol (or D-Bus Secret Service integration on Linux).
2. **Library view**: If PRD H2 is confirmed by user feedback.
3. **Signed auto-update** (tauri-updater).
4. **Official Linux target** (AppImage/Flatpak) — with full QA commitment.

### Milestone v1.2: Global System Integration
1. **Send via Right-Click (OS Context Menu)**: Integration into the OS file manager's native context menu (Windows Explorer, macOS Finder, Linux file managers such as Nautilus/Dolphin) to send a PDF/EPUB directly via right-click without opening the main interface.
2. **`GET /ui/api/sync` post-upload**: Immediate sync notification sent to the tablet.
