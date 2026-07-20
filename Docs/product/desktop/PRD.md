# PRD — FkCloud Share Desktop (v1, Rust base / Tauri)

French: [PRD.fr.md](PRD.fr.md).

Method: Mertonian ethos. Core: [core-rust.md](../core-rust.md) ·
API: [api-rmfakecloud.md](../../api-rmfakecloud.md)

## 1. Universal context and beneficiaries

On desktop, documents to send to the tablet (PDF articles,
EPUB books, research papers) are produced or downloaded in a
browser or file manager. Target gesture: drag-and-drop
onto a window or icon, choose a folder, done. Beneficiaries:
the same self-hosters, on Windows 10/11 x64, macOS Intel (x86_64) and
macOS Apple Silicon (arm64). Linux treated as a compile target from the
start (marginal cost with Tauri) but out of v1 commitment.

## 2. Impersonal criteria and hypotheses

| # | Criterion | Observable metric |
|---|---|---|
| C1 | Portability | same features and same tests on win-x64, mac-x64, mac-arm64; no OS-conditional logic except secret storage and packaging |
| C2 | Gesture | drag & drop → confirmation ≤ 3 interactions; visible queue |
| C3 | Footprint | binary < 25 MB; idle RAM < 150 MB (excludes Electron, ~10× above) |
| C4 | Security | secrets via native vault (Keychain / Credential Manager-DPAPI / Secret Service); HTTPS by default, HTTP opt-in |
| C5 | Verifiability | automated E2E (WebDriver tauri-driver) against rmfakecloud Docker on all 3 targets in CI |

Falsifiable hypotheses:
- **H1**: the system webview (WebView2 / WKWebView) is sufficient — no blocking
  rendering divergence between OSes. Invalidation: compared screenshots at milestone M2;
  blocking divergence → native `egui` UI (documented alternative).
- **H2**: a desktop user wants to see their library, not just
  send. Not tested in v1 (out of scope); user survey after
  v1.0 — if confirmed, library view becomes the first v1.1 item.

## 3. Audit of the existing

| Solution | Observed limits |
|---|---|
| rmfakecloud web UI (browser) | works, but no global drag & drop, no tray, re-login |
| `rmapi` (Go, CLI) | *device* sync protocol, not the web UI API; code pairing; no GUI |
| Official reMarkable desktop app | third-party server impossible |
| Electron + web code | violates C3 (footprint) — ruled out on criterion, not preference |
| **Selected: Tauri 2** | system webview + Rust backend: `fkcloud-core` crate called without FFI, C3 met |

Structural justification: Tauri is the only candidate where the Rust core is
a direct dependency (zero binding layer), making desktop the
reference platform for validating the crate.

## 4. Requirements

### Functional

| ID | Requirement | Acceptance |
|---|---|---|
| F1 | Drag & drop of PDF/EPUB (multiple) onto the window | drop → folder dialog → upload; motivated rejection of other types |
| F2 | Folder choice + folder creation | server tree; `POST folders` verified server-side |
| F3 | Upload queue | progress per file; failure does not stop the queue; per-item retry |
| F4 | System tray icon | drop on icon or « Send a file… » menu; closing window ≠ quit |
| F5 | Configuration | URL + credentials + connection test; distinct errors 401/network/forbidden HTTP |
| F6 | Persistent session | 23 h token, auto re-login on 401 (crate behavior, identical to mobile) |

### Non-functional

- **S1**: secrets via `keyring` crate (native vaults C4); never in a
  config file nor in logs.
- **S2**: strict Tauri CSP; APIs exposed to webview limited to
  declared commands (allowlist).
- **P1**: streaming upload; 1 GB file without memory overflow.
- **A11y**: full keyboard navigation; AA contrast (visual identity).
- Out of v1 scope: library view (H2), download, auto-update
  (v1.1 — requires signing).

## 5. Planned verification evidence

1. Crate tests (already required by `core-m1`) — not duplicated.
2. E2E `tauri-driver`: drop a PDF → Docker server API assertion;
   run in CI on all 3 targets (win/mac-intel/mac-arm runners).
3. Load test: 10 files in queue, one corrupted → 9 successes,
   1 named failure.
4. C3 measurements published at each tag (binary size, idle RSS).

## 6. Three main risks

1. **macOS signing/notarization and Windows SmartScreen** (high impact,
   certain likelihood, immediate detection) — unsigned binaries =
   blocking warnings for the user. Mitigation: developer certificate
   + notarization from `desktop-m3` (not after); recurring cost
   named (counter-norm: dependency on proprietary authorities to
   distribute free software; compensation: published SHA-256 hashes).
2. **Webview divergences** (medium impact, medium likelihood) — WebView2
   ≠ WKWebView (drag & drop, file API). Mitigation: H1 tested at midpoint M,
   not at the end; egui fallback sized before commitment.
3. **Real mac Intel parity** (medium impact, low likelihood,
   difficult detection without hardware) — arm64 tested on maintainer's
   machine, x86_64 only in CI. Mitigation: CI runs full E2E
   on Intel runner; no « it compiles so it works ».

## 7. Verified sources

- API contract: rmfakecloud source + E2E 2026-07-16 (this repo).
- Tauri 2 (msi/dmg bundler, tauri-driver, CSP): https://v2.tauri.app/ .
- `keyring` crate (Keychain/DPAPI/Secret Service): crates.io/crates/keyring .
- macOS notarization: Apple documentation; SmartScreen: Microsoft
  documentation (general knowledge, procedures to re-verify at milestone m3).
