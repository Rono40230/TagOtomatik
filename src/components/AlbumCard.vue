<script setup lang="ts">
import type { Album } from '../types';
import { computed, ref, onMounted, watch, onUnmounted } from 'vue';
import { useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import ConversionModal from './ConversionModal.vue';

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
    case 'Clean': return 'bg-green-900 text-green-200 border border-green-700';
    case 'Dirty': return 'bg-yellow-900 text-yellow-200 border border-yellow-700';
    case 'Incomplete': return 'bg-red-900 text-red-200 border border-red-700';
    case 'Processing': return 'bg-blue-900 text-blue-200 border border-blue-700';
    default: return 'bg-gray-700 text-gray-300 border border-gray-600';
  }
});

const statusTooltip = computed(() => {
  if (props.album.status === 'Clean') return 'Album complet et conforme';
  if (props.album.status === 'Processing') return 'Analyse en cours...';
  
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

function openDetail() {
  router.push(`/album/${props.album.id}`);
}

function openConversionModal() {
  showConversionModal.value = true;
}
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

    <!-- Selection Checkbox -->
    <div class="absolute top-2 left-2 z-20" @click.stop>
      <input 
        type="checkbox" 
        :checked="selected"
        @change="emit('toggle-selection', album.id, ($event.target as HTMLInputElement).checked)"
        class="w-5 h-5 rounded border-gray-600 text-blue-600 focus:ring-blue-500 bg-gray-700"
      />
    </div>

    <!-- Status Badge (Top Right) -->
    <div class="absolute top-2 right-2 z-10 group/status">
      <span 
        :class="['text-xs font-bold px-2 py-1 rounded-full shadow-md cursor-help inline-block', statusColor]"
      >
        {{ album.status }}
      </span>
      <!-- Custom Tooltip -->
      <div 
        v-if="album.status !== 'Clean'"
        class="absolute top-full right-0 mt-2 w-56 p-3 bg-gray-900 text-white text-xs rounded-lg shadow-xl border border-gray-700 opacity-0 group-hover/status:opacity-100 transition-opacity pointer-events-none z-30 whitespace-pre-line leading-relaxed"
      >
        <div class="font-bold mb-1 border-b border-gray-700 pb-1 text-yellow-500">Problèmes détectés :</div>
        {{ statusTooltip }}
      </div>
    </div>

    <div class="aspect-square bg-gray-700 flex items-center justify-center relative">
      <img v-if="coverUrl" :src="coverUrl" alt="Cover" class="w-full h-full object-cover" />
      <div v-else class="flex flex-col items-center justify-center text-gray-400 h-full w-full">
        <span class="text-sm font-medium">Cover absente</span>
      </div>
    </div>
    
    <div class="p-4">
      <h3 class="font-bold text-lg truncate text-white" :title="album.title">{{ album.title || 'Sans titre' }}</h3>
      <p class="text-gray-300 truncate" :title="album.artist">{{ album.artist || 'Artiste inconnu' }}</p>
      
      <div class="mt-3 flex justify-between items-center text-xs text-gray-400">
        <span>{{ album.year || '????' }}</span>
        <button 
          @click.stop="openConversionModal"
          class="bg-gray-700 hover:bg-blue-600 hover:text-white px-2 py-0.5 rounded transition-colors cursor-pointer flex items-center gap-1 group/format"
          title="Convertir le format"
        >
          {{ formatBadge }}
          <span class="hidden group-hover/format:inline text-[10px]">➜ MP3</span>
        </button>
        <div class="flex items-center gap-2">
          <span>{{ album.tracks.length }} pistes</span>
          <button 
            @click.stop="emit('delete', album.id)"
            class="text-gray-500 hover:text-red-500 transition-colors p-1"
            title="Supprimer de la bibliothèque"
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
            </svg>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
