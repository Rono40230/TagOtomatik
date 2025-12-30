#!/bin/bash
# auto-fix.sh - Correction et Optimisation Automatique
# Ce script est appelé par la Sentinelle pour nettoyer le code en temps réel.

set -e

echo "✨ [AUTO-FIX] Démarrage du nettoyage..."

# 1. Formatage standard (Indentation, espaces)
(cd src-tauri && cargo fmt --quiet)

# 2. Correction automatique des erreurs courantes (Clippy)
# --allow-dirty permet de corriger même si git n'est pas clean
# --allow-staged permet de corriger même si des fichiers sont stagés
(cd src-tauri && cargo clippy --fix --allow-dirty --allow-staged --quiet -- -D warnings 2>/dev/null || true)

# 3. Vérification des règles de nommage (Frontend/Backend)
./scripts/check-french-naming.sh > /dev/null 2>&1

echo "✅ [AUTO-FIX] Code nettoyé et optimisé."
