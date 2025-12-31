import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useToastStore } from './toast'

export interface Playlist {
  path: string
  name: string
  track_count: number
}

export const usePlaylistStore = defineStore('playlist', () => {
  const playlists = ref<Playlist[]>([])
  const currentPlaylistTracks = ref<string[]>([])
  const isLoading = ref(false)
  const error = ref<string | null>(null)
  const toastStore = useToastStore()

  async function loadPlaylists(rootPath: string) {
    isLoading.value = true
    error.value = null
    try {
      playlists.value = await invoke('list_playlists', { rootPath })
    } catch (e) {
      error.value = String(e)
      toastStore.error(`Erreur chargement playlists: ${e}`)
    } finally {
      isLoading.value = false
    }
  }

  async function loadPlaylistTracks(playlistPath: string) {
    isLoading.value = true
    error.value = null
    try {
      currentPlaylistTracks.value = await invoke('get_playlist_tracks', { playlistPath })
    } catch (e) {
      error.value = String(e)
      toastStore.error(`Erreur chargement pistes: ${e}`)
    } finally {
      isLoading.value = false
    }
  }

  async function createPlaylist(name: string, tracks: string[], rootPath: string) {
    isLoading.value = true
    try {
      await invoke('create_playlist', { name, tracks, rootPath })
      toastStore.success(`Playlist "${name}" créée`)
      await loadPlaylists(rootPath)
    } catch (e) {
      toastStore.error(`Erreur création playlist: ${e}`)
    } finally {
      isLoading.value = false
    }
  }

  async function addToPlaylist(playlistPath: string, tracks: string[]) {
    isLoading.value = true
    try {
      await invoke('add_to_playlist', { playlistPath, tracks })
      toastStore.success('Pistes ajoutées à la playlist')
      // Update track count locally or reload
      const pl = playlists.value.find(p => p.path === playlistPath)
      if (pl) pl.track_count += tracks.length
    } catch (e) {
      toastStore.error(`Erreur ajout pistes: ${e}`)
    } finally {
      isLoading.value = false
    }
  }

  return {
    playlists,
    currentPlaylistTracks,
    isLoading,
    error,
    loadPlaylists,
    loadPlaylistTracks,
    createPlaylist,
    addToPlaylist
  }
})
