---
name: bottom-signature
description: Intègre la signature professionnelle de Mehdi Guiraud (bandeau de don + footer) dans les projets web
---

# Bottom Signature - Composant de Signature Réutilisable

## Quand Activer

- Lors de la création d'un nouveau projet web pour Mehdi Guiraud
- Quand l'utilisateur demande d'ajouter une signature ou un footer
- Quand l'utilisateur mentionne "bottom-signature", "signature", ou "donation ribbon"
- Pour tout site web nécessitant identification de l'auteur et possibilité de don

## Description

**Bottom Signature** est un composant HTML/CSS/JS qui ajoute :

1. **Bandeau de don diagonal** (coin supérieur gauche) avec lien PayPal
2. **Footer professionnel** avec :
   - Liens vers les réseaux sociaux (GitLab, Twitter, Email, PayPal)
   - Copyright automatique avec année courante
   - Message incitant aux dons
   - Barre de couleur décorative

## Fichiers du Composant

### Structure

```
web/
├── signature.html   # Markup HTML de la signature
├── signature.css    # Styles CSS avec variables personnalisables
└── signature.js     # Script d'initialisation (année auto, API)
```

### signature.html

```html
<!-- Bandeau de don en biais (coin supérieur gauche) -->
<a href="https://www.paypal.com/paypalme/mehdiguiraud"
   target="_blank"
   rel="noopener noreferrer"
   class="donation-ribbon"
   title="Soutenez ce projet via PayPal"
   aria-label="Faire un don via PayPal">
    <i class="fas fa-heart"></i>
    <span class="ribbon-text">Soutenez ce projet</span>
</a>

<!-- Footer de signature -->
<footer class="signature-footer">
    <!-- Barre de couleur décorative -->
    <div class="footer-decoration"></div>

    <!-- Liens sociaux -->
    <div class="social-links">
        <a href="https://github.com/Guiraud"
           target="_blank"
           rel="noopener noreferrer"
           title="GitHub - Code source"
           class="social-icon github-icon"
           aria-label="Voir le profil GitHub">
            <i class="fab fa-github"></i>
        </a>
        <a href="https://twitter.com/mguiraud"
           target="_blank"
           rel="noopener noreferrer"
           title="Twitter @mguiraud"
           class="social-icon twitter-icon"
           aria-label="Suivre sur Twitter">
            <i class="fab fa-twitter"></i>
        </a>
        <a href="mailto:lecourrieldemehdi@gmail.com"
           title="Email: lecourrieldemehdi@gmail.com"
           class="social-icon email-icon"
           aria-label="Envoyer un email">
            <i class="fas fa-envelope"></i>
        </a>
        <a href="https://www.paypal.com/paypalme/mehdiguiraud"
           target="_blank"
           rel="noopener noreferrer"
           title="Faire un don via PayPal"
           class="social-icon paypal-icon"
           aria-label="Faire un don via PayPal">
            <i class="fab fa-paypal"></i>
        </a>
    </div>

    <!-- Copyright -->
    <div class="copyright">
        &copy; <span class="copyright-year"></span> Mehdi Guiraud - Tous droits réservés
    </div>

    <!-- Message de don -->
    <div class="donation-text">
        Si vous appréciez cet outil, vous pouvez
        <a href="https://www.paypal.com/paypalme/mehdiguiraud"
           target="_blank"
           rel="noopener noreferrer">faire un don</a>
    </div>
</footer>
```

### Variables CSS Personnalisables

```css
:root {
    --signature-primary-color: #4361ee;
    --signature-secondary-color: #48cae4;
    --signature-accent-color: #ff9e00;
    --signature-text-color: #333;
    --signature-text-light: #666;
    --signature-bg-color: #f8f9fa;
}
```

### API JavaScript

```javascript
window.BottomSignature.init()        // Réinitialise le composant
window.BottomSignature.updateYear()  // Met à jour l'année du copyright
window.BottomSignature.load(selector, path)  // Charge dynamiquement
```

## Instructions d'Intégration

