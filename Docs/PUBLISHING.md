# Public publication

French: [PUBLISHING.fr.md](PUBLISHING.fr.md).

## Language policy

- **Canonical public docs** are in **English** (`README.md` and `Docs/*.md` except `*.fr.md`).
- French copies use the `.fr.md` suffix (or root `Lisezmoi.md`).
- Product PRDs/roadmaps ship EN + `.fr.md`.

## Never commit

- `VAULT_LOCAL.md`, `handoff.md`, `.continues-handoff.md`
- Personal server URLs, LAN IPs, emails, credentials
- `*.keystore`, `*.jks`, `.env`

## Placeholders

| Local | Public |
|-------|--------|
| rmfakecloud server | `https://rm-cloud.example.invalid` |
| Username | `user.example` |
| Android / Tauri package | `net.example.fkcloud` |

## Git history

History was rewritten (`git filter-repo`) before publication.

## GitHub repository

Remote: `git@github.com:Guiraud/fkcloid.git`

```bash
git remote add origin git@github.com:Guiraud/fkcloid.git
gh repo create Guiraud/fkcloid --public --source=. --remote=origin --push
# or:
git push -u --force origin main
```
