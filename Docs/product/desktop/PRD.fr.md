# PRD — FkCloud Share Desktop (v1, base Rust / Tauri)

English: [`PRD.md`](PRD.md).

Méthode : éthos mertonien. Socle : [core-rust.md](../core-rust.md) ·
API : [api-rmfakecloud.md](../../api-rmfakecloud.md)

## 1. Contexte universel et bénéficiaires

Sur poste de travail, les documents à envoyer vers la tablette (articles PDF,
livres EPUB, papiers de recherche) sont produits ou téléchargés dans un
navigateur ou un gestionnaire de fichiers. Le geste cible : glisser-déposer
sur une fenêtre ou une icône, choisir un dossier, terminé. Bénéficiaires :
les mêmes self-hosters, sur Windows 10/11 x64, macOS Intel (x86_64) et
macOS Apple Silicon (arm64). Linux traité en cible de compilation dès le
départ (coût marginal nul avec Tauri) mais hors engagement v1.

## 2. Critères impersonnels et hypothèses

| # | Critère | Métrique observable |
|---|---|---|
| C1 | Portabilité | mêmes fonctionnalités et mêmes tests sur win-x64, mac-x64, mac-arm64 ; aucune logique conditionnelle par OS hors stockage des secrets et packaging |
| C2 | Geste | drag & drop → confirmation ≤ 3 interactions ; file d'attente visible |
| C3 | Empreinte | binaire < 25 Mo ; RAM au repos < 150 Mo (exclut Electron, ~10× au-dessus) |
| C4 | Sécurité | secrets via coffre natif (Keychain / Credential Manager-DPAPI / Secret Service) ; HTTPS par défaut, HTTP opt-in |
| C5 | Vérifiabilité | E2E automatisé (WebDriver tauri-driver) contre rmfakecloud Docker sur les 3 cibles en CI |

Hypothèses falsifiables :
- **H1** : la webview système (WebView2 / WKWebView) suffit — pas de rendu
  divergent bloquant entre OS. Invalidation : captures comparées au jalon M2 ;
  divergence bloquante → UI native `egui` (alternative documentée).
- **H2** : un utilisateur desktop veut voir sa bibliothèque, pas seulement
  envoyer. Non testée en v1 (hors périmètre) ; sondage utilisateurs après
  v1.0 — si confirmée, la vue bibliothèque devient le premier item v1.1.

## 3. Audit de l'existant

| Solution | Limites observées |
|---|---|
| UI web rmfakecloud (navigateur) | fonctionne, mais pas de drag & drop global, pas de tray, re-login |
| `rmapi` (Go, CLI) | protocole *device* sync, pas l'API web UI ; pairing par code ; pas de GUI |
| App desktop officielle reMarkable | serveur tiers impossible |
| Electron + code web | viole C3 (empreinte) — écarté sur critère, pas sur préférence |
| **Retenu : Tauri 2** | webview système + backend Rust : le crate `fkcloud-core` est appelé sans FFI, C3 tenu |

Justification structurelle : Tauri est le seul candidat où le cœur Rust est
une dépendance directe (zéro couche de binding), ce qui fait du desktop la
plateforme de référence pour valider le crate.

## 4. Exigences

### Fonctionnelles

| ID | Exigence | Acceptation |
|---|---|---|
| F1 | Drag & drop de PDF/EPUB (multiple) sur la fenêtre | dépôt → dialog dossier → upload ; refus motivé des autres types |
| F2 | Choix du dossier + création de dossier | arbre serveur ; `POST folders` vérifié serveur |
| F3 | File d'attente d'envois | progression par fichier ; échec n'interrompt pas la file ; réessai unitaire |
| F4 | Icône de zone de notification (tray) | dépôt sur l'icône ou menu « Envoyer un fichier… » ; fermeture fenêtre ≠ quitter |
| F5 | Configuration | URL + identifiants + test connexion ; erreurs distinctes 401/réseau/HTTP interdit |
| F6 | Session persistante | jeton 23 h, re-login auto sur 401 (comportement du crate, identique aux mobiles) |

### Non fonctionnelles

- **S1** : secrets via crate `keyring` (coffres natifs C4) ; jamais dans un
  fichier de config ni dans les logs.
- **S2** : CSP Tauri stricte ; API exposées au webview limitées aux
  commandes déclarées (allowlist).
- **P1** : upload streaming ; fichier 1 Go sans dépassement mémoire.
- **A11y** : navigation clavier complète ; contrastes AA (charte graphique).
- Hors périmètre v1 : vue bibliothèque (H2), téléchargement, auto-update
  (v1.1 — nécessite signature).

## 5. Preuves de vérification prévues

1. Tests du crate (déjà exigés par `core-m1`) — non redondés.
2. E2E `tauri-driver` : dépôt d'un PDF → assertion API serveur Docker ;
   exécuté en CI sur les 3 cibles (runners win/mac-intel/mac-arm).
3. Test de charge : 10 fichiers en file, dont un corrompu → 9 succès,
   1 échec nommé.
4. Mesures C3 publiées à chaque tag (taille binaire, RSS au repos).

## 6. Trois risques principaux

1. **Signature/notarisation macOS et SmartScreen Windows** (impact fort,
   vraisemblance certaine, détection immédiate) — binaires non signés =
   avertissements bloquants pour l'utilisateur. Réduction : certificat
   développeur + notarisation dès `desktop-m3` (pas après) ; coût récurrent
   nommé (contre-norme : dépendance à des autorités propriétaires pour
   distribuer du logiciel libre ; compensation : hachés SHA-256 publiés).
2. **Divergences webview** (impact moyen, vraisemblance moyenne) — WebView2
   ≠ WKWebView (drag & drop, file API). Réduction : H1 testée au milieu M,
   pas à la fin ; repli egui chiffré avant engagement.
3. **Parité mac Intel réelle** (impact moyen, vraisemblance faible,
   détection difficile sans matériel) — arm64 testé sur machine du
   mainteneur, x86_64 seulement en CI. Réduction : la CI exécute l'E2E
   complet sur runner Intel ; pas de « ça compile donc ça marche ».

## 7. Sources vérifiées

- Contrat API : source rmfakecloud + E2E du 2026-07-16 (ce dépôt).
- Tauri 2 (bundler msi/dmg, tauri-driver, CSP) : https://v2.tauri.app/ .
- crate `keyring` (Keychain/DPAPI/Secret Service) : crates.io/crates/keyring .
- Notarisation macOS : documentation Apple ; SmartScreen : documentation
  Microsoft (connaissance générale, procédures à re-vérifier au jalon m3).
