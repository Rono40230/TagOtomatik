<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { Album } from '../types'
import { useToastStore } from '../stores/toast'
import ConversionTrackRow from './ConversionTrackRow.vue'
import ConversionSummary from './ConversionSummary.vue'

const props = defineProps<{
  album: Album
  isOpen: boolean
}>()

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'refresh'): void
}>()

const toast = useToastStore()

// Configuration
const quality = ref('320k')
const deleteOriginals = ref(true)

// State
const converting = ref(false)
const progress = ref(0)
const currentTrackIndex = ref(-1)
const trackStatuses = ref<Map<string, 'pending' | 'converting' | 'success' | 'error'>>(new Map())
const showSummary = ref(false)
const conversionStats = ref({ success: 0, total: 0, errors: 0 })

// Reset state when modal opens
watch(() => props.isOpen, (newVal) => {
  if (newVal) {
    converting.value = false
    progress.value = 0
    currentTrackIndex.value = -1
    showSummary.value = false
    trackStatuses.value.clear()
    props.album.tracks.forEach(t => {
      trackStatuses.value.set(t.path, 'pending')
    })
  }
})

const tracksToConvert = computed(() => {
  // Filter tracks that are not already MP3 if needed, or just take all
  // Assuming we want to convert everything in the album to MP3
  return props.album.tracks
})

const canConvert = computed(() => {
  return tracksToConvert.value.length > 0 && !converting.value
})

async function startConversion() {
  if (!canConvert.value) return

  converting.value = true
  progress.value = 0
  currentTrackIndex.value = 0
  let successCount = 0

  const total = tracksToConvert.value.length

  for (let i = 0; i < total; i++) {
    const track = tracksToConvert.value[i]
    currentTrackIndex.value = i
    trackStatuses.value.set(track.path, 'converting')

    try {
      // 1. Convert
      await invoke('convert_file', { 
        inputPath: track.path, 
        bitrate: quality.value 
      })

      // 2. Delete original if requested
      if (deleteOriginals.value) {
        await invoke('delete_file', { path: track.path })
      }

      trackStatuses.value.set(track.path, 'success')
      successCount++
    } catch (e) {
      trackStatuses.value.set(track.path, 'error')
      toast.error(`Erreur sur "${track.title}": ${e}`)
    }

    progress.value = ((i + 1) / total) * 100
  }

  converting.value = false
  currentTrackIndex.value = -1
  
  conversionStats.value = {
    success: successCount,
    total: total,
    errors: total - successCount
  }
  
  if (successCount > 0) {
    emit('refresh')
    showSummary.value = true
  }
}

function close() {
  if (converting.value) return
  emit('close')
}
</script>

<template>
  <div v-if="isOpen" class="fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-sm" @click.stop="close">
    <div 
      class="bg-gray-900 border border-gray-700 rounded-xl w-full flex flex-col shadow-2xl overflow-hidden relative transition-all duration-300 ease-in-out"
      :class="showSummary ? 'max-w-sm' : 'max-w-2xl max-h-[90vh]'"
      @click.stop
    >
      
      <!-- Summary Overlay -->
      <ConversionSummary 
        v-if="showSummary" 
        :stats="conversionStats" 
        @close="close" 
      />

      <template v-else>
        <!-- Header -->
      <div class="p-4 border-b border-gray-700 bg-gray-800 flex justify-between items-center">
        <div>
          <h3 class="text-lg font-bold text-white">Conversion MP3</h3>
          <p class="text-sm text-gray-400">{{ album.artist }} - {{ album.title }}</p>
        </div>
        <button @click="close" :disabled="converting" class="text-gray-400 hover:text-white disabled:opacity-50">✕</button>
      </div>

      <!-- Configuration -->
      <div class="p-4 bg-gray-800/50 border-b border-gray-700 grid grid-cols-2 gap-4">
        <div>
          <label class="block text-xs font-medium text-gray-400 mb-1">Qualité (Bitrate)</label>
          <select v-model="quality" :disabled="converting" class="w-full bg-gray-700 border border-gray-600 text-white rounded px-3 py-2 text-sm focus:ring-2 focus:ring-blue-500 outline-none h-10">
            <option value="320k">320 kbps (Excellent - Recommandé)</option>
            <option value="256k">256 kbps (Très bon)</option>
            <option value="192k">192 kbps (Standard)</option>
            <option value="vbr">VBR (Variable - Optimisé)</option>
          </select>
        </div>
        
        <div>
          <label class="block text-xs font-medium text-gray-400 mb-1">Conserver ou supprimer les fichiers originaux</label>
          <div class="w-full bg-gray-700 border border-gray-600 text-white rounded px-3 text-sm flex items-center justify-between h-10 cursor-pointer hover:bg-gray-600/50 transition-colors" @click="!converting && (deleteOriginals = !deleteOriginals)">
            <span class="text-sm text-gray-300 select-none">Supprimer les originaux</span>
            <button 
              type="button"
              :class="deleteOriginals ? 'bg-red-600' : 'bg-gray-600'"
              class="relative inline-flex h-5 w-9 items-center rounded-full transition-colors focus:outline-none"
              :disabled="converting"
            >
              <span
                :class="deleteOriginals ? 'translate-x-4' : 'translate-x-1'"
                class="inline-block h-3 w-3 transform rounded-full bg-white transition-transform"
              />
            </button>
          </div>
        </div>
      </div>

      <!-- Track List -->
      <div class="flex-1 overflow-y-auto p-4 space-y-2 bg-gray-900">
        <ConversionTrackRow
          v-for="(track, index) in tracksToConvert"
          :key="track.path"
          :track="track"
          :index="index"
          :status="trackStatuses.get(track.path) || 'pending'"
          :quality="quality"
        />
      </div>

      <!-- Footer -->
      <div class="p-4 border-t border-gray-700 bg-gray-800">
        <!-- Progress Bar -->
        <div v-if="converting || progress > 0" class="mb-4">
          <div class="flex justify-between text-xs text-gray-400 mb-1">
            <span>Progression globale</span>
            <span>{{ Math.round(progress) }}%</span>
          </div>
          <div class="w-full bg-gray-700 rounded-full h-2.5 overflow-hidden">
            <div class="bg-blue-600 h-2.5 rounded-full transition-all duration-300" :style="{ width: `${progress}%` }"></div>
          </div>
        </div>

        <div class="flex justify-end gap-3">
          <button 
            @click="close" 
            class="px-4 py-2 text-sm text-gray-300 hover:text-white transition-colors"
            :disabled="converting"
          >
            {{ progress === 100 ? 'Fermer' : 'Annuler' }}
          </button>
          
          <button 
            @click="startConversion"
            :disabled="!canConvert"
            class="px-6 py-2 bg-blue-600 hover:bg-blue-500 text-white text-sm font-bold rounded shadow-lg transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
          >
            <span v-if="converting" class="animate-spin">↻</span>
            {{ converting ? 'Conversion en cours...' : 'Lancer la conversion' }}
          </button>
        </div>
      </div>
      </template>

    </div>
  </div>
</template>

<style scoped>
/* Styles spécifiques si nécessaire */
</style>
