#!/bin/bash
# check-security.sh - Audit de sécurité des dépendances

EXIT_CODE=0

echo "🛡️  Audit de sécurité..."

# 1. BACKEND (Rust)
echo "  🦀 Backend (Rust)..."
if command -v cargo-audit >/dev/null 2>&1; then
    if ! cargo audit -q; then
        echo "❌ Failles de sécurité détectées dans les dépendances Rust"
        EXIT_CODE=1
    else
        echo "  ✅ Rust audit OK"
    fi
else
    echo "  ℹ️  (Install 'cargo-audit' pour vérifier les failles Rust: cargo install cargo-audit)"
fi

# 2. FRONTEND (NPM)
echo "  🎨 Frontend (NPM)..."
if [ -f "package.json" ]; then
    if ! npm audit --audit-level=high >/dev/null 2>&1; then
        echo "❌ Failles de sécurité détectées dans les dépendances NPM (High+)"
        npm audit --audit-level=high --json | grep "title" | head -n 5
        EXIT_CODE=1
    else
        echo "  ✅ NPM audit OK"
    fi
else
    echo "  ℹ️  (Pas de package.json, skip npm audit)"
fi

exit $EXIT_CODE
