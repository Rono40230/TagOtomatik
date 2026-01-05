<script setup lang="ts">
import type { Album } from '../types';
import { computed } from 'vue';
// import { useLibraryStore } from '../stores/library';

const props = defineProps<{
  albums: Album[]
}>();

// const libraryStore = useLibraryStore();

// Helper to get common value
function getCommonValue<K extends keyof Album>(field: K): Album[K] | undefined {
  if (props.albums.length === 0) return undefined;
  const first = props.albums[0][field];
  const allSame = props.albums.every(a => a[field] === first);
  return allSame ? first : undefined;
}

// Computed properties for v-model
const commonTitle = computed({
  get: () => {
    const val = getCommonValue('title');
    return val === undefined ? '' : String(val);
  },
  set: (val: string) => {
    if (!val) return; // Don't apply empty string if it was mixed
    props.albums.forEach(a => a.title = val);
  }
});

const commonArtist = computed({
  get: () => {
    const val = getCommonValue('artist');
    return val === undefined ? '' : String(val);
  },
  set: (val: string) => {
    props.albums.forEach(a => a.artist = val);
  }
});

const commonYear = computed({
  get: () => {
    const val = getCommonValue('year');
    return val === undefined ? '' : String(val);
  },
  set: (val: string) => {
    const num = parseInt(val);
    if (!isNaN(num)) {
      props.albums.forEach(a => a.year = num);
    }
  }
});

/*
const commonGenre = computed({
  get: () => {
    const val = getCommonValue('genre');
    return val === undefined ? '' : String(val);
  },
  set: (val: string) => {
    // Update genre for all albums AND their tracks
    props.albums.forEach(a => {
        // Update album genre (local state)
        // Note: libraryStore.updateAlbumTracksField usually updates the store/backend
        // We should probably call it for each album
        libraryStore.updateAlbumTracksField(a.id, 'genre', val);
    });
  }
});
*/

/*
const GENRES = [
    "Acid Jazz", "B.O. de Films", "Blues", "Chansons Française", "Disco",
    "Electronique", "Flamenco", "Folk", "Funk", "Jazz", "Musique Afriquaine",
    "Musique Andine", "Musique Brésilienne", "Musique Classique", "Musique Cubaine",
    "Musique Franco-Hispanique", "New-Wave", "Pop", "Rap", "Reggae", "Rock",
    "Soul", "Top 50", "Trip-Hop", "Zouk"
];
*/

const placeholderTitle = computed(() => getCommonValue('title') === undefined ? '(Valeurs multiples)' : '');
const placeholderArtist = computed(() => getCommonValue('artist') === undefined ? '(Valeurs multiples)' : '');
const placeholderYear = computed(() => getCommonValue('year') === undefined ? '(Valeurs multiples)' : '');
// const placeholderGenre = computed(() => getCommonValue('genre') === undefined ? '(Valeurs multiples)' : '');

</script>

<template>
  <aside class="w-96 flex-shrink-0 h-full overflow-y-auto custom-scrollbar">
    <div class="bg-gray-800 rounded-lg shadow p-6 border border-gray-700 min-h-min">
      <!-- Multi-Album Icon/Visual -->
      <div class="aspect-square bg-gray-700 rounded-md mb-6 overflow-hidden flex items-center justify-center relative shadow-lg">
          <div class="text-center">
            <div class="text-6xl mb-2">📚</div>
            <div class="text-gray-400 font-medium">{{ albums.length }} albums</div>
          </div>
      </div>
      
      <div class="space-y-4">
        <div>
          <label class="block text-xs font-medium text-gray-400 uppercase mb-1">Album (Titre)</label>
          <input 
            v-model="commonTitle"
            :placeholder="placeholderTitle"
            class="w-full h-10 p-2.5 border border-gray-600 rounded-lg text-base font-semibold bg-gray-700 text-white focus:border-blue-500 focus:ring-1 focus:ring-blue-500 transition-colors placeholder-gray-500" 
          />
          <p class="text-xs text-gray-500 mt-1" v-if="placeholderTitle">Attention: modifier ceci changera le titre de TOUS les albums sélectionnés.</p>
        </div>
        <div>
          <label class="block text-xs font-medium text-gray-400 uppercase mb-1">Artiste Album</label>
          <input 
            v-model="commonArtist"
            :placeholder="placeholderArtist"
            class="w-full h-10 p-2.5 border border-gray-600 rounded-lg text-base font-semibold bg-gray-700 text-white focus:border-blue-500 focus:ring-1 focus:ring-blue-500 transition-colors placeholder-gray-500" 
          />
        </div>
        
        <div class="grid grid-cols-2 gap-4">
          <div>
            <label class="block text-xs font-medium text-gray-400 uppercase mb-1">Année</label>
            <input 
              v-model="commonYear"
              type="number"
              :placeholder="placeholderYear"
              class="w-full h-10 p-2.5 border border-gray-600 rounded-lg text-base font-semibold bg-gray-700 text-white focus:border-blue-500 focus:ring-1 focus:ring-blue-500 transition-colors placeholder-gray-500" 
            />
          </div>
          <div>
            <label class="block text-xs font-medium text-gray-400 uppercase mb-1">Genre</label>
            <!-- Genre editing disabled because Album doesn't have genre field directly -->
            <div class="relative opacity-50 pointer-events-none">
                <input 
                    disabled
                    placeholder="Non disponible"
                    class="w-full h-10 p-2.5 border border-gray-600 rounded-lg text-base font-semibold bg-gray-700 text-white focus:border-blue-500 focus:ring-1 focus:ring-blue-500 transition-colors placeholder-gray-500"
                />
            </div>
          </div>
        </div>
      </div>
      
      <div class="mt-6 p-4 bg-blue-900/20 border border-blue-800 rounded-lg">
        <p class="text-sm text-blue-300 text-center">
          Les modifications ici s'appliquent à la sélection entière.
        </p>
      </div>
    </div>
  </aside>
</template>
