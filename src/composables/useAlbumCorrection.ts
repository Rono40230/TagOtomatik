import { ref, type Ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { Album } from '../types';
import { useToastStore } from '../stores/toast';

function handleError(e: unknown, toast: ReturnType<typeof useToastStore>, context: string): string {
    let errMsg = '';
    if (e instanceof Error) {
        errMsg = e.message;
    } else if (typeof e === 'string') {
        errMsg = e;
    } else if (typeof e === 'object' && e !== null) {
        // Try to find a message property or stringify
        if ('message' in e) {
            errMsg = String((e as any).message);
        } else {
            try {
                errMsg = JSON.stringify(e);
            } catch {
                errMsg = String(e);
            }
        }
    } else {
        errMsg = String(e);
    }
    
    toast.error(`${context}: ${errMsg}`);
    return errMsg;
}

export function useAlbumCorrection(
    albums: Ref<Album[]>, 
    isLoading: Ref<boolean>, 
    error: Ref<string | null>
) {
    const originalAlbums = ref<Map<string, Album>>(new Map());
    const toast = useToastStore();

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
            'Corrections appliquées.',
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
            'Album sauvegardé.',
            'Erreur sauvegarde'
        );
    }

    function applyMetadata(albumId: string, metadata: { title: string, artist: string, year?: string }) {
        const index = albums.value.findIndex(a => a.id === albumId);
        if (index === -1) return;

        const album = albums.value[index];
        
        // Save original if not already saved (for undo/cancel)
        if (!originalAlbums.value.has(albumId)) {
            originalAlbums.value.set(albumId, JSON.parse(JSON.stringify(album)));
        }

        // Update fields
        album.title = metadata.title;
        album.artist = metadata.artist;
        if (metadata.year) {
             const year = parseInt(metadata.year.split('-')[0]);
             if (!isNaN(year)) album.year = year;
        }

        // Update tracks
        album.tracks.forEach(track => {
            track.album = metadata.title;
            track.album_artist = metadata.artist;
            if (metadata.year) {
                 const year = parseInt(metadata.year.split('-')[0]);
                 if (!isNaN(year)) track.year = year;
            }
            track.is_modified = true;
        });
        
        toast.success('Métadonnées appliquées (en attente de validation)');
    }

    return {
        autoCorrectAlbum,
        applyAutoCorrect,
        cancelAutoCorrect,
        hasPendingCorrection,
        saveAlbum,
        applyMetadata
    };
}