### Étape 1 : Ajouter les dépendances dans `<head>`

```html
<!-- Font Awesome pour les icônes -->
<link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.2/css/all.min.css">
<!-- Styles de la signature -->
<link rel="stylesheet" href="signature.css">
```

### Étape 2 : Ajouter le bandeau après `<body>`

```html
<body>
    <!-- Bandeau de don -->
    <a href="https://www.paypal.com/paypalme/mehdiguiraud" class="donation-ribbon">
        <i class="fas fa-heart"></i>
        <span class="ribbon-text">Soutenez ce projet</span>
    </a>

    <!-- Contenu de la page -->
</body>
```

### Étape 3 : Ajouter le footer avant `</body>`

```html
    <!-- Footer signature -->
    <footer class="signature-footer">
        <!-- ... contenu du footer ... -->
    </footer>

    <script src="signature.js"></script>
</body>
```

## Checklist d'Intégration

- [ ] Fichiers copiés (signature.html, signature.css, signature.js)
- [ ] Font Awesome ajouté dans `<head>`
- [ ] CSS de la signature inclus dans `<head>`
- [ ] Bandeau ajouté juste après `<body>`
- [ ] Footer ajouté avant `</body>`
- [ ] Script JS inclus après le footer
- [ ] Test responsive effectué
- [ ] Vérification des liens sociaux

## Responsive Design

| Écran | Comportement |
|-------|-------------|
| Desktop | Tous les éléments visibles |
| Tablette (< 768px) | Ajustements de taille |
| Mobile (< 480px) | Bandeau diagonal masqué |

## Personnalisation

### Modifier les couleurs

```css
:root {
    --signature-primary-color: #votre-couleur;
    --signature-accent-color: #votre-couleur;
}
```

### Modifier les liens

Mettre à jour les URLs dans le HTML :
- GitLab : `https://github.com/Guiraud`
- Twitter : `https://twitter.com/mguiraud`
- Email : `lecourrieldemehdi@gmail.com`
- PayPal : `https://www.paypal.com/paypalme/mehdiguiraud`

## Exemple Complet

```html
<!DOCTYPE html>
<html lang="fr">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Mon Projet</title>
    <!-- Font Awesome -->
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.2/css/all.min.css">
    <!-- Signature CSS -->
    <link rel="stylesheet" href="signature.css">
</head>
<body>
    <!-- Bandeau de don -->
    <a href="https://www.paypal.com/paypalme/mehdiguiraud"
       class="donation-ribbon"
       target="_blank"
       rel="noopener noreferrer">
        <i class="fas fa-heart"></i>
        <span class="ribbon-text">Soutenez ce projet</span>
    </a>

    <!-- Contenu principal -->
    <main>
        <h1>Bienvenue</h1>
        <!-- Votre contenu ici -->
    </main>

    <!-- Footer signature -->
    <footer class="signature-footer">
        <div class="footer-decoration"></div>
        <div class="social-links">
            <a href="https://github.com/Guiraud" class="social-icon github-icon">
                <i class="fab fa-github"></i>
            </a>
            <a href="https://twitter.com/mguiraud" class="social-icon twitter-icon">
                <i class="fab fa-twitter"></i>
            </a>
            <a href="mailto:lecourrieldemehdi@gmail.com" class="social-icon email-icon">
                <i class="fas fa-envelope"></i>
            </a>
            <a href="https://www.paypal.com/paypalme/mehdiguiraud" class="social-icon paypal-icon">
                <i class="fab fa-paypal"></i>
            </a>
        </div>
        <div class="copyright">
            &copy; <span class="copyright-year"></span> Mehdi Guiraud - Tous droits réservés
        </div>
        <div class="donation-text">
            Si vous appréciez cet outil, vous pouvez
            <a href="https://www.paypal.com/paypalme/mehdiguiraud">faire un don</a>
        </div>
    </footer>

    <script src="signature.js"></script>
</body>
</html>
```

---

**Auteur** : Mehdi Guiraud
**Source** : [bottom-signature](https://github.com/Guiraud/bottom-signature)
