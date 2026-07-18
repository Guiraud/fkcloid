# Objectif

FkCloud Share doit ressembler à un outil de transfert fiable, pas à une application cloud classique. Son interaction dure quelques secondes. L’identité doit donc privilégier la lisibilité, la vitesse et la confirmation immédiate. Les contraintes retenues sont Material 3, thèmes clair et sombre, diffusion F-Droid, SVG, fonctionnement Android/Desktop et conformité WCAG AA. 

## 1. Convoi éditorial

### Positionnement

Une interface inspirée de l’édition, de la mise en page technique et du papier imprimé. Le fichier est représenté comme un objet qui franchit un passage, et non comme une donnée envoyée vers un nuage.

C’est la proposition la plus ergonomique et la plus équilibrée.

### Signature visuelle

* Fonds ivoire et gris chauds
* Noir légèrement verdi
* Accent vert encre
* Filets fins pour structurer les écrans
* Angles de 4 à 6 dp
* Aucune ombre décorative
* Espaces généreux et alignements typographiques stricts
* Asymétrie légère, proche d’une couverture éditoriale

### Palette

| Rôle            | Thème clair | Thème sombre |
| --------------- | ----------: | -----------: |
| Background      |   `#F5F1E8` |    `#151714` |
| Surface         |   `#FFFDF8` |    `#1D211E` |
| Texte principal |   `#1D211F` |    `#F2EFE7` |
| Primary         |   `#1F5E4A` |    `#7BC7A6` |
| On primary      |   `#FFFFFF` |    `#0E2E22` |
| Error           |   `#A9362B` |    `#FFB4AA` |
| Outline         |   `#726B61` |    `#9B958B` |

Le texte principal atteint un contraste supérieur à 14:1. Le vert primaire dépasse 7:1 sur les surfaces claires.

### Typographie

* Interface et textes: Atkinson Hyperlegible
* URL, serveur, chemin et informations techniques: IBM Plex Mono
* Chiffres tabulaires pour la progression et la taille des fichiers

La distinction entre texte fonctionnel et métadonnées rend l’écran lisible sans ajouter de cartes ou de couleurs.

### Icône

Un document simplifié traverse deux montants verticaux.

```text
│  ▱  │  →
```

Le symbole évoque une passerelle, un transfert et une infrastructure privée. Il reste reconnaissable à 16 px et fonctionne en monochrome dans la barre système.

### Interface Android

L’écran de partage contient seulement:

1. Le nom et la taille des documents
2. Le dossier de destination
3. Le bouton « Envoyer »

La progression remplace directement le bouton. Le succès affiche ensuite:

```text
Transféré dans /Documents/Travail
```

Pas de toast isolé. Pas d’animation de confettis. L’information reste visible jusqu’à la fermeture.

### Élément différenciant

Les séparateurs, filets et blocs typographiques remplacent les grandes cartes arrondies. L’ensemble évoque un outil éditorial libre, pas un tableau de bord produit par un générateur d’interfaces.

---

## 2. Aiguillage souverain

### Positionnement

Une identité inspirée des systèmes de routage, de la signalétique ferroviaire et des étiquettes logistiques. FkCloud Share devient un aiguillage entre un appareil source et une infrastructure privée.

C’est la proposition la plus originale et la plus technique.

### Signature visuelle

* Lignes directionnelles
* Numéros d’étapes
* Codes courts
* Étiquettes rectangulaires
* Flèches épaisses et franches
* Coins de 6 dp
* Contrastes forts
* Information organisée comme un bordereau d’acheminement

### Palette

| Rôle            | Thème clair | Thème sombre |
| --------------- | ----------: | -----------: |
| Background      |   `#F2F0E9` |    `#11161C` |
| Surface         |   `#FFFFFF` |    `#18212A` |
| Texte principal |   `#111827` |    `#F4F0E8` |
| Primary         |   `#1C3D5A` |    `#A8C8DF` |
| On primary      |   `#FFFFFF` |    `#0B2739` |
| Accent          |   `#C14B22` |    `#FF9F6E` |
| Error           |   `#B3261E` |    `#FFB4AB` |

L’orange atteint 4,89:1 sur fond blanc. Il convient aux textes de taille normale selon WCAG AA, mais reste réservé aux actions ou indications importantes.

### Typographie

* Interface: IBM Plex Sans
* Adresses, ports, chemins et statuts: IBM Plex Mono
* Capitales uniquement pour les micro-étiquettes courtes

Exemple:

```text
SOURCE
Pixel 9

DESTINATION
fkcloud.local → /Presse
```

### Icône

Un document suit une ligne et traverse un embranchement:

```text
▱━━┳━━▶
    ┗━━
```

La forme évite totalement le nuage et donne une identité propre au projet.

### Interface Android

Le transfert est représenté comme un trajet:

```text
TÉLÉPHONE     SERVEUR       TABLETTE
    ●────────────●────────────○
```

L’état final remplit le dernier point. En cas d’échec, l’interface indique l’endroit précis:

```text
Serveur joint
Authentification refusée
```

Cette approche évite le message imprécis « Une erreur est survenue ».

### Élément différenciant

La grammaire visuelle repose sur le routage, les destinations et les statuts. Elle reflète exactement le rôle du produit. Elle ne ressemble ni à un service SaaS ni à une application de stockage.

### Risque à contrôler

Les codes, lignes et étiquettes deviennent vite envahissants. Limite recommandée:

* Deux niveaux typographiques par écran
* Une seule ligne de trajet
* Un seul accent orange visible
* Aucun encadrement autour des informations secondaires

