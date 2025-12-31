import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { Track } from '../types'
import { useToastStore } from './toast'

export const usePlayerStore = defineStore('player', () => {
  const currentTrack = ref<Track | null>(null)
  const isPlaying = ref(false)
  const volume = ref(1.0)
  const toast = useToastStore()

  async function play(track: Track) {
    try {
      // Si c'est la même piste, on toggle
      if (currentTrack.value?.path === track.path) {
        if (isPlaying.value) {
          await pause()
        } else {
          await resume()
        }
        return
      }

      // Nouvelle piste
      await invoke('play_track', { path: track.path })
      currentTrack.value = track
      isPlaying.value = true
    } catch (error) {
      toast.error(`Erreur lecture: ${error}`)
    }
  }

  async function pause() {
    try {
      await invoke('pause_track')
      isPlaying.value = false
    } catch (error) {
      toast.error(`Erreur pause: ${error}`)
    }
  }

  async function resume() {
    try {
      await invoke('resume_track')
      isPlaying.value = true
    } catch (error) {
      toast.error(`Erreur resume: ${error}`)
    }
  }

  async function stop() {
    try {
      await invoke('stop_track')
      currentTrack.value = null
      isPlaying.value = false
    } catch (error) {
      toast.error(`Erreur stop: ${error}`)
    }
  }

  async function setVolume(val: number) {
    try {
      await invoke('set_volume', { volume: val })
      volume.value = val
    } catch (error) {
      toast.error(`Erreur volume: ${error}`)
    }
  }

  return {
    currentTrack,
    isPlaying,
    volume,
    play,
    pause,
    resume,
    stop,
    setVolume
  }
})
