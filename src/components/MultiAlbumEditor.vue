<script setup lang="ts">
import { computed } from 'vue';
import type { Album, Track } from '../types';
import MultiAlbumSidebar from './MultiAlbumSidebar.vue';
import TrackList from './TrackList.vue';

const props = defineProps<{
  albums: Album[]
}>();

const emit = defineEmits<{
  (e: 'play', track: Track): void
  (e: 'add-to-playlist', trackPath: string): void
}>();

// Flatten tracks from all albums into a single list
// Since flatMap creates a shallow copy, the Track objects are references
// so v-model in TrackList will update the original objects in the albums.
const allTracks = computed(() => {
  return props.albums.flatMap(a => a.tracks);
});

</script>

<template>
  <div class="flex gap-8 h-full overflow-hidden">
    <MultiAlbumSidebar 
      :albums="albums"
    />

    <div class="flex-1 min-w-0 flex flex-col h-full">
      <div class="mb-4 flex items-baseline gap-2 flex-shrink-0">
        <h2 class="text-2xl font-bold text-white truncate">Édition Multiple</h2>
        <span class="text-gray-400 text-sm">{{ albums.length }} albums sélectionnés</span>
        <span class="text-gray-500 text-xs ml-2">{{ allTracks.length }} pistes au total</span>
      </div>

      <TrackList 
        :tracks="allTracks"
        @play="emit('play', $event)"
        @add-to-playlist="emit('add-to-playlist', $event)"
      />
    </div>
  </div>
</template>
