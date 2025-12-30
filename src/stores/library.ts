import { defineStore } from 'pinia';
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { Album } from '../types';
import { useToastStore } from './toast';

export const useLibraryStore = defineStore('library', () => {
    const albums = ref<Album[]>([]);
    const isLoading = ref(false);
    const error = ref<string | null>(null);
    const toast = useToastStore();

    async function scanDirectory(path: string) {
        if (!path) return;
        
        isLoading.value = true;
        error.value = null;
        
        try {
            const result = await invoke<Album[]>('scan_directory', { path });
            albums.value = result;
            if (result.length === 0) {
                toast.info('Aucun album trouvé dans ce dossier.');
            } else {
                toast.success(`${result.length} albums trouvés.`);
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
            // Clone to avoid direct mutation issues before result
            const albumToCorrect = JSON.parse(JSON.stringify(albums.value[albumIndex]));
            const correctedAlbum = await invoke<Album>('auto_correct_album', { album: albumToCorrect });
            albums.value[albumIndex] = correctedAlbum;
            toast.success('Auto-correction appliquée.');
        } catch (e: unknown) {
            const errMsg = e instanceof Error ? e.message : String(e);
            error.value = errMsg;
            toast.error(`Erreur auto-correction: ${errMsg}`);
        } finally {
            isLoading.value = false;
        }
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

    return {
        albums,
        isLoading,
        error,
        scanDirectory,
        getAlbumById,
        autoCorrectAlbum,
        saveAlbum
    };
});
