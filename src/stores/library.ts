import { defineStore } from 'pinia';
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { Album, Track, ScanResult } from '../types';
import { useToastStore } from './toast';
import { useLibraryPersistence } from '../composables/useLibraryPersistence';
import { useAlbumCorrection } from '../composables/useAlbumCorrection';

function handleError(e: unknown, toast: ReturnType<typeof useToastStore>, context: string): string {
    const errMsg = e instanceof Error ? e.message : String(e);
    toast.error(`${context}: ${errMsg}`);
    return errMsg;
}

export const useLibraryStore = defineStore('library', () => {
    const albums = ref<Album[]>([]);
    const currentPath = ref<string>('');
    const isLoading = ref(false);
    const error = ref<string | null>(null);
    const toast = useToastStore();
    
    // Load state immediately to restore albums from cache
    const { scannedPaths, blacklistedPaths, saveState, loadState } = useLibraryPersistence(albums);
    loadState();
    
    // Pass saveState as callback to persist changes made during correction
    const { 
        autoCorrectAlbum, applyAutoCorrect, cancelAutoCorrect, 
        hasPendingCorrection, saveAlbum, applyMetadata
    } = useAlbumCorrection(albums, isLoading, error, saveState);

    async function scanDirectory(path: string, isAutoLoad = false) {
        if (!path) return;
        currentPath.value = path;
        isLoading.value = true;
        error.value = null;
        
        try {
            if (!isAutoLoad) {
                scannedPaths.value.add(path);
                saveState();
            }

            const result = await invoke<ScanResult>('scan_directory', { path });
            // Force refresh of albums from result
            const foundAlbums = result.albums;
            
            if (result.errors.length > 0 && !isAutoLoad) {
                toast.warning(`${result.errors.length} fichiers ont été ignorés (erreurs de lecture).`);
                // Si besoin, stocker les erreurs dans le store pour affichage dans une modale
            }
            
            let restoredCount = 0;
            if (!isAutoLoad) {
                foundAlbums.forEach(a => {
                    if (blacklistedPaths.value.has(a.path)) {
                        // User explicitly scanned this path (or parent), so we restore ANY album found
                        // ignoring previous deletions.
                        blacklistedPaths.value.delete(a.path);
                        restoredCount++;
                    }
                });
                if (restoredCount > 0) saveState();
            }

            const validAlbums = foundAlbums.filter(a => !blacklistedPaths.value.has(a.path));
            
            // Sync Logic: Remove albums that are under this scan path but not in the new result
            // This handles folders that were renamed, moved, or deleted externally
            const scanPathNormalized = path.replace(/\\/g, '/');
            const foundIds = new Set(validAlbums.map(a => a.id));

            albums.value = albums.value.filter(existing => {
                const existingPath = existing.path.replace(/\\/g, '/');
                // Check if existing album belongs to the scope of current scan
                // We use startsWith. We append '/' to ensure we match folders properly (avoid /Music vs /Music2 matching)
                // Exception: if path is the album itself.
                
                // If the scan path IS the album path (direct import), then we replace it.
                // If scan path is parent, we replace children.
                
                const isUnderScope = existingPath === scanPathNormalized || existingPath.startsWith(scanPathNormalized + '/');
                
                if (isUnderScope) {
                     // Only keep if it was found in the new result
                     return foundIds.has(existing.id);
                }
                return true;
            });

            const existingIds = new Set(albums.value.map(a => a.id));
            const newAlbums = validAlbums.filter(a => !existingIds.has(a.id));
            albums.value.push(...newAlbums);
            
            if (!isAutoLoad) {
                if (newAlbums.length > 0) {
                    toast.success(`${newAlbums.length} albums ajoutés.`);
                } else if (restoredCount > 0) {
                    toast.success('Album restauré de la liste des ignorés.');
                } else if (foundAlbums.length > 0) {
                    const hiddenCount = foundAlbums.length - validAlbums.length;
                    if (hiddenCount > 0) {
                        toast.info(`${hiddenCount} album(s) ignoré(s) car précédemment supprimé(s).`);
                    } else {
                        toast.info('Albums déjà présents.');
                    }
                } else {
                    if (result.errors.length === 0) {
                         toast.info('Aucun album trouvé.');
                    }
                }
            }
        } catch (e) {
            if (!isAutoLoad) {
                error.value = handleError(e, toast, 'Erreur de scan');
            } else {
                // If auto-load fails (e.g. folder moved/deleted externally), silent cleanup
                // We remove the path from scannedPaths to prevent future errors
                // and we rely on the implementation above that cleared albums for valid paths.
                // But since scan failed, we haven't reached the cleanup logic above.
                // So if the ROOT path is gone, we should remove any album starting with it.
                

                scannedPaths.value.delete(path);
                saveState();
                
                // Cleanup associated albums from memory
                const deadPathNormalized = path.replace(/\\/g, '/');
                albums.value = albums.value.filter(a => {
                    const p = a.path.replace(/\\/g, '/');
                    return !(p === deadPathNormalized || p.startsWith(deadPathNormalized + '/'));
                });
            }
        } finally {
            isLoading.value = false;
        }
    }

    async function loadLibrary() {
        if (scannedPaths.value.size === 0) return;
        isLoading.value = true;
        try {
            for (const path of scannedPaths.value) await scanDirectory(path, true);
        } finally {
            isLoading.value = false;
        }
    }

    function getAlbumById(id: string): Album | undefined {
        return albums.value.find(a => a.id === id);
    }

    function removeAlbum(albumId: string) {
        const album = albums.value.find(a => a.id === albumId);
        if (album) {
            blacklistedPaths.value.add(album.path);
            if (scannedPaths.value.has(album.path)) scannedPaths.value.delete(album.path);
            saveState();
        }
        albums.value = albums.value.filter(a => a.id !== albumId);
        toast.info('Album retiré.');
    }

    function updateAlbumTracksField(albumId: string, field: string, value: Track[keyof Track]) {
        const album = albums.value.find(a => a.id === albumId);
        if (!album) return;
        
        album.tracks.forEach(track => {
            // @ts-ignore
            if (field in track && track[field] !== value) {
                // @ts-ignore
                track[field] = value;
                track.is_modified = true;
            }
        });
    }

    async function refreshAlbum(albumId: string) {
        const index = albums.value.findIndex(a => a.id === albumId);
        if (index === -1) return;
        
        isLoading.value = true;
        try {
            const result = await invoke<ScanResult>('scan_directory', { path: albums.value[index].path });
            if (result.albums.length > 0) {
                const updated = result.albums.find(a => a.path === albums.value[index].path) || result.albums[0];
                // Use splice to ensure reactivity trigger is clean
                albums.value.splice(index, 1, updated);
            }
        } catch (e) { /* Silent fail */ } finally { isLoading.value = false; }
    }

    return {
        albums, currentPath, isLoading, error,
        scanDirectory, getAlbumById, autoCorrectAlbum, applyAutoCorrect,
        cancelAutoCorrect, hasPendingCorrection, saveAlbum, removeAlbum,
        updateAlbumTracksField, refreshAlbum, loadLibrary, applyMetadata
    };
});
