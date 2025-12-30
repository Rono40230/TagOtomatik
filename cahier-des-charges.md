# 📘 Cahier des Charges : Réécriture Nonotags (Rust + Vue)

## 1. Présentation et Objectifs
**Nonotags** est une application de bureau destinée à la gestion, au nettoyage et à la standardisation de bibliothèques musicales.
L'objectif de la réécriture est de migrer d'une architecture Python/GTK vers une architecture **Tauri (Rust + Vue.js)** pour gagner en performance, en sécurité (typage fort) et offrir une interface moderne.

### 1.1 Stack Technique Cible
*   **Application Shell :** [Tauri v2](https://tauri.app/) (Léger, sécurisé, natif).
*   **Backend (Logique Métier) :** **Rust**.
*   **Frontend (Interface) :** **Vue 3** (Composition API) + **TypeScript** + **Vite**.
*   **State Management :** **Pinia**.
*   **UI Framework :** **TailwindCSS** (pour le styling) + **Shadcn-vue** ou **PrimeVue** (composants).
*   **Base de Données :** **SQLite** (via `rusqlite` ou `sqlx`).
*   **Manipulation Audio :** Crate **`lofty`** (recommandé pour le support large MP3/FLAC/M4A) ou `id3`.

---

## 2. Architecture Fonctionnelle

L'application suit un flux de données unidirectionnel :
`Disque -> Scan -> Analyse (Mémoire) -> Correction (Mémoire) -> Validation Utilisateur -> Écriture Disque`.

### 2.1 Module : Scanner (Rust)
*   **Entrée :** Chemin d'un dossier racine sélectionné par l'utilisateur.
*   **Fonctionnalités :**
    *   Parcours récursif rapide (utiliser crate `walkdir`).
    *   Filtrage des fichiers audio supportés : `.mp3`, `.flac`, `.ogg`, `.m4a`, `.wav`.
    *   Regroupement logique des fichiers en **Albums** (basé sur le dossier parent).
    *   Détection des fichiers "parasites" (images basse résolution, fichiers .txt, .nfo, .url).
*   **Performance :** Doit être asynchrone pour ne pas geler l'UI.

### 2.2 Module : Processeur de Métadonnées (Rust)
C'est le cœur du "nettoyage". Il doit porter la logique de `core/metadata_processor.py`.
*   **Lecture des Tags :** Titre, Artiste, Album, Année, Genre, Piste, Disque, Cover Art.
*   **Règles de Nettoyage (Regex) :**
    *   Suppression des patterns indésirables (ex: "www.site.com", "[320kbps]", commentaires).
    *   Suppression des espaces multiples et trim.
    *   Normalisation des séparateurs (ex: remplacer " feat. ", " ft. ", " with " par un standard).
    *   Normalisation des connecteurs (ex: " and " -> " & ").
*   **Correction de Casse (Case Corrector) :**
    *   Application du "Title Case" (Majuscule à chaque mot significatif).
    *   Gestion des exceptions grammaticales (ne pas mettre de majuscule à "le", "la", "de", "du", "a", "the", "of"... sauf en début de phrase).
    *   **Système d'Exceptions DB :** Avant de corriger, vérifier si le terme existe dans la table `case_exceptions` (ex: "AC/DC", "iPhone").

### 2.3 Module : Base de Données (Rust/SQLite)
Portage de `database/models.py`.
*   **Fichier :** `nonotags.db` (dans le dossier de config utilisateur).
*   **Table `case_exceptions` :**
    *   `id` (PK)
    *   `original_text` (ex: "acdc")
    *   `corrected_text` (ex: "AC/DC")
    *   `type` (ex: "artist", "album", "global")
*   **Table `history` (Optionnel phase 1) :** Pour garder une trace des fichiers modifiés.

### 2.4 Module : Écriture et Renommage (Rust)
*   **Tagging :** Écriture atomique des métadonnées validées dans les fichiers.
*   **Renommage de Fichiers :**
    *   Renommer le fichier physique selon un pattern configurable (ex: `{track_number} - {title}.{ext}`).
    *   Gérer la sanitisation des noms de fichiers (suppression des caractères interdits par l'OS : `/`, `\`, `:`, `*`, `?`, `"`, `<`, `>`, `|`).
*   **Renommage de Dossiers :** Renommer le dossier parent selon `{Artist} - {Album} ({Year})`.
*   **Nettoyage :** Suppression des fichiers parasites identifiés lors du scan.

---

## 3. Interface Utilisateur (Vue.js)

L'interface doit être divisée en 3 vues principales.

### 3.1 Vue : Dashboard (Accueil)
*   **Composants :**
    *   Gros bouton "Ouvrir un dossier" (appel dialogue natif).
    *   Liste des "Derniers dossiers ouverts".
    *   Accès rapide aux paramètres et aux exceptions.

### 3.2 Vue : Liste des Albums (Grid View)
*   Une fois le scan terminé, afficher les albums sous forme de cartes.
*   **Carte Album :**
    *   Pochette (Cover Art).
    *   Nom de l'Artiste et de l'Album.
    *   Indicateur d'état (ex: "À corriger", "Validé", "Incomplet").
    *   Badge indiquant le format (MP3, FLAC).

### 3.3 Vue : Éditeur de Détail (Detail View)
*   S'ouvre au clic sur un album.
*   **Header :** Champs globaux éditables (Artiste Album, Album, Année, Genre, Cover). Bouton "Appliquer à toutes les pistes".
*   **Tableau des Pistes (Data Grid) :**
    *   Colonnes : Piste, Titre, Artiste (si différent), Durée.
    *   Comparaison visuelle : Afficher l'ancienne valeur barrée si une correction est proposée par le système.
    *   Édition inline des cellules.
*   **Actions :**
    *   Bouton "Sauvegarder" (déclenche l'écriture disque).
    *   Bouton "Ajouter aux exceptions" (sur clic droit d'un mot corrigé à tort).

### 3.4 Vue : Gestion des Exceptions
*   Tableau CRUD (Create, Read, Update, Delete) pour gérer les entrées de la table `case_exceptions`.

---

## 4. Modèle de Données (Structures Rust)

Voici les structures de données recommandées pour le backend Rust.

```rust
// Représente une piste audio unique
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Track {
    pub path: String,           // Chemin complet
    pub filename: String,       // Nom de fichier
    pub title: String,
    pub artist: String,
    pub album: String,
    pub year: Option<u32>,
    pub track_number: Option<u32>,
    pub genre: Option<String>,
    pub duration_sec: u64,
    pub format: String,         // "mp3", "flac"...
    pub bit_rate: Option<u32>,
    pub has_cover: bool,
    
    // État de modification
    pub original_metadata: Option<Box<Track>>, // Pour le diff/undo
    pub is_modified: bool,
}

// Représente un album (regroupement de pistes)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Album {
    pub id: String,             // Hash unique ou chemin dossier
    pub path: String,           // Chemin dossier
    pub title: String,
    pub artist: String,         // Artiste principal (Album Artist)
    pub year: Option<u32>,
    pub cover_path: Option<String>, // Chemin vers cover.jpg locale ou cache
    pub tracks: Vec<Track>,
    pub status: AlbumStatus,    // Enum: Clean, Dirty, Processing
}

// Exception de casse
#[derive(Debug, Serialize, Deserialize)]
pub struct CaseException {
    pub id: Option<i64>,
    pub original: String,
    pub corrected: String,
    pub category: String,
}
```

## 5. Commandes Tauri (API Frontend <-> Backend)

Le frontend Vue.js communiquera avec Rust via ces commandes (`invoke`) :

1.  `scan_directory(path: String) -> Result<Vec<Album>, String>`
2.  `get_album_details(album_path: String) -> Result<Album, String>`
3.  `save_album_changes(album: Album) -> Result<bool, String>`
4.  `auto_correct_album(album: Album) -> Result<Album, String>` (Applique les règles regex/case)
5.  `get_exceptions() -> Result<Vec<CaseException>, String>`
6.  `add_exception(original: String, corrected: String) -> Result<bool, String>`

## 6. Plan de Développement Suggéré

1.  **Setup :** Initialiser le projet `npm create tauri-app@latest`.
2.  **Backend Core :** Implémenter `struct Track` et la lecture de fichiers avec `lofty`.
3.  **Backend Scan :** Implémenter le scan récursif (`walkdir`) qui retourne une liste JSON brute.
4.  **Frontend Base :** Créer la vue Dashboard et l'appel au scan.
5.  **Backend Logic :** Porter les Regex de `metadata_processor.py` vers Rust.
6.  **Frontend Edit :** Créer la vue détail et le binding des données.
7.  **Backend Write :** Implémenter la sauvegarde des tags.
8.  **Database :** Ajouter SQLite pour les exceptions.
