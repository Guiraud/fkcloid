# Spécification API partagée — rmfakecloud (web UI API)

English: [`api-rmfakecloud.md`](api-rmfakecloud.md).

Contrat commun aux trois interfaces (Android, iOS, Desktop). Chaque client
implémente exactement ce protocole ; toute divergence se corrige ici d'abord.
Vérifié contre le source de [ddvk/rmfakecloud](https://github.com/ddvk/rmfakecloud)
(`internal/ui/routes.go`, `handlers.go`, `middleware.go`) et testé en réel
(émulateur Android + serveur Docker, 2026-07-16).

Serveur de référence : `https://rm-cloud.example.invalid`.

## Authentification

| Étape | Détail |
|-------|--------|
| Login | `POST /ui/api/login`, JSON `{"email": "...", "password": "..."}` |
| Réponse | Corps texte brut = JWT (pas de JSON), validité 24 h |
| Usage | Header `Authorization: Bearer <jwt>` sur tous les appels suivants |
| Expiration | Recommandation client : cache 23 h, re-login automatique sur 401 |
| Erreurs | `401` identifiants invalides, `400` JSON malformé |

Note : sur un serveur vierge (zéro utilisateur), le premier login **crée**
le compte admin avec les identifiants fournis.

## Arbre des documents

`GET /ui/api/documents` →

```json
{
  "Entries": [
    { "id": "uuid", "name": "…", "isFolder": true, "children": [ … ],
      "lastModified": "RFC3339" },
    { "id": "uuid", "name": "…", "type": "pdf|epub|notebook",
      "size": 123, "lastModified": "RFC3339" }
  ],
  "Trash": [ … ]
}
```

Clés racine `Entries`/`Trash` en **PascalCase** (structs Go sans tags) ;
clés des entrées en camelCase. Dossiers = `isFolder: true` + `children`.

## Envoi de document

`POST /ui/api/documents/upload` — multipart/form-data :

| Champ | Valeur |
|-------|--------|
| `parent` | id du dossier destination, ou `root` pour la racine |
| `file` | fichier binaire, nom de fichier avec extension `.pdf` / `.epub` (répétable pour envoi multiple) |

Réponses : `200` OK, `409` document déjà existant (`{"error": …, "docId": …}`),
`401` jeton invalide/expiré, `400` multipart invalide.

**Important :** le nom de fichier doit porter l'extension — le serveur en
déduit le type. Si le fournisseur de contenu ne donne qu'un id opaque,
dériver l'extension du type MIME (`application/pdf` → `.pdf`,
`application/epub+zip` → `.epub`).

## Autres endpoints utiles (non implémentés côté clients pour l'instant)

- `GET /ui/api/documents/:docid?type=pdf` — export/téléchargement
- `DELETE /ui/api/documents/:docid` — suppression
- `PUT /ui/api/documents` — déplacement/renommage
- `POST /ui/api/folders` — création de dossier
- `GET /ui/api/sync` — notifier la tablette (rafraîchissement)

## Politique de sécurité commune aux clients

1. HTTPS obligatoire par défaut ; HTTP uniquement sur opt-in explicite
   de l'utilisateur (usage LAN), avec avertissement visible.
2. Mot de passe et JWT stockés chiffrés via le coffre natif de la
   plateforme : Android Keystore (AES-256-GCM), iOS Keychain,
   Desktop : Keychain macOS / DPAPI Windows / Secret Service Linux.
3. Jamais de secret en clair dans logs, backups ou fichiers de config.
4. Fichiers transmis en streaming, sans copie temporaire persistante.
