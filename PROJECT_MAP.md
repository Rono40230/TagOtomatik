# 🗺️ PROJECT MAP (Généré automatiquement)
Date: mar. 30 déc. 2025 20:06:35 CET

## 📂 Arborescence (Backend)
```
  src
    commands
      correct.rs
      exception.rs
      mod.rs
      scan.rs
      write.rs
    db
      database.rs
      mod.rs
    lib.rs
    main.rs
    models
      album.rs
      error.rs
      exception.rs
      mod.rs
      track.rs
    services
      audio.rs
      exception.rs
      io.rs
      mod.rs
      processor.rs
      scanner.rs
```

## 📂 Arborescence (Frontend)
```
src
  App.vue
  assets
    vue.svg
  components
    AlbumCard.vue
    ToastContainer.vue
  main.ts
  router
    index.ts
  stores
    exceptions.ts
    library.ts
    toast.ts
  style.css
  types
    index.ts
  views
    AlbumDetailView.vue
    Dashboard.vue
    LibraryView.vue
    SettingsView.vue
  vite-env.d.ts
```

## 🦀 Interfaces Publiques (Rust)
Liste des fonctions publiques et structs exposés :
```rust
lib.rs:pub fn run() {
models/track.rs:pub struct Track {
models/album.rs:pub enum AlbumStatus {
models/album.rs:pub struct Album {
models/error.rs:pub enum AppError {
models/exception.rs:pub struct CaseException {
services/audio.rs:pub struct AudioService;
services/scanner.rs:pub struct ScannerService;
services/processor.rs:pub struct MetadataProcessorService;
services/exception.rs:pub struct ExceptionService;
services/io.rs:pub struct IOService;
db/database.rs:pub struct Database {
```
