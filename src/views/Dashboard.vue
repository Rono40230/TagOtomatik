<script setup lang="ts">
import { ref } from 'vue';
import { open } from '@tauri-apps/plugin-dialog';
import { useLibraryStore } from '../stores/library';
import { useExceptionsStore } from '../stores/exceptions';
import { useRouter } from 'vue-router';

const libraryStore = useLibraryStore();
const exceptionsStore = useExceptionsStore();
const router = useRouter();
const isScanning = ref(false);

async function openFolder() {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
    });

    if (selected && typeof selected === 'string') {
      isScanning.value = true;
      await libraryStore.scanDirectory(selected);
      isScanning.value = false;
      
      if (libraryStore.albums.length > 0) {
        router.push('/library');
      }
    }
  } catch (err) {
    exceptionsStore.addError(err instanceof Error ? err : new Error(String(err)));
    isScanning.value = false;
  }
}
</script>

<template>
  <div class="flex flex-col items-center justify-center min-h-screen p-8 text-center">
    <h1 class="mb-8 text-4xl font-bold text-gray-800">TagOtomatik</h1>
    
    <div class="max-w-md w-full bg-white rounded-xl shadow-lg p-8">
      <p class="mb-6 text-gray-600">
        Sélectionnez un dossier musical pour commencer le nettoyage et l'organisation de votre bibliothèque.
      </p>
      
      <button 
        @click="openFolder" 
        :disabled="isScanning || libraryStore.isLoading"
        class="w-full py-3 px-6 text-white bg-blue-600 hover:bg-blue-700 rounded-lg font-medium transition-colors flex items-center justify-center gap-2 disabled:opacity-70 disabled:cursor-not-allowed"
      >
        <span v-if="isScanning || libraryStore.isLoading" class="animate-spin">⏳</span>
        <span v-else>📂</span>
        {{ isScanning || libraryStore.isLoading ? 'Analyse en cours...' : 'Ouvrir un dossier' }}
      </button>

      <div class="mt-4 pt-4 border-t border-gray-100">
        <router-link 
          to="/settings"
          class="text-gray-500 hover:text-gray-700 text-sm font-medium flex items-center justify-center gap-1"
        >
          <span>⚙️</span> Gérer les exceptions
        </router-link>
      </div>
      
      <div v-if="libraryStore.error" class="mt-4 p-3 bg-red-100 text-red-700 rounded-lg text-sm">
        {{ libraryStore.error }}
      </div>
    </div>
    
    <!-- Historique (Placeholder) -->
    <div class="mt-12 w-full max-w-2xl">
      <h2 class="text-xl font-semibold mb-4 text-gray-700">Derniers dossiers</h2>
      <div class="bg-white rounded-lg shadow p-4 text-gray-500 italic">
        Aucun historique récent.
      </div>
    </div>
  </div>
</template>
