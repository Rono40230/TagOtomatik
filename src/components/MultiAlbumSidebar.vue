<script setup lang="ts">
import type { Album } from '../types';
import { computed } from 'vue';
import { useLibraryStore } from '../stores/library';
import { GENRES } from '../constants';

const props = defineProps<{
  albums: Album[]
}>();

const libraryStore = useLibraryStore();

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

function getCommonGenreValue(): string | undefined {
    if (props.albums.length === 0) return undefined;
    
    let firstVal: string | undefined;
    let initialized = false;

    for (const album of props.albums) {
        for (const track of album.tracks) {
            if (!initialized) {
                firstVal = track.genre;
                initialized = true;
            } else {
                if (track.genre !== firstVal) return undefined;
            }
        }
    }
    
    if (!initialized) return undefined; // No tracks
    return firstVal;
}

const commonGenre = computed({
  get: () => {
    const val = getCommonGenreValue();
    return val === undefined ? '' : val;
  },
  set: (val: string) => {
    props.albums.forEach(a => {
        libraryStore.updateAlbumTracksField(a.id, 'genre', val);
    });
  }
});

function cycleGenre(direction: 1 | -1) {
    const current = commonGenre.value;
    let index = GENRES.indexOf(current);
    
    if (index === -1) {
        index = direction === 1 ? 0 : GENRES.length - 1;
    } else {
        index = (index + direction + GENRES.length) % GENRES.length;
    }
    
    commonGenre.value = GENRES[index];
}

const placeholderTitle = computed(() => getCommonValue('title') === undefined ? '(Valeurs multiples)' : '');
const placeholderArtist = computed(() => getCommonValue('artist') === undefined ? '(Valeurs multiples)' : '');
const placeholderYear = computed(() => getCommonValue('year') === undefined ? '(Valeurs multiples)' : '');
const placeholderGenre = computed(() => getCommonGenreValue() === undefined ? '(Valeurs multiples)' : '');

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
            <div class="relative">
                <input 
                    v-model="commonGenre"
                    :placeholder="placeholderGenre"
                    class="w-full h-10 p-2.5 pr-8 border border-gray-600 rounded-lg text-base font-semibold bg-gray-700 text-white focus:border-blue-500 focus:ring-1 focus:ring-blue-500 transition-colors placeholder-gray-500"
                />
                <div class="absolute right-1 top-1 bottom-1 flex flex-col justify-center">
                  <button 
                    @click="cycleGenre(-1)" 
                    class="text-xs text-gray-400 hover:text-white focus:outline-none leading-none p-0.5 hover:bg-gray-600 rounded"
                    tabindex="-1"
                  >▲</button>
                  <button 
                    @click="cycleGenre(1)" 
                    class="text-xs text-gray-400 hover:text-white focus:outline-none leading-none p-0.5 hover:bg-gray-600 rounded"
                    tabindex="-1"
                  >▼</button>
                </div>
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
