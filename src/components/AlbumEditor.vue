<script setup lang="ts">
import { computed } from 'vue';
import type { Album, Track } from '../types';
import AlbumSidebar from './AlbumSidebar.vue';
import TrackList from './TrackList.vue';
import { useLibraryStore } from '../stores/library';

const props = defineProps<{
  album: Album
}>();

const emit = defineEmits<{
  (e: 'play', track: Track): void
  (e: 'add-to-playlist', trackPath: string): void
  (e: 'change-cover'): void
  (e: 'search-cover'): void
  (e: 'suggest-exception', data: { original: string; corrected: string; category: string }): void
}>();

const libraryStore = useLibraryStore();

// Utilitaire de formatage (identique à AlbumCard)
const formattedTitle = computed(() => {
  const { yearMin, yearMax, year, title } = props.album;
  let yearPrefix = '????';
  
  if (yearMin && yearMax && yearMin !== yearMax) {
    const maxShort = yearMax % 100;
    yearPrefix = `${yearMin}-${maxShort.toString().padStart(2, '0')}`;
  } else if (year) {
    yearPrefix = year.toString();
  } else if (yearMin) {
    yearPrefix = yearMin.toString();
  }
  
  // Nettoyer le titre s'il contient déjà une parenthèse d'année au début (pour éviter (YYYY) (YYYY) Titre)
  const cleanTitle = title.replace(/^\(\d{4}(?:-\d{2})?\)\s*/, '');
  
  return `(${yearPrefix}) ${cleanTitle}`;
});

// Proxy pour les v-model de l'album (car props.album est réactif via le parent)
// On modifie directement l'objet album qui est passé par référence depuis le store
</script>

<template>
  <div class="flex gap-8 h-full overflow-hidden">
    <AlbumSidebar 
      :album="album"
      @update:title="(val) => { album.title = val; libraryStore.updateAlbumTracksField(album.id, 'album', val); }"
      @update:artist="(val) => { album.artist = val; libraryStore.updateAlbumTracksField(album.id, 'artist', val); libraryStore.updateAlbumTracksField(album.id, 'album_artist', val); }"
      @update:year="(val) => { album.year = val; libraryStore.updateAlbumTracksField(album.id, 'year', val); }"
      @update:genre="libraryStore.updateAlbumTracksField(album.id, 'genre', $event)"
      @change-cover="emit('change-cover')"
      @search-cover="emit('search-cover')"
    />

    <div class="flex-1 min-w-0 flex flex-col h-full">
      <!-- Header local pour le titre de l'album si on est en mode liste (optionnel, mais utile pour distinguer) -->
      <div class="mb-4 flex items-baseline gap-2 flex-shrink-0">
        <h2 class="text-2xl font-bold text-white truncate" :title="formattedTitle">{{ formattedTitle }}</h2>
        <span class="text-gray-400 text-sm">par {{ album.artist }}</span>
      </div>

      <TrackList 
        :tracks="album.tracks"
        @play="emit('play', $event)"
        @add-to-playlist="emit('add-to-playlist', $event)"
        @suggest-exception="emit('suggest-exception', $event)"
      />
    </div>
  </div>
</template>
