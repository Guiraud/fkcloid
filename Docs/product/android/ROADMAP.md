# Roadmap — FkCloud Share Android (progressive bisection)

French: [ROADMAP.fr.md](ROADMAP.fr.md).

Method: progressive bisection (short iterations, tests at each step).
Ordered axis: **bound A** = current Kotlin v1.0 (E2E green 2026-07-16) →
**bound B** = v2.0 published on F-Droid, Rust core.
Minimal step: one milestone ≤ 1 week, binary verifiable.
Each milestone = git tag ⇒ any regression is located in log₂(N) trials
(`git bisect` between two tags).

## Bisection passes

- **Pass 1 — bounds**: A (local v1.0) and B (F-Droid v2.0).
- **Pass 2 — midpoint M**: *Rust parity* — `fkcloud-core` replaces
  `RmfcClient.kt`/`Session.kt` via UniFFI, v1 E2E suite replayed green.
  This is the point that best splits the space: everything before is
  core, everything after is product.
- **Pass 3 — quarters**:
  - A–M: *crate alone* — `fkcloud-core` tested against Docker, without Android.
  - M–B: *F-Droid candidate* — v2 features (F3 folder, WorkManager,
    monochrome icon) + reproducible build.
- **Pass 4 — eighths**: fine breakdown below. Gaps < 1 week
  thereafter: stop bisection.

## Milestones (rough delivery order → end)

| Tag | Milestone | Binary verification |
|---|---|---|
| `android-v1.0` | Bound A: functional Kotlin app | emulator E2E green (done, 2026-07-16) |
| `core-m1` | Quarter A–M: `fkcloud-core` crate (login, tree, upload) | `cargo test` green against rmfakecloud Docker; clippy no warnings |
| `android-m2` | **Midpoint M**: UniFFI bindings integrated, Kotlin network removed | same E2E suite as v1.0 green; APK no longer contains OkHttp |
| `android-m3` | Quarter M–B: F3 folder creation + WorkManager upload + monochrome icon | extended E2E green; 200 MB upload survives sleep |
| `android-v2.0` | Bound B: F-Droid publication | reproducible build (diffoscope Ø); fdroiddata MR accepted; εxodus 0 tracker |

## Risk segments (for bisection on breakage)

`v1.0 → core-m1 → m2 → m3 → v2.0`: if E2E breaks at `m3`, test `m2`
first (midpoint), not the eight intermediate commits — faulty half
identified in one trial.

## After B (outside bisection, ordered backlog)

1. Integrated file picker (if PRD H1 invalidated).
2. `GET /ui/api/sync` after upload (tablet refresh).
3. Document download/export.
