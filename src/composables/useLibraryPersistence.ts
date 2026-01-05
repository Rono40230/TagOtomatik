import { ref } from 'vue';
import { useToastStore } from '../stores/toast';

export function useLibraryPersistence() {
    const scannedPaths = ref<Set<string>>(new Set());
    const blacklistedPaths = ref<Set<string>>(new Set());
    const toast = useToastStore();

    function loadState() {
        const savedPaths = localStorage.getItem('tagotomatik_scanned_paths');
        if (savedPaths) {
            try {
                scannedPaths.value = new Set(JSON.parse(savedPaths));
            } catch (e) { 
                toast.error('Erreur chargement chemins scannés');
            }
        }

        const savedBlacklist = localStorage.getItem('tagotomatik_blacklist');
        if (savedBlacklist) {
            try {
                blacklistedPaths.value = new Set(JSON.parse(savedBlacklist));
            } catch (e) { 
                toast.error('Erreur chargement blacklist');
            }
        }
    }


    function saveState() {
        localStorage.setItem('tagotomatik_scanned_paths', JSON.stringify(Array.from(scannedPaths.value)));
        localStorage.setItem('tagotomatik_blacklist', JSON.stringify(Array.from(blacklistedPaths.value)));
    }

    // Load immediately
    loadState();

    return {
        scannedPaths,
        blacklistedPaths,
        saveState
    };
}
