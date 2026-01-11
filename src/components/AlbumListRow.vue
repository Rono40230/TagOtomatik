<script setup lang="ts">
import type { Album } from '../types';
import { computed, ref, onMounted, watch, onUnmounted } from 'vue';
import { useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import ConversionModal from './ConversionModal.vue';
import PlaylistModal from './PlaylistModal.vue';

const props = defineProps<{
  album: Album,
  selected?: boolean
}>();

const emit = defineEmits<{
  (e: 'delete', id: string): void,
  (e: 'toggle-selection', id: string, selected: boolean): void,
  (e: 'refresh', id: string): void
}>();

const router = useRouter();
const coverUrl = ref<string | null>(null);
const showConversionModal = ref(false);
const showPlaylistModal = ref(false);
const isRefreshing = ref(false);

async function loadCover() {
  if (!props.album.cover_path) {
    coverUrl.value = null;
    return;
  }

  try {
    const contents = await invoke<number[]>('read_cover', { path: props.album.cover_path });
    const uint8Array = new Uint8Array(contents);
    const blob = new Blob([uint8Array]);
    const url = URL.createObjectURL(blob);
    
    if (coverUrl.value && coverUrl.value.startsWith('blob:')) {
      URL.revokeObjectURL(coverUrl.value);
    }
    
    coverUrl.value = url;
  } catch (e) {
    coverUrl.value = null;
  }
}

function handleRefresh() {
    isRefreshing.value = true;
    emit('refresh', props.album.id);
    setTimeout(() => {
        isRefreshing.value = false;
    }, 1500);
}

onMounted(loadCover);
watch(() => props.album.cover_path, loadCover);
onUnmounted(() => {
  if (coverUrl.value && coverUrl.value.startsWith('blob:')) {
    URL.revokeObjectURL(coverUrl.value);
  }
});

const statusColor = computed(() => {
  switch (props.album.status) {
    case 'Clean':
      return 'bg-green-500 text-white border border-green-400';
    case 'Dirty': 
    case 'Incomplete': 
      return 'bg-red-500 text-white border border-red-400';
    default:
      return 'bg-blue-400 text-white border border-blue-300';
  }
});

const statusTooltip = computed(() => {
  if (props.album.status === 'Clean') return 'Album complet et conforme';
  if (props.album.status === 'Processing') return 'Analyse en cours...';
  
  if (props.album.issues && props.album.issues.length > 0) {
    return props.album.issues.map(i => `• ${i}`).join('\n');
  }
  
  const issues: string[] = [];
  
  if (!props.album.cover_path) issues.push('• Cover manquante');
  if (!props.album.has_playlist) issues.push('• Playlist manquante');
  if (!props.album.year || props.album.year === 0) issues.push('• Année manquante');
  
  let missingTitles = 0;
  let missingArtists = 0;
  let missingAlbums = 0;
  let missingGenres = 0;

  props.album.tracks.forEach(t => {
    if (!t.title || t.title.trim() === '') missingTitles++;
    if (!t.artist || t.artist.trim() === '') missingArtists++;
    if (!t.album || t.album.trim() === '') missingAlbums++;
    if (!t.genre || t.genre.trim() === '') missingGenres++;
  });

  if (missingTitles > 0) issues.push(`• Titre manquant (${missingTitles} pistes)`);
  if (missingArtists > 0) issues.push(`• Artiste manquant (${missingArtists} pistes)`);
  if (missingAlbums > 0) issues.push(`• Album manquant (${missingAlbums} pistes)`);
  if (missingGenres > 0) issues.push(`• Genre manquant (${missingGenres} pistes)`);
  
  if (props.album.tracks.length === 0) issues.push('• Aucune piste audio');

  return issues.length > 0 ? issues.join('\n') : 'Statut inconnu';
});

const formatBadge = computed(() => {
  const formats = new Set(props.album.tracks.map(t => t.format.toUpperCase()));
  return Array.from(formats).join('/');
});

const formatColor = computed(() => {
  return 'bg-blue-400 text-white border-blue-300 hover:bg-blue-300';
});

function openDetail() {
  router.push(`/album/${props.album.id}`);
}

function openConversionModal() {
  showConversionModal.value = true;
}

function openPlaylistModal() {
  showPlaylistModal.value = true;
}

const displayYear = computed(() => {
  const { yearMin, yearMax, year } = props.album;
  if (yearMin && yearMax && yearMin !== yearMax) {
    const maxShort = yearMax % 100;
    return `${yearMin}-${maxShort.toString().padStart(2, '0')}`;
  }
  return year?.toString() || '????';
});
</script>

<template>
  <div 
    class="group relative flex items-center gap-4 p-3 bg-gray-800 rounded-lg shadow-sm border border-gray-700 hover:shadow-cyan-400/20 hover:border-cyan-500/50 transition-all cursor-pointer"
    :class="{'ring-2 ring-blue-500 bg-gray-700': selected}"
    @click="openDetail"
  >
    <!-- Cover (Small) -->
    <div class="h-16 w-16 flex-shrink-0 relative rounded overflow-hidden bg-gray-700">
      <img v-if="coverUrl" :src="coverUrl" alt="Cover" class="w-full h-full object-cover" />
      <div v-else class="flex items-center justify-center h-full w-full text-gray-500 text-xl">🎵</div>
      <!-- Checkbox overlay -->
      <div @click.stop class="absolute top-1 left-1 bg-gray-900/50 rounded pointer-events-auto">
        <input 
            type="checkbox" 
            :checked="selected"
            @change="emit('toggle-selection', album.id, ($event.target as HTMLInputElement).checked)"
            class="w-4 h-4 rounded border-gray-400 text-blue-600 focus:ring-blue-500 cursor-pointer"
        />
      </div>
    </div>
    
    <!-- Info -->
    <div class="flex-1 min-w-0 flex flex-col justify-center">
      <div class="flex items-baseline gap-2">
          <h3 class="font-bold text-white truncate text-base" :title="album.title">{{ album.title || 'Sans titre' }}</h3>
          <span class="text-xs text-gray-400 font-mono">{{ displayYear }}</span>
      </div>
      <p class="text-sm text-gray-300 truncate" :title="album.artist">{{ album.artist || 'Artiste inconnu' }}</p>
      <span class="text-xs text-gray-500">{{ album.tracks.length }} pistes</span>
    </div>

    <!-- Actions / Badges -->
    <div class="flex items-center gap-2 flex-wrap justify-end">
          
        <!-- STATUS BADGE -->
        <div class="relative group/status flex-shrink-0">
            <div 
                :class="['h-6 px-2 flex items-center justify-center text-xs font-bold rounded cursor-help border min-w-[50px]', statusColor]"
            >
                {{ album.status }}
            </div>
            <!-- Custom Tooltip -->
            <div 
                v-if="album.status !== 'Clean'"
                class="absolute bottom-full right-0 mb-2 w-56 p-3 bg-gray-900 text-white text-xs rounded-lg shadow-xl border border-gray-700 opacity-0 group-hover/status:opacity-100 transition-opacity pointer-events-none z-30 whitespace-pre-line leading-relaxed"
            >
                <div class="font-bold mb-1 border-b border-gray-700 pb-1 text-yellow-500">Problèmes détectés :</div>
                {{ statusTooltip }}
            </div>
        </div>

        <!-- Format Badge -->
        <button 
            @click.stop="openConversionModal"
            :class="['h-6 px-2 flex items-center justify-center text-xs rounded border gap-1 group/format transition-colors flex-shrink-0', formatColor]"
            title="Convertir le format"
        >
            {{ formatBadge }}
            <span class="hidden group-hover/format:inline text-xs ml-1">➜ MP3</span>
        </button>

        <!-- Playlist -->
        <button 
            @click.stop="openPlaylistModal"
            class="h-6 w-6 flex items-center justify-center text-xs rounded border border-blue-300 bg-blue-400 text-white hover:bg-blue-300 transition-colors flex-shrink-0"
            :class="{'!bg-green-600 !border-green-500': album.has_playlist}"
            title="Générer une playlist"
        >
            <span v-if="album.has_playlist">✓</span>
            <span v-else>+</span>
        </button>

        <!-- Refresh -->
        <button 
             @click.stop="handleRefresh"
             class="h-6 w-6 flex items-center justify-center text-xs rounded border border-blue-300 bg-blue-400 text-white hover:bg-blue-300 transition-colors flex-shrink-0"
             title="Scanner à nouveau le dossier"
             :disabled="isRefreshing"
        >
            <svg v-if="isRefreshing" class="animate-spin h-3 w-3 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
              <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
            </svg>
        </button>
    </div>

    <!-- Modals -->
    <ConversionModal
      :isOpen="showConversionModal"
      :album="album"
      @close="showConversionModal = false"
      @refresh="emit('refresh', album.id)"
    />

    <PlaylistModal
      :album="album"
      :isOpen="showPlaylistModal"
      @close="showPlaylistModal = false"
      @refresh="emit('refresh', album.id)"
    />
  </div>
</template>
