# Socle commun Rust — `fkcloud-core`

Document d'architecture transverse aux trois PRD. Méthode : éthos mertonien
(critères annoncés, audit de l'existant, hypothèses réfutables, risques classés).

## 1. Critères et hypothèses

Critères impersonnels d'évaluation du socle, annoncés avant le choix :

| Critère | Métrique observable |
|---|---|
| C1 Mutualisation | ≥ 80 % de la logique protocole (auth, arbre, upload) en code partagé |
| C2 Portabilité | même crate compilé pour android (4 ABI), ios (arm64 + sim), win x64, mac x64/arm64, linux x64 |
| C3 Sécurité | TLS moderne sans OpenSSL système ; secrets jamais dans le crate (délégués aux coffres natifs) |
| C4 Vérifiabilité | suite de tests du crate exécutable seule contre un rmfakecloud Docker ; CI matricielle verte |
| C5 Acceptabilité F-Droid | build Android reproductible, toolchain libre uniquement |

Hypothèse falsifiable H1 : « un cœur Rust unique coûte moins cher que trois
implémentations natives dès la 2ᵉ plateforme ». Réfutation possible : si le coût
d'intégration UniFFI (build, debug, CI) dépasse la réécriture Swift+Kotlin de
~600 lignes de logique, H1 tombe — à réévaluer à la fin du jalon iOS M2.

## 2. Audit de l'existant (originalité)

| Option | Limites observées |
|---|---|
| Statu quo : Kotlin (Android) + Swift (iOS) + ? (desktop) | logique protocole écrite 3×, dérive de comportement entre clients (déjà vu : gestion extension MIME corrigée côté Android seulement) |
| Kotlin Multiplatform | couvre Android/desktop/iOS, mais runtime JVM lourd au desktop, interop iOS moins mûre que UniFFI, et contredit la contrainte « base Rust » du propriétaire du projet |
| Bibliothèques rmfakecloud existantes | `rmapi` (Go, protocole sync device, pas l'API web UI), `rmfakecloud-proxy` (hors sujet) — aucune ne couvre `/ui/api` ; vérifié sur GitHub 2026-07 |
| **Retenu : crate Rust + UniFFI + Tauri** | coût : toolchains NDK/Xcode à outiller ; bénéfice : un seul code protocole, desktop natif léger, écosystème crates audité |

## 3. Architecture retenue

```
core/                        # crate fkcloud-core (Rust 2021)
  src/
    client.rs                # RmfcClient : login, list_folders, upload (streaming)
    session.rs               # cache JWT 23 h, re-login sur 401, politique HTTPS
    model.rs                 # Folder, Document, ApiError (thiserror)
    lib.rs + fkcloud.udl     # surface UniFFI
apps/android/                # UI Kotlin existante ; RmfcClient.kt remplacé par bindings
apps/ios/                    # SwiftUI + Share Extension ; XCFramework du core
apps/desktop/                # Tauri 2 : le core est une dépendance Cargo directe
```

Dépendances du crate (toutes licences libres, vérifiées crates.io) :
`reqwest` (rustls, pas d'OpenSSL — C3), `serde`/`serde_json`, `thiserror`,
`uniffi`. Pas de tokio exposé : API bloquante + exécuteur interne, plus simple
à binder sur les trois plateformes.

Répartition des responsabilités (C3) : le crate parle le protocole
([Docs/api-rmfakecloud.md](../api-rmfakecloud.md)) ; le stockage des secrets
reste natif — Android Keystore, iOS Keychain, `keyring` (DPAPI/Keychain/
Secret Service) côté desktop. Le crate reçoit les secrets en mémoire,
ne les écrit jamais.

## 4. Chaîne de build

| Cible | Outil | Sortie |
|---|---|---|
| Android | `cargo-ndk` (arm64-v8a, armeabi-v7a, x86_64) + `uniffi-bindgen` Kotlin | `.so` + Kotlin dans `apps/android` |
| iOS | `cargo build` aarch64-apple-ios + sim, `xcodebuild -create-xcframework` + bindgen Swift | `FkCloudCore.xcframework` |
| Desktop | dépendance Cargo du binaire Tauri | msi (win x64), dmg (mac x64), dmg (mac arm64) |

CI (GitLab) : matrice lint (`clippy -D warnings`) + tests crate contre
rmfakecloud Docker + builds croisés. Chaque jalon des roadmaps = tag git,
ce qui rend toute régression isolable par bissection (`git bisect` sur les
tags, log₂(N) essais).

## 5. Trois risques principaux (scepticisme organisé)

1. **Reproductibilité F-Droid avec NDK** (impact fort, vraisemblance moyenne,
   détection facile) — les .so Rust doivent être bit-identiques.
   Réduction : versions épinglées (rust-toolchain.toml, NDK r27 fixé),
   `SOURCE_DATE_EPOCH`, vérification `diffoscope` en CI dès le jalon A-M1.
2. **Coût UniFFI iOS sous-estimé** (impact moyen, vraisemblance moyenne,
   détection tardive) — c'est le point de réfutation de H1. Réduction :
   jalon iOS M1 limité à « login + list » ; si > 2 semaines, bascule
   documentée vers un client Swift natif de ~300 lignes (contre-norme
   assumée : particularisme plateforme, écart limité à iOS).
3. **API bloquante dans le crate** (impact moyen, vraisemblance faible) —
   risque de blocage d'UI si un binding l'appelle sur le thread principal.
   Réduction : contrat documenté « jamais sur le main thread », wrappers
   async fournis par plateforme (coroutines, Swift concurrency, tauri async).

## 6. Sources vérifiées

- Source rmfakecloud (`internal/ui/*.go`), lu le 2026-07-16 — contrat API.
- Test E2E réel Android ↔ rmfakecloud Docker, 2026-07-16 (ce dépôt).
- UniFFI : https://mozilla.github.io/uniffi-rs/ — bindings Kotlin/Swift.
- Tauri v2 : https://v2.tauri.app/ — cibles win/mac/linux.
- Politique F-Droid (builds reproductibles, toolchains libres) :
  https://f-droid.org/docs/Inclusion_Policy/
