<script setup lang="ts">
import type { Album } from '../types';
import { ref, onMounted, watch } from 'vue';
import { readFile } from '@tauri-apps/plugin-fs';
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
    const contents = await readFile(props.album.cover_path);
    const blob = new Blob([contents]);
    coverUrl.value = URL.createObjectURL(blob);
  } catch {
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
watch(() => props.album.path, checkJunkFiles);

const GENRES = [
    "Acid Jazz", "B.O. de Films", "Blues", "Chansons Française", "Disco",
    "Electronique", "Flamenco", "Folk", "Funk", "Jazz", "Musique Afriquaine",
    "Musique Andine", "Musique Brésilienne", "Musique Classique", "Musique Cubaine",
    "Musique Franco-Hispanique", "New-Wave", "Pop", "Rap", "Reggae", "Rock",
    "Soul", "Top 50", "Trip-Hop", "Zouk"
];
</script>

<template>
  <aside class="w-64 flex-shrink-0">
    <div class="bg-gray-800 rounded-lg shadow p-4 sticky top-24 border border-gray-700">
      <div class="aspect-square bg-gray-700 rounded-md mb-4 overflow-hidden relative group">
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
      
      <div class="space-y-3">
        <div>
          <label class="block text-xs font-medium text-gray-400 uppercase">Album</label>
          <input 
            :value="album.title"
            @input="$emit('update:title', ($event.target as HTMLInputElement).value)"
            class="w-full mt-1 p-2 border border-gray-600 rounded text-sm font-semibold bg-gray-700 text-white focus:border-blue-500 focus:ring-1 focus:ring-blue-500" 
          />
        </div>
        <div>
          <label class="block text-xs font-medium text-gray-400 uppercase">Artiste Album</label>
          <input 
            :value="album.artist"
            @input="$emit('update:artist', ($event.target as HTMLInputElement).value)"
            class="w-full mt-1 p-2 border border-gray-600 rounded text-sm bg-gray-700 text-white focus:border-blue-500 focus:ring-1 focus:ring-blue-500" 
          />
        </div>
        <div>
          <label class="block text-xs font-medium text-gray-400 uppercase">Année</label>
          <input 
            :value="album.year"
            @input="$emit('update:year', Number(($event.target as HTMLInputElement).value))"
            type="number" 
            class="w-full mt-1 p-2 border border-gray-600 rounded text-sm bg-gray-700 text-white focus:border-blue-500 focus:ring-1 focus:ring-blue-500" 
          />
        </div>
        <div>
          <label class="block text-xs font-medium text-gray-400 uppercase">Genre</label>
          <select 
            :value="album.tracks[0]?.genre || ''"
            @change="$emit('update:genre', ($event.target as HTMLSelectElement).value)"
            class="w-full mt-1 p-2 border border-gray-600 rounded text-sm bg-gray-700 text-white focus:border-blue-500 focus:ring-1 focus:ring-blue-500"
          >
            <option value="">Sélectionner...</option>
            <option v-for="genre in GENRES" :key="genre" :value="genre">{{ genre }}</option>
          </select>
        </div>

        <!-- Junk Files Notification -->
        <div v-if="junkFiles.length > 0" class="mt-4 p-3 bg-red-900/50 border border-red-700 rounded-md">
          <div class="flex items-center gap-2 text-red-200 mb-2">
            <span class="text-xl">🗑️</span>
            <span class="font-medium text-sm">Fichiers inutiles détectés</span>
          </div>
          <ul class="text-xs text-red-300 list-disc list-inside">
            <li v-for="file in junkFiles" :key="file" class="truncate">{{ file }}</li>
          </ul>
        </div>
      </div>
    </div>
  </aside>
</template>
