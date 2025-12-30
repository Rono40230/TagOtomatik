# 🚀 Starter Pack "Vibe Coding" (Rust + Vue)

Ce starter pack est conçu pour le **Vibe Coding** : tu te concentres sur la création et la logique métier, le système gère la rigueur technique, la qualité et la non-régression en arrière-plan.

## 🧠 Philosophie

- **Zéro Charge Mentale** : Le système formate, corrige et vérifie pour toi.
- **Zéro Régression** : La "Sentinelle" surveille chaque sauvegarde.
- **Zéro Dette Technique** : Les audits bloquent le code sale, mort ou trop complexe.

---

## 🏁 Phase 0 : Initialisation (Une fois par projet)

Dès que tu copies ce dossier pour un nouveau projet :

1. **Ouvre un terminal** à la racine.
2. **Active les protections** (Git Hooks) :
   ```bash
   ./scripts/install-hooks.sh
   ```
   *Cela t'empêchera physiquement de commiter du code qui ne respecte pas les standards.*

---

## 🚀 Phase 1 : Ton Quotidien (La Session de Code)

À chaque fois que tu ouvres VS Code :

1. **Lance la Sentinelle** dans un terminal dédié :
   ```bash
   ./scripts/sentinel.sh
   ```
2. **Garde ce terminal visible** (en bas ou sur le côté).
3. **Code normalement**. Ne te soucie pas du formatage.

### Ce que fait la Sentinelle :
Dès que tu sauvegardes (`Ctrl+S`) :
- 🧹 **Auto-Fix** : Elle reformate et corrige les erreurs techniques mineures.
- 🧪 **Tests** : Elle lance les tests Backend et Frontend.
- 🗺️ **Contexte** : Elle met à jour la carte du projet pour l'IA.

### Le Code Couleur :
- 🟢 **VERT** : Tout est clean. Continue ta vibe.
- 🟡 **JAUNE** : Avertissement (ex: fichier bientôt trop gros). Prépare-toi à refactoriser.
- 🔴 **ROUGE** : Stop. Régression ou erreur bloquante. Corrige avant de continuer.

---

## 🛑 Phase 2 : Validation (Avant de finir)

Quand tu as terminé une fonctionnalité ou une tâche importante :

1. **Lance l'Audit Complet** (Le "Crash Test") :
   ```bash
   ./scripts/full-code-audit.sh
   ```
   *Vérifie en profondeur : Sécurité, Architecture, Complexité, Code mort, Nommage.*

2. Si tout est ✅, tu peux commiter sereinement.

---

## 🤖 Comment travailler avec l'IA

### 🗣️ Démarrage Automatique
Puisque tu utilises **GitHub Copilot**, il lit automatiquement le fichier `.github/copilot-instructions.md` qui contient désormais toutes les règles (Sentinelle, Nommage, Workflow).

Tu n'as **PLUS BESOIN** de lui dire de lire un fichier au démarrage. Commence directement à coder !

### 💡 Interactions courantes
- **Si la Sentinelle est ROUGE** : Copie l'erreur à l'IA, elle saura la corriger.
- **Si la Sentinelle est JAUNE** : Demande à l'IA : *"La Sentinelle dit que ce fichier est trop gros, propose un refactoring."*
- **Architecture** : L'IA a accès à `PROJECT_MAP.md` (généré automatiquement), elle connaît donc toujours la structure à jour de ton projet.

---

## ⚡ Résumé des Commandes

| Action | Commande | Quand ? |
| :--- | :--- | :--- |
| **Protéger** | `./scripts/install-hooks.sh` | Au début du projet |
| **Coder** | `./scripts/sentinel.sh` | En permanence (arrière-plan) |
| **Valider** | `./scripts/full-code-audit.sh` | Avant de commiter |
