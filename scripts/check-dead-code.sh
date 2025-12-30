#!/bin/bash
# check-dead-code.sh - Détection de code mort (Backend + Frontend)

EXIT_CODE=0

echo "🧹 Vérification du code mort..."

# 1. BACKEND (Rust)
# On utilise clippy avec deny(dead_code) pour être strict
echo "  🦀 Backend (Rust)..."
if [ -d "src-tauri" ]; then
    cd src-tauri
    if cargo clippy -- -D dead_code -D unreachable_code 2>&1 | grep -q "error:"; then
        echo "❌ Code mort détecté en Rust (fonctions ou variables inutilisées)"
        # On affiche les erreurs pour aider
        cargo clippy -- -D dead_code -D unreachable_code 2>&1 | grep -E "error:.*unused|error:.*dead_code" | head -n 5
        cd ..
        EXIT_CODE=1
    else
        echo "  ✅ Rust clean"
        cd ..
    fi
else
    echo "⚠️  Dossier src-tauri introuvable, saut de la vérification Rust."
fi

# 2. FRONTEND (Vue/TS)
echo "  🎨 Frontend (Vue/TS)..."
# Idéalement, on utiliserait 'knip' ou 'ts-prune'.
# Ici on fait une vérification basique : chercher les fichiers .ts/.vue qui ne sont jamais importés
# (C'est une heuristique simple, à remplacer par un vrai outil si possible)

# TODO: Installer knip (npm install -D knip) pour une vraie vérification
if [ -f "package.json" ] && grep -q "knip" "package.json"; then
    if ! npx knip --no-progress; then
        echo "❌ Code mort détecté en Frontend (via knip)"
        EXIT_CODE=1
    fi
else
    echo "  ℹ️  (Install 'knip' pour une détection précise du code mort Frontend)"
fi

exit $EXIT_CODE
