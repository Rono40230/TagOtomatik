<script setup lang="ts">
import { useLibraryStore } from '../stores/library';
import AlbumCard from '../components/AlbumCard.vue';
import { useRouter } from 'vue-router';

const libraryStore = useLibraryStore();
const router = useRouter();

function goBack() {
  router.push('/');
}
</script>

<template>
  <div class="min-h-screen bg-gray-900 text-white">
    <!-- Header -->
    <header class="bg-gray-800 shadow-sm sticky top-0 z-10 border-b border-gray-700">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 h-16 flex items-center justify-between">
        <div class="flex items-center gap-4">
          <button @click="goBack" class="p-2 hover:bg-gray-700 rounded-full transition-colors text-gray-300">
            ⬅️
          </button>
          <h1 class="text-xl font-bold text-white">Bibliothèque</h1>
          <span class="bg-blue-900 text-blue-200 text-xs font-medium px-2.5 py-0.5 rounded-full">
            {{ libraryStore.albums.length }} albums
          </span>
        </div>
        
        <div class="flex gap-2">
          <router-link 
            to="/playlists"
            class="px-4 py-2 bg-gray-700 border border-gray-600 text-gray-200 rounded-lg hover:bg-gray-600 text-sm font-medium transition-colors"
          >
            📂 Playlists
          </router-link>
          <!-- Actions globales (Filtrer, Trier...) -->
        </div>
      </div>
    </header>

    <!-- Grid -->
    <main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <div v-if="libraryStore.albums.length === 0" class="text-center py-20">
        <p class="text-gray-400 text-lg">Aucun album trouvé.</p>
        <button @click="goBack" class="mt-4 text-blue-400 hover:underline">Scanner un autre dossier</button>
      </div>
      
      <div v-else class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-6">
        <AlbumCard 
          v-for="album in libraryStore.albums" 
          :key="album.id" 
          :album="album" 
        />
      </div>
    </main>
  </div>
</template>
