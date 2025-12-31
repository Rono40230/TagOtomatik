# 📋 Reste à Faire - Parité Fonctionnelle (Python -> Rust)

Ce plan détaille les fonctionnalités présentes dans l'application Python originale (`Nonotags`) qui doivent être implémentées dans la version Rust (`TagOtomatik`) pour atteindre la parité complète.

## 🎵 Phase 9 : Lecteur Audio Intégré
L'application Python permettait de pré-écouter les pistes.
- [x] **Backend** : Intégrer une librairie audio (ex: `rodio` ou bindings `gstreamer`). <!-- id: 30 -->
- [x] **Backend** : Commandes `play`, `pause`, `stop`, `seek`. <!-- id: 31 -->
- [x] **Frontend** : Créer un composant "Player" persistant (Barre de lecture). <!-- id: 32 -->
- [x] **Frontend** : Intégration dans la vue Détail (Bouton Play sur chaque piste). <!-- id: 33 -->

## 🖼️ Phase 10 : Recherche de Pochettes (Web)
L'application Python interrogeait MusicBrainz/Discogs.
- [x] **Backend** : Client HTTP (`reqwest`) pour interroger les APIs MusicBrainz/Discogs. <!-- id: 34 -->
- [x] **Backend** : Logique de téléchargement et redimensionnement d'images. <!-- id: 35 -->
- [x] **Frontend** : Modal de recherche de pochette (par Artiste/Album). <!-- id: 36 -->
- [x] **Frontend** : Grille de résultats et sélection de l'image à appliquer. <!-- id: 37 -->

## 📜 Phase 11 : Gestion de Playlists (.m3u)
L'application Python gérait les fichiers `.m3u`.
- [x] **Backend** : Parser et Writer pour le format `.m3u` / `.m3u8`. <!-- id: 38 -->
- [x] **Backend** : Service de gestion (Créer, Lire, Mettre à jour, Supprimer des playlists). <!-- id: 39 -->
- [x] **Frontend** : Nouvelle vue "Playlists" dans la sidebar. <!-- id: 40 -->
- [x] **Frontend** : Drag & Drop de pistes vers une playlist. <!-- id: 41 -->

## 🔄 Phase 12 : Convertisseur Audio
L'application Python utilisait FFmpeg pour convertir les formats.
- [x] **Backend** : Intégration de FFmpeg (via `std::process::Command` ou crate dédiée). <!-- id: 42 -->
- [x] **Backend** : Gestionnaire de tâches asynchrone pour les conversions longues. <!-- id: 43 -->
- [x] **Frontend** : Interface de conversion (Choix format cible : MP3, FLAC, etc.). <!-- id: 44 -->
- [x] **Frontend** : Indicateur de progression des conversions. <!-- id: 45 -->

## 🛡️ Phase 13 : Sauvegarde Persistante (Undo Avancé)
L'application Python stockait l'historique en base de données pour survivre au redémarrage.
- [x] **DB** : Créer une table `history` dans SQLite pour stocker les snapshots JSON des métadonnées. <!-- id: 46 -->
- [x] **Backend** : Service pour enregistrer l'état avant modification (Snapshot). <!-- id: 47 -->
- [x] **Backend** : Commande `restore_version` pour revenir à un état antérieur. <!-- id: 48 -->
- [x] **Frontend** : Vue "Historique des modifications" avec possibilité d'annuler. <!-- id: 49 -->

## ✅ Phase 14 : Audit Final & Stabilisation
- [x] **Audit** : Validation complète (`./scripts/full-code-audit.sh`).
- [x] **Fix** : Correction des erreurs de compilation Rust (Send trait, rodio).
- [x] **Fix** : Correction des erreurs Frontend (Vue duplication, console.error).
- [x] **Refactor** : Découpage des composants trop volumineux (`AlbumDetailView`).

## 🐍 Phase 15 : Parité Legacy (Règles de Gestion)
Implémentation des règles métier extraites de l'application Python originale.
- [x] **Backend** : Implémentation du nettoyage de fichiers (`CleanerService`). <!-- id: 50 -->
- [x] **Backend** : Implémentation du renommage de fichiers et dossiers (`processor.rs`). <!-- id: 51 -->
- [x] **Backend** : Implémentation de la correction de casse avancée (Sentence Case + Exceptions). <!-- id: 52 -->
- [x] **Backend** : Implémentation du formatage des métadonnées (Track padding, Genre). <!-- id: 53 -->
- [x] **Refactor** : Extraction des dictionnaires statiques (`dictionaries.rs`). <!-- id: 54 -->
- [x] **Test** : Tests unitaires validant la parité avec Python. <!-- id: 55 -->

# 📚 Règles de Gestion (Legacy Python)

Ces règles ont été extraites du code source de l'application Python originale (`Nonotags`) et servent de référence pour l'implémentation Rust.

