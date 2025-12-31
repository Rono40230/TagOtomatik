<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { usePlaylistStore, type Playlist } from '../stores/playlist'
import { useLibraryStore } from '../stores/library'
import { usePlayerStore } from '../stores/player'
import { useRouter } from 'vue-router'

const playlistStore = usePlaylistStore()
const libraryStore = useLibraryStore()
const playerStore = usePlayerStore()
const router = useRouter()

const selectedPlaylist = ref<Playlist | null>(null)
const newPlaylistName = ref('')
const showCreateModal = ref(false)

onMounted(async () => {
  if (libraryStore.currentPath) {
    await playlistStore.loadPlaylists(libraryStore.currentPath)
  }
})

async function selectPlaylist(playlist: Playlist) {
  selectedPlaylist.value = playlist
  await playlistStore.loadPlaylistTracks(playlist.path)
}

async function playTrack(path: string) {
  // Play single track or maybe the whole playlist starting from here?
  // For now, just play the track.
  // Ideally we should queue the whole playlist.
  await playerStore.playTrack(path)
}

async function createNewPlaylist() {
  if (!newPlaylistName.value) return
  if (!libraryStore.currentPath) return
  
  await playlistStore.createPlaylist(newPlaylistName.value, [], libraryStore.currentPath)
  newPlaylistName.value = ''
  showCreateModal.value = false
}

function goBack() {
  router.push('/library')
}
</script>

<template>
  <div class="h-full flex flex-col bg-gray-50">
    <!-- Header -->
    <header class="bg-white shadow-sm px-6 py-4 flex items-center justify-between">
      <div class="flex items-center gap-4">
        <button @click="goBack" class="p-2 hover:bg-gray-100 rounded-full transition-colors">
          ⬅️
        </button>
        <h1 class="text-xl font-bold text-gray-900">Playlists</h1>
      </div>
      <button 
        @click="showCreateModal = true"
        class="bg-primary text-white px-4 py-2 rounded-lg hover:bg-primary-hover transition-colors"
      >
        + Nouvelle Playlist
      </button>
    </header>

    <div class="flex-1 flex overflow-hidden">
      <!-- Sidebar List -->
      <aside class="w-64 bg-white border-r border-gray-200 overflow-y-auto">
        <div v-if="playlistStore.isLoading && !playlistStore.playlists.length" class="p-4 text-center text-gray-500">
          Chargement...
        </div>
        <ul v-else class="divide-y divide-gray-100">
          <li 
            v-for="playlist in playlistStore.playlists" 
            :key="playlist.path"
            @click="selectPlaylist(playlist)"
            class="p-4 hover:bg-gray-50 cursor-pointer transition-colors"
            :class="{ 'bg-blue-50 border-l-4 border-blue-500': selectedPlaylist?.path === playlist.path }"
          >
            <div class="font-medium text-gray-900">{{ playlist.name }}</div>
            <div class="text-xs text-gray-500">{{ playlist.track_count }} pistes</div>
          </li>
        </ul>
      </aside>

      <!-- Main Content (Tracks) -->
      <main class="flex-1 overflow-y-auto p-6">
        <div v-if="selectedPlaylist">
          <h2 class="text-2xl font-bold mb-4">{{ selectedPlaylist.name }}</h2>
          
          <div v-if="playlistStore.isLoading" class="text-gray-500">Chargement des pistes...</div>
          
          <div v-else-if="playlistStore.currentPlaylistTracks.length === 0" class="text-gray-500 italic">
            Playlist vide.
          </div>

          <ul v-else class="bg-white rounded-lg shadow divide-y divide-gray-100">
            <li 
              v-for="(track, index) in playlistStore.currentPlaylistTracks" 
              :key="index"
              class="p-3 hover:bg-gray-50 flex items-center justify-between group"
            >
              <div class="truncate flex-1 pr-4 text-sm text-gray-700" :title="track">
                {{ track.split('/').pop() }}
              </div>
              <button 
                @click="playTrack(track)"
                class="opacity-0 group-hover:opacity-100 p-2 text-primary hover:bg-primary/10 rounded-full transition-all"
              >
                ▶️
              </button>
            </li>
          </ul>
        </div>
        <div v-else class="h-full flex items-center justify-center text-gray-400">
          Sélectionnez une playlist
        </div>
      </main>
    </div>

    <!-- Create Modal -->
    <div v-if="showCreateModal" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div class="bg-white rounded-xl p-6 w-96 shadow-2xl">
        <h3 class="text-lg font-bold mb-4">Nouvelle Playlist</h3>
        <input 
          v-model="newPlaylistName"
          type="text" 
          placeholder="Nom de la playlist"
          class="w-full border border-gray-300 rounded-lg px-3 py-2 mb-4 focus:ring-2 focus:ring-primary focus:border-transparent outline-none"
          @keyup.enter="createNewPlaylist"
        >
        <div class="flex justify-end gap-2">
          <button 
            @click="showCreateModal = false"
            class="px-4 py-2 text-gray-600 hover:bg-gray-100 rounded-lg"
          >
            Annuler
          </button>
          <button 
            @click="createNewPlaylist"
            class="px-4 py-2 bg-primary text-white rounded-lg hover:bg-primary-hover"
            :disabled="!newPlaylistName"
          >
            Créer
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
