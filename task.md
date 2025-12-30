# � Plan de Développement - TagOtomatik (Nonotags Rewrite)

Ce fichier suit la progression du projet. Les tâches sont priorisées pour respecter l'architecture DAG et le workflow "Vibe Coding".

## 🏁 Phase 1 : Initialisation & Socle Technique
- [x] **Setup Rust** : Ajouter les dépendances (`lofty`, `walkdir`, `serde`, `serde_json`, `thiserror`, `rusqlite`, `lazy_static` ou `regex`). <!-- id: 1 -->
- [x] **Setup Frontend** : Configurer TailwindCSS, Pinia, Vue Router. <!-- id: 2 -->
- [x] **Architecture** : Créer la structure de dossiers Backend (`models`, `services`, `commands`, `db`). <!-- id: 3 -->

## 🧠 Phase 2 : Backend Core (Lecture & Scan)
- [x] **Modèles Rust** : Définir `Track`, `Album`, `AppError` dans `src-tauri/src/models/`. <!-- id: 4 -->
- [x] **Service Audio** : Implémenter `AudioService` pour lire les métadonnées avec `lofty`. <!-- id: 5 -->
- [x] **Service Scanner** : Implémenter `ScannerService` pour parcourir les dossiers récursivement (`walkdir`) et grouper par album. <!-- id: 6 -->
- [x] **Commande Scan** : Créer `scan_directory` et l'exposer au Frontend. <!-- id: 7 -->

## 🖥️ Phase 3 : Frontend Core (Visualisation)
- [x] **Store Pinia** : Créer `useLibraryStore` pour gérer les albums chargés. <!-- id: 8 -->
- [x] **Vue Dashboard** : Créer l'écran d'accueil avec sélection de dossier. <!-- id: 9 -->
- [x] **Vue Liste** : Afficher les albums scannés sous forme de grille (Cover, Artiste, Album). <!-- id: 10 -->

## ⚙️ Phase 4 : Logique Métier (Nettoyage & Correction)
- [x] **Service Processeur** : Créer `MetadataProcessorService`. <!-- id: 11 -->
- [x] **Regex Cleaning** : Implémenter les règles de nettoyage (espaces, patterns indésirables). <!-- id: 12 -->
- [x] **Case Corrector** : Implémenter l'algorithme "Title Case" avec gestion des particules (le, la, of, the...). <!-- id: 13 -->
- [x] **Commande Auto-Correct** : Exposer `auto_correct_album` pour prévisualiser les corrections. <!-- id: 14 -->

## 📝 Phase 5 : Édition & Détail
- [x] **Vue Détail** : Créer l'écran d'édition d'un album (Header + Liste des pistes). <!-- id: 15 -->
- [x] **Comparaison Visuelle** : Afficher les différences entre valeurs originales et corrigées (barré/vert). <!-- id: 16 -->
- [x] **Édition Inline** : Permettre la modification manuelle des champs. <!-- id: 17 -->

## 💾 Phase 6 : Persistance & Exceptions (DB)
- [x] **Setup SQLite** : Initialiser la DB `nonotags.db` avec `rusqlite`. <!-- id: 18 -->
- [x] **Modèle Exception** : Créer la table `case_exceptions`. <!-- id: 19 -->
- [x] **Service Exception** : CRUD pour les exceptions de casse. <!-- id: 20 -->
- [x] **Intégration** : Connecter le `MetadataProcessorService` à la DB pour respecter les exceptions. <!-- id: 21 -->
- [x] **Vue Exceptions** : Interface pour gérer les exceptions manuellement. <!-- id: 22 -->

## 💿 Phase 7 : Écriture & Renommage
- [x] **Écriture Tags** : Implémenter la sauvegarde des métadonnées dans `AudioService`. <!-- id: 23 -->
- [x] **Renommage Fichiers** : Implémenter le renommage physique (`{track} - {title}.ext`). <!-- id: 24 -->
- [x] **Renommage Dossiers** : Implémenter le renommage du dossier parent. <!-- id: 25 -->
- [x] **Commande Save** : Exposer `save_album_changes` (Atomique : Tags -> Rename File -> Rename Folder). <!-- id: 26 -->

## 🎨 Phase 8 : Polish & UX
- [x] **Gestion Erreurs** : Afficher les erreurs Rust proprement dans l'UI (Toasts). <!-- id: 27 -->
- [x] **Loading States** : Squelettes de chargement et spinners. <!-- id: 28 -->
- [x] **Audit Final** : Vérification complète avec `./scripts/full-code-audit.sh`. <!-- id: 29 -->