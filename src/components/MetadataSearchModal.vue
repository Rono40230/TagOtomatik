<script setup lang="ts">
import { ref, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { MusicBrainzRelease } from '../types';

const props = defineProps<{
  isOpen: boolean;
  artist: string;
  album: string;
}>();

const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'select', release: MusicBrainzRelease): void;
}>();

const loading = ref(false);
const results = ref<MusicBrainzRelease[]>([]);
const error = ref<string | null>(null);

async function search() {
  if (!props.artist || !props.album) return;
  
  loading.value = true;
  error.value = null;
  results.value = [];

  try {
    const data = await invoke('search_musicbrainz', { 
      artist: props.artist, 
      album: props.album 
    });
    results.value = data as MusicBrainzRelease[];
  } catch (e) {
    error.value = String(e);
  } finally {
    loading.value = false;
  }
}

watch(() => props.isOpen, (newVal) => {
  if (newVal) {
    search();
  }
});

function selectRelease(release: MusicBrainzRelease) {
  emit('select', release);
  emit('close');
}
</script>

<template>
  <div v-if="isOpen" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
    <div class="bg-gray-800 rounded-lg shadow-xl w-full max-w-2xl max-h-[80vh] flex flex-col border border-gray-700">
      <div class="p-4 border-b border-gray-700 flex justify-between items-center">
        <h3 class="text-lg font-bold text-white">Recherche MusicBrainz</h3>
        <button @click="$emit('close')" class="text-gray-400 hover:text-white">✕</button>
      </div>

      <div class="p-4 flex-1 overflow-y-auto">
        <div v-if="loading" class="text-center py-8 text-gray-400">
          Recherche en cours...
        </div>

        <div v-else-if="error" class="text-red-400 p-4 bg-red-900/20 rounded">
          {{ error }}
        </div>

        <div v-else-if="results.length === 0" class="text-center py-8 text-gray-400">
          Aucun résultat trouvé pour "{{ album }}" de "{{ artist }}"
        </div>

        <div v-else class="space-y-2">
          <div 
            v-for="release in results" 
            :key="release.id"
            class="p-3 bg-gray-700/50 hover:bg-gray-700 rounded cursor-pointer transition-colors border border-transparent hover:border-blue-500"
            @click="selectRelease(release)"
          >
            <div class="font-bold text-white">{{ release.title }}</div>
            <div class="text-sm text-gray-300">
              {{ release['artist-credit']?.[0]?.name }} • {{ release.date || 'Date inconnue' }} • {{ release.country || 'Pays inconnu' }}
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
