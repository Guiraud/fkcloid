# FkCloud Share — Desktop

Client desktop (Windows, macOS Intel & Apple Silicon) pour envoyer des PDF/EPUB vers un serveur rmfakecloud.
Stack : [Tauri 2](https://tauri.app/) + Rust (`fkcloud-core`) + HTML/CSS/JS vanilla (pas de framework front).

## Fonctionnalités

- Connexion à un serveur rmfakecloud (login, session persistée via keyring OS).
- Envoi de fichiers par glisser-déposer ou sélecteur de fichier natif.
- Navigation dans l'arborescence de dossiers du serveur, création de dossier.
- Icône de tray avec menu (afficher/masquer, envoyer un fichier, quitter), pastille d'activité tablette.
- Intégration menu contextuel OS : "Envoyer vers Paper pure" dans Finder (macOS, service Automator) et clic droit Explorer (Windows, clé de registre).
- Démarrage automatique (autostart) configurable dans les réglages.

## Commandes

```bash
npm install
npm run tauri dev            # mode développement
npm run tauri build          # build de production
npm run tauri build -- --debug   # bundle de debug local
```

## Point d'attention

Le menu contextuel OS ("Envoyer vers Paper pure") exécute le binaire directement avec `--upload <chemin>`,
en contournant LaunchServices/l'ouverture normale de l'app. Sans `tauri-plugin-single-instance`, cela pouvait
faire démarrer un second processus (fenêtre blanche) si l'app tournait déjà en tray. Le plugin est enregistré
en premier dans `src-tauri/src/lib.rs` : un second lancement transmet `--upload` à l'instance déjà active via
l'événement `upload-file-selected` au lieu d'ouvrir une nouvelle fenêtre.
