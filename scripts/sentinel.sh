#!/bin/bash
# sentinel.sh - Gardien en Temps Réel
# Surveille les modifications et assure la NON-RÉGRESSION en continu.

GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}��️  SENTINELLE ACTIVÉE - Mode Vibe Coding${NC}"
echo "Je surveille ton code. Code tranquille, je gère la technique."

LAST_CHECKSUM=""

while true; do
    # Calculer une signature de tous les fichiers sources (Rust + Vue + TS)
    # On ignore les fichiers cachés, le dossier target et node_modules
    CURRENT_CHECKSUM=$(find . -type f \( -name "*.rs" -o -name "*.vue" -o -name "*.ts" \) -not -path "*/target/*" -not -path "*/node_modules/*" -not -path "*/dist/*" -not -path "*/.*" -exec md5sum {} + 2>/dev/null | sort | md5sum)

    if [ "$LAST_CHECKSUM" != "$CURRENT_CHECKSUM" ]; then
        if [ -n "$LAST_CHECKSUM" ]; then
            echo ""
            echo -e "${BLUE}🔄 Changement détecté... Analyse en cours...${NC}"
            
            # 1. AUTO-CORRECTION
            ./scripts/auto-fix.sh
            
            # 2. VÉRIFICATION DES RÈGLES (Taille)
            # On laisse check-file-size.sh afficher ses warnings (jaune) et erreurs (rouge)
            if ! ./scripts/check-file-size.sh; then
                echo -e "${RED}⚠️  ALERTE: Bloquant - Fichier trop gros !${NC}"
            fi

            # 3. NON-RÉGRESSION (Tests Rapides)
            echo -e "🧪 Lancement des tests unitaires..."
            
            # Tests Backend (Rust)
            if (cd src-tauri && cargo test --lib --quiet); then
                echo -e "${GREEN}✅ RUST: OK${NC}"
            else
                echo -e "${RED}❌ RUST: ERREUR${NC}"
            fi

            # Tests Frontend (Vue) - Uniquement si package.json existe
            if [ -f "package.json" ]; then
                if npm run test:unit -- --run >/dev/null 2>&1; then
                     echo -e "${GREEN}✅ VUE: OK${NC}"
                else
                     # On ne bloque pas tout si les tests frontend ne sont pas encore setup
                     echo -e "${YELLOW}⚠️  VUE: Tests échoués ou non configurés${NC}"
                fi
            fi
            
            # 4. GÉNÉRATION DE CONTEXTE (Pour l'IA)
            ./scripts/generate-context.sh > /dev/null 2>&1
        fi
        LAST_CHECKSUM="$CURRENT_CHECKSUM"
    fi
    
    # Pause pour économiser le CPU
    sleep 2
done
