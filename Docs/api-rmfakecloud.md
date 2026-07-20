# Shared API specification — rmfakecloud (web UI API)

French: [api-rmfakecloud.fr.md](api-rmfakecloud.fr.md).

Common contract for the three clients (Android, iOS, Desktop). Each client
implements this protocol exactly; any divergence is fixed here first.
Checked against [ddvk/rmfakecloud](https://github.com/ddvk/rmfakecloud)
(`internal/ui/routes.go`, `handlers.go`, `middleware.go`) and tested live
(Android emulator + Docker server, 2026-07-16).

Example server: `https://rm-cloud.example.invalid`.

## Authentication

| Step | Detail |
|------|--------|
| Login | `POST /ui/api/login`, JSON `{"email": "...", "password": "..."}` |
| Response | Raw text body = JWT (not JSON), valid 24 h |
| Usage | `Authorization: Bearer <jwt>` on all later calls |
| Expiry | Client recommendation: cache 23 h, auto re-login on 401 |
| Errors | `401` bad credentials, `400` malformed JSON |

Note: on a blank server (no users), the first login **creates** the admin
account with the credentials supplied.

## Document tree

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

Root keys `Entries`/`Trash` are **PascalCase** (Go structs without tags);
entry keys are camelCase. Folders = `isFolder: true` + `children`.

## Document upload

`POST /ui/api/documents/upload` — multipart/form-data:

| Field | Value |
|-------|--------|
| `parent` | destination folder id, or `root` for the library root |
| `file` | binary file, filename with `.pdf` / `.epub` extension (repeatable for multi-upload) |

Responses: `200` OK, `409` document already exists (`{"error": …, "docId": …}`),
`401` invalid/expired token, `400` invalid multipart.

**Important:** the filename must carry the extension — the server derives the
type from it. If the content provider only gives an opaque id, derive the
extension from the MIME type (`application/pdf` → `.pdf`,
`application/epub+zip` → `.epub`).

## Other useful endpoints (not yet wired in clients)

- `GET /ui/api/documents/:docid?type=pdf` — export/download
- `DELETE /ui/api/documents/:docid` — delete
- `PUT /ui/api/documents` — move/rename
- `POST /ui/api/folders` — create folder
- `GET /ui/api/sync` — notify the tablet (refresh)

## Shared client security policy

1. HTTPS required by default; HTTP only on explicit user opt-in (LAN use),
   with a visible warning.
2. Password and JWT stored encrypted via the platform vault: Android Keystore
   (AES-256-GCM), iOS Keychain, Desktop: macOS Keychain / Windows DPAPI /
   Linux Secret Service.
3. Never log or backup secrets in clear text, nor write them to config files.
4. Stream uploads; no persistent temporary copies of user files.
