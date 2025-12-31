# 🗺️ PROJECT MAP (Généré automatiquement)
Date: mer. 31 déc. 2025 18:48:06 CET

## 📂 Arborescence (Backend)
```
  src
    commands
      converter.rs
      correct.rs
      cover.rs
      exception.rs
      history.rs
      mod.rs
      player.rs
      playlist.rs
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
      playlist.rs
      track.rs
    services
      audio.rs
      cleaner.rs
      converter.rs
      cover.rs
      dictionaries.rs
      exception.rs
      io.rs
      mod.rs
      player.rs
      playlist.rs
      processor.rs
      processor_tests.rs
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
    AlbumSidebar.vue
    CoverSearchModal.vue
    PlayerBar.vue
    ToastContainer.vue
    TrackList.vue
  main.ts
  router
    index.ts
  stores
    exceptions.ts
    library.ts
    player.ts
    playlist.ts
    toast.ts
  style.css
  types
    index.ts
  views
    AlbumDetailView.vue
    ConverterView.vue
    Dashboard.vue
    LibraryView.vue
    PlaylistView.vue
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
models/playlist.rs:pub struct Playlist {
models/playlist.rs:pub struct PlaylistTrack {
services/audio.rs:pub struct AudioService;
services/scanner.rs:pub struct ScannerService;
services/processor.rs:pub struct MetadataProcessorService;
services/exception.rs:pub struct ExceptionService;
services/io.rs:pub struct IOService;
services/cover.rs:pub struct CoverResult {
services/cover.rs:pub struct CoverService {
services/playlist.rs:pub struct PlaylistService;
services/converter.rs:pub struct ConverterService;
services/player.rs:pub enum PlayerCommand {
services/player.rs:pub struct AudioPlayerState {
services/cleaner.rs:pub struct CleanerService;
commands/cover.rs:pub struct CoverServiceState(pub Mutex<CoverService>);
commands/playlist.rs:pub struct PlaylistServiceState(pub Mutex<PlaylistService>);
commands/history.rs:pub fn get_scan_history(db: State<Database>) -> Result<Vec<String>, String> {
db/database.rs:pub struct Database {
```
