<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import { useLibraryStore } from '../stores/library';
import { usePlayerStore } from '../stores/player';
import { usePlaylistStore } from '../stores/playlist';
import { useToastStore } from '../stores/toast';
import CoverSearchModal from '../components/CoverSearchModal.vue';
import AlbumEditor from '../components/AlbumEditor.vue';
import MultiAlbumEditor from '../components/MultiAlbumEditor.vue';
import AlbumDetailToolbar from '../components/AlbumDetailToolbar.vue';
import type { Track, Album } from '../types';

const route = useRoute();
const router = useRouter();
const libraryStore = useLibraryStore();
const playerStore = usePlayerStore();
const playlistStore = usePlaylistStore();
const toastStore = useToastStore();

const albums = computed(() => {
  if (route.name === 'MultiAlbumEdit') {
    const ids = (route.query.ids as string)?.split(',') || [];
    return ids.map(id => libraryStore.getAlbumById(id)).filter((a): a is Album => !!a);
  } else {
    const a = libraryStore.getAlbumById(route.params.id as string);
    return a ? [a] : [];
  }
});

const showPlaylistModal = ref(false);
const showCoverModal = ref(false);
const selectedTrackForPlaylist = ref<string | null>(null);
const activeAlbumForCover = ref<Album | null>(null);
const componentKey = ref(0);

// Si aucun album trouvé, retour
onMounted(() => {
  if (albums.value.length === 0) {
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

function openCoverModal(album: Album) {
  activeAlbumForCover.value = album;
  showCoverModal.value = true;
}

async function addTrackToPlaylist(playlistPath: string) {
  if (selectedTrackForPlaylist.value) {
    await playlistStore.addToPlaylist(playlistPath, [selectedTrackForPlaylist.value]);
    showPlaylistModal.value = false;
    selectedTrackForPlaylist.value = null;
  }
}

async function handleCoverSelect(url: string) {
  if (!activeAlbumForCover.value) return;
  
  const targetAlbum = activeAlbumForCover.value; // Capture ref
  
  try {
    libraryStore.isLoading = true;
    await invoke<string>('apply_cover', { 
      albumPath: targetAlbum.path,
      coverUrl: url
    });
    
    // Refresh album data from disk to get updated tracks and cover
    await libraryStore.refreshAlbum(targetAlbum.id);
    
    // Force UI refresh (re-mount components to reload images)
    componentKey.value++;
    
    toastStore.success('Pochette appliquée à tout l\'album');
    
  } catch (e) {
    toastStore.error(String(e));
  } finally {
    libraryStore.isLoading = false;
    showCoverModal.value = false;
    activeAlbumForCover.value = null;
  }
}

function goBack() {
  router.push('/library');
}

function playTrack(track: Track) {
  playerStore.play(track);
}
</script>

<template>
  <div class="h-screen overflow-hidden bg-gray-900 text-white flex flex-col">
    <!-- Loading Overlay -->
    <div v-if="libraryStore.isLoading" class="fixed inset-0 bg-black/50 z-50 flex items-center justify-center backdrop-blur-sm">
      <div class="flex flex-col items-center gap-4">
        <div class="animate-spin rounded-full h-12 w-12 border-4 border-blue-500 border-t-transparent"></div>
        <p class="text-blue-400 font-medium">Traitement en cours...</p>
      </div>
    </div>

    <!-- Header / Toolbar -->
    <AlbumDetailToolbar :albums="albums" />

    <!-- Content -->
    <main class="flex-1 w-full mx-auto px-4 py-4 flex flex-col overflow-hidden">
      <template v-if="albums.length > 1">
        <MultiAlbumEditor 
          :albums="albums"
          @play="playTrack"
          @add-to-playlist="openPlaylistModal"
        />
      </template>
      <template v-else-if="albums.length === 1">
        <AlbumEditor 
          :key="componentKey"
          :album="albums[0]"
          @play="playTrack"
          @add-to-playlist="openPlaylistModal"
          @change-cover="openCoverModal(albums[0])"
          @search-cover="openCoverModal(albums[0])"
        />
      </template>
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
      v-if="showCoverModal && activeAlbumForCover"
      :is-open="showCoverModal"
      :artist="activeAlbumForCover.tracks[0]?.album_artist || activeAlbumForCover.tracks[0]?.artist || activeAlbumForCover.artist"
      :album="activeAlbumForCover.tracks[0]?.album || activeAlbumForCover.title"
      @close="showCoverModal = false"
      @select="handleCoverSelect"
    />
  </div>
</template>
