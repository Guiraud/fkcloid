# Handoff — FkCloud Share Desktop (Rust / Tauri v2)

Ce document résume le travail effectué et définit les prochaines étapes pour continuer l'implémentation du client Desktop en Rust/Tauri.

---

## 1. Travaux Réalisés

### 🔒 Configuration & Vault
- **Décryptage et extraction des identifiants :** Les variables d'environnement du coffre Ansible Vault ont été décryptées et importées dans le fichier `~/.env` local.
- **Support multi-formats :** Chaque credential est exposé sous sa forme originale en minuscules et sous la forme standardisée en majuscules (ex: `BRAVE_API_KEY`, `GITLAB_TOKEN`, `CLOUDFLARE_API_TOKEN_PL`, etc.).

### 📋 Cadrage et Plan d'implémentation
- Création du plan d'implémentation complet dans le fichier d'artefact : [implementation_plan_desktop_rust.md](file:///Users/user.example/.gemini/antigravity-cli/brain/c0a25ffa-c569-4c69-a7ac-0b51a2e57819/implementation_plan_desktop_rust.md).
- **Mise à jour de la Roadmap :** Modification de [ROADMAP.md](file:///Users/user.example/Documents/Gitlab/Mehdiguiraud-tld/ReMarkable-tools/fkcloid_example_net/Docs/product/desktop/ROADMAP.md) pour intégrer :
  - **v1.1 :** Intégration de KeePassXC via socket local / protocole NaCl (avec les crates `kpx` ou `keepassxc-proxy-lib`) pour la gestion sécurisée des credentials.
  - **v1.2 :** Intégration dans le menu contextuel natif de l'OS (clic droit "Envoyer vers FkCloud" sur Windows, macOS et Linux) et appel à `/ui/api/sync` après upload.

### 🦀 Initialisation du socle Rust (`core`)
- Création du crate bibliothèque Cargo `fkcloud-core` dans le dossier `core/`.
- Configuration des dépendances de base dans [core/Cargo.toml](file:///Users/user.example/Documents/Gitlab/Mehdiguiraud-tld/ReMarkable-tools/fkcloid_example_net/core/Cargo.toml) :
  - `reqwest` (compilé avec `rustls-tls` et sans OpenSSL natif pour des raisons de portabilité).
  - `serde` et `serde_json` pour le parsing des API.
  - `thiserror` pour les erreurs typées.
- Création du modèle de données de l'arbre et des erreurs d'API dans [core/src/model.rs](file:///Users/user.example/Documents/Gitlab/Mehdiguiraud-tld/ReMarkable-tools/fkcloid_example_net/core/src/model.rs).
- Création de la gestion de session et de la politique HTTPS/HTTP sécurisée dans [core/src/session.rs](file:///Users/user.example/Documents/Gitlab/Mehdiguiraud-tld/ReMarkable-tools/fkcloid_example_net/core/src/session.rs).

---

## 2. Prochaines Étapes pour l'implémentation (Jalon 1 - `desktop-m1`)

L'agent reprenant le projet devra finaliser le **Jalon 1 (Quart A-M : Validation CLI)** :

1. **Créer `core/src/client.rs` :**
   Implémenter le client `RmfcClient` :
   - Requête de Login (`POST /ui/api/login` -> retourne JWT brut).
   - Récupération de l'arbre (`GET /ui/api/documents` -> désérialise `DocumentTree`).
   - Upload de document (`POST /ui/api/documents/upload` en Multipart).
   - Mécanisme de re-login automatique transparent sur erreur HTTP `401`.

2. **Créer `core/src/lib.rs` :**
   Exposer proprement les modules `model`, `session`, et `client`.

3. **Implémenter l'utilitaire CLI `fkcloud-cli` :**
   Ajouter un point d'entrée binaire sous `core/src/bin/fkcloud-cli.rs` (ou `core/src/main.rs` si besoin) pour permettre à l'utilisateur de tester et valider le cœur en ligne de commande :
   ```bash
   # Exemples de commandes cibles :
   fkcloud-cli login <url> <email>
   fkcloud-cli ls
   fkcloud-cli put <chemin_fichier> --folder <parent_id>
   ```

4. **Lancer les tests et valider le jalon :**
   Vérifier le bon fonctionnement du binaire `fkcloud-cli` en le compilant et en l'exécutant contre une instance locale `rmfakecloud` (ou via le serveur de référence `https://rm-cloud.example.invalid`).