---

## 3. Registre libre

### Positionnement

Une identité issue des fichiers de bibliothèque, des chemises cartonnées et des fiches d’archives. Le produit gère une destination documentaire, sans se présenter comme un gestionnaire de fichiers complet.

C’est la proposition la plus chaleureuse et la plus communautaire.

### Signature visuelle

* Onglets de classement
* Lignes horizontales rappelant une fiche
* États signalés par des tampons typographiques
* Surfaces crème
* Bleu de classement
* Rouge brun pour les alertes et validations éditoriales
* Coins de 2 à 4 dp

### Palette

| Rôle            | Thème clair | Thème sombre |
| --------------- | ----------: | -----------: |
| Background      |   `#F6F2E9` |    `#171615` |
| Surface         |   `#FFFCF5` |    `#201E1A` |
| Texte principal |   `#22201C` |    `#F4EEE3` |
| Primary         |   `#345A7E` |    `#AFC7E1` |
| On primary      |   `#FFFFFF` |    `#10253A` |
| Accent          |   `#8E3B2E` |    `#E7A397` |
| Error           |   `#A9362B` |    `#FFB4AA` |

Le bleu primaire et le rouge brun dépassent 7:1 sur les surfaces claires.

### Typographie

* Interface: Source Sans 3
* Titres et écrans vides: Literata
* Métadonnées: Source Code Pro

La fonte à empattements reste limitée aux titres. Les contrôles et libellés conservent une fonte sans empattements.

### Icône

Une fiche rectangulaire entre dans un casier:

```text
▱  →  ▥
```

La forme fonctionne en version pleine pour l’icône Android et en contour pour la zone de notification.

### Interface Android

Le sélecteur de destination reprend une structure d’index:

```text
Récents
01  Presse
02  Administration
03  À lire

Tous les dossiers
```

Le succès prend la forme d’un libellé discret:

```text
CLASSÉ · /Presse/Enquêtes
```

### Élément différenciant

Le produit adopte une culture visuelle documentaire plutôt qu’une esthétique technologique. Cette direction correspond bien au logiciel libre et au public F-Droid.

### Risque à contrôler

La texture papier ne doit pas apparaître derrière les champs, les listes ou les boutons. Elle reste limitée à l’icône, aux illustrations d’état vide et à la fiche F-Droid. Sinon, la lisibilité baisse et l’interface devient décorative.

---

# Comparaison

| Critère                   | Convoi éditorial | Aiguillage souverain | Registre libre |
| ------------------------- | ---------------: | -------------------: | -------------: |
| Rapidité de lecture       |              5/5 |                4,5/5 |            4/5 |
| Différenciation           |            4,5/5 |                  5/5 |          4,5/5 |
| Confiance et sobriété     |              5/5 |                4,5/5 |          4,5/5 |
| Adaptation Android        |              5/5 |                4,5/5 |            4/5 |
| Adaptation Desktop        |              5/5 |                  5/5 |          4,5/5 |
| Complexité de réalisation |           Faible |              Moyenne |        Moyenne |
| Risque de surcharge       |           Faible |                Moyen |          Moyen |

# Recommandation

Je retiendrais Convoi éditorial comme système principal.

Il offre le meilleur équilibre entre:

* rapidité d’usage
* sobriété e-ink
* accessibilité
* compatibilité Material 3
* fonctionnement à petite taille
* distinction face aux interfaces génériques

J’ajouterais un seul élément issu d’Aiguillage souverain: la ligne de progression entre appareil, serveur et tablette.

Le résultat donnerait une identité cohérente:

```text
Convoi éditorial
+
visualisation du trajet
+
aucun symbole de nuage
```

# Règles communes aux trois directions

Pour éviter l’apparence typique des interfaces générées par IA:

| À conserver                       | À exclure                                      |
| --------------------------------- | ---------------------------------------------- |
| Grille de 8 dp                    | Dégradés violet et cyan                        |
| Zones tactiles de 48 dp minimum   | Glassmorphism                                  |
| Largeur de texte limitée          | Cartes arrondies imbriquées                    |
| Une action primaire par écran     | Illustrations 3D génériques                    |
| Filets et espaces pour structurer | Ombres diffuses                                |
| Icônes dessinées pour le produit  | Bibliothèque d’icônes utilisée sans adaptation |
| Animations de 120 à 180 ms        | Rebonds et célébrations                        |
| Erreurs précises et correctives   | « Une erreur est survenue »                    |

# Première version à prototyper

L’écran de partage pourrait suivre cette structure:

```text
ENVOI VERS FKCloud

rapport-enquete.pdf
2,4 Mo

Destination
/Presse/Enquêtes                  Modifier

Téléphone ━━━━━ Serveur ━━━━━ Tablette

[ Envoyer le document ]
```

Après déclenchement:

```text
rapport-enquete.pdf

Téléphone ━━━━━ Serveur ━━━━━ Tablette
     ●━━━━━━━━━━━━●━━━━━━━━━━━━●

Transféré dans /Presse/Enquêtes

[ Fermer ]
```

Votre brief décrit correctement le produit comme un convoyeur discret. L’icône actuelle en forme de nuage contredit toutefois ce positionnement et rapproche FkCloud Share des services commerciaux qu’il cherche à remplacer.

L’amélioration prioritaire consiste à dessiner les trois icônes en monochrome à 16, 24 et 48 px avant les écrans complets. Une identité qui échoue à 16 px échouera aussi dans la barre système et le menu de partage.

Confiance: 0,94.
