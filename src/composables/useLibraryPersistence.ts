import { ref, type Ref, watch } from 'vue';
import { useToastStore } from '../stores/toast';
import type { Album } from '../types';

export function useLibraryPersistence(albums: Ref<Album[]>) {
    const scannedPaths = ref<Set<string>>(new Set());
    const blacklistedPaths = ref<Set<string>>(new Set());
    const toast = useToastStore();

    function loadState() {
        // Load paths
        const savedPaths = localStorage.getItem('tagotomatik_scanned_paths');
        if (savedPaths) {
            try { scannedPaths.value = new Set(JSON.parse(savedPaths)); } 
            catch (e) { /* Silent recovery */ }
        }

        // Load blacklist
        const savedBlacklist = localStorage.getItem('tagotomatik_blacklist');
        if (savedBlacklist) {
            try { blacklistedPaths.value = new Set(JSON.parse(savedBlacklist)); } 
            catch (e) { /* Silent recovery */ }
        }

        // Load albums cache
        const savedAlbums = localStorage.getItem('tagotomatik_albums_cache');
        if (savedAlbums) {
            try {
                const parsed = JSON.parse(savedAlbums);
                if (Array.isArray(parsed) && parsed.length > 0) {
                    albums.value = parsed;
                }
            } catch (e) {
                // Silent fail for cache
            }
        }
    }

    function saveState() {
        try {
            localStorage.setItem('tagotomatik_scanned_paths', JSON.stringify(Array.from(scannedPaths.value)));
            localStorage.setItem('tagotomatik_blacklist', JSON.stringify(Array.from(blacklistedPaths.value)));
            
            // Only save cache if not empty to avoid accidental wipe during load race conditions
            if (albums.value.length > 0) {
                localStorage.setItem('tagotomatik_albums_cache', JSON.stringify(albums.value));
            }
        } catch (e) {
            toast.error("Erreur sauvegarde état: " + String(e));
        }
    }

    // Auto-save on structural changes
    // We avoid deep watching 'albums' for performance, rely on explicit saveState() calls 
    // for content updates, but we watch the container list length.
    watch([scannedPaths, blacklistedPaths], () => {
        saveState();
    }, { deep: true });
    
    watch(() => albums.value.length, () => {
        saveState();
    });

    return {
        scannedPaths,
        blacklistedPaths,
        saveState,
        loadState
    };
}