## 1. Nettoyage des Fichiers (`file_cleaner.py`)

### Suppression
*   **Extensions interdites :** `.DS_Store`, `Thumbs.db`, `.png`, `.nfo`, `.txt`, `.m3u`, `bs.db`, `.tmp`, `.temp`, `.bak`, `.log`, `.sfv`, `.md5`, `.pdf`, `.doc`, `.docx`.
*   **Fichiers système :** `desktop.ini`, `.fuse_hidden`, `._metadata`, `#recycle`, `recycle.bin`.
*   **Images non-pertinentes :** Les fichiers `.gif` et `.bmp` sont supprimés **sauf** si leur nom contient "cover", "front", "album" ou "artwork".
*   **Sous-dossiers :** Tous les sous-dossiers sont supprimés récursivement.

### Renommage (Pochettes)
*   **Cibles :** `front.jpg`, `Front.jpg`, `Cover.jpg`.
*   **Action :** Renommage en `cover.jpg`.
*   **Formats acceptés :** Uniquement `.jpg`, `.jpeg`, `.bmp`, `.gif` (les `.png` sont supprimés).

## 2. Renommage des Fichiers & Dossiers (`file_renamer.py`)

### Sanitization (Nettoyage de caractères)
*   **Caractères interdits :**
    *   `<` → `(`
    *   `>` → `)`
    *   `:` → `-`
    *   `"` → `'`
    *   `/`, `\`, `|` → `-`
    *   `?`, `*` → (supprimés)
*   **Espaces :** Remplacement des espaces multiples par un espace simple. Suppression des espaces en début/fin (`trim`).
*   **Longueur :** Tronqué à 200 caractères max.

### Formatage des Fichiers (Pistes)
*   **Pattern :** `(N° piste) - (Titre).(Extension)`
*   **Numéro de piste :**
    *   Toujours sur 2 chiffres (padding avec zéro : `1` → `01`).
    *   Si format `1/12`, seule la partie avant le `/` est utilisée (`01`).

### Formatage des Dossiers (Albums)
*   **Pattern :** `(Année) Album`
*   **Gestion Multi-Années (Compilations) :**
    *   Si plusieurs années détectées (ex: 1995, 1998, 2000) : Format `MinYear-YY` (ex: `1995-00`).
    *   Si année unique : Format `(Année)`.

## 3. Correction de la Casse (`case_corrector.py`)

### Règle Générale (Sentence Case)
*   Appliquée aux **Titres** et **Albums**.
*   **Règle :** Première lettre en majuscule, tout le reste en minuscule.

### Exceptions & Protections (Prioritaires)
1.  **Chiffres Romains :** Maintien en majuscules (I, II, III, IV, V, VI, VII, VIII, IX, X, XI, XII, XIII, XIV, XV, XVI, XVII, XVIII, XIX, XX, L, C, D, M, etc.).
2.  **"I" Isolé :** Le mot "i" seul devient toujours "I".
3.  **Prépositions (Minuscules) :** Sauf en début de phrase.
    *   *Anglais :* a, an, the, and, but, or, nor, at, by, for, from, in, into, of, off, on, onto, out, over, up, with, to, as, via, under.
    *   *Français :* le, la, les, un, une, des, du, de, et, ou, mais, ni, car, dans, par, pour, en, vers, avec, sans, sous, sur, chez.
4.  **Abréviations (Majuscules) :** USA, UK, US, DJ, MC, NYC, LA, SF, DC, CD, DVD, TV, FM, AM, PM, BC, AD, CEO, FBI, CIA, NASA, BBC, CNN, ESPN, MTV, VHS, GPS, WWW, HTTP, FTP.
5.  **Artiste dans l'Album :** Si le nom de l'artiste apparaît dans le titre de l'album, la casse de l'artiste est préservée.
6.  **Exceptions Personnalisées :** Liste chargée depuis la base de données.

## 4. Formatage des Métadonnées (`metadata_formatter.py`)

### Règles de Champs
*   **Artiste (TPE1) → Album Artist (TPE2) :** Si `Album Artist` est vide, copie la valeur de `Artist`.
*   **Numéro de Piste (TRCK) :**
    *   Padding avec zéro (`01`).
    *   Si total présent (`1/12`), il est préservé (`01/12`).
*   **Année (TYER/TDRC) :**
    *   Si compilation (plusieurs années) : Format `MinYear-MaxYear` (ex: `1995-2000`).
    *   Validation : Doit être > 1900 et < AnnéeCourante + 1.
*   **Genre (TCON) :**
    *   Conversion des genres numériques ID3v1 (ex: `(13)` → `Pop`).
    *   Nettoyage : Suppression des chiffres et caractères spéciaux (sauf `&`, `/`, `+`, `-`).
    *   Normalisation : Title Case (Première lettre de chaque mot majuscule).
    *   Validation par rapport à une liste standard de genres.
