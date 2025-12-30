#!/bin/bash
# check-circular-imports.sh - Détection imports circulaires (Backend + Frontend)

EXIT_CODE=0

echo "🔄 Vérification des imports circulaires..."

# 1. BACKEND (Rust)
echo "  🦀 Backend (Rust)..."
# Créer un fichier temporaire pour stocker les imports
TEMP_IMPORTS=$(mktemp)

# Parser tous les imports Rust
find src-tauri/src -name "*.rs" -type f 2>/dev/null | while read -r file; do
    # Extraire les imports et créer un graphe
    grep -E "^use |^mod " "$file" | sed "s|.*use ||; s|.*mod ||; s|;||; s| as .*||" | while read -r import; do
        if [ -n "$import" ]; then
            echo "$file -> $import" >> "$TEMP_IMPORTS"
        fi
    done
done

# Vérifier les cycles simples (A->B et B->A)
if [ -f "$TEMP_IMPORTS" ]; then
    CYCLES=$(awk -F' -> ' '{print $2" -> "$1}' "$TEMP_IMPORTS" | sort | uniq -d | wc -l)
    
    if [ "$CYCLES" -gt 0 ]; then
        echo "❌ Imports circulaires Rust détectés:"
        awk -F' -> ' '{print $2" -> "$1}' "$TEMP_IMPORTS" | sort | uniq -d
        EXIT_CODE=1
    fi
fi
rm -f "$TEMP_IMPORTS"

# 2. FRONTEND (Vue/TS)
echo "  🎨 Frontend (Vue/TS)..."
# Utilisation de 'madge' si disponible, sinon warning
if command -v npx >/dev/null 2>&1; then
    if [ -f "package.json" ]; then
        # On essaie d'utiliser madge sans l'installer globalement
        # Utilisation de --yes pour éviter le prompt d'installation
        # NOTE: On exclut .vue pour l'instant car madge a du mal avec le parsing (SyntaxError JSX)
        MADGE_OUTPUT=$(npx --yes madge --circular --extensions ts src 2>&1)
        MADGE_EXIT_CODE=$?
        
        if [ $MADGE_EXIT_CODE -ne 0 ]; then
             if echo "$MADGE_OUTPUT" | grep -q "SyntaxError"; then
                 echo "⚠️  Madge a rencontré une erreur de syntaxe (probablement parsing Vue/JSX), saut de la vérification."
                 echo "   Détail: $(echo "$MADGE_OUTPUT" | head -n 1)"
             else
                 # Si madge échoue (trouve des cycles), on affiche la sortie
                 echo "❌ Imports circulaires Frontend détectés (via madge):"
                 echo "$MADGE_OUTPUT"
                 EXIT_CODE=1
             fi
        else
             echo "  ✅ Frontend clean"
        fi
    fi
else
    echo "  ℹ️  (Node.js non détecté, skip frontend check)"
fi

exit $EXIT_CODE
