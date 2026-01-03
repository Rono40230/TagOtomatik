import { defineStore } from 'pinia';
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { Album, Track } from '../types';
import { useToastStore } from './toast';

// Extracted helper for error handling
// eslint-disable-next-line @typescript-eslint/no-explicit-any
function handleError(e: unknown, toast: any, context: string): string {
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
    const originalAlbums = ref<Map<string, Album>>(new Map());

    async function scanDirectory(path: string) {
        if (!path) return;
        currentPath.value = path;
        isLoading.value = true;
        error.value = null;
        
        try {
            const result = await invoke<Album[]>('scan_directory', { path });
            const existingIds = new Set(albums.value.map(a => a.id));
            const newAlbums = result.filter(a => !existingIds.has(a.id));
            albums.value.push(...newAlbums);
            
            if (newAlbums.length === 0) {
                toast.info(result.length > 0 ? 'Tous les albums trouvés sont déjà dans la bibliothèque.' : 'Aucun album trouvé dans ce dossier.');
            } else {
                toast.success(`${newAlbums.length} albums ajoutés.`);
            }
        } catch (e) {
            error.value = handleError(e, toast, 'Erreur de scan');
        } finally {
            isLoading.value = false;
        }
    }

    function getAlbumById(id: string): Album | undefined {
        return albums.value.find(a => a.id === id);
    }

    // Helper to handle album operations
    async function handleAlbumOperation<T>(
        albumId: string, 
        operation: (album: Album) => Promise<T>,
        onSuccess: (result: T, index: number) => void,
        successMsg: string,
        errorContext: string
    ) {
        const index = albums.value.findIndex(a => a.id === albumId);
        if (index === -1) return;

        isLoading.value = true;
        try {
            const result = await operation(JSON.parse(JSON.stringify(albums.value[index])));
            onSuccess(result, index);
            toast.success(successMsg);
        } catch (e) {
            error.value = handleError(e, toast, errorContext);
        } finally {
            isLoading.value = false;
        }
    }

    async function autoCorrectAlbum(albumId: string) {
        await handleAlbumOperation(
            albumId,
            (album) => {
                if (!originalAlbums.value.has(albumId)) {
                    originalAlbums.value.set(albumId, JSON.parse(JSON.stringify(album)));
                }
                return invoke<Album>('preview_auto_correct', { album });
            },
            (corrected, index) => albums.value[index] = corrected,
            'Prévisualisation de l\'auto-correction.',
            'Erreur auto-correction'
        );
    }

    async function applyAutoCorrect(albumId: string) {
        await handleAlbumOperation(
            albumId,
            (album) => invoke<Album>('apply_auto_correct', { album }),
            (final, index) => {
                albums.value[index] = final;
                originalAlbums.value.delete(albumId);
            },
            'Corrections appliquées avec succès.',
            'Erreur application'
        );
    }

    function cancelAutoCorrect(albumId: string) {
        const original = originalAlbums.value.get(albumId);
        if (original) {
            const index = albums.value.findIndex(a => a.id === albumId);
            if (index !== -1) {
                albums.value[index] = JSON.parse(JSON.stringify(original));
                originalAlbums.value.delete(albumId);
                toast.info('Auto-correction annulée.');
            }
        }
    }

    function hasPendingCorrection(albumId: string): boolean {
        return originalAlbums.value.has(albumId);
    }

    async function saveAlbum(albumId: string) {
        await handleAlbumOperation(
            albumId,
            (album) => invoke<Album>('save_album_changes', { album }),
            (saved, index) => albums.value[index] = saved,
            'Album sauvegardé avec succès.',
            'Erreur sauvegarde'
        );
    }

    function removeAlbum(albumId: string) {
        albums.value = albums.value.filter(a => a.id !== albumId);
        toast.info('Album retiré de la bibliothèque.');
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
            const result = await invoke<Album[]>('scan_directory', { path: albums.value[index].path });
            if (result.length > 0) {
                const updated = result.find(a => a.path === albums.value[index].path) || result[0];
                albums.value[index] = updated;
                if (originalAlbums.value.has(albumId)) {
                    originalAlbums.value.set(albumId, JSON.parse(JSON.stringify(updated)));
                }
            }
        } catch (e) {
            // Silent fail
        } finally {
            isLoading.value = false;
        }
    }

    return {
        albums, currentPath, isLoading, error,
        scanDirectory, getAlbumById, autoCorrectAlbum, applyAutoCorrect,
        cancelAutoCorrect, hasPendingCorrection, saveAlbum, removeAlbum,
        updateAlbumTracksField, refreshAlbum
    };
});
