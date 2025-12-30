#!/bin/bash
# check-complexity.sh - Vérification de la complexité cyclomatique

EXIT_CODE=0

echo "🧠 Vérification de la complexité..."

# 1. BACKEND (Rust)
# Nécessite: cargo install cargo-clippy
echo "  🦀 Backend (Rust)..."
# On utilise clippy::cognitive_complexity avec une limite de 20
# On se déplace dans src-tauri pour que cargo trouve le projet
if (cd src-tauri && cargo clippy -- -D clippy::cognitive_complexity -A clippy::all 2>&1) | grep -q "cognitive_complexity"; then
    echo "❌ Complexité trop élevée détectée en Rust (>20)"
    (cd src-tauri && cargo clippy -- -D clippy::cognitive_complexity -A clippy::all 2>&1) | grep "cognitive_complexity" | head -n 5
    EXIT_CODE=1
else
    echo "  ✅ Rust complexity OK"
fi

# 2. FRONTEND (Vue/TS)
echo "  🎨 Frontend (Vue/TS)..."
# On vérifie si eslint est configuré pour la complexité
if [ -f "package.json" ]; then
    if [ -f "node_modules/.bin/eslint" ]; then
        # Utilisation de --yes pour éviter le prompt d'installation
        if npx --yes eslint src --rule 'complexity: ["error", 20]' >/dev/null 2>&1; then
            echo "  ✅ Frontend complexity OK"
        else
            echo "❌ Complexité trop élevée détectée en Frontend (ou eslint failed)"
            # On affiche un résumé
            npx --yes eslint src --rule 'complexity: ["error", 20]' 2>&1 | head -n 5
            EXIT_CODE=1
        fi
    else
        echo "  ⚠️  ESLint non installé localement, saut de la vérification complexité Frontend."
    fi
else
    echo "  ℹ️  (Pas de package.json, skip frontend)"
fi

exit $EXIT_CODE
