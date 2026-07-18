# FkCloud Share — iOS (à venir)

Client iOS de partage vers rmfakecloud. Même rôle que l'app Android :
apparaître dans la feuille de partage, choisir un dossier, envoyer.

## Stack prévue

- **Swift 5 / SwiftUI**, cible iOS 16+
- **Share Extension** (`NSExtensionActivationRule` : PDF + EPUB) — équivalent
  du `ACTION_SEND` Android
- App conteneur : configuration serveur (URL, identifiants, test connexion)
- Secrets dans le **Keychain** (kSecAttrAccessibleAfterFirstUnlockThisDeviceOnly)
- Protocole : crate partagé `fkcloud-core` (Rust) via UniFFI/XCFramework —
  voir [Docs/product/core-rust.md](../../Docs/product/core-rust.md) ;
  repli Swift natif si l'hypothèse H1 du PRD est invalidée au jalon m1

PRD : [Docs/product/ios/PRD.md](../../Docs/product/ios/PRD.md) ·
Roadmap : [Docs/product/ios/ROADMAP.md](../../Docs/product/ios/ROADMAP.md)

## Contrat API

Implémenter strictement [Docs/api-rmfakecloud.md](../../Docs/api-rmfakecloud.md)
(login JWT, arbre documents, upload multipart `parent` + `file`).

## Structure cible

```
apps/ios/
  FkCloudShare.xcodeproj
  FkCloudShare/          # app conteneur (config)
  ShareExtension/        # extension de partage
  Shared/                # RmfcClient.swift, Session.swift, SecureStore.swift
```

## Distribution

App Store ou sideload (AltStore/TrollStore) — pas d'équivalent F-Droid.
Compte développeur Apple requis pour la distribution signée.
