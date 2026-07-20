/**
 * Bottom Signature JavaScript
 * Script d'initialisation pour le composant de signature
 * @author Mehdi Guiraud
 */

(function() {
    'use strict';

    /**
     * Initialise le composant de signature
     */
    function initSignature() {
        // Met à jour l'année du copyright automatiquement
        updateCopyrightYear();

        // Peut être étendu avec d'autres fonctionnalités
        // comme des animations, tracking d'événements, etc.
    }

    /**
     * Met à jour l'année du copyright avec l'année courante
     */
    function updateCopyrightYear() {
        const yearElement = document.querySelector('.copyright-year');
        if (yearElement) {
            const currentYear = new Date().getFullYear();
            yearElement.textContent = currentYear;
        }
    }

    /**
     * Fonction utilitaire pour charger la signature de manière asynchrone
     * @param {string} targetSelector - Sélecteur CSS de l'élément cible
     * @param {string} signaturePath - Chemin vers le fichier signature.html
     * @returns {Promise<void>}
     */
    async function loadSignature(targetSelector, signaturePath = 'signature.html') {
        try {
            const response = await fetch(signaturePath);
            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }
            const html = await response.text();
            const target = document.querySelector(targetSelector);
            if (target) {
                target.innerHTML = html;
                initSignature();
            }
        } catch (error) {
            console.error('Erreur lors du chargement de la signature:', error);
        }
    }

    /**
     * Initialisation automatique au chargement de la page
     */
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', initSignature);
    } else {
        initSignature();
    }

    // Expose les fonctions publiques
    window.BottomSignature = {
        init: initSignature,
        load: loadSignature,
        updateYear: updateCopyrightYear
    };

})();
