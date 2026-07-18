# Signature et Notarisation de FkCloud Share (Tauri 2)

Ce document décrit la procédure de signature de code et de notarisation pour distribuer FkCloud Share en production pour macOS et Windows sans avertissements de sécurité (SmartScreen, Gatekeeper).

---

## 1. Distribution macOS (Gatekeeper & Notarisation)

Pour macOS, l'application doit être signée avec un certificat de développeur Apple et notarisée (soumise aux serveurs d'Apple pour analyse de sécurité) afin de s'exécuter sans message d'avertissement de blocage.

### Prérequis
- Un compte **Apple Developer** payant (99$/an).
- Une machine macOS ou un runner macOS équipé de Xcode.
- Un certificat de type **Developer ID Application** installé dans le trousseau d'accès du runner.
- Un identifiant Apple ID avec un mot de passe d'application dédié pour la notarisation.

### Variables d'environnement pour le build
Lors du lancement du build Tauri, définissez les variables suivantes dans votre environnement de CI :

```bash
# Nom exact du certificat dans votre Trousseau (Keychain)
export APPLE_SIGNING_IDENTITY="Developer ID Application: Votre Nom/Entreprise (TEAMID)"

# Identifiant de votre compte Apple Developer
export APPLE_ID="votre.email@compte-dev.apple.com"

# Mot de passe d'application généré sur appleid.apple.com
export APPLE_PASSWORD="abcd-efgh-ijkl-mnop"

# Votre Team ID Apple Developer
export APPLE_TEAM_ID="TEAMID1234"
```

### Entitlements (Droits d'accès)
Tauri gère automatiquement la notarisation via `notarytool` lors de l'appel à `tauri build` si les variables ci-dessus sont présentes. Si vous utilisez la notarisation, configurez les droits d'accès réseau si l'App Sandbox est activé. 

---

## 2. Distribution Windows (SmartScreen)

Pour Windows, le programme d'installation MSI doit être signé avec un certificat de signature de code (idéalement de type EV - Extended Validation pour supprimer instantanément le filtre Microsoft SmartScreen).

### Prérequis
- Un certificat de signature de code Windows (fichier `.pfx` ou via un module HSM/Cloud).
- L'outil `signtool.exe` (inclus dans le Windows SDK).

### Configuration de Tauri pour Windows
Dans `tauri.conf.json`, vous pouvez configurer l'adresse du serveur de signature d'horodatage :

```json
{
  "bundle": {
    "windows": {
      "digestAlgorithm": "sha256",
      "timestampUrl": "http://timestamp.digicert.com"
    }
  }
}
```

### Variables de build (SignTool)
Définissez ces variables sur votre runner Windows avant le build :

```powershell
# Chemin local vers le fichier de certificat PFX
$env:TAURI_SIGNING_IDENTITY = "C:\Certificates\my-code-signing.pfx"

# Mot de passe protégeant le certificat PFX
$env:TAURI_SIGNING_IDENTITY_PASSWORD = "MonMotDePasseSecret"
```

---

## 3. Publication et Intégration CI (GitLab CI/CD)

Le script de build de production générera automatiquement les binaires finaux et calculera les hachages SHA-256 pour des raisons de traçabilité et de sécurité.

### Commande de build
```bash
npm run tauri build
```

### Calcul automatique des sommes de contrôle (SHA-256)
Après le build, générez les hachages dans le répertoire des artefacts :

```bash
# macOS
shasum -a 256 src-tauri/target/release/bundle/dmg/*.dmg > SHA256SUMS.txt

# Windows
certutil -hashfile src-tauri/target/release/bundle/msi/*.msi SHA256 > SHA256SUMS.txt
```
Ces sommes de contrôle doivent être publiées sur la page de version (Release) de votre dépôt GitLab ou site web.
