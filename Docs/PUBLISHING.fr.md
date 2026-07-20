# Publication publique


## Politique de langue

- Les docs publiques **canoniques** sont en **anglais** (`*.md` hors `*.fr.md`, et `README.md`).
- Les copies françaises portent le suffixe `.fr.md` (ou `Lisezmoi.md` à la racine).

English: [`PUBLISHING.md`](PUBLISHING.md).

## Fichiers à ne jamais ajouter

- `VAULT_LOCAL.md`, `handoff.md`, `.continues-handoff.md`
- URLs serveur personnelles, IP LAN, emails, identifiants
- `*.keystore`, `*.jks`, `.env`

## Placeholders

| Local | Public |
|-------|--------|
| Serveur rmfakecloud | `https://rm-cloud.example.invalid` |
| Identifiant | `user.example` |
| Package Android / Tauri | `net.example.fkcloud` |

## Historique git

L'historique a été réécrit (`git filter-repo`) avant publication.

## Dépôt GitHub

Remote cible : `git@github.com:Guiraud/fkcloid.git`

```bash
git remote add origin git@github.com:Guiraud/fkcloid.git
gh repo create Guiraud/fkcloid --public --source=. --remote=origin --push
# ou :
git push -u --force origin main
```
