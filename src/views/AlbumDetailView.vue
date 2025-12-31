<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import { useLibraryStore } from '../stores/library';
import { usePlayerStore } from '../stores/player';
import { usePlaylistStore } from '../stores/playlist';
import { useToastStore } from '../stores/toast';
import CoverSearchModal from '../components/CoverSearchModal.vue';
import AlbumSidebar from '../components/AlbumSidebar.vue';
import TrackList from '../components/TrackList.vue';
import type { Track } from '../types';

const route = useRoute();
const router = useRouter();
const libraryStore = useLibraryStore();
const playerStore = usePlayerStore();
const playlistStore = usePlaylistStore();
const toastStore = useToastStore();

const albumId = route.params.id as string;
const album = computed(() => libraryStore.getAlbumById(albumId));

const showPlaylistModal = ref(false);
const showCoverModal = ref(false);
const selectedTrackForPlaylist = ref<string | null>(null);

// Si l'album n'existe pas (refresh page), retour à la bibliothèque
onMounted(() => {
  if (!album.value) {
    router.push('/library');
  }
  if (libraryStore.currentPath) {
    playlistStore.loadPlaylists(libraryStore.currentPath);
  }
});

function openPlaylistModal(trackPath: string) {
  selectedTrackForPlaylist.value = trackPath;
  showPlaylistModal.value = true;
}

async function addTrackToPlaylist(playlistPath: string) {
  if (selectedTrackForPlaylist.value) {
    await playlistStore.addToPlaylist(playlistPath, [selectedTrackForPlaylist.value]);
    showPlaylistModal.value = false;
    selectedTrackForPlaylist.value = null;
  }
}

async function handleCoverSelect(url: string) {
  if (!album.value) return;
  
  try {
    libraryStore.isLoading = true;
    const localPath = await invoke<string>('download_cover', { 
      url, 
      albumPath: album.value.path 
    });
    
    // Update local state
    album.value.cover_path = localPath;
    // Force image refresh hack
    album.value.cover_path = `${localPath}?t=${Date.now()}`;
    
  } catch (e) {
    toastStore.error(String(e));
  } finally {
    libraryStore.isLoading = false;
  }
}

function goBack() {
  router.push('/library');
}

async function handleAutoCorrect() {
  if (album.value) {
    await libraryStore.autoCorrectAlbum(album.value.id);
  }
}

async function handleApplyCorrection() {
  if (album.value) {
    await libraryStore.applyAutoCorrect(album.value.id);
  }
}

function handleCancelCorrection() {
  if (album.value) {
    libraryStore.cancelAutoCorrect(album.value.id);
  }
}

async function handleSave() {
  if (album.value) {
    await libraryStore.saveAlbum(album.value.id);
  }
}

function playTrack(track: Track) {
  playerStore.play(track);
}
</script>

<template>
  <div v-if="album" class="min-h-screen bg-gray-900 text-white flex flex-col">
    <!-- Loading Overlay -->
    <div v-if="libraryStore.isLoading" class="fixed inset-0 bg-black/50 z-50 flex items-center justify-center backdrop-blur-sm">
      <div class="flex flex-col items-center gap-4">
        <div class="animate-spin rounded-full h-12 w-12 border-4 border-blue-500 border-t-transparent"></div>
        <p class="text-blue-400 font-medium">Traitement en cours...</p>
      </div>
    </div>

    <!-- Header / Toolbar -->
    <header class="bg-gray-800 shadow-sm sticky top-0 z-20 border-b border-gray-700">
      <div class="w-full px-6 h-16 flex items-center justify-between">
        <div class="flex items-center gap-4">
          <button @click="goBack" class="p-2 hover:bg-gray-700 rounded-full transition-colors text-gray-300">
            ⬅️
          </button>
          <h1 class="text-xl font-bold text-white truncate max-w-md">
            {{ album.title }}
            <span class="text-gray-400 font-normal text-sm ml-2">par {{ album.artist }}</span>
          </h1>
        </div>
        
        <div class="flex gap-2">
          <template v-if="libraryStore.hasPendingCorrection(album.id)">
            <div class="flex items-center gap-2 bg-yellow-900/30 px-3 py-1 rounded-lg border border-yellow-700/50 mr-2">
              <span class="text-yellow-400 text-xs font-medium">Prévisualisation</span>
              <button 
                @click="handleApplyCorrection"
                class="px-3 py-1.5 bg-green-600 text-white text-xs rounded hover:bg-green-500 transition-colors"
              >
                Appliquer
              </button>
              <button 
                @click="handleCancelCorrection"
                class="px-3 py-1.5 bg-red-600 text-white text-xs rounded hover:bg-red-500 transition-colors"
              >
                Annuler
              </button>
            </div>
          </template>
          <template v-else>
            <button 
              @click="handleAutoCorrect"
              :disabled="libraryStore.isLoading"
              class="px-4 py-2 bg-blue-900/50 text-blue-300 border border-blue-700 rounded-lg hover:bg-blue-900 font-medium text-sm transition-colors flex items-center gap-2"
            >
              ✨ Auto-Correction
            </button>
            <button 
              @click="handleSave"
              :disabled="libraryStore.isLoading"
              class="px-4 py-2 bg-green-700 text-white rounded-lg hover:bg-green-600 font-medium text-sm transition-colors shadow-sm disabled:opacity-50"
            >
              {{ libraryStore.isLoading ? '...' : '💾 Sauvegarder' }}
            </button>
          </template>
        </div>
      </div>
    </header>

    <!-- Content -->
    <main class="flex-1 w-full mx-auto px-4 py-8 flex gap-8">
      
      <AlbumSidebar 
        :album="album"
        @update:title="album.title = $event"
        @update:artist="album.artist = $event"
        @update:year="album.year = $event"
        @update:genre="libraryStore.updateAlbumTracksField(albumId, 'genre', $event)"
        @change-cover="showCoverModal = true"
      />

      <TrackList 
        :tracks="album.tracks"
        @play="playTrack"
        @add-to-playlist="openPlaylistModal"
      />
    </main>

    <!-- Playlist Selection Modal -->
    <div v-if="showPlaylistModal" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div class="bg-white rounded-xl p-6 w-96 shadow-2xl max-h-[80vh] flex flex-col">
        <h3 class="text-lg font-bold mb-4">Ajouter à la playlist</h3>
        
        <div class="flex-1 overflow-y-auto mb-4 border rounded-lg">
          <div v-if="playlistStore.playlists.length === 0" class="p-4 text-center text-gray-500">
            Aucune playlist trouvée.
          </div>
          <ul v-else class="divide-y divide-gray-100">
            <li 
              v-for="playlist in playlistStore.playlists" 
              :key="playlist.path"
              @click="addTrackToPlaylist(playlist.path)"
              class="p-3 hover:bg-blue-50 cursor-pointer flex justify-between items-center"
            >
              <span class="font-medium">{{ playlist.name }}</span>
              <span class="text-xs text-gray-500">{{ playlist.track_count }} pistes</span>
            </li>
          </ul>
        </div>

        <div class="flex justify-end">
          <button 
            @click="showPlaylistModal = false"
            class="px-4 py-2 text-gray-600 hover:bg-gray-100 rounded-lg"
          >
            Annuler
          </button>
        </div>
      </div>
    </div>

    <CoverSearchModal 
      v-if="showCoverModal"
      :is-open="showCoverModal"
      :artist="album.artist"
      :album="album.title"
      @close="showCoverModal = false"
      @select="handleCoverSelect"
    />
  </div>
</template>
