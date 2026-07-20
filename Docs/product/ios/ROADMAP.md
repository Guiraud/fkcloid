# Roadmap — FkCloud Share iOS (progressive bisection)

French: [ROADMAP.fr.md](ROADMAP.fr.md).

Axis: **bound A** = nothing (empty folder) → **bound B** = v1.0 distributed
(TestFlight then App Store). Minimal step: milestone ≤ 1 week, binary
verification. Each milestone = git tag (regression locatable in log₂(N)).

## Bisection passes

- **Pass 1 — bounds**: A (zero iOS code) and B (v1.0 distributed).
- **Pass 2 — midpoint M**: *functional container app* — login, tree,
  upload of a PDF chosen via UIDocumentPicker, Rust core in XCFramework.
  Splits the space in two: before = toolchain, after = share experience
  and distribution.
- **Pass 3 — quarters**:
  - A–M: *XCFramework* — `fkcloud-core` compiled arm64 + simulator,
    callable from an empty Swift project.
  - M–B: *Share Extension* — complete share gesture, App Group,
    shared Keychain.
- **Pass 4**: gaps ≤ 1 week — stop.

## Milestones (rough order → end)

| Tag | Milestone | Binary verification |
|---|---|---|
| — | Bound A | `apps/ios/` folder: README only (current state) |
| `ios-m1` | Quarter A–M: XCFramework + smoke test | minimal Swift project calls crate `login()` on simulator; **H1 decision recorded here** (< 2 wk → continue; otherwise native Swift client) |
| `ios-m2` | **Midpoint M**: container app (config + picker upload) | XCTest green against Docker; 200 MB test → **H2 decision recorded here** |
| `ios-m3` | Quarter M–B: Share Extension + shared session | share from Files → document on server; no re-login |
| `ios-v1.0` | Bound B: distribution | installable TestFlight build; App Store review checklist passed |

## Risk segments

Breakage at milestone `m3` → test `m2` (midpoint) before inspecting commits:
faulty half identified in one trial, then internal bisection within the segment.

## After B (ordered backlog)

1. iPad multitasking (Split View) if requested.
2. Document download.
3. « Recent uploads » widget.
