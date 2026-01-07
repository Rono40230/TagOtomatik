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
    
    const { scannedPaths, blacklistedPaths, saveState } = useLibraryPersistence();
    const { 
        autoCorrectAlbum, applyAutoCorrect, cancelAutoCorrect, 
        hasPendingCorrection, saveAlbum, applyMetadata
    } = useAlbumCorrection(albums, isLoading, error);

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
            if (!isAutoLoad) error.value = handleError(e, toast, 'Erreur de scan');
            else toast.error(`Erreur chargement auto ${path}`);
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
                albums.value[index] = updated;
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
