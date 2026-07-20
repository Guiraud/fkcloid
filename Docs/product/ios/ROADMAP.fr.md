# Roadmap — FkCloud Share iOS (bissection progressive)

English: [`ROADMAP.md`](ROADMAP.md).

Axe : **borne A** = rien (dossier vide) → **borne B** = v1.0 distribuée
(TestFlight puis App Store). Pas minimal : jalon ≤ 1 semaine, vérification
binaire. Chaque jalon = tag git (régression localisable en log₂(N)).

## Passes de bissection

- **Passe 1 — bornes** : A (zéro code iOS) et B (v1.0 distribuée).
- **Passe 2 — milieu M** : *app conteneur fonctionnelle* — login, arbre,
  upload d'un PDF choisi via UIDocumentPicker, cœur Rust en XCFramework.
  Coupe l'espace en deux : avant = toolchain, après = expérience partage
  et distribution.
- **Passe 3 — quarts** :
  - A–M : *XCFramework* — `fkcloud-core` compilé arm64 + simulateur,
    appelable depuis un projet Swift vide.
  - M–B : *Share Extension* — le geste de partage complet, App Group,
    Keychain partagé.
- **Passe 4** : écarts ≤ 1 semaine — arrêt.

## Jalons (ordre grossier → fin)

| Tag | Jalon | Vérification binaire |
|---|---|---|
| — | Borne A | dossier `apps/ios/` : README seul (état actuel) |
| `ios-m1` | Quart A–M : XCFramework + smoke test | projet Swift minimal appelle `login()` du crate sur simulateur ; **décision H1 consignée ici** (< 2 sem. → continuer ; sinon client Swift natif) |
| `ios-m2` | **Milieu M** : app conteneur (config + upload picker) | XCTest vert contre Docker ; test 200 Mo → **décision H2 consignée ici** |
| `ios-m3` | Quart M–B : Share Extension + session partagée | partage depuis Fichiers → document sur serveur ; sans re-login |
| `ios-v1.0` | Borne B : distribution | build TestFlight installable ; checklist review App Store passée |

## Segments de risque

Casse au jalon `m3` → tester `m2` (le milieu) avant d'inspecter les commits :
moitié fautive identifiée en un essai, puis bissection interne au segment.

## Après B (backlog ordonné)

1. iPad multitâche (Split View) si demande.
2. Téléchargement de documents.
3. Widget « derniers envois ».
