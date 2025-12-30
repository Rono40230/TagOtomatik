<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { useLibraryStore } from '../stores/library';
import type { Album, Track } from '../types';

const route = useRoute();
const router = useRouter();
const libraryStore = useLibraryStore();

const albumId = route.params.id as string;
const album = computed(() => libraryStore.getAlbumById(albumId));

// Si l'album n'existe pas (refresh page), retour à la bibliothèque
onMounted(() => {
  if (!album.value) {
    router.push('/library');
  }
});

function goBack() {
  router.push('/library');
}

async function handleAutoCorrect() {
  if (album.value) {
    await libraryStore.autoCorrectAlbum(album.value.id);
  }
}

async function handleSave() {
  if (album.value) {
    await libraryStore.saveAlbum(album.value.id);
  }
}

// Helper pour détecter les changements
function hasChanged(track: Track, field: keyof Track): boolean {
    if (!track.original_metadata) return false;
    // @ts-ignore - Accès dynamique sûr ici
    return track[field] !== track.original_metadata[field];
}

// Helper pour afficher la valeur originale si changée
function getOriginalValue(track: Track, field: keyof Track): string {
    if (!track.original_metadata) return '';
    // @ts-ignore
    return String(track.original_metadata[field] || '');
}
</script>

<template>
  <div v-if="album" class="min-h-screen bg-gray-50 flex flex-col">
    <!-- Loading Overlay -->
    <div v-if="libraryStore.isLoading" class="fixed inset-0 bg-white/50 z-50 flex items-center justify-center backdrop-blur-sm">
      <div class="flex flex-col items-center gap-4">
        <div class="animate-spin rounded-full h-12 w-12 border-4 border-blue-500 border-t-transparent"></div>
        <p class="text-blue-600 font-medium">Traitement en cours...</p>
      </div>
    </div>

    <!-- Header / Toolbar -->
    <header class="bg-white shadow-sm sticky top-0 z-20">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 h-16 flex items-center justify-between">
        <div class="flex items-center gap-4">
          <button @click="goBack" class="p-2 hover:bg-gray-100 rounded-full transition-colors">
            ⬅️
          </button>
          <h1 class="text-xl font-bold text-gray-900 truncate max-w-md">
            {{ album.title }}
            <span class="text-gray-500 font-normal text-sm ml-2">par {{ album.artist }}</span>
          </h1>
        </div>
        
        <div class="flex gap-2">
          <button 
            @click="handleAutoCorrect"
            :disabled="libraryStore.isLoading"
            class="px-4 py-2 bg-blue-100 text-blue-700 rounded-lg hover:bg-blue-200 font-medium text-sm transition-colors flex items-center gap-2"
          >
            ✨ Auto-Correction
          </button>
          <button 
            @click="handleSave"
            :disabled="libraryStore.isLoading"
            class="px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 font-medium text-sm transition-colors shadow-sm disabled:opacity-50"
          >
            {{ libraryStore.isLoading ? '...' : '💾 Sauvegarder' }}
          </button>
        </div>
      </div>
    </header>

    <!-- Content -->
    <main class="flex-1 max-w-7xl w-full mx-auto px-4 sm:px-6 lg:px-8 py-8 flex gap-8">
      
      <!-- Sidebar: Album Info -->
      <aside class="w-64 flex-shrink-0">
        <div class="bg-white rounded-lg shadow p-4 sticky top-24">
          <div class="aspect-square bg-gray-200 rounded-md mb-4 overflow-hidden relative">
             <img v-if="album.cover_path" :src="album.cover_path" class="w-full h-full object-cover" />
             <div v-else class="w-full h-full flex items-center justify-center text-6xl text-gray-400">🎵</div>
          </div>
          
          <div class="space-y-3">
            <div>
              <label class="block text-xs font-medium text-gray-500 uppercase">Album</label>
              <input v-model="album.title" class="w-full mt-1 p-2 border rounded text-sm font-semibold" />
            </div>
            <div>
              <label class="block text-xs font-medium text-gray-500 uppercase">Artiste Album</label>
              <input v-model="album.artist" class="w-full mt-1 p-2 border rounded text-sm" />
            </div>
            <div>
              <label class="block text-xs font-medium text-gray-500 uppercase">Année</label>
              <input v-model="album.year" type="number" class="w-full mt-1 p-2 border rounded text-sm" />
            </div>
          </div>
        </div>
      </aside>

      <!-- Main: Tracks List -->
      <div class="flex-1 bg-white rounded-lg shadow overflow-hidden">
        <table class="min-w-full divide-y divide-gray-200">
          <thead class="bg-gray-50">
            <tr>
              <th scope="col" class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider w-12">#</th>
              <th scope="col" class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Titre</th>
              <th scope="col" class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Artiste</th>
              <th scope="col" class="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider w-20">Durée</th>
            </tr>
          </thead>
          <tbody class="bg-white divide-y divide-gray-200">
            <tr v-for="track in album.tracks" :key="track.path" class="hover:bg-gray-50 group">
              <!-- Track Number -->
              <td class="px-4 py-2 whitespace-nowrap text-sm text-gray-500">
                <input v-model="track.track_number" type="number" class="w-12 p-1 border-transparent bg-transparent hover:border-gray-300 border rounded text-center focus:bg-white focus:border-blue-500 focus:ring-0" />
              </td>
              
              <!-- Title -->
              <td class="px-4 py-2 whitespace-nowrap text-sm">
                <div class="relative">
                  <div v-if="hasChanged(track, 'title')" class="text-xs text-red-400 line-through mb-0.5">
                    {{ getOriginalValue(track, 'title') }}
                  </div>
                  <input 
                    v-model="track.title" 
                    :class="{'text-green-700 font-medium': hasChanged(track, 'title')}"
                    class="w-full p-1 border-transparent bg-transparent hover:border-gray-300 border rounded focus:bg-white focus:border-blue-500 focus:ring-0" 
                  />
                </div>
              </td>
              
              <!-- Artist -->
              <td class="px-4 py-2 whitespace-nowrap text-sm">
                <div class="relative">
                  <div v-if="hasChanged(track, 'artist')" class="text-xs text-red-400 line-through mb-0.5">
                    {{ getOriginalValue(track, 'artist') }}
                  </div>
                  <input 
                    v-model="track.artist" 
                    :class="{'text-green-700 font-medium': hasChanged(track, 'artist')}"
                    class="w-full p-1 border-transparent bg-transparent hover:border-gray-300 border rounded focus:bg-white focus:border-blue-500 focus:ring-0" 
                  />
                </div>
              </td>
              
              <!-- Duration -->
              <td class="px-4 py-2 whitespace-nowrap text-sm text-gray-500 text-right">
                {{ Math.floor(track.duration_sec / 60) }}:{{ (track.duration_sec % 60).toString().padStart(2, '0') }}
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </main>
  </div>
</template>
