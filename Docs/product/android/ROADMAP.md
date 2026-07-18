# Roadmap — FkCloud Share Android (bissection progressive)

Méthode : [skill bissection-progressive](https://github.com/Guiraud/Skills).
Axe ordonné : **borne A** = v1.0 Kotlin actuelle (E2E vert 2026-07-16) →
**borne B** = v2.0 publiée sur F-Droid, cœur Rust.
Pas minimal : un jalon ≤ 1 semaine, vérifiable binairement.
Chaque jalon = tag git ⇒ toute régression se localise en log₂(N) essais
(`git bisect` entre deux tags).

## Passes de bissection

- **Passe 1 — bornes** : A (v1.0 locale) et B (v2.0 F-Droid).
- **Passe 2 — milieu M** : *parité Rust* — `fkcloud-core` remplace
  `RmfcClient.kt`/`Session.kt` via UniFFI, suite E2E v1 rejouée verte.
  C'est le point qui coupe le mieux l'espace : tout ce qui précède est
  du socle, tout ce qui suit est du produit.
- **Passe 3 — quarts** :
  - A–M : *crate seul* — `fkcloud-core` testé contre Docker, sans Android.
  - M–B : *candidat F-Droid* — features v2 (F3 dossier, WorkManager,
    icône monochrome) + build reproductible.
- **Passe 4 — huitièmes** : découpage fin ci-dessous. Écarts < 1 semaine
  ensuite : arrêt de la bissection.

## Jalons (ordre de livraison grossier → fin)

| Tag | Jalon | Vérification binaire |
|---|---|---|
| `android-v1.0` | Borne A : app Kotlin fonctionnelle | E2E émulateur vert (fait, 2026-07-16) |
| `core-m1` | Quart A–M : crate `fkcloud-core` (login, arbre, upload) | `cargo test` vert contre rmfakecloud Docker ; clippy sans warning |
| `android-m2` | **Milieu M** : bindings UniFFI intégrés, Kotlin réseau supprimé | même suite E2E que v1.0 verte ; APK ne contient plus OkHttp |
| `android-m3` | Quart M–B : F3 création dossier + upload WorkManager + icône monochrome | E2E étendu vert ; upload 200 Mo survit à la mise en veille |
| `android-v2.0` | Borne B : publication F-Droid | build reproductible (diffoscope Ø) ; MR fdroiddata acceptée ; εxodus 0 tracker |

## Segments de risque (pour bissection en cas de casse)

`v1.0 → core-m1 → m2 → m3 → v2.0` : si l'E2E casse à `m3`, tester `m2`
d'abord (milieu), pas les huit commits intermédiaires — moitié fautive
identifiée en un essai.

## Après B (hors bissection, backlog ordonné)

1. Sélecteur de fichiers intégré (si H1 du PRD invalidée).
2. `GET /ui/api/sync` après upload (rafraîchissement tablette).
3. Téléchargement/export de documents.
