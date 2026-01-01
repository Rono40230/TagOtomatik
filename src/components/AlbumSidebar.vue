<script setup lang="ts">
import type { Album } from '../types';
import { computed, ref, onMounted, watch, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const props = defineProps<{
  album: Album
}>();

const emit = defineEmits<{
  (e: 'update:title', value: string): void
  (e: 'update:artist', value: string): void
  (e: 'update:year', value: number): void
  (e: 'update:genre', value: string): void
  (e: 'change-cover'): void
}>();

const coverUrl = ref<string | null>(null);
const junkFiles = ref<string[]>([]);

async function loadCover() {
  if (!props.album.cover_path) {
    coverUrl.value = null;
    return;
  }

  try {
    // Utilisation de la commande Rust 'read_cover' pour contourner les restrictions de sécurité
    const contents = await invoke<number[]>('read_cover', { path: props.album.cover_path });
    const uint8Array = new Uint8Array(contents);
    const blob = new Blob([uint8Array]);
    const url = URL.createObjectURL(blob);
    
    if (coverUrl.value && coverUrl.value.startsWith('blob:')) {
      URL.revokeObjectURL(coverUrl.value);
    }
    coverUrl.value = url;
  } catch (e) {
    // Fail silently
    coverUrl.value = null;
  }
}

async function checkJunkFiles() {
  if (!props.album.path) return;
  try {
    junkFiles.value = await invoke('scan_junk', { path: props.album.path });
  } catch (e) {
    junkFiles.value = [];
  }
}

onMounted(() => {
  loadCover();
  checkJunkFiles();
});
watch(() => props.album.cover_path, loadCover);
onUnmounted(() => {
  if (coverUrl.value) URL.revokeObjectURL(coverUrl.value);
});
watch(() => props.album.path, checkJunkFiles);

const GENRES = [
    "Acid Jazz", "B.O. de Films", "Blues", "Chansons Française", "Disco",
    "Electronique", "Flamenco", "Folk", "Funk", "Jazz", "Musique Afriquaine",
    "Musique Andine", "Musique Brésilienne", "Musique Classique", "Musique Cubaine",
    "Musique Franco-Hispanique", "New-Wave", "Pop", "Rap", "Reggae", "Rock",
    "Soul", "Top 50", "Trip-Hop", "Zouk"
];

// Computed property to get the common album tag from tracks
const commonAlbumTag = computed(() => {
  if (props.album.tracks.length > 0) {
    return props.album.tracks[0].album;
  }
  return props.album.title; // Fallback
});
</script>

<template>
  <aside class="w-96 flex-shrink-0 h-full overflow-y-auto custom-scrollbar">
    <div class="bg-gray-800 rounded-lg shadow p-6 border border-gray-700 min-h-min">
      <div class="aspect-square bg-gray-700 rounded-md mb-6 overflow-hidden relative group shadow-lg">
          <img v-if="coverUrl" :src="coverUrl" class="w-full h-full object-cover" />
          <div v-else class="w-full h-full flex items-center justify-center text-6xl text-gray-500">🎵</div>
          
          <!-- Edit Cover Overlay -->
          <div class="absolute inset-0 bg-black/50 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center">
            <button 
              @click="$emit('change-cover')"
              class="bg-white text-gray-900 px-4 py-2 rounded-full font-medium text-sm hover:bg-gray-100 transform hover:scale-105 transition-all"
            >
              📷 Changer
            </button>
          </div>
      </div>
      
      <div class="space-y-4">
        <div>
          <label class="block text-xs font-medium text-gray-400 uppercase mb-1">Album</label>
          <input 
            :value="commonAlbumTag"
            @input="$emit('update:title', ($event.target as HTMLInputElement).value)"
            class="w-full h-10 p-2.5 border border-gray-600 rounded-lg text-base font-semibold bg-gray-700 text-white focus:border-blue-500 focus:ring-1 focus:ring-blue-500 transition-colors" 
          />
        </div>
        <div>
          <label class="block text-xs font-medium text-gray-400 uppercase mb-1">Artiste Album</label>
          <input 
            :value="album.artist"
            @input="$emit('update:artist', ($event.target as HTMLInputElement).value)"
            class="w-full h-10 p-2.5 border border-gray-600 rounded-lg text-sm bg-gray-700 text-white focus:border-blue-500 focus:ring-1 focus:ring-blue-500 transition-colors" 
          />
        </div>
        <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="block text-xs font-medium text-gray-400 uppercase mb-1">Année</label>
              <input 
                :value="album.year"
                @input="$emit('update:year', Number(($event.target as HTMLInputElement).value))"
                type="number" 
                class="w-full h-10 p-2.5 border border-gray-600 rounded-lg text-sm bg-gray-700 text-white focus:border-blue-500 focus:ring-1 focus:ring-blue-500 transition-colors" 
              />
            </div>
            <div>
              <label class="block text-xs font-medium text-gray-400 uppercase mb-1">Genre</label>
              <select 
                :value="album.tracks[0]?.genre || ''"
                @change="$emit('update:genre', ($event.target as HTMLSelectElement).value)"
                class="w-full h-10 p-2.5 border border-gray-600 rounded-lg text-sm bg-gray-700 text-white focus:border-blue-500 focus:ring-1 focus:ring-blue-500 transition-colors"
              >
                <option value=""></option>
                <option v-for="genre in GENRES" :key="genre" :value="genre">{{ genre }}</option>
              </select>
            </div>
        </div>

        <!-- Junk Files Notification -->
        <div v-if="junkFiles.length > 0" class="mt-6 p-4 bg-red-900/30 border border-red-800/50 rounded-lg">
          <div class="flex items-center gap-2 text-red-300 mb-2">
            <span class="text-lg">🗑️</span>
            <span class="font-medium text-sm">Fichiers inutiles détectés</span>
          </div>
          <ul class="text-xs text-red-400/80 list-disc list-inside space-y-1">
            <li v-for="file in junkFiles" :key="file" class="truncate">{{ file }}</li>
          </ul>
        </div>
      </div>
    </div>
  </aside>
</template>
