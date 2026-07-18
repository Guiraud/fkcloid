# Brief charte graphique — FkCloud Share

## 1. Identité produit

**Nom actuel :** FkCloud Share (modifiable si la charte impose un naming).
**Baseline possible :** « Vos documents, votre cloud, votre reMarkable. »

**Ce que fait l'application :** envoyer des documents (PDF, EPUB) vers un
serveur **rmfakecloud** auto-hébergé — l'alternative libre au cloud officiel
de la tablette e-ink reMarkable. Sur Android, l'application vit dans le
**menu de partage** : on partage un fichier depuis n'importe quelle app, on
choisit un dossier de destination, il apparaît sur la tablette. La version
Desktop (à venir) fera la même chose par glisser-déposer et gestion de
bibliothèque.

**Ce que l'application n'est pas :** pas un lecteur de documents, pas un
éditeur, pas un service cloud commercial. C'est un **convoyeur** discret et
fiable entre le téléphone/l'ordinateur et la tablette.

## 2. Public cible

- Possesseurs de tablette reMarkable qui refusent le cloud officiel
  (abonnement, données chez un tiers) et auto-hébergent rmfakecloud.
- Profil : technophiles, self-hosters, sensibles à la vie privée et au
  logiciel libre. Utilisateurs de F-Droid, Docker, home-labs.
- Usage : ponctuel et utilitaire — quelques secondes par interaction.
  L'app doit être invisible quand tout va bien.

## 3. Valeurs à traduire visuellement

1. **Souveraineté / confiance** — vos fichiers ne quittent votre
   infrastructure que chiffrés, vers VOTRE serveur. La charte doit inspirer
   solidité et sécurité, sans tomber dans l'imagerie « cadenas/hacker ».
2. **Sobriété papier / e-ink** — l'univers reMarkable : noir, blanc, gris
   chauds, textures papier, encre. Fort contraste, pas de dégradés criards.
3. **Minimalisme fonctionnel** — deux écrans, trois boutons. La charte doit
   assumer le vide et la typographie plutôt que la décoration.
4. **Libre / communautaire** — publiable sur F-Droid : aucune ressource
   propriétaire, aucune imitation de la marque reMarkable AS (mention
   « non affilié » obligatoire).

## 4. Existant (point de départ, tout est modifiable)

- **Couleur primaire actuelle :** `#1E5EFF` (bleu vif) — héritée du badge
  du dépôt, pas un choix de marque définitif.
- **Icône actuelle :** nuage blanc + flèche montante sur fond bleu
  (adaptive icon Android).
- **UI Android :** Material 3 DayNight (clair + sombre obligatoires),
  composants Material (champs texte, boutons pleins, checkbox, spinner).
- **Langues :** français + anglais.

## 5. Surfaces à couvrir par la charte

### Android
- Icône launcher **adaptive icon** : zone 108×108 dp, contenu significatif
  dans le cercle de sécurité de 66 dp ; déclinaison monochrome (thèmes
  Android 13+) à prévoir.
- Écran de configuration (URL serveur, identifiants, test de connexion).
- Boîte de dialogue de partage (liste fichiers, sélecteur de dossier,
  progression, états d'erreur).
- États : succès (✔ connecté), erreur (rouge accessible), avertissement.
- Fiche F-Droid : bannière/« feature graphic » optionnelle, captures.

### Desktop (application future)
- Icône multi-tailles (16→512 px, macOS/Windows/Linux).
- Fenêtre principale : zone de glisser-déposer, liste de la bibliothèque,
  file d'envois.
- Icône de zone de notification (tray) monochrome.
- Mêmes thèmes clair/sombre.

## 6. Contraintes techniques pour la charte

- **Contraste :** WCAG AA minimum sur tous les couples texte/fond,
  y compris le rouge d'erreur sur fond sombre.
- **Formats livrables :** SVG vectoriel source ; icône Android en vector
  drawable (pas de PNG multi-densités si évitable) ; palette en tokens
  (hex + rôles : primary, on-primary, surface, error…) compatibles
  Material 3 / thèmes dynamiques.
- **Typographie :** libre de droits et embarquable (licence OFL) ou
  système (Roboto/Inter par défaut). Éviter toute fonte propriétaire.
- **Aucune ressource** issue de la marque reMarkable (logo, gris
  signature, photos produit).

## 7. Ambiances suggérées (pistes, non imposées)

- « Papier & encre » : fond blanc cassé / noir doux, une seule couleur
  d'accent (le bleu actuel ou un vert encre), iconographie trait fin.
- « Nuage souverain » : le motif nuage + flèche décliné en monogramme
  géométrique simple, utilisable en tray icon 16 px.
- Micro-illustrations façon croquis (clin d'œil au stylet reMarkable)
  pour les états vides et les erreurs.
