# Roadmap — FkCloud Share Desktop (bissection progressive)

English: [`ROADMAP.md`](ROADMAP.md).

Axe : **borne A** = rien → **borne B** = v1.0 signée, distribuée pour
win-x64, mac-x64, mac-arm64. Pas minimal : jalon ≤ 1 semaine, vérification
binaire. Chaque jalon = tag git.

## Passes de bissection

- **Passe 1 — bornes** : A (zéro code desktop) et B (v1.0 trois cibles).
- **Passe 2 — milieu M** : *fenêtre Tauri qui envoie* — drag & drop d'un
  PDF → dossier → upload vérifié serveur, sur l'OS de développement
  (mac-arm64). Coupe l'espace : avant = socle Rust/CLI, après =
  multi-plateforme, tray, packaging.
- **Passe 3 — quarts** :
  - A–M : *CLI de validation* — binaire `fkcloud-cli` (login, ls, put)
    au-dessus du crate ; prouve le cœur sans UI. H1 webview testée ici
    sur les 3 OS (page blanche + drop test).
  - M–B : *trois cibles empaquetées* — msi + 2 dmg non signés, E2E CI vert
    partout.
- **Passe 4** : tray, file d'attente, signature — écarts ≤ 1 semaine, arrêt.

## Jalons (ordre grossier → fin)

| Tag | Jalon | Statut | Vérification binaire |
|---|---|---|---|
| — | Borne A | Fait | `apps/desktop/` : README seul (état initial) |
| `desktop-m1` | Quart A–M : `fkcloud-cli` | Fait | `fkcloud-cli` fonctionnel (login, ls, put, sync) ; tests core ok |
| `desktop-m2` | **Milieu M** : fenêtre Tauri drag & drop | Fait | Fenêtre Tauri D&D, arbre de dossiers, création en ligne |
| `desktop-m3` | Quart M–B : file d'attente & tray icon | Fait | File d'attente asynchrone unitaire (F3), systray icon, menu, dialogs native (F4) |
| `desktop-v1.0` | Borne B : CI & publication | Fait | Paramètres (auto-démarrage, menu contextuel), tray icon glow actif, À propos signature, CI & publication |

## Segments de risque

Casse à `m3` → tester `m2` (milieu) d'abord ; moitié fautive en un essai.
Le CLI `desktop-m1` reste dans le dépôt en permanence : c'est l'outil de
bissection le plus rapide pour distinguer « bug du crate » de « bug de l'UI ».

## Après B (backlog ordonné par version)

### Jalon v1.1 : Intégrations & Améliorations
1. **Intégration KeePassXC** : Option permettant de stocker/récupérer les identifiants via le protocole local de KeePassXC (ou intégration D-Bus Secret Service sur Linux).
2. **Vue bibliothèque** : Si H2 du PRD est confirmée par les retours utilisateurs.
3. **Auto-update signé** (tauri-updater).
4. **Cible Linux officielle** (AppImage/Flatpak) — avec engagement de QA complet.

### Jalon v1.2 : Intégration Système Globale
1. **Envoi via Clic Droit (Context Menu OS)** : Intégration dans le menu contextuel natif du gestionnaire de fichiers de l'OS (Explorateur Windows, Finder macOS, gestionnaires Linux type Nautilus/Dolphin) pour envoyer un PDF/EPUB directement via clic droit sans ouvrir l'interface principale.
2. **`GET /ui/api/sync` post-upload** : Notification de synchronisation immédiate envoyée à la tablette.

