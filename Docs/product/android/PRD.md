# PRD — FkCloud Share Android (v2, base Rust)

Méthode : éthos mertonien (développement incrémental vérifiable).
Socle : [core-rust.md](../core-rust.md) · API : [api-rmfakecloud.md](../../api-rmfakecloud.md)
Charte graphique : [Registre libre](../../charte-graphique/charte-graphique.md#3-registre-libre)

## 1. Contexte universel et bénéficiaires

Des possesseurs de tablette reMarkable auto-hébergent rmfakecloud pour garder
leurs documents hors du cloud commercial. Sur téléphone Android, envoyer un
PDF/EPUB vers ce serveur exige aujourd'hui un navigateur et l'UI web — friction
forte pour un geste quotidien. Bénéficiaires : self-hosters (profil F-Droid),
sans hypothèse sur leur distribution Android (min API 26, aucun service Google).

**Fait établi** : une v1 Kotlin fonctionne, testée E2E le 2026-07-16 (émulateur
API 36 ↔ rmfakecloud Docker : login, arbre dossiers, upload vérifié serveur).
Charte graphique « Registre libre » appliquée et sélecteur de dossier
navigable (F2), fermeture auto (F7) et gestionnaire de fichiers web (F8)
ajoutés et testés E2E les 2026-07-17/18 (mêmes conditions de test).

## 2. Critères impersonnels et hypothèses

| # | Critère | Métrique observable |
|---|---|---|
| C1 | Rapidité du geste | partage → confirmation ≤ 4 interactions ; upload 10 Mo < 30 s en Wi-Fi |
| C2 | Sécurité | MASVS-STORAGE-1 : secrets uniquement via Keystore ; HTTPS par défaut, HTTP opt-in explicite |
| C3 | Compatibilité | API 26 → 36 ; ACTION_SEND et SEND_MULTIPLE ; fournisseurs de contenu hétérogènes (MediaStore, FileProvider, SAF) |
| C4 | Acceptabilité F-Droid | 0 dépendance propriétaire, 0 tracker (scan exodus vide), build reproductible |
| C5 | Accessibilité | contrastes WCAG AA, cibles tactiles ≥ 48 dp, TalkBack utilisable |

Hypothèses falsifiables :
- **H1** : les utilisateurs préfèrent le menu de partage à une app à ouvrir.
  Invalidation : si > 30 % des retours demandent un sélecteur de fichiers
  intégré, ajouter un écran « choisir un fichier » (v2.1).
- **H2** : le cœur Rust (UniFFI) n'apporte aucune régression fonctionnelle ni
  de latence perceptible vs Kotlin. Invalidation : suite E2E comparative ;
  si échec, conserver le client Kotlin (H1 du socle tombe pour Android).

## 3. Audit de l'existant

| Solution | Limites observées |
|---|---|
| UI web rmfakecloud sur mobile | pas dans le menu de partage ; re-login fréquent ; pas de choix de dossier au moment du partage |
| App officielle reMarkable | refuse un serveur tiers ; propriétaire ; hors F-Droid |
| v1 Kotlin de ce dépôt | fonctionnelle mais logique protocole non partagée avec iOS/desktop (dérive déjà observée sur la gestion des extensions) |

Contribution nouvelle, incrémentale : v2 = même UX que v1, logique protocole
déplacée dans `fkcloud-core` (Rust), plus création de dossier et icône
monochrome. Pas de réécriture UI.

## 4. Exigences

### Fonctionnelles (critère d'acceptation binaire pour chacune)

| ID | Exigence | Acceptation |
|---|---|---|
| F1 | Cible de partage PDF/EPUB (mono et multiple) | intent SEND/SEND_MULTIPLE → dialog s'ouvre avec noms corrects, y compris nom opaque → extension déduite du MIME |
| F2 | Choix du dossier destination | sélecteur navigable (fil d'Ariane, tap pour entrer dans un sous-dossier, documents affichés en contexte non cliquables) ; sélection appliquée (vérif API `parent`) — implémenté (`FolderPickerActivity`), remplace l'ancienne liste plate |
| F3 | Création de dossier depuis le dialog | `POST /ui/api/folders` ; nouveau dossier visible et sélectionné — **non implémenté**, reste à faire |
| F4 | Session persistante | jeton 23 h ; re-login auto sur 401 ; zéro re-saisie du mot de passe |
| F5 | Écran de configuration | URL + identifiants + test connexion ; états succès/échec distincts (401 vs réseau vs HTTP interdit) |
| F6 | Rapport d'erreur actionnable | chaque échec d'upload nomme le fichier et la cause ; bouton réessayer |
| F7 | Fermeture automatique après succès | dialog affiche la confirmation ~1,4 s puis se ferme seul ; bouton « Fermer » reste disponible pour un dismiss immédiat |
| F8 | Gestionnaire de fichiers web | bouton dédié ouvre une WebView pointant sur le serveur configuré (politique HTTPS/HTTP identique à F5) ; navigation limitée au domaine configuré, liens externes ouverts dans le navigateur système ; CSS `zoom: 50%` injecté post-chargement car la nav de rmfakecloud ne s'adapte pas aux écrans étroits (`setInitialScale`/`zoomBy` inefficaces, cf. `WebFileManagerActivity`) |

### Non fonctionnelles

- **S1** : secrets chiffrés AES-256-GCM, clé Keystore non exportable ;
  `allowBackup=false` (existant, conservé).
- **S2** : URL http:// refusée sauf case « autoriser HTTP » cochée, avec
  libellé de mise en garde (existant, conservé).
- **P1** : upload en streaming (pas de copie locale du fichier partagé).
- **A11y** : libellés contentDescription sur toutes les commandes ; erreurs
  annoncées par TalkBack.
- Hors périmètre v2 : téléchargement/lecture de documents *natifs à
  l'app* (viewer intégré), notifications push, gestion multi-comptes.
  Revu depuis la v2 initiale : l'accès en lecture/gestion des documents
  existants est couvert par F8 (WebView vers l'UI web du serveur) —
  choix délibéré de ne pas réimplémenter un gestionnaire de fichiers
  natif plutôt qu'un changement de périmètre caché.

## 5. Preuves de vérification prévues

1. Suite E2E émulateur ↔ rmfakecloud Docker (protocole déjà éprouvé le
   2026-07-16), rejouée à chaque jalon : login, arbre, upload, 401, 409,
   fichier sans extension, SEND_MULTIPLE ×3.
2. Tests du crate Rust seuls (nominaux + négatifs : serveur down, TLS
   invalide, JSON tronqué).
3. `diffoscope` sur deux builds pour la reproductibilité (C4).
4. Scan εxodus avant soumission F-Droid (0 tracker attendu).

## 6. Trois risques principaux

1. **Fournisseurs de contenu adversariaux** (impact moyen, vraisemblance
   forte, détection facile) — URI sans permission, nom nul, taille nulle,
   MIME menteur. Déjà rencontré en test (SecurityException MediaProvider).
   Réduction : F1/F6 + tests dédiés ; jamais de crash, toujours un message.
2. **Reproductibilité .so Rust pour F-Droid** (impact fort, vraisemblance
   moyenne) — bloque la publication. Réduction : toolchain épinglée dès M1,
   vérification CI continue, contact précoce avec les mainteneurs F-Droid.
3. **Écran de partage tué par le système pendant un gros upload** (impact
   moyen, vraisemblance moyenne, détection difficile) — perte silencieuse.
   Réduction : v2 passe l'upload en `WorkManager` avec notification de
   progression ; test avec fichier 200 Mo + mise en veille forcée.

Contre-norme assumée : l'UI reste en Views/Kotlin (particularisme plateforme)
— coût de migration Compose non justifié par les critères ; réévaluation si
une refonte UI devient nécessaire.

## 7. Sources vérifiées

- Test E2E du 2026-07-16 (ce dépôt, captures scratchpad + API vérifiée).
- Source rmfakecloud `internal/ui/` (GitHub ddvk/rmfakecloud, lu 2026-07-16).
- OWASP MASVS v2 (MASVS-STORAGE, MASVS-NETWORK) — exigences S1/S2.
- Politique d'inclusion F-Droid — critère C4.
- Android : documentation intents de partage et Keystore (developer.android.com).
