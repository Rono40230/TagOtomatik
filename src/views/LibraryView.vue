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
  <div class="min-h-screen bg-gray-50">
    <!-- Header -->
    <header class="bg-white shadow-sm sticky top-0 z-10">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 h-16 flex items-center justify-between">
        <div class="flex items-center gap-4">
          <button @click="goBack" class="p-2 hover:bg-gray-100 rounded-full transition-colors">
            ⬅️
          </button>
          <h1 class="text-xl font-bold text-gray-900">Bibliothèque</h1>
          <span class="bg-blue-100 text-blue-800 text-xs font-medium px-2.5 py-0.5 rounded-full">
            {{ libraryStore.albums.length }} albums
          </span>
        </div>
        
        <div class="flex gap-2">
          <router-link 
            to="/playlists"
            class="px-4 py-2 bg-white border border-gray-300 text-gray-700 rounded-lg hover:bg-gray-50 text-sm font-medium transition-colors"
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
        <p class="text-gray-500 text-lg">Aucun album trouvé.</p>
        <button @click="goBack" class="mt-4 text-blue-600 hover:underline">Scanner un autre dossier</button>
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
