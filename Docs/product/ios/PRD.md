# PRD — FkCloud Share iOS (v1, Rust base)

French: [PRD.fr.md](PRD.fr.md).

Method: Mertonian ethos. Core: [core-rust.md](../core-rust.md) ·
API: [api-rmfakecloud.md](../../api-rmfakecloud.md)

## 1. Universal context and beneficiaries

Same beneficiaries as the Android app (rmfakecloud self-hosters), equipped
with iPhone/iPad. iOS offers no free distribution channel equivalent to
F-Droid: distribution is a structural constraint, not a detail
(see counter-norms, §6). Target: iOS 16+, iPhone and iPad.

## 2. Impersonal criteria and hypotheses

| # | Criterion | Observable metric |
|---|---|---|
| C1 | Protocol parity | same calls, same behaviors as the Android app (single source: `fkcloud-core`) |
| C2 | Native gesture | presence in the Share Sheet for PDF/EPUB; share → confirmation ≤ 4 interactions |
| C3 | Security | secrets in Keychain (`AfterFirstUnlockThisDeviceOnly`); ATS active — HTTPS required, local HTTP exception declared and opt-in |
| C4 | Footprint | app + extension < 20 MB; zero third-party Swift dependency |
| C5 | Accessibility | full VoiceOver, Dynamic Type respected |

Falsifiable hypotheses:
- **H1**: UniFFI→Swift is viable (integration cost < 2 weeks for
  login+tree). Invalidation measured at milestone M1; otherwise switch to native
  Swift client (~300 lines), documented decision.
- **H2**: iOS extension memory limit (~120 MB) allows streaming upload
  of large files. Invalidation: 200 MB test at milestone M2;
  otherwise the extension delegates to the container app (background URLSession).

## 3. Audit of the existing

| Solution | Observed limits |
|---|---|
| rmfakecloud web UI in Safari | no Share Sheet, re-login, degraded mobile UX |
| Official reMarkable app | third-party server impossible; proprietary |
| Apple Shortcuts (Shortcuts + REST API) | feasible for a power user, but password in plain text in the shortcut — violates C3 |

New contribution: first native iOS client for the rmfakecloud web API,
protocol shared with Android/desktop via the common crate.

## 4. Requirements

### Functional

| ID | Requirement | Acceptance |
|---|---|---|
| F1 | PDF/EPUB Share Extension (single + multiple) | file shared from Files/Safari/Mail → dialog with correct name |
| F2 | Destination folder choice | server tree displayed; correct `parent` verified server-side |
| F3 | Container app: configuration | URL + credentials + connection test; distinct errors (401 / network / ATS) |
| F4 | Shared session app ↔ extension | App Group + shared Keychain; extension never requires re-login if token valid |
| F5 | Actionable error report | failure named per file; retry possible |

### Non-functional

- **S1**: no secret outside Keychain; no logs containing token/password.
- **S2**: ATS by default; HTTP only via `NSAllowsLocalNetworking`
  + UI opt-in (parity with Android policy).
- **P1**: streaming upload; extension compliant with memory limits (H2).
- Out of v1 scope: download, editing, multi-accounts, widget.

## 5. Planned verification evidence

1. Container app XCTest against rmfakecloud Docker (simulator).
2. The crate is already covered by its own tests (Android roadmap milestone `core-m1`) — not duplicated on Swift side.
3. Scripted manual test on device: share from Files, Mail,
   Safari; 200 MB file (H2); airplane/latency (clean failure).

## 6. Three main risks and counter-norms

1. **Extension memory limit** (high impact, medium
   likelihood, easy detection) — silent crash beyond quota. Mitigation: H2
   tested early (M2); fallback = handoff to container app.
2. **Fragile Rust→XCFramework build chain** (medium impact, medium
   likelihood) — breaks on Xcode updates. Mitigation: pinned versions,
   build reproduced in macOS CI at each milestone.
3. **App Store rejection** (high impact, low likelihood) — « client
   for a private server » app sometimes rejected for « minimal functionality ».
   Mitigation: TestFlight submission first; open source argument;
   documented sideload fallback (AltStore).

Accepted counter-norms: App Store distribution = proprietary channel
(deviation from communalism, imposed by the platform); paid developer
account required. Compensation: GPL code in this repo, functional parity
with free channels on other platforms; re-evaluation if EU sideload
becomes practical for this audience.

## 7. Verified sources

- API contract: rmfakecloud source read 2026-07-16 + Android E2E same day.
- UniFFI Swift bindings: Mozilla documentation (mozilla.github.io/uniffi-rs).
- App extension memory limits: Apple documentation (developer.apple.com,
  App Extension Programming Guide) — exact figure to be re-measured at milestone M2
  (general knowledge, no contractual guarantee from Apple).
- ATS / NSAppTransportSecurity: Apple documentation verifiable online.
