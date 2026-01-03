import { defineStore } from 'pinia';
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { Album, Track } from '../types';
import { useToastStore } from './toast';

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
            
            // Filter duplicates based on ID
            const existingIds = new Set(albums.value.map(a => a.id));
            const newAlbums = result.filter(a => !existingIds.has(a.id));
            
            albums.value.push(...newAlbums);
            
            if (newAlbums.length === 0) {
                if (result.length > 0) {
                    toast.info('Tous les albums trouvés sont déjà dans la bibliothèque.');
                } else {
                    toast.info('Aucun album trouvé dans ce dossier.');
                }
            } else {
                toast.success(`${newAlbums.length} albums ajoutés.`);
            }
        } catch (e: unknown) {
            const errMsg = e instanceof Error ? e.message : String(e);
            error.value = errMsg;
            toast.error(`Erreur de scan: ${errMsg}`);
        } finally {
            isLoading.value = false;
        }
    }

    function getAlbumById(id: string): Album | undefined {
        return albums.value.find(a => a.id === id);
    }

    async function autoCorrectAlbum(albumId: string) {
        const albumIndex = albums.value.findIndex(a => a.id === albumId);
        if (albumIndex === -1) return;

        isLoading.value = true;
        try {
            // Save original state if not already saved
            if (!originalAlbums.value.has(albumId)) {
                originalAlbums.value.set(albumId, JSON.parse(JSON.stringify(albums.value[albumIndex])));
            }

            const albumToCorrect = JSON.parse(JSON.stringify(albums.value[albumIndex]));
            const correctedAlbum = await invoke<Album>('preview_auto_correct', { album: albumToCorrect });
            albums.value[albumIndex] = correctedAlbum;
            toast.success('Prévisualisation de l\'auto-correction.');
        } catch (e: unknown) {
            const errMsg = e instanceof Error ? e.message : String(e);
            error.value = errMsg;
            toast.error(`Erreur auto-correction: ${errMsg}`);
        } finally {
            isLoading.value = false;
        }
    }

    async function applyAutoCorrect(albumId: string) {
        const albumIndex = albums.value.findIndex(a => a.id === albumId);
        if (albumIndex === -1) return;

        isLoading.value = true;
        try {
            const albumToApply = JSON.parse(JSON.stringify(albums.value[albumIndex]));
            const finalAlbum = await invoke<Album>('apply_auto_correct', { album: albumToApply });
            albums.value[albumIndex] = finalAlbum;
            originalAlbums.value.delete(albumId); // Clear backup
            toast.success('Corrections appliquées avec succès.');
        } catch (e: unknown) {
            const errMsg = e instanceof Error ? e.message : String(e);
            error.value = errMsg;
            toast.error(`Erreur application: ${errMsg}`);
        } finally {
            isLoading.value = false;
        }
    }

    function cancelAutoCorrect(albumId: string) {
        const original = originalAlbums.value.get(albumId);
        if (original) {
            const albumIndex = albums.value.findIndex(a => a.id === albumId);
            if (albumIndex !== -1) {
                albums.value[albumIndex] = JSON.parse(JSON.stringify(original));
                originalAlbums.value.delete(albumId);
                toast.info('Auto-correction annulée.');
            }
        }
    }

    function hasPendingCorrection(albumId: string): boolean {
        return originalAlbums.value.has(albumId);
    }

    async function saveAlbum(albumId: string) {
        const albumIndex = albums.value.findIndex(a => a.id === albumId);
        if (albumIndex === -1) return;

        isLoading.value = true;
        try {
            const albumToSave = JSON.parse(JSON.stringify(albums.value[albumIndex]));
            const savedAlbum = await invoke<Album>('save_album_changes', { album: albumToSave });
            albums.value[albumIndex] = savedAlbum;
            toast.success('Album sauvegardé avec succès.');
        } catch (e: unknown) {
            const errMsg = e instanceof Error ? e.message : String(e);
            error.value = errMsg;
            toast.error(`Erreur sauvegarde: ${errMsg}`);
        } finally {
            isLoading.value = false;
        }
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
            if (field in track) {
                // @ts-ignore
                if (track[field] !== value) {
                    // @ts-ignore
                    track[field] = value;
                    track.is_modified = true;
                }
            }
        });
    }

    async function refreshAlbum(albumId: string) {
        const index = albums.value.findIndex(a => a.id === albumId);
        if (index === -1) return;
        
        const album = albums.value[index];
        isLoading.value = true;
        try {
            // Re-scan just this album's folder
            const result = await invoke<Album[]>('scan_directory', { path: album.path });
            if (result.length > 0) {
                // Find the matching album in results
                const updated = result.find(a => a.path === album.path) || result[0];
                albums.value[index] = updated;
                
                // Update original backup if it exists
                if (originalAlbums.value.has(albumId)) {
                    originalAlbums.value.set(albumId, JSON.parse(JSON.stringify(updated)));
                }
            }
        } catch (e: unknown) {
            // Silent fail or toast
        } finally {
            isLoading.value = false;
        }
    }

    return {
        albums,
        currentPath,
        isLoading,
        error,
        scanDirectory,
        getAlbumById,
        autoCorrectAlbum,
        applyAutoCorrect,
        cancelAutoCorrect,
        hasPendingCorrection,
        saveAlbum,
        removeAlbum,
        updateAlbumTracksField,
        refreshAlbum
    };
});
