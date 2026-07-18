# PRD — FkCloud Share iOS (v1, base Rust)

Méthode : éthos mertonien. Socle : [core-rust.md](../core-rust.md) ·
API : [api-rmfakecloud.md](../../api-rmfakecloud.md)
Charte graphique : [Registre libre](../../charte-graphique/charte-graphique.md#3-registre-libre)

## 1. Contexte universel et bénéficiaires

Mêmes bénéficiaires que l'app Android (self-hosters rmfakecloud), équipés
d'iPhone/iPad. iOS n'offre aucun canal de distribution libre équivalent à
F-Droid : la distribution est une contrainte structurante, pas un détail
(voir contre-normes, §6). Cible : iOS 16+, iPhone et iPad.

## 2. Critères impersonnels et hypothèses

| # | Critère | Métrique observable |
|---|---|---|
| C1 | Parité protocole | mêmes appels, mêmes comportements que l'app Android (source unique : `fkcloud-core`) |
| C2 | Geste natif | présence dans la Share Sheet pour PDF/EPUB ; partage → confirmation ≤ 4 interactions |
| C3 | Sécurité | secrets dans le Keychain (`AfterFirstUnlockThisDeviceOnly`) ; ATS actif — HTTPS obligatoire, exception HTTP locale déclarée et opt-in |
| C4 | Empreinte | app + extension < 20 Mo ; zéro dépendance tierce Swift |
| C5 | Accessibilité | VoiceOver complet, Dynamic Type respecté |

Hypothèses falsifiables :
- **H1** : UniFFI→Swift est viable (coût d'intégration < 2 semaines pour
  login+arbre). Invalidation mesurée au jalon M1 ; sinon bascule client
  Swift natif (~300 lignes), décision documentée.
- **H2** : la limite mémoire des extensions iOS (~120 Mo) permet l'upload
  streaming de gros fichiers. Invalidation : test 200 Mo au jalon M2 ;
  sinon l'extension délègue à l'app conteneur (background URLSession).

## 3. Audit de l'existant

| Solution | Limites observées |
|---|---|
| UI web rmfakecloud dans Safari | pas de Share Sheet, re-login, UX mobile dégradée |
| App officielle reMarkable | serveur tiers impossible ; propriétaire |
| Raccourcis Apple (Shortcuts + API REST) | faisable pour un power-user, mais mot de passe en clair dans le raccourci — viole C3 |

Contribution nouvelle : premier client iOS natif pour l'API web rmfakecloud,
protocole partagé avec Android/desktop via le crate commun.

## 4. Exigences

### Fonctionnelles

| ID | Exigence | Acceptation |
|---|---|---|
| F1 | Share Extension PDF/EPUB (mono + multiple) | fichier partagé depuis Fichiers/Safari/Mail → dialog avec nom correct |
| F2 | Choix du dossier destination | arbre serveur affiché ; `parent` correct vérifié côté serveur |
| F3 | App conteneur : configuration | URL + identifiants + test connexion ; erreurs distinctes (401 / réseau / ATS) |
| F4 | Session partagée app ↔ extension | App Group + Keychain partagé ; l'extension n'exige jamais de re-login si jeton valide |
| F5 | Rapport d'erreur actionnable | échec nommé par fichier ; réessai possible |

### Non fonctionnelles

- **S1** : aucun secret hors Keychain ; pas de logs contenant jeton/mot de passe.
- **S2** : ATS par défaut ; HTTP uniquement via `NSAllowsLocalNetworking`
  + opt-in UI (parité avec la politique Android).
- **P1** : upload streaming ; extension conforme aux limites mémoire (H2).
- Hors périmètre v1 : téléchargement, édition, multi-comptes, widget.

## 5. Preuves de vérification prévues

1. Tests XCTest de l'app conteneur contre rmfakecloud Docker (simulateur).
2. Le crate est déjà couvert par ses propres tests (jalon `core-m1` de la
   roadmap Android) — non redondés côté Swift.
3. Test manuel scripté sur appareil : partage depuis Fichiers, Mail,
   Safari ; fichier 200 Mo (H2) ; avion/latence (échec propre).

## 6. Trois risques principaux et contre-normes

1. **Limite mémoire de l'extension** (impact fort, vraisemblance moyenne,
   détection facile) — crash silencieux au-delà du quota. Réduction : H2
   testée tôt (M2) ; repli = handoff vers l'app conteneur.
2. **Chaîne de build Rust→XCFramework fragile** (impact moyen, vraisemblance
   moyenne) — casse aux mises à jour Xcode. Réduction : versions épinglées,
   build reproduit en CI macOS à chaque jalon.
3. **Refus App Store** (impact fort, vraisemblance faible) — app « client
   d'un serveur privé » parfois recalée pour « fonctionnalité minimale ».
   Réduction : soumission TestFlight d'abord ; argumentaire open source ;
   repli sideload (AltStore) documenté.

Contre-normes assumées : distribution App Store = canal propriétaire
(écart au communalisme, imposé par la plateforme) ; compte développeur
payant requis. Compensation : code GPL dans ce dépôt, parité fonctionnelle
avec les canaux libres des autres plateformes ; réévaluation si le sideload
UE devient praticable pour ce public.

## 7. Sources vérifiées

- Contrat API : source rmfakecloud lu 2026-07-16 + E2E Android du même jour.
- UniFFI Swift bindings : documentation Mozilla (mozilla.github.io/uniffi-rs).
- Limites mémoire des app extensions : documentation Apple (developer.apple.com,
  App Extension Programming Guide) — chiffre exact à re-mesurer au jalon M2
  (connaissance générale, pas de garantie contractuelle d'Apple).
- ATS / NSAppTransportSecurity : documentation Apple vérifiable en ligne.
