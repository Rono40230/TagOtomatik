# 🗺️ PROJECT MAP (Généré automatiquement)
Date: lun. 05 janv. 2026 08:47:04 CET

## 📂 Arborescence (Backend)
```
  src
    commands
      apply.rs
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
      renamer.rs
      scanner.rs
      validator.rs
```

## 📂 Arborescence (Frontend)
```
src
  App.vue
  assets
    vue.svg
  components
    AlbumCard.vue
    AlbumDetailToolbar.vue
    AlbumEditor.vue
    AlbumSidebar.vue
    ConfirmationModal.vue
    ConversionModal.vue
    ConversionSummary.vue
    ConversionTrackRow.vue
    CoverSearchModal.vue
    DashboardTitle.vue
    ExceptionDialog.vue
    ImportCard.vue
    MultiAlbumEditor.vue
    MultiAlbumSidebar.vue
    PlayerBar.vue
    PlaylistModal.vue
    settings
    SettingsModal.vue
      SettingsAudioTab.vue
      SettingsExceptionsTab.vue
      SettingsPlaylistTab.vue
    ToastContainer.vue
    TrackList.vue
    TrackRow.vue
  composables
    useAlbumCorrection.ts
    useLibraryPersistence.ts
    useSmartDiff.ts
  constants.ts
  main.ts
  router
    index.ts
  stores
    exceptions.ts
    library.ts
    player.ts
    playlist.ts
    settings.ts
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
models/track.rs:pub struct Track {
models/album.rs:pub enum AlbumStatus {
models/album.rs:pub struct Album {
models/error.rs:pub enum AppError {
models/playlist.rs:pub struct Playlist {
models/playlist.rs:pub struct PlaylistTrack {
models/playlist.rs:pub struct PlaylistOptions {
models/exception.rs:pub struct CaseException {
services/audio.rs:pub struct AudioService;
services/io.rs:pub struct IOService;
services/cover.rs:pub struct CoverResult {
services/cover.rs:pub struct CoverService {
services/playlist.rs:pub struct PlaylistService;
services/converter.rs:pub struct ConverterService;
services/player.rs:pub enum PlayerCommand {
services/player.rs:pub struct AudioPlayerState {
services/cleaner.rs:pub struct CleanerService;
services/renamer.rs:pub struct RenamerService;
services/exception.rs:pub struct ExceptionService;
services/processor.rs:pub struct ReplacementRule {
services/processor.rs:pub struct MetadataProcessorService;
services/scanner.rs:pub struct ScannerService;
services/validator.rs:pub struct ValidatorService;
commands/cover.rs:pub struct CoverServiceState(pub Mutex<CoverService>);
commands/playlist.rs:pub struct PlaylistServiceState(pub Mutex<PlaylistService>);
commands/history.rs:pub fn get_scan_history(db: State<Database>) -> Result<Vec<String>, String> {
db/database.rs:pub struct Database {
lib.rs:pub fn run() {
```
