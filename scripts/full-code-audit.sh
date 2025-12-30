#!/bin/bash
# full-code-audit.sh - Audit complète du code entier (Backend + Frontend)
# Applique TOUTES les règles .clinerules sur l'intégrité complète du projet

set -e

echo ""
echo "╔═══════════════════════════════════════════════════════════════════╗"
echo "║        🔍 AUDIT COMPLET DU CODE - BACKEND + FRONTEND              ║"
echo "╚═══════════════════════════════════════════════════════════════════╝"
echo ""

EXIT_CODE=0
PHASE_COUNT=1
TOTAL_PHASES=9

# ═══════════════════════════════════════════════════════════════
# PHASE 1 : TAILLES DE FICHIERS (Frontend + Backend)
# ═══════════════════════════════════════════════════════════════

echo "PHASE $PHASE_COUNT/$TOTAL_PHASES: 📏 Tailles de fichiers (Backend + Frontend)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
# On utilise check-file-size.sh qui est maintenant robuste
if ! ./scripts/check-file-size.sh --verbose; then
    echo "❌ PHASE $PHASE_COUNT ÉCHOUÉE: Fichiers trop volumineux"
    EXIT_CODE=1
fi
PHASE_COUNT=$((PHASE_COUNT + 1))
echo ""

# ═══════════════════════════════════════════════════════════════
# PHASE 2 : QUALITÉ BACKEND (Rust)
# ═══════════════════════════════════════════════════════════════

echo "PHASE $PHASE_COUNT/$TOTAL_PHASES: 🦀 Qualité Backend (Rust) - Anti-patterns"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
if ! ./scripts/check-antipatterns.sh; then
    echo "❌ PHASE $PHASE_COUNT ÉCHOUÉE: Anti-patterns Rust détectés"
    EXIT_CODE=1
fi
PHASE_COUNT=$((PHASE_COUNT + 1))
echo ""

# ═══════════════════════════════════════════════════════════════
# PHASE 3 : UNWRAP() BACKEND
# ═══════════════════════════════════════════════════════════════

echo "PHASE $PHASE_COUNT/$TOTAL_PHASES: 🔒 Backend - unwrap() en production"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
if ! ./scripts/check-unwrap.sh; then
    echo "❌ PHASE $PHASE_COUNT ÉCHOUÉE: unwrap() détecté"
    EXIT_CODE=1
fi
PHASE_COUNT=$((PHASE_COUNT + 1))
echo ""

# ═══════════════════════════════════════════════════════════════
# PHASE 4 : CODE MORT (Backend + Frontend)
# ═══════════════════════════════════════════════════════════════

echo "PHASE $PHASE_COUNT/$TOTAL_PHASES: 💀 Code mort (Backend + Frontend)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
if ! ./scripts/check-dead-code.sh; then
    echo "❌ PHASE $PHASE_COUNT ÉCHOUÉE: Code mort détecté"
    EXIT_CODE=1
fi
PHASE_COUNT=$((PHASE_COUNT + 1))
echo ""

# ═══════════════════════════════════════════════════════════════
# PHASE 5 : ARCHITECTURE & IMPORTS CIRCULAIRES
# ═══════════════════════════════════════════════════════════════

echo "PHASE $PHASE_COUNT/$TOTAL_PHASES: 🏗️  Architecture DAG & Imports Circulaires"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
if ! ./scripts/check-circular-imports.sh; then
    echo "❌ PHASE $PHASE_COUNT ÉCHOUÉE: Imports circulaires détectés"
    EXIT_CODE=1
fi

if ! ./scripts/check-architecture.sh; then
    echo "❌ PHASE $PHASE_COUNT ÉCHOUÉE: Violation de hiérarchie"
    EXIT_CODE=1
fi
PHASE_COUNT=$((PHASE_COUNT + 1))
echo ""

# ═══════════════════════════════════════════════════════════════
# PHASE 6 : COMPLEXITÉ CYCLOMATIQUE
# ═══════════════════════════════════════════════════════════════

echo "PHASE $PHASE_COUNT/$TOTAL_PHASES: 🧠 Complexité Cyclomatique"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
if ! ./scripts/check-complexity.sh; then
    echo "❌ PHASE $PHASE_COUNT ÉCHOUÉE: Code trop complexe"
    EXIT_CODE=1
fi
PHASE_COUNT=$((PHASE_COUNT + 1))
echo ""

# ═══════════════════════════════════════════════════════════════
# PHASE 7 : SÉCURITÉ
# ═══════════════════════════════════════════════════════════════

echo "PHASE $PHASE_COUNT/$TOTAL_PHASES: 🛡️  Audit de Sécurité"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
if ! ./scripts/check-security.sh; then
    echo "❌ PHASE $PHASE_COUNT ÉCHOUÉE: Failles de sécurité détectées"
    EXIT_CODE=1
fi
PHASE_COUNT=$((PHASE_COUNT + 1))
echo ""

# ═══════════════════════════════════════════════════════════════
# PHASE 8 : QUALITÉ FRONTEND (Vue.js/TypeScript)
# ═══════════════════════════════════════════════════════════════

echo "PHASE $PHASE_COUNT/$TOTAL_PHASES: 🎨 Qualité Frontend (Vue.js/TypeScript)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [ -f "./scripts/check-vue-quality.sh" ]; then
    if ! ./scripts/check-vue-quality.sh; then
        echo "❌ PHASE $PHASE_COUNT ÉCHOUÉE: Violations Frontend détectées"
        EXIT_CODE=1
    fi
else
    echo "⚠️  Script check-vue-quality.sh manquant"
fi
PHASE_COUNT=$((PHASE_COUNT + 1))
echo ""

# ═══════════════════════════════════════════════════════════════
# PHASE 9 : NOMMAGE FRANÇAIS
# ═══════════════════════════════════════════════════════════════

echo "PHASE $PHASE_COUNT/$TOTAL_PHASES: 🇫🇷 Nommage Français"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
if ! ./scripts/check-french-naming.sh; then
    echo "❌ PHASE $PHASE_COUNT ÉCHOUÉE: Nommage incorrect"
    EXIT_CODE=1
fi

echo ""
echo "═══════════════════════════════════════════════════════════════"
if [ $EXIT_CODE -eq 0 ]; then
    echo "✅ AUDIT COMPLET RÉUSSI - PRÊT POUR COMMIT"
else
    echo "❌ AUDIT ÉCHOUÉ - CORRIGEZ LES ERREURS AVANT COMMIT"
fi
echo "═══════════════════════════════════════════════════════════════"

exit $EXIT_CODE
