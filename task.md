# 📋 Tâches & Roadmap - TagOtomatik (L'Atelier de Normalisation)

**Vision** : TagOtomatik est un outil de transition (Staging Area). Il sert à nettoyer, identifier, taguer et normaliser des fichiers audio en vrac avant de les déplacer vers leur stockage définitif (NAS, Bibliothèque Hi-Fi, Smartphone).

---

## 🚀 Phase 1 : Identification Avancée (Le "Cerveau")
*Objectif : Identifier les fichiers même sans nom ni métadonnées.*

- [x] **Connecteurs de Métadonnées (APIs)**
    - [x] **MusicBrainz** : Récupération des tags officiels (ID Release, Date précise, Label).
    - [ ] **Cover Art Archive** : Récupération des pochettes en haute résolution (remplacement des images 200x200 floues).
- [ ] **Intégration AcoustID / Chromaprint (Rust)**
    - [ ] Implémenter le calcul d'empreinte audio (fingerprinting) côté Rust.
    - [ ] Interroger l'API AcoustID pour identifier les fichiers "Track01.mp3".
- [ ] **Détection de Doublons Audio**
    - [ ] Comparer les hashs audio pour supprimer les vrais doublons (pas juste par nom).

## 🎛️ Phase 2 : Normalisation Audio (L'Oreille)
*Objectif : Rendre les fichiers techniquement parfaits et homogènes.*

- [ ] **Calcul ReplayGain**
    - [ ] Scanner les albums pour calculer le gain (Track & Album).
    - [ ] Écrire les tags `REPLAYGAIN_*` (standard EBU R128 si possible).
- [ ] **Visualisation Waveform**
    - [ ] Générer une forme d'onde simplifiée pour repérer visuellement les silences excessifs ou les fichiers corrompus (bruit blanc).
- [ ] **Transcodage à l'Export**
    - [ ] Ajouter des profils de conversion lors de l'export final (ex: "Convertir en FLAC", "MP3 V0 pour mobile").

## 📦 Phase 3 : Workflow d'Export (La Sortie)
*Objectif : Automatiser le rangement vers la bibliothèque finale.*

- [x] **Correction & Sauvegarde (Fiabilité)**
    - [x] Correction du bug de sauvegarde des tags (ISRC invalide).
    - [x] Correction du bug de renommage de dossier (chemins relatifs).
- [ ] **Moteur de Renommage et Déplacement (Move/Copy)**
    - [ ] Créer une interface de définition de masque de sortie (ex: `{Artist}/{Year} - {Album}/{Track} - {Title}.{Ext}`).
    - [ ] Bouton "Finaliser & Déplacer" : Applique les tags, renomme, déplace vers le dossier cible, et nettoie le dossier source.
- [ ] **Nettoyage des "Déchets"**
    - [ ] Option pour supprimer automatiquement les fichiers `.nfo`, `.m3u`, `.txt` ou les images inutilisées après l'export.

## 🎨 Phase 4 : UX & Validation (Le Contrôle Qualité)
*Objectif : Donner confiance à l'utilisateur avant validation.*

- [x] **Améliorations UI**
    - [x] Bouton "Recherche de métadonnées" dédié.
    - [x] Double-clic pour maximiser la fenêtre.
    - [x] Uniformisation des boutons de navigation.
- [ ] **Vue "Diff" Avancée**
    - [ ] Améliorer la comparaison visuelle "Avant / Après" (mise en évidence des changements de tags).
- [ ] **Indicateurs de Qualité**
    - [ ] Badges visuels : "Bitrate faible", "Pas de pochette", "Tags manquants".
- [ ] **Support des Paroles (Lyrics)**
    - [ ] Récupération et intégration des paroles (tag `USLT` ou `.lrc`).

---

## 🛠️ Maintenance & Dette Technique
- [ ] **Refactoring Rust** : Continuer à surveiller la taille des fichiers `services/*.rs`.
- [ ] **Tests E2E** : Ajouter des tests sur le cycle complet (Import -> Correction -> Export).
