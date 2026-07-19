# fkcloid_example_net — AI agent notes

FkCloud Share: GPL-3.0 clients (Android, iOS, Desktop) for a self-hosted rmfakecloud server.
API contract: `Docs/api-rmfakecloud.md`. PRDs/roadmaps: `Docs/product/{android,ios,desktop}/`.

## Layout & status

- `core/` — Rust crate `fkcloud-core` (shared HTTP client logic) + `fkcloud-cli` bin (auto-discovered from `core/src/bin/`).
- `apps/android/` — Kotlin/Gradle, **functional**, share-target upload + folder browser + embedded web file manager.
- `apps/desktop/` — Tauri 2 + vanilla JS, **functional** (tray icon, Finder/Explorer context-menu send). Matches `Docs/product/desktop/PRD.md`, which is explicitly Tauri-specific ("Retenu : Tauri 2").
- `apps/ios/` — planning only, no code yet (README describes intended SwiftUI + Share Extension).

**No root `Cargo.toml`** — `core/` and `apps/desktop/src-tauri/` are independent crates. Always `cd` into the crate dir before running `cargo`.

## Commands

```bash
# core (shared Rust logic + CLI)
cd core && cargo build
cd core && cargo test --verbose
cd core && cargo clippy --all-targets -- -D warnings   # needs `rustup component add clippy` on rust:1.80-slim CI image

# desktop (Tauri)
cd apps/desktop && npm install
cd apps/desktop && npm run tauri dev      # dev mode
cd apps/desktop && npm run tauri build -- --debug   # local debug bundle

# android
cd apps/android && ./gradlew assembleDebug
```

## Gotchas

- **git-ai bot**: this repo has automation that auto-commits/pushes on its own schedule (commits authored `git-ai <git-ai@local>`, empty subject lines). A clean `git status` does not mean no recent work happened — check `git log --oneline -10` before assuming.
- **CI clippy job**: only lints `core/` (`cd core && cargo clippy`), not `apps/desktop/src-tauri`. `rust:1.80-slim` image needs the clippy component installed explicitly.
- **Desktop Automator "send" service**: right-click-send on macOS execs the raw app binary with `--upload <path>`, bypassing LaunchServices — without `tauri-plugin-single-instance` this used to spawn a second, blank-window process when the app was already running in the tray. Fixed (confirmed working) by registering the plugin first in `apps/desktop/src-tauri/src/lib.rs`'s builder chain; second launches forward `--upload` via the `upload-file-selected` event instead of opening a new window.
- **Charte graphique**: "Registre libre" is the chosen visual identity for FkCloud Share (warm cream/dark surfaces, blue primary, thin-outline "filet" borders instead of Material3 elevation shadows) — see `Docs/charte-graphique/charte-graphique.md#3-registre-libre`. This differs from the generic "Place de l'Info" palette in `SYSTEM_PROMPT.md`; the FkCloud-specific charter wins for anything under `apps/`.
- **Material3 dialog tint bug** (Android): `MaterialCardView` ignores explicit background color inside `Theme.*.Dialog` windows due to tonal elevation overlay, even at `cardElevation=0dp`. Fix used: plain shape `drawable` instead of `MaterialCardView` in dialog-themed activities.
