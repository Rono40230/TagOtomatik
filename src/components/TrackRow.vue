<script setup lang="ts">
import type { Track } from '../types';
import { watch } from 'vue';

const props = defineProps<{
  track: Track,
  colWidths: Record<string, number>,
  coverUrl?: string
}>();

const emit = defineEmits<{
  (e: 'play', track: Track): void
  (e: 'add-to-playlist', path: string): void
}>();

// Sync Filename -> Title
watch(() => props.track.filename, (newVal) => {
  if (!newVal) return;
  
  // Remove extension
  let title = newVal.replace(/\.[^/.]+$/, "");
  
  // Remove "NN - " prefix (e.g. "01 - ", "1 - ")
  title = title.replace(/^\d+\s*-\s*/, "");
  
  if (props.track.title !== title) {
      props.track.title = title;
  }
});

import { GENRES } from '../constants';

// Helper pour détecter les changements
function hasChanged(field: keyof Track): boolean {
    if (!props.track.original_metadata) return false;
    // @ts-ignore - Accès dynamique sûr ici
    return props.track[field] !== props.track.original_metadata[field];
}

// Helper pour afficher la valeur originale si changée
function getOriginalValue(field: keyof Track): string {
    if (!props.track.original_metadata) return '';
    // @ts-ignore
    return String(props.track.original_metadata[field] || '');
}
</script>

<template>
  <tr class="hover:bg-gray-700 group transition-colors">
    <!-- Play Button -->
    <td class="px-4 py-2 whitespace-nowrap text-sm text-gray-400">
      <button 
        @click="emit('play', track)"
        class="w-8 h-8 rounded-full bg-gray-700 hover:bg-blue-900 text-blue-400 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity"
      >
        ▶️
      </button>
    </td>

    <!-- Cover -->
    <td class="px-4 py-2 whitespace-nowrap text-sm text-gray-300">
      <div v-if="track.has_cover" class="w-10 h-10 rounded overflow-hidden bg-gray-900">
          <img v-if="coverUrl" :src="coverUrl" class="w-full h-full object-cover" />
          <div v-else class="w-full h-full flex items-center justify-center text-xs text-gray-500">...</div>
      </div>
      <div v-else class="w-10 h-10 flex items-center justify-center text-gray-600">
          -
      </div>
    </td>

    <!-- Track Number -->
    <td class="px-4 py-2 whitespace-nowrap text-sm text-gray-300">
      <input 
        type="number" 
        v-model.number="track.track_number"
        class="w-12 bg-transparent border-b border-transparent hover:border-gray-500 focus:border-blue-500 focus:outline-none text-center"
        :class="{'text-yellow-400': hasChanged('track_number')}"
      />
    </td>

    <!-- Filename -->
    <td class="px-4 py-2 whitespace-nowrap text-sm text-gray-300">
      <div class="relative">
        <div v-if="hasChanged('filename')" class="text-xs text-red-400 line-through mb-0.5">
          {{ getOriginalValue('filename') }}
        </div>
        <input 
          v-model="track.filename"
          class="w-full bg-transparent border-b border-transparent hover:border-gray-500 focus:border-blue-500 focus:outline-none"
          :class="{'text-green-400 font-medium': hasChanged('filename')}"
        />
      </div>
    </td>

    <!-- Title -->
    <td class="px-4 py-2 whitespace-nowrap text-sm text-gray-300">
      <div class="relative">
        <div v-if="hasChanged('title')" class="text-xs text-red-400 line-through mb-0.5">
          {{ getOriginalValue('title') }}
        </div>
        <input 
          v-model="track.title"
          class="w-full bg-transparent border-b border-transparent hover:border-gray-500 focus:border-blue-500 focus:outline-none"
          :class="{'text-green-400 font-medium': hasChanged('title')}"
        />
      </div>
    </td>

    <!-- Artist -->
    <td class="px-4 py-2 whitespace-nowrap text-sm text-gray-300">
      <div class="relative">
        <div v-if="hasChanged('artist')" class="text-xs text-red-400 line-through mb-0.5">
          {{ getOriginalValue('artist') }}
        </div>
        <input 
          v-model="track.artist"
          class="w-full bg-transparent border-b border-transparent hover:border-gray-500 focus:border-blue-500 focus:outline-none"
          :class="{'text-green-400 font-medium': hasChanged('artist')}"
        />
      </div>
    </td>

    <!-- Album Artist -->
    <td class="px-4 py-2 whitespace-nowrap text-sm text-gray-300">
      <div class="relative">
        <div v-if="hasChanged('album_artist')" class="text-xs text-red-400 line-through mb-0.5">
          {{ getOriginalValue('album_artist') }}
        </div>
        <input 
          v-model="track.album_artist"
          class="w-full bg-transparent border-b border-transparent hover:border-gray-500 focus:border-blue-500 focus:outline-none"
          :class="{'text-green-400 font-medium': hasChanged('album_artist')}"
        />
      </div>
    </td>

    <!-- Album -->
    <td class="px-4 py-2 whitespace-nowrap text-sm text-gray-300">
      <div class="relative">
        <div v-if="hasChanged('album')" class="text-xs text-red-400 line-through mb-0.5">
          {{ getOriginalValue('album') }}
        </div>
        <input 
          v-model="track.album"
          class="w-full bg-transparent border-b border-transparent hover:border-gray-500 focus:border-blue-500 focus:outline-none"
          :class="{'text-green-400 font-medium': hasChanged('album')}"
        />
      </div>
    </td>

    <!-- Genre -->
    <td class="px-4 py-2 whitespace-nowrap text-sm text-gray-300">
      <input 
        v-model="track.genre"
        class="w-full bg-transparent border-b border-transparent hover:border-gray-500 focus:border-blue-500 focus:outline-none"
        :class="{'text-green-400 font-medium': hasChanged('genre')}"
      />
    </td>

    <!-- Year -->
    <td class="px-4 py-2 whitespace-nowrap text-sm text-gray-300 text-right">
      <div class="relative">
        <div v-if="hasChanged('year')" class="text-xs text-red-400 line-through mb-0.5">
          {{ getOriginalValue('year') }}
        </div>
        <input 
          type="number"
          v-model.number="track.year"
          class="w-16 bg-transparent border-b border-transparent hover:border-gray-500 focus:border-blue-500 focus:outline-none text-right"
          :class="{'text-green-400 font-medium': hasChanged('year')}"
        />
      </div>
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
</template>

<style scoped>
/* Hide spinner for number inputs */
input[type=number]::-webkit-inner-spin-button, 
input[type=number]::-webkit-outer-spin-button { 
  -webkit-appearance: none; 
  margin: 0; 
}
input[type=number] {
  -moz-appearance: textfield;
}
</style>

<style scoped>
/* Hide spinners for number input */
input[type=number]::-webkit-inner-spin-button, 
input[type=number]::-webkit-outer-spin-button { 
  -webkit-appearance: none; 
  margin: 0; 
}
input[type=number] {
  -moz-appearance: textfield;
}
</style>
