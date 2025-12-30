#!/bin/bash
# check-file-size.sh - Vérification des tailles de fichiers (Backend + Frontend)
# Affiche des warnings à 80% de la limite et des erreurs à 100%

EXIT_CODE=0
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

# Limites
LIMIT_RUST_SERVICE=300
LIMIT_RUST_COMMAND=200
LIMIT_RUST_MAIN=120
LIMIT_VUE=250
LIMIT_TS=200

# Seuils d'avertissement (80%)
WARN_RUST_SERVICE=$((LIMIT_RUST_SERVICE * 80 / 100))
WARN_RUST_COMMAND=$((LIMIT_RUST_COMMAND * 80 / 100))
WARN_RUST_MAIN=$((LIMIT_RUST_MAIN * 80 / 100))
WARN_VUE=$((LIMIT_VUE * 80 / 100))
WARN_TS=$((LIMIT_TS * 80 / 100))

check_file() {
    local file=$1
    local limit=$2
    local warn=$3
    
    if [ ! -f "$file" ]; then return; fi
    
    lines=$(wc -l < "$file")
    
    if [ "$lines" -gt "$limit" ]; then
        echo -e "${RED}❌ $file: $lines lignes (MAX $limit) - TROP GROS !${NC}"
        EXIT_CODE=1
    elif [ "$lines" -gt "$warn" ]; then
        echo -e "${YELLOW}⚠️  $file: $lines lignes (Approche limite $limit) - Pense à refactoriser${NC}"
    fi
}

# Trouver tous les fichiers pertinents en excluant les dossiers ignorés
# Utilisation de -prune pour ne pas descendre dans les dossiers exclus
find . \
    \( -name ".git" -o -name "target" -o -name "node_modules" -o -name "dist" -o -name ".vscode" \) -prune \
    -o -type f \( -name "*.rs" -o -name "*.vue" -o -name "*.ts" \) -print | while read -r file; do
    
    # Rust Services
    if [[ "$file" == *"/services/"* && "$file" == *".rs" ]]; then
        check_file "$file" $LIMIT_RUST_SERVICE $WARN_RUST_SERVICE
    
    # Rust Commands
    elif [[ "$file" == *"/commands/"* && "$file" == *".rs" ]]; then
        check_file "$file" $LIMIT_RUST_COMMAND $WARN_RUST_COMMAND
        
    # Rust Main
    elif [[ "$file" == *"/main.rs" ]]; then
        check_file "$file" $LIMIT_RUST_MAIN $WARN_RUST_MAIN
        
    # Vue Components
    elif [[ "$file" == *".vue" ]]; then
        check_file "$file" $LIMIT_VUE $WARN_VUE
        
    # TypeScript Logic
    elif [[ "$file" == *".ts" && "$file" != *".d.ts" ]]; then
        check_file "$file" $LIMIT_TS $WARN_TS
    fi
done

if [ $EXIT_CODE -eq 0 ]; then
    # On n'affiche rien si tout est OK pour ne pas polluer la sentinelle, sauf si demandé
    if [ "$1" == "--verbose" ]; then
        echo "✅ Tous les fichiers respectent les limites de taille"
    fi
fi

exit $EXIT_CODE
