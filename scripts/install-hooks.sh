#!/bin/bash
# install-hooks.sh - Installe les hooks Git pour empêcher les mauvais commits

HOOK_FILE=".git/hooks/pre-commit"

echo "🪝 Installation des Git Hooks..."

if [ ! -d ".git" ]; then
    echo "❌ Erreur: Ce n'est pas un dépôt Git. Initialisez git d'abord (git init)."
    exit 1
fi

cat << 'EOF' > "$HOOK_FILE"
#!/bin/bash
# pre-commit hook
# Empêche le commit si l'audit rapide échoue

echo "🛡️  [PRE-COMMIT] Vérification de la qualité..."

# 1. Vérifier les noms de fichiers (Français/Structure)
if ! ./scripts/check-french-naming.sh; then
    echo "❌ Commit bloqué : Problème de nommage."
    exit 1
fi

# 2. Vérifier la taille des fichiers (Critique)
if ! ./scripts/check-file-size.sh; then
    echo "❌ Commit bloqué : Fichiers trop volumineux."
    exit 1
fi

# 3. Vérifier le code mort (Rapide)
if ! ./scripts/check-dead-code.sh; then
    echo "❌ Commit bloqué : Code mort détecté."
    exit 1
fi

# 4. Vérifier les secrets/sécurité
if ! ./scripts/check-security.sh; then
    echo "❌ Commit bloqué : Problème de sécurité."
    exit 1
fi

echo "✅ [PRE-COMMIT] Tout est bon. Commit autorisé."
exit 0
EOF

chmod +x "$HOOK_FILE"
echo "✅ Hook pre-commit installé. Tu es maintenant protégé contre toi-même."
