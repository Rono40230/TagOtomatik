<script setup lang="ts">
import type { Track } from '../types';
import { reactive, ref, computed, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import TrackRow from './TrackRow.vue';

const props = defineProps<{
  tracks: Track[]
}>();

const emit = defineEmits<{
  (e: 'play', track: Track): void
  (e: 'add-to-playlist', path: string): void
}>();

// Column Resizing Logic
const colWidths = reactive<Record<string, number>>({
  play: 50,
  cover: 60,
  number: 60,
  filename: 200,
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

// Sorting Logic
const sortColumn = ref<keyof Track | null>(null);
const sortDirection = ref<'asc' | 'desc'>('asc');

function toggleSort(column: keyof Track) {
  if (sortColumn.value === column) {
    sortDirection.value = sortDirection.value === 'asc' ? 'desc' : 'asc';
  } else {
    sortColumn.value = column;
    sortDirection.value = 'asc';
  }
}

const sortedTracks = computed(() => {
  if (!sortColumn.value) return props.tracks;
  
  return [...props.tracks].sort((a, b) => {
    const col = sortColumn.value!;
    const valA = a[col];
    const valB = b[col];
    
    if (valA === valB) return 0;
    
    // Handle undefined/null
    if (valA === undefined || valA === null) return 1;
    if (valB === undefined || valB === null) return -1;
    
    let result = 0;
    if (typeof valA === 'string' && typeof valB === 'string') {
      result = valA.localeCompare(valB);
    } else if (typeof valA === 'number' && typeof valB === 'number') {
      result = valA - valB;
    } else {
        // Fallback
        result = String(valA).localeCompare(String(valB));
    }
    
    return sortDirection.value === 'asc' ? result : -result;
  });
});

// Cover Loading Logic
const trackCovers = reactive<Map<string, string>>(new Map());

async function loadCover(track: Track) {
    if (!track.has_cover || trackCovers.has(track.path)) return;
    
    try {
        const bytes = await invoke<number[]>('read_track_cover', { path: track.path });
        const blob = new Blob([new Uint8Array(bytes)]);
        const url = URL.createObjectURL(blob);
        trackCovers.set(track.path, url);
    } catch (e) {
        // Fail silently
    }
}

// Watch for new tracks to load covers (lazy loading could be better but let's start simple)
watch(() => props.tracks, (newTracks) => {
    newTracks.forEach(t => {
        if (t.has_cover) loadCover(t);
    });
}, { immediate: true });

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
</script>

<template>
  <div class="flex-1 bg-gray-800 rounded-lg shadow overflow-hidden border border-gray-700 flex flex-col h-full">
    <div class="overflow-auto flex-1">
      <table class="min-w-full divide-y divide-gray-700 table-fixed">
        <thead class="bg-gray-900 sticky top-0 z-20 shadow-md">
          <tr>
            <th scope="col" class="px-4 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider relative group" :style="{ width: colWidths.play + 'px' }">
              <div class="absolute right-0 top-0 bottom-0 w-1 cursor-col-resize hover:bg-blue-500 z-10" @mousedown="startResize($event, 'play')"></div>
            </th>
            <th scope="col" class="px-4 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider relative group cursor-pointer hover:bg-gray-800" :style="{ width: colWidths.cover + 'px' }" @click="toggleSort('has_cover')">
              Cover
              <span v-if="sortColumn === 'has_cover'" class="ml-1">{{ sortDirection === 'asc' ? '▲' : '▼' }}</span>
              <div class="absolute right-0 top-0 bottom-0 w-1 cursor-col-resize hover:bg-blue-500 z-10" @mousedown.stop="startResize($event, 'cover')"></div>
            </th>
            <th scope="col" class="px-4 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider relative group cursor-pointer hover:bg-gray-800" :style="{ width: colWidths.number + 'px' }" @click="toggleSort('track_number')">
              #
              <span v-if="sortColumn === 'track_number'" class="ml-1">{{ sortDirection === 'asc' ? '▲' : '▼' }}</span>
              <div class="absolute right-0 top-0 bottom-0 w-1 cursor-col-resize hover:bg-blue-500 z-10" @mousedown.stop="startResize($event, 'number')"></div>
            </th>
            <th scope="col" class="px-4 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider relative group cursor-pointer hover:bg-gray-800" :style="{ width: colWidths.filename + 'px' }" @click="toggleSort('filename')">
              Nom du fichier
              <span v-if="sortColumn === 'filename'" class="ml-1">{{ sortDirection === 'asc' ? '▲' : '▼' }}</span>
              <div class="absolute right-0 top-0 bottom-0 w-1 cursor-col-resize hover:bg-blue-500 z-10" @mousedown.stop="startResize($event, 'filename')"></div>
            </th>
            <th scope="col" class="px-4 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider relative group cursor-pointer hover:bg-gray-800" :style="{ width: colWidths.title + 'px' }" @click="toggleSort('title')">
              Titre
              <span v-if="sortColumn === 'title'" class="ml-1">{{ sortDirection === 'asc' ? '▲' : '▼' }}</span>
              <div class="absolute right-0 top-0 bottom-0 w-1 cursor-col-resize hover:bg-blue-500 z-10" @mousedown.stop="startResize($event, 'title')"></div>
            </th>
            <th scope="col" class="px-4 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider relative group cursor-pointer hover:bg-gray-800" :style="{ width: colWidths.artist + 'px' }" @click="toggleSort('artist')">
              Artiste
              <span v-if="sortColumn === 'artist'" class="ml-1">{{ sortDirection === 'asc' ? '▲' : '▼' }}</span>
              <div class="absolute right-0 top-0 bottom-0 w-1 cursor-col-resize hover:bg-blue-500 z-10" @mousedown.stop="startResize($event, 'artist')"></div>
            </th>
            <th scope="col" class="px-4 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider relative group cursor-pointer hover:bg-gray-800" :style="{ width: colWidths.album_artist + 'px' }" @click="toggleSort('album_artist')">
              Album Artiste
              <span v-if="sortColumn === 'album_artist'" class="ml-1">{{ sortDirection === 'asc' ? '▲' : '▼' }}</span>
              <div class="absolute right-0 top-0 bottom-0 w-1 cursor-col-resize hover:bg-blue-500 z-10" @mousedown.stop="startResize($event, 'album_artist')"></div>
            </th>
            <th scope="col" class="px-4 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider relative group cursor-pointer hover:bg-gray-800" :style="{ width: colWidths.album + 'px' }" @click="toggleSort('album')">
              Album
              <span v-if="sortColumn === 'album'" class="ml-1">{{ sortDirection === 'asc' ? '▲' : '▼' }}</span>
              <div class="absolute right-0 top-0 bottom-0 w-1 cursor-col-resize hover:bg-blue-500 z-10" @mousedown.stop="startResize($event, 'album')"></div>
            </th>
            <th scope="col" class="px-4 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider relative group cursor-pointer hover:bg-gray-800" :style="{ width: colWidths.genre + 'px' }" @click="toggleSort('genre')">
              Genre
              <span v-if="sortColumn === 'genre'" class="ml-1">{{ sortDirection === 'asc' ? '▲' : '▼' }}</span>
              <div class="absolute right-0 top-0 bottom-0 w-1 cursor-col-resize hover:bg-blue-500 z-10" @mousedown.stop="startResize($event, 'genre')"></div>
            </th>
            <th scope="col" class="px-4 py-3 text-right text-xs font-medium text-gray-400 uppercase tracking-wider relative group cursor-pointer hover:bg-gray-800" :style="{ width: colWidths.duration + 'px' }" @click="toggleSort('duration_sec')">
              Durée
              <span v-if="sortColumn === 'duration_sec'" class="ml-1">{{ sortDirection === 'asc' ? '▲' : '▼' }}</span>
              <div class="absolute right-0 top-0 bottom-0 w-1 cursor-col-resize hover:bg-blue-500 z-10" @mousedown.stop="startResize($event, 'duration')"></div>
            </th>
            <th scope="col" class="px-4 py-3 text-right text-xs font-medium text-gray-400 uppercase tracking-wider relative group" :style="{ width: colWidths.actions + 'px' }">
              <div class="absolute right-0 top-0 bottom-0 w-1 cursor-col-resize hover:bg-blue-500 z-10" @mousedown.stop="startResize($event, 'actions')"></div>
            </th>
          </tr>
        </thead>
        <tbody class="bg-gray-800 divide-y divide-gray-700">
          <TrackRow 
            v-for="track in sortedTracks" 
            :key="track.path"
            :track="track"
            :col-widths="colWidths"
            :cover-url="trackCovers.get(track.path)"
            @play="emit('play', $event)"
            @add-to-playlist="emit('add-to-playlist', $event)"
          />
        </tbody>
      </table>
    </div>
  </div>
</template>
