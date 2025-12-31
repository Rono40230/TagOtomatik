<script setup lang="ts">
import type { Track } from '../types';
import { reactive, ref } from 'vue';

defineProps<{
  tracks: Track[]
}>();

const emit = defineEmits<{
  (e: 'play', track: Track): void
  (e: 'add-to-playlist', path: string): void
}>();

// Column Resizing Logic
const colWidths = reactive<Record<string, number>>({
  play: 50,
  number: 60,
  title: 300,
  artist: 200,
  album_artist: 200,
  album: 200,
  genre: 150,
  duration: 100,
  actions: 50
});

const resizing = ref<string | null>(null);
const startX = ref(0);
const startW = ref(0);

function startResize(e: MouseEvent, col: string) {
  resizing.value = col;
  startX.value = e.pageX;
  startW.value = colWidths[col];
  document.addEventListener('mousemove', handleMouseMove);
  document.addEventListener('mouseup', stopResize);
  document.body.style.cursor = 'col-resize';
}

function handleMouseMove(e: MouseEvent) {
  if (!resizing.value) return;
  const diff = e.pageX - startX.value;
  colWidths[resizing.value] = Math.max(50, startW.value + diff);
}

function stopResize() {
  resizing.value = null;
  document.removeEventListener('mousemove', handleMouseMove);
  document.removeEventListener('mouseup', stopResize);
  document.body.style.cursor = '';
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
  <div class="flex-1 bg-gray-800 rounded-lg shadow overflow-hidden border border-gray-700 flex flex-col">
    <div class="overflow-x-auto flex-1">
      <table class="min-w-full divide-y divide-gray-700 table-fixed">
        <thead class="bg-gray-900">
          <tr>
            <th scope="col" class="px-4 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider relative group" :style="{ width: colWidths.play + 'px' }">
              <div class="absolute right-0 top-0 bottom-0 w-1 cursor-col-resize hover:bg-blue-500 z-10" @mousedown="startResize($event, 'play')"></div>
            </th>
            <th scope="col" class="px-4 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider relative group" :style="{ width: colWidths.number + 'px' }">
              #
              <div class="absolute right-0 top-0 bottom-0 w-1 cursor-col-resize hover:bg-blue-500 z-10" @mousedown="startResize($event, 'number')"></div>
            </th>
            <th scope="col" class="px-4 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider relative group" :style="{ width: colWidths.title + 'px' }">
              Titre
              <div class="absolute right-0 top-0 bottom-0 w-1 cursor-col-resize hover:bg-blue-500 z-10" @mousedown="startResize($event, 'title')"></div>
            </th>
            <th scope="col" class="px-4 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider relative group" :style="{ width: colWidths.artist + 'px' }">
              Artiste
              <div class="absolute right-0 top-0 bottom-0 w-1 cursor-col-resize hover:bg-blue-500 z-10" @mousedown="startResize($event, 'artist')"></div>
            </th>
            <th scope="col" class="px-4 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider relative group" :style="{ width: colWidths.album_artist + 'px' }">
              Album Artiste
              <div class="absolute right-0 top-0 bottom-0 w-1 cursor-col-resize hover:bg-blue-500 z-10" @mousedown="startResize($event, 'album_artist')"></div>
            </th>
            <th scope="col" class="px-4 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider relative group" :style="{ width: colWidths.album + 'px' }">
              Album
              <div class="absolute right-0 top-0 bottom-0 w-1 cursor-col-resize hover:bg-blue-500 z-10" @mousedown="startResize($event, 'album')"></div>
            </th>
            <th scope="col" class="px-4 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider relative group" :style="{ width: colWidths.genre + 'px' }">
              Genre
              <div class="absolute right-0 top-0 bottom-0 w-1 cursor-col-resize hover:bg-blue-500 z-10" @mousedown="startResize($event, 'genre')"></div>
            </th>
            <th scope="col" class="px-4 py-3 text-right text-xs font-medium text-gray-400 uppercase tracking-wider relative group" :style="{ width: colWidths.duration + 'px' }">
              Durée
              <div class="absolute right-0 top-0 bottom-0 w-1 cursor-col-resize hover:bg-blue-500 z-10" @mousedown="startResize($event, 'duration')"></div>
            </th>
            <th scope="col" class="px-4 py-3 text-right text-xs font-medium text-gray-400 uppercase tracking-wider relative group" :style="{ width: colWidths.actions + 'px' }">
              <div class="absolute right-0 top-0 bottom-0 w-1 cursor-col-resize hover:bg-blue-500 z-10" @mousedown="startResize($event, 'actions')"></div>
            </th>
          </tr>
        </thead>
        <tbody class="bg-gray-800 divide-y divide-gray-700">
        <tr v-for="track in tracks" :key="track.path" class="hover:bg-gray-700 group transition-colors">
          <!-- Play Button -->
          <td class="px-4 py-2 whitespace-nowrap text-sm text-gray-400">
            <button 
              @click="emit('play', track)"
              class="w-8 h-8 rounded-full bg-gray-700 hover:bg-blue-900 text-blue-400 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity"
            >
              ▶️
            </button>
          </td>

          <!-- Track Number -->
          <td class="px-4 py-2 whitespace-nowrap text-sm text-gray-300">
            <input 
              type="number" 
              v-model.number="track.track_number"
              class="w-12 bg-transparent border-b border-transparent hover:border-gray-500 focus:border-blue-500 focus:outline-none text-center"
              :class="{'text-yellow-400': hasChanged(track, 'track_number')}"
            />
          </td>

          <!-- Title -->
          <td class="px-4 py-2 whitespace-nowrap text-sm text-gray-300">
            <div class="relative">
              <div v-if="hasChanged(track, 'title')" class="text-xs text-red-400 line-through mb-0.5">
                {{ getOriginalValue(track, 'title') }}
              </div>
              <input 
                v-model="track.title"
                class="w-full bg-transparent border-b border-transparent hover:border-gray-500 focus:border-blue-500 focus:outline-none"
                :class="{'text-green-400 font-medium': hasChanged(track, 'title')}"
              />
            </div>
          </td>

          <!-- Artist -->
          <td class="px-4 py-2 whitespace-nowrap text-sm text-gray-300">
            <div class="relative">
              <div v-if="hasChanged(track, 'artist')" class="text-xs text-red-400 line-through mb-0.5">
                {{ getOriginalValue(track, 'artist') }}
              </div>
              <input 
                v-model="track.artist"
                class="w-full bg-transparent border-b border-transparent hover:border-gray-500 focus:border-blue-500 focus:outline-none"
                :class="{'text-green-400 font-medium': hasChanged(track, 'artist')}"
              />
            </div>
          </td>

          <!-- Album Artist -->
          <td class="px-4 py-2 whitespace-nowrap text-sm text-gray-300">
            <div class="relative">
              <div v-if="hasChanged(track, 'album_artist')" class="text-xs text-red-400 line-through mb-0.5">
                {{ getOriginalValue(track, 'album_artist') }}
              </div>
              <input 
                v-model="track.album_artist"
                class="w-full bg-transparent border-b border-transparent hover:border-gray-500 focus:border-blue-500 focus:outline-none"
                :class="{'text-green-400 font-medium': hasChanged(track, 'album_artist')}"
              />
            </div>
          </td>

          <!-- Album -->
          <td class="px-4 py-2 whitespace-nowrap text-sm text-gray-300">
            <div class="relative">
              <div v-if="hasChanged(track, 'album')" class="text-xs text-red-400 line-through mb-0.5">
                {{ getOriginalValue(track, 'album') }}
              </div>
              <input 
                v-model="track.album"
                class="w-full bg-transparent border-b border-transparent hover:border-gray-500 focus:border-blue-500 focus:outline-none"
                :class="{'text-green-400 font-medium': hasChanged(track, 'album')}"
              />
            </div>
          </td>

          <!-- Genre -->
          <td class="px-4 py-2 whitespace-nowrap text-sm text-gray-300">
            <div class="relative">
              <div v-if="hasChanged(track, 'genre')" class="text-xs text-red-400 line-through mb-0.5">
                {{ getOriginalValue(track, 'genre') }}
              </div>
              <input 
                v-model="track.genre"
                class="w-full bg-transparent border-b border-transparent hover:border-gray-500 focus:border-blue-500 focus:outline-none"
                :class="{'text-green-400 font-medium': hasChanged(track, 'genre')}"
              />
            </div>
          </td>

          <!-- Duration -->
          <td class="px-4 py-2 whitespace-nowrap text-sm text-gray-400 text-right">
            {{ Math.floor(track.duration_sec / 60) }}:{{ (track.duration_sec % 60).toString().padStart(2, '0') }}
          </td>

          <!-- Actions -->
          <td class="px-4 py-2 whitespace-nowrap text-right text-sm font-medium">
            <button 
              @click="emit('add-to-playlist', track.path)"
              class="text-gray-500 hover:text-blue-400 opacity-0 group-hover:opacity-100 transition-opacity"
              title="Ajouter à une playlist"
            >
              ➕
            </button>
          </td>
        </tr>
      </tbody>
    </table>
    </div>
  </div>
</template>
