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

async function loadCover() {
  if (!props.album.cover_path) {
    coverUrl.value = null;
    return;
  }

  try {
    // Solution robuste pour Linux/Tauri 2.0 : Lire le fichier en binaire via une commande Rust custom
    // Cela contourne complètement les problèmes de CSP, d'encodage d'URL asset:// et de permissions plugin-fs
    const contents = await invoke<number[]>('read_cover', { path: props.album.cover_path });
    const uint8Array = new Uint8Array(contents);
    const blob = new Blob([uint8Array]);
    const url = URL.createObjectURL(blob);
    
    // Nettoyage de l'ancienne URL si elle existe
    if (coverUrl.value && coverUrl.value.startsWith('blob:')) {
      URL.revokeObjectURL(coverUrl.value);
    }
    
    coverUrl.value = url;
  } catch (e) {
    // Fail silently for cover loading
    coverUrl.value = null;
  }
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
    case 'Incomplete': // Usually implies dirty/problematic too
      return 'bg-red-500 text-white border border-red-400';
    default:
      return 'bg-blue-400 text-white border border-blue-300';
  }
});

const statusTooltip = computed(() => {
  if (props.album.status === 'Clean') return 'Album complet et conforme';
  if (props.album.status === 'Processing') return 'Analyse en cours...';
  
  // Use backend issues if available (Source of Truth)
  if (props.album.issues && props.album.issues.length > 0) {
    return props.album.issues.map(i => `• ${i}`).join('\n');
  }
  
  // Fallback for legacy/transition state
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
  // Uniform look: Blue background, White text.
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
    @click="openDetail"
    class="bg-gray-800 rounded-lg shadow-md overflow-hidden hover:shadow-lg transition-shadow cursor-pointer border border-gray-700 relative group"
  >
    <ConversionModal 
      :album="album" 
      :isOpen="showConversionModal" 
      @close="showConversionModal = false"
      @refresh="emit('refresh', album.id)"
    />

    <PlaylistModal
      :album="album"
      :isOpen="showPlaylistModal"
      @close="showPlaylistModal = false"
      @refresh="emit('refresh', album.id)"
    />

    <!-- Status Badge (Moved to bottom) & Tooltip container if needed logic handled below -->
    
    <div class="aspect-square bg-gray-700 flex items-center justify-center relative">
      <img v-if="coverUrl" :src="coverUrl" alt="Cover" class="w-full h-full object-cover" />
      <div v-else class="flex flex-col items-center justify-center text-gray-400 h-full w-full">
        <span class="text-sm font-medium">Cover absente</span>
      </div>
    </div>
    
    <div class="p-4">
      <h3 class="font-bold text-lg truncate text-white" :title="album.title">{{ album.title || 'Sans titre' }}</h3>
      <p class="text-gray-300 truncate" :title="album.artist">{{ album.artist || 'Artiste inconnu' }}</p>
      
      <div class="mt-3 flex flex-col gap-2">
        <!-- Info & Selection (Checkbox moved here) -->
        <div class="flex justify-between items-center text-xs text-gray-400">
          <div class="flex gap-2">
             <span>{{ displayYear }}</span>
             <span>• {{ album.tracks.length }} pistes</span>
          </div>
          <!-- Selection Checkbox -->
          <div @click.stop>
            <input 
                type="checkbox" 
                :checked="selected"
                @change="emit('toggle-selection', album.id, ($event.target as HTMLInputElement).checked)"
                class="w-4 h-4 rounded border-gray-600 text-blue-600 focus:ring-blue-500 bg-gray-700 cursor-pointer"
            />
          </div>
        </div>

        <!-- Action Badges (Delete moved here) -->
        <div class="flex flex-wrap gap-2 items-center">
            
          <!-- STATUS BADGE -->
          <div class="relative group/status">
            <div 
                :class="['h-5 px-2 flex items-center justify-center text-[10px] font-bold rounded cursor-help border min-w-[40px]', statusColor]"
            >
                {{ album.status }}
            </div>
            <!-- Custom Tooltip -->
            <div 
                v-if="album.status !== 'Clean'"
                class="absolute bottom-full left-0 mb-2 w-56 p-3 bg-gray-900 text-white text-xs rounded-lg shadow-xl border border-gray-700 opacity-0 group-hover/status:opacity-100 transition-opacity pointer-events-none z-30 whitespace-pre-line leading-relaxed"
            >
                <div class="font-bold mb-1 border-b border-gray-700 pb-1 text-yellow-500">Problèmes détectés :</div>
                {{ statusTooltip }}
            </div>
          </div>

          <!-- Format Conversion -->
          <button 
            @click.stop="openConversionModal"
            :class="['h-5 px-2 flex items-center justify-center text-[10px] rounded border gap-1 group/format transition-colors min-w-[40px]', formatColor]"
            title="Convertir le format"
          >
            {{ formatBadge }}
            <span class="hidden group-hover/format:inline text-[10px] ml-1">➜ MP3</span>
          </button>

          <!-- Create Playlist -->
          <button 
            @click.stop="openPlaylistModal"
            class="h-5 px-2 flex items-center justify-center text-[10px] rounded border border-blue-300 bg-blue-400 text-white hover:bg-blue-300 transition-colors font-medium"
            :class="{'!bg-green-600 !border-green-500': album.has_playlist}"
            title="Générer une playlist"
          >
            {{ album.has_playlist ? 'Playlist OK' : 'Créer playlist' }}
          </button>

          <!-- Refresh -->
          <button 
             @click.stop="emit('refresh', album.id)"
             class="h-5 px-2 flex items-center justify-center text-[10px] rounded border border-blue-300 bg-blue-400 text-white hover:bg-blue-300 transition-colors font-medium"
             title="Scanner à nouveau le dossier"
          >
            Actualiser la carte
          </button>

        </div>
      </div>
    </div>
  </div>
</template>
