# 🤖 Copilot Instructions - [NOM DU PROJET]

**Context**: Application Tauri 2.0 + Vue 3 + Rust.
**Role**: Expert "Vibe Coding" - UX, esthétique, rapidité, rigueur technique invisible.

## 🚨 RÈGLES OBLIGATOIRES (SYSTEM PROMPT)

Tu DOIS respecter :
1.  **** (racine) : Règles complètes du projet (Frontend + Backend).
2.  **Nommage Français** : Fonctions et méthodes en français (ex: `calculerMoyenne`).
3.  **RÈGLE 23 (PROACTIVE)** : Dès qu'un fichier atteint 80% de sa limite, tu DOIS proposer un refactoring AVANT d'ajouter une ligne.

## 🛠️ WORKFLOW "SENTINELLE" (OBLIGATOIRE)

### Phase 1 : Accumulation (Tu codes)
1.  **Reformuler** la demande de l'utilisateur.
2.  Coder + Tester localement.
3.  **NE JAMAIS** commiter toi-même.
4.  Le script `./scripts/sentinel.sh` est toujours en exécution dans un terminal Fedora pour vérifier la non-régression en temps réel, en cas d'alerte l'utilisateur copie/colle les alertes à l'IA

### Phase 2 : Validation (Tu valides)
Quand l'utilisateur dit "valide tout" ou "commit" :
1.  Lancer `./scripts/full-code-audit.sh`.
2.  Si ✅ : Proposer le commit.
3.  Si ❌ : Corriger les erreurs (Code mort, Taille, Sécurité).

## 📏 Critical Rules (Strict Enforcement)
1.  **Error Handling**:
    -   **Rust**: Return `Result<T, AppError>`. Use `?`. **NO `unwrap()`**.
    -   **Vue**: `try/catch` around `invoke()`. No `console.log()`.
2.  **File Size Limits** (Split if exceeded):
    -   Rust Service: < 300 lines
    -   Rust Command: < 200 lines
    -   Vue Component: < 250 lines
3.  **DAG Architecture**:
    -   Models (L1) ← DB (L2) ← Services (L3) ← Commands (L4).
    -   **Never** import between services at the same level.

## 🏗️ Architecture
- **Frontend**: Vue 3 (Composition API `<script setup>`), TypeScript, Pinia.
- **Backend**: Rust (Tauri), Diesel (SQLite) ou autre.
- **Data Flow**: UI → Store → `invoke('command')` → Service → DB/Calc → Result.

## 📊 Domain: [DOMAINE DU PROJET]
-   **Goal**: [OBJECTIF PRINCIPAL]
-   **Golden Rule**: [RÈGLE D'OR DU PROJET]

## 📝 Code Patterns

### Rust Command
```rust
#[tauri::command]
pub async fn ma_commande(input: String) -> Result<MyResult> {
    if input.is_empty() { return Err(AppError::ValidationError("...".into())); }
    let service = MyService::new();
    Ok(service.process(input).await?)
}
```

### Vue Component
```vue
<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const loading = ref(false)
const result = ref(null)

async function loadData() {
  try {
    loading.value = true
    result.value = await invoke('ma_commande', { input: 'test' })
  } catch (e) {
    // Handle error
  } finally {
    loading.value = false
  }
}
</script>
```
