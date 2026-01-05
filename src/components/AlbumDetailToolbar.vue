<script setup lang="ts">
import { computed } from 'vue';
import { useRouter } from 'vue-router';
import { useLibraryStore } from '../stores/library';
import type { Album } from '../types';

const props = defineProps<{
  albums: Album[]
}>();

const router = useRouter();
const libraryStore = useLibraryStore();

const pageTitle = computed(() => {
  if (props.albums.length === 1) {
    return props.albums[0].title;
  }
  return `${props.albums.length} albums sélectionnés`;
});

const pageSubtitle = computed(() => {
  if (props.albums.length === 1) {
    return `par ${props.albums[0].artist}`;
  }
  return 'Édition multiple';
});

const hasAnyPending = computed(() => {
  return props.albums.some(a => libraryStore.hasPendingCorrection(a.id));
});

function goBack() {
  router.push('/library');
}

async function handleAutoCorrectAll() {
  for (const album of props.albums) {
    await libraryStore.autoCorrectAlbum(album.id);
  }
}

async function handleApplyCorrectionAll() {
  for (const album of props.albums) {
    if (libraryStore.hasPendingCorrection(album.id)) {
      await libraryStore.applyAutoCorrect(album.id);
    }
  }
}

function handleCancelCorrectionAll() {
  for (const album of props.albums) {
    if (libraryStore.hasPendingCorrection(album.id)) {
      libraryStore.cancelAutoCorrect(album.id);
    }
  }
}

async function handleSaveAll() {
  for (const album of props.albums) {
    await libraryStore.saveAlbum(album.id);
  }
}
</script>

<template>
    <header class="bg-gray-800 shadow-sm sticky top-0 z-20 border-b border-gray-700">
      <div class="w-full px-6 h-16 grid grid-cols-3 items-center">
        <div class="flex items-center justify-start gap-4">
          <button @click="goBack" class="p-2 hover:bg-gray-700 rounded-full transition-colors text-gray-300">
            ⬅️
          </button>
        </div>
        
        <div class="flex items-center justify-center gap-2">
          <template v-if="hasAnyPending">
            <div class="flex items-center gap-2 bg-yellow-900/30 px-3 py-1 rounded-lg border border-yellow-700/50 mr-2">
              <span class="text-yellow-400 text-xs font-medium">Prévisualisation ({{ albums.filter(a => libraryStore.hasPendingCorrection(a.id)).length }})</span>
              <button 
                @click="handleApplyCorrectionAll"
                class="px-3 py-1.5 bg-green-600 text-white text-xs rounded hover:bg-green-500 transition-colors"
              >
                Tout Appliquer
              </button>
              <button 
                @click="handleCancelCorrectionAll"
                class="px-3 py-1.5 bg-red-600 text-white text-xs rounded hover:bg-red-500 transition-colors"
              >
                Tout Annuler
              </button>
            </div>
          </template>
          <template v-else>
            <button 
              @click="handleAutoCorrectAll"
              :disabled="libraryStore.isLoading"
              class="px-4 py-2 bg-blue-900/50 text-blue-300 border border-blue-700 rounded-lg hover:bg-blue-900 font-medium text-sm transition-colors flex items-center gap-2"
            >
              ✨ Auto-Correction (Tout)
            </button>
            <button 
              @click="handleSaveAll"
              :disabled="libraryStore.isLoading"
              class="px-4 py-2 bg-green-700 text-white rounded-lg hover:bg-green-600 font-medium text-sm transition-colors shadow-sm disabled:opacity-50"
            >
              {{ libraryStore.isLoading ? '...' : '💾 Tout Sauvegarder' }}
            </button>
          </template>
        </div>

        <div class="flex items-center justify-end">
          <!-- Empty right column for balance -->
        </div>
      </div>
    </header>
</template>
