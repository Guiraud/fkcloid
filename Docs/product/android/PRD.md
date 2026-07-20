# PRD — FkCloud Share Android (v2, Rust base)

French: [PRD.fr.md](PRD.fr.md).

Method: Mertonian ethos (verifiable incremental development).
Core: [core-rust.md](../core-rust.md) · API: [api-rmfakecloud.md](../../api-rmfakecloud.md)

## 1. Universal context and beneficiaries

reMarkable tablet owners self-host rmfakecloud to keep
their documents off the commercial cloud. On Android phones, sending a
PDF/EPUB to that server today requires a browser and the web UI — high friction
for a daily gesture. Beneficiaries: self-hosters (F-Droid profile),
with no assumption about their Android distribution (min API 26, no Google services).

**Established fact**: a working Kotlin v1, E2E tested 2026-07-16 (API 36 emulator
↔ rmfakecloud Docker: login, folder tree, upload verified on server).
« Registre libre » visual identity applied and navigable folder picker
(F2), auto-close (F7) and web file manager (F8)
added and E2E tested 2026-07-17/18 (same test conditions).

## 2. Impersonal criteria and hypotheses

| # | Criterion | Observable metric |
|---|---|---|
| C1 | Gesture speed | share → confirmation ≤ 4 interactions; 10 MB upload < 30 s on Wi-Fi |
| C2 | Security | MASVS-STORAGE-1: secrets only via Keystore; HTTPS by default, explicit HTTP opt-in |
| C3 | Compatibility | API 26 → 36; ACTION_SEND and SEND_MULTIPLE; heterogeneous content providers (MediaStore, FileProvider, SAF) |
| C4 | F-Droid acceptability | 0 proprietary dependency, 0 tracker (empty exodus scan), reproducible build |
| C5 | Accessibility | WCAG AA contrast, touch targets ≥ 48 dp, usable TalkBack |

Falsifiable hypotheses:
- **H1**: users prefer the share menu to opening an app.
  Invalidation: if > 30% of feedback requests an integrated file picker,
  add a « choose a file » screen (v2.1).
- **H2**: the Rust core (UniFFI) brings no functional regression nor
  perceptible latency vs Kotlin. Invalidation: comparative E2E suite;
  if failure, keep the Kotlin client (core H1 falls for Android).

## 3. Audit of the existing

| Solution | Observed limits |
|---|---|
| rmfakecloud web UI on mobile | not in the share menu; frequent re-login; no folder choice at share time |
| Official reMarkable app | refuses a third-party server; proprietary; not on F-Droid |
| Kotlin v1 in this repo | functional but protocol logic not shared with iOS/desktop (drift already observed on extension handling) |

New, incremental contribution: v2 = same UX as v1, protocol logic
moved to `fkcloud-core` (Rust), plus folder creation and monochrome
icon. No UI rewrite.

## 4. Requirements

### Functional (binary acceptance criterion for each)

| ID | Requirement | Acceptance |
|---|---|---|
| F1 | PDF/EPUB share target (single and multiple) | SEND/SEND_MULTIPLE intent → dialog opens with correct names, including opaque name → extension deduced from MIME |
| F2 | Destination folder choice | navigable picker (breadcrumb, tap to enter subfolder, documents shown as non-clickable context); selection applied (`parent` API verified) — implemented (`FolderPickerActivity`), replaces the old flat list |
| F3 | Folder creation from the dialog | `POST /ui/api/folders`; new folder visible and selected — **not implemented**, remains to be done |
| F4 | Persistent session | 23 h token; auto re-login on 401; zero password re-entry |
| F5 | Configuration screen | URL + credentials + connection test; distinct success/failure states (401 vs network vs forbidden HTTP) |
| F6 | Actionable error report | each upload failure names the file and cause; retry button |
| F7 | Auto-close after success | dialog shows confirmation ~1.4 s then closes on its own; « Close » button remains available for immediate dismiss |
| F8 | Web file manager | dedicated button opens a WebView pointing to the configured server (HTTPS/HTTP policy identical to F5); navigation limited to configured domain, external links opened in system browser; `zoom: 50%` CSS injected post-load because rmfakecloud nav does not adapt to narrow screens (`setInitialScale`/`zoomBy` ineffective, cf. `WebFileManagerActivity`) |

### Non-functional

- **S1**: AES-256-GCM encrypted secrets, non-exportable Keystore key;
  `allowBackup=false` (existing, kept).
- **S2**: http:// URL refused unless « allow HTTP » checkbox checked, with
  warning label (existing, kept).
- **P1**: streaming upload (no local copy of the shared file).
- **A11y**: contentDescription labels on all controls; errors
  announced by TalkBack.
- Out of v2 scope: download/reading of documents *native to
  the app* (integrated viewer), push notifications, multi-account management.
  Revised since initial v2: read/manage access to existing documents
  is covered by F8 (WebView to the server's web UI) —
  deliberate choice not to reimplement a native file manager
  rather than a hidden scope change.

## 5. Planned verification evidence

1. Emulator E2E suite ↔ rmfakecloud Docker (protocol already proven
   2026-07-16), replayed at each milestone: login, tree, upload, 401, 409,
   file without extension, SEND_MULTIPLE ×3.
2. Rust crate tests alone (nominal + negative: server down, invalid
   TLS, truncated JSON).
3. `diffoscope` on two builds for reproducibility (C4).
4. εxodus scan before F-Droid submission (0 tracker expected).

## 6. Three main risks

1. **Adversarial content providers** (medium impact, high
   likelihood, easy detection) — URI without permission, null name, zero size,
   lying MIME. Already encountered in testing (SecurityException MediaProvider).
   Mitigation: F1/F6 + dedicated tests; never crash, always a message.
2. **Rust .so reproducibility for F-Droid** (high impact, medium
   likelihood) — blocks publication. Mitigation: pinned toolchain from M1,
   continuous CI verification, early contact with F-Droid maintainers.
3. **Share screen killed by the system during a large upload** (medium
   impact, medium likelihood, difficult detection) — silent loss.
   Mitigation: v2 moves upload to `WorkManager` with progress
   notification; test with 200 MB file + forced sleep.

Accepted counter-norm: UI remains Views/Kotlin (platform particularism)
— Compose migration cost not justified by the criteria; re-evaluation if
a UI overhaul becomes necessary.

## 7. Verified sources

- E2E test 2026-07-16 (this repo, scratchpad captures + verified API).
- rmfakecloud source `internal/ui/` (GitHub ddvk/rmfakecloud, read 2026-07-16).
- OWASP MASVS v2 (MASVS-STORAGE, MASVS-NETWORK) — S1/S2 requirements.
- F-Droid inclusion policy — C4 criterion.
- Android: share intent and Keystore documentation (developer.android.com).
