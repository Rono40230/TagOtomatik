<script setup lang="ts">
import type { Track } from '../types';

defineProps<{
  tracks: Track[]
}>();

const emit = defineEmits<{
  (e: 'play', track: Track): void
  (e: 'add-to-playlist', path: string): void
}>();

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
  <div class="flex-1 bg-gray-800 rounded-lg shadow overflow-hidden border border-gray-700">
    <table class="min-w-full divide-y divide-gray-700">
      <thead class="bg-gray-900">
        <tr>
          <th scope="col" class="px-4 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider w-10"></th>
          <th scope="col" class="px-4 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider w-12">#</th>
          <th scope="col" class="px-4 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">Titre</th>
          <th scope="col" class="px-4 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">Artiste</th>
          <th scope="col" class="px-4 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">Album Artiste</th>
          <th scope="col" class="px-4 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">Genre</th>
          <th scope="col" class="px-4 py-3 text-right text-xs font-medium text-gray-400 uppercase tracking-wider w-20">Durée</th>
          <th scope="col" class="px-4 py-3 text-right text-xs font-medium text-gray-400 uppercase tracking-wider w-10"></th>
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
</template>
