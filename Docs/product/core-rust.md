# Shared Rust Core — `fkcloud-core`

French: [core-rust.fr.md](core-rust.fr.md).

Cross-cutting architecture document for the three PRDs. Method: Mertonian ethos
(announced criteria, audit of the existing, refutable hypotheses, ranked risks).

## 1. Criteria and hypotheses

Impersonal evaluation criteria for the core, announced before the choice:

| Criterion | Observable metric |
|---|---|
| C1 Shared logic | ≥ 80% of protocol logic (auth, tree, upload) in shared code |
| C2 Portability | same crate compiled for android (4 ABIs), ios (arm64 + sim), win x64, mac x64/arm64, linux x64 |
| C3 Security | modern TLS without system OpenSSL; secrets never in the crate (delegated to native vaults) |
| C4 Verifiability | crate test suite runnable alone against rmfakecloud Docker; green matrix CI |
| C5 F-Droid acceptability | reproducible Android build, free toolchain only |

Falsifiable hypothesis H1: « a single Rust core costs less than three
native implementations starting from the 2nd platform ». Possible refutation: if the
UniFFI integration cost (build, debug, CI) exceeds rewriting Swift+Kotlin for
~600 lines of logic, H1 falls — to be re-evaluated at the end of iOS milestone M2.

## 2. Audit of the existing (originality)

| Option | Observed limits |
|---|---|
| Status quo: Kotlin (Android) + Swift (iOS) + ? (desktop) | protocol logic written 3×, behavioral drift between clients (already seen: MIME extension handling fixed on Android only) |
| Kotlin Multiplatform | covers Android/desktop/iOS, but heavy JVM runtime on desktop, iOS interop less mature than UniFFI, and contradicts the project owner's « Rust base » constraint |
| Existing rmfakecloud libraries | `rmapi` (Go, device sync protocol, not the web UI API), `rmfakecloud-proxy` (off topic) — none covers `/ui/api`; verified on GitHub 2026-07 |
| **Selected: Rust crate + UniFFI + Tauri** | cost: NDK/Xcode toolchains to set up; benefit: single protocol code, lightweight native desktop, audited crates ecosystem |

## 3. Selected architecture

```
core/                        # fkcloud-core crate (Rust 2021)
  src/
    client.rs                # RmfcClient: login, list_folders, upload (streaming)
    session.rs               # 23 h JWT cache, re-login on 401, HTTPS policy
    model.rs                 # Folder, Document, ApiError (thiserror)
    lib.rs + fkcloud.udl     # UniFFI surface
apps/android/                # existing Kotlin UI; RmfcClient.kt replaced by bindings
apps/ios/                    # SwiftUI + Share Extension; core XCFramework
apps/desktop/                # Tauri 2: core is a direct Cargo dependency
```

Crate dependencies (all free licenses, verified on crates.io):
`reqwest` (rustls, no OpenSSL — C3), `serde`/`serde_json`, `thiserror`,
`uniffi`. No exposed tokio: blocking API + internal executor, simpler
to bind on all three platforms.

Responsibility split (C3): the crate speaks the protocol
([Docs/api-rmfakecloud.md](../api-rmfakecloud.md)); secret storage
remains native — Android Keystore, iOS Keychain, `keyring` (DPAPI/Keychain/
Secret Service) on desktop. The crate receives secrets in memory,
never writes them.

## 4. Build chain

| Target | Tool | Output |
|---|---|---|
| Android | `cargo-ndk` (arm64-v8a, armeabi-v7a, x86_64) + `uniffi-bindgen` Kotlin | `.so` + Kotlin in `apps/android` |
| iOS | `cargo build` aarch64-apple-ios + sim, `xcodebuild -create-xcframework` + bindgen Swift | `FkCloudCore.xcframework` |
| Desktop | Tauri binary Cargo dependency | msi (win x64), dmg (mac x64), dmg (mac arm64) |

CI (GitLab): matrix lint (`clippy -D warnings`) + crate tests against
rmfakecloud Docker + cross builds. Each roadmap milestone = git tag,
making any regression isolable by bisection (`git bisect` on
tags, log₂(N) trials).

## 5. Three main risks (organized skepticism)

1. **F-Droid reproducibility with NDK** (high impact, medium likelihood,
   easy detection) — Rust .so files must be bit-identical.
   Mitigation: pinned versions (rust-toolchain.toml, NDK r27 fixed),
   `SOURCE_DATE_EPOCH`, `diffoscope` verification in CI from Android milestone A-M1.
2. **Underestimated UniFFI iOS cost** (medium impact, medium likelihood,
   late detection) — this is H1's refutation point. Mitigation:
   iOS milestone M1 limited to « login + list »; if > 2 weeks, documented switch
   to a native Swift client of ~300 lines (accepted counter-norm:
   platform particularism, limited drift to iOS).
3. **Blocking API in the crate** (medium impact, low likelihood) —
   risk of UI blocking if a binding calls it on the main thread.
   Mitigation: documented contract « never on the main thread », async
   wrappers provided per platform (coroutines, Swift concurrency, tauri async).

## 6. Verified sources

- rmfakecloud source (`internal/ui/*.go`), read 2026-07-16 — API contract.
- Real Android E2E test ↔ rmfakecloud Docker, 2026-07-16 (this repo).
- UniFFI: https://mozilla.github.io/uniffi-rs/ — Kotlin/Swift bindings.
- Tauri v2: https://v2.tauri.app/ — win/mac/linux targets.
- F-Droid policy (reproducible builds, free toolchains):
  https://f-droid.org/docs/Inclusion_Policy/
