import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { Track } from '../types'
import { useToastStore } from './toast'

export const usePlayerStore = defineStore('player', () => {
  const currentTrack = ref<Track | null>(null)
  const isPlaying = ref(false)
  const volume = ref(1.0)
  const currentTime = ref(0)
  const duration = ref(0)
  const currentCoverPath = ref<string | null>(null)
  const queue = ref<Track[]>([])
  const currentIndex = ref(-1)
  
  // Equalizer state (-10 to 10)
  const eqSettings = ref({
    bass: 0,
    mid: 0,
    treble: 0
  })

  const toast = useToastStore()
  
  let timer: number | null = null

  function startTimer() {
    if (timer) clearInterval(timer)
    timer = setInterval(() => {
      if (currentTime.value < duration.value) {
        currentTime.value += 1
      } else {
        // Fin de piste : essayer de passer à la suivante
        if (currentIndex.value < queue.value.length - 1) {
          next()
        } else {
          stop()
        }
      }
    }, 1000) as unknown as number
  }

  function stopTimer() {
    if (timer) clearInterval(timer)
    timer = null
  }

  async function play(track: Track, albumCoverPath?: string | null, newQueue?: Track[]) {
    try {
      // Mise à jour de la queue si fournie
      if (newQueue && newQueue.length > 0) {
        queue.value = newQueue
        currentIndex.value = newQueue.findIndex(t => t.path === track.path)
      } else if (queue.value.length === 0 || !queue.value.find(t => t.path === track.path)) {
        // Fallback: queue de 1 élément
        queue.value = [track]
        currentIndex.value = 0
      } else {
        // La piste est dans la queue existante, on met à jour l'index
        currentIndex.value = queue.value.findIndex(t => t.path === track.path)
      }

      // Si c'est la même piste en cours, on toggle pause/play
      if (currentTrack.value?.path === track.path && isPlaying.value) {
        await pause()
        return
      } else if (currentTrack.value?.path === track.path && !isPlaying.value) {
        await resume()
        return
      }

      // Nouvelle lecture
      await invoke('play_track', { path: track.path })
      currentTrack.value = track
      duration.value = track.duration_sec || 0
      currentTime.value = 0
      if (albumCoverPath !== undefined) {
        currentCoverPath.value = albumCoverPath
      }
      isPlaying.value = true
      startTimer()
    } catch (error) {
      toast.error(`Erreur lecture: ${error}`)
    }
  }

  async function next() {
    if (queue.value.length > 0 && currentIndex.value < queue.value.length - 1) {
      const nextTrack = queue.value[currentIndex.value + 1]
      await play(nextTrack, currentCoverPath.value) // Garde la cover actuelle
    }
  }

  async function previous() {
    // Si on est à > 3 secondes, on restart la piste, sinon précédente
    if (currentTime.value > 3) {
      await invoke('play_track', { path: currentTrack.value?.path })
      currentTime.value = 0
      return
    }

    if (queue.value.length > 0 && currentIndex.value > 0) {
      const prevTrack = queue.value[currentIndex.value - 1]
      await play(prevTrack, currentCoverPath.value)
    }
  }

  async function pause() {
    try {
      await invoke('pause_track')
      isPlaying.value = false
      stopTimer()
    } catch (error) {
      toast.error(`Erreur pause: ${error}`)
    }
  }

  async function resume() {
    try {
      await invoke('resume_track')
      isPlaying.value = true
      startTimer()
    } catch (error) {
      toast.error(`Erreur resume: ${error}`)
    }
  }

  async function stop() {
    try {
      await invoke('stop_track')
      currentTrack.value = null
      isPlaying.value = false
      currentTime.value = 0
      stopTimer()
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

  async function seek(time: number) {
    try {
      // Backend seek
      await invoke('seek_track', { time })
      // Frontend sync
      currentTime.value = time
    } catch (error) {
      toast.error(`Erreur seek: ${error}`)
    }
  }

  async function setEq(bass: number, mid: number, treble: number) {
    eqSettings.value = { bass, mid, treble }
    try {
      await invoke('set_eq', { bass, mid, treble })
    } catch (error) {
      toast.error(`Erreur EQ: ${error}`)
    }
  }

  return {
    currentTrack,
    isPlaying,
    volume,
    currentTime,
    duration,
    currentCoverPath,
    queue,
    currentIndex,
    eqSettings,
    play,
    pause,
    resume,
    stop,
    next,
    previous,
    setVolume,
    seek,
    setEq
  }
})
