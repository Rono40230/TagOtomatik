<script setup lang="ts">
import { ref, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { Album } from '../types'
import { useToastStore } from '../stores/toast'
import { useSettingsStore } from '../stores/settings'

const props = defineProps<{
  album: Album
  isOpen: boolean
}>()

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'refresh'): void
}>()

const toast = useToastStore()
const settingsStore = useSettingsStore()

// Configuration
const filename = ref('')
const format = ref('m3u')
const useExtendedInfo = ref(true)
const pathSeparator = ref('Auto') // 'Auto' | 'Slash' | 'Backslash'

const creating = ref(false)

// Reset state when modal opens
watch(() => props.isOpen, (newVal) => {
  if (newVal) {
    // Default filename based on pattern
    const pattern = settingsStore.playlist.defaultNamePattern || '{artist} - {album}.m3u'
    
    const safeArtist = props.album.artist.replace(/[<>:"/\\|?*]/g, '')
    const safeTitle = props.album.title.replace(/[<>:"/\\|?*]/g, '')
    const safeYear = (props.album.year || '').toString().replace(/[<>:"/\\|?*]/g, '')

    let name = pattern
      .replace(/{artist}/g, safeArtist)
      .replace(/{album}/g, safeTitle)
      .replace(/{year}/g, safeYear)
    
    // Smart cleanup: If the title itself starts with a date-like structure (e.g. "(1971-15)...")
    // and the pattern injected the metadata year (e.g. "(2015)..."), it creates a redundant double date.
    // We remove the injected year in this specific visual clash.
    if (safeYear && props.album.title.trim().startsWith('(')) {
      const yearStr = `(${safeYear}) `;
      if (name.includes(yearStr)) {
        name = name.replace(yearStr, '');
      }
    }

    // Remove extension from name if present, as it might be added later or confusing
    // Actually, the pattern includes extension usually. 
    // But the UI below has format selection? No, format is just m3u.
    // Let's check if the pattern has an extension.
    // If the pattern ends with .m3u or .m3u8, we strip it because the user might want to change it?
    // Wait, the previous code was `filename.value = ...` and `format.value = 'm3u'`.
    // The createPlaylist function uses `filename.value` and `format.value`.
    // If filename has extension, it might be double extension if backend adds it.
    // Let's look at createPlaylist again.
    // It sends filename and format separately.
    // Usually backend appends format if missing.
    // Let's strip extension from the generated name to be safe and let the format handle it, 
    // OR just let the user edit the full filename including extension.
    // The previous code set filename to "Artist - Album" (no extension).
    // So I should probably strip the extension from the pattern result if it matches the selected format.
    
    name = name.replace(/\.m3u8?$/i, '')
    
    filename.value = name
    
    format.value = 'm3u'
    // Sync from store
    useExtendedInfo.value = true
    pathSeparator.value = 'Auto'
    creating.value = false
  }
})

async function createPlaylist() {
  if (!filename.value) return

  creating.value = true
  try {
    await invoke('write_playlist', {
      options: {
        album_path: props.album.path,
        tracks: props.album.tracks,
        filename: filename.value,
        format: format.value,
        path_type: settingsStore.playlist.useRelativePaths ? 'Relative' : 'Absolute',
        use_extended_info: useExtendedInfo.value,
        path_separator: pathSeparator.value
      }
    })
    
    toast.success(`Playlist créée : ${filename.value}.${format.value}`)
    emit('refresh')
    emit('close')
  } catch (e) {
    toast.error(`Erreur création playlist: ${e}`)
  } finally {
    creating.value = false
  }
}

function close() {
  if (creating.value) return
  emit('close')
}
</script>

<template>
  <div v-if="isOpen" class="fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-sm" @click.stop="close">
    <div class="bg-gray-900 border border-gray-700 rounded-xl w-full max-w-lg flex flex-col shadow-2xl overflow-hidden" @click.stop>
      
      <!-- Header -->
      <div class="p-4 border-b border-gray-700 bg-gray-800 flex justify-between items-center">
        <h3 class="text-lg font-bold text-white">Créer une Playlist</h3>
        <button @click="close" :disabled="creating" class="text-gray-400 hover:text-white disabled:opacity-50">✕</button>
      </div>

      <!-- Content -->
      <div class="p-6 space-y-4">
        
        <!-- Filename -->
        <div>
          <label class="block text-xs font-medium text-gray-400 mb-1">Nom du fichier</label>
          <div class="flex gap-2">
            <input 
              v-model="filename" 
              type="text" 
              class="flex-1 bg-gray-700 border border-gray-600 text-white rounded px-3 py-2 text-sm focus:ring-2 focus:ring-blue-500 outline-none"
              placeholder="Nom de la playlist"
              :disabled="creating"
            />
            <div class="bg-gray-800 border border-gray-700 text-gray-400 px-3 py-2 rounded text-sm flex items-center">
              .{{ format }}
            </div>
          </div>
        </div>

        <!-- Format & Path Type -->
        <div class="grid grid-cols-2 gap-4">
          <div>
            <label class="block text-xs font-medium text-gray-400 mb-1">Format</label>
            <select v-model="format" :disabled="creating" class="w-full bg-gray-700 border border-gray-600 text-white rounded px-3 py-2 text-sm focus:ring-2 focus:ring-blue-500 outline-none">
              <option value="m3u">M3U (Standard)</option>
              <option value="m3u8">M3U8 (UTF-8)</option>
              <option value="pls">PLS</option>
            </select>
          </div>
          
          <div>
            <label class="block text-xs font-medium text-gray-400 mb-1">Chemin des fichiers</label>
            <select v-model="settingsStore.playlist.useRelativePaths" :disabled="creating" class="w-full bg-gray-700 border border-gray-600 text-white rounded px-3 py-2 text-sm focus:ring-2 focus:ring-blue-500 outline-none">
              <option :value="false">Absolu (/home/user/...)</option>
              <option :value="true">Relatif (./track.mp3)</option>
            </select>
          </div>
        </div>

        <!-- Advanced Options -->
        <div class="grid grid-cols-2 gap-4">
           <!-- Separator -->
           <div>
            <label class="block text-xs font-medium text-gray-400 mb-1">Séparateur de dossier</label>
            <select v-model="pathSeparator" :disabled="creating" class="w-full bg-gray-700 border border-gray-600 text-white rounded px-3 py-2 text-sm focus:ring-2 focus:ring-blue-500 outline-none">
              <option value="Auto">Auto (Système)</option>
              <option value="Slash">Slash (Linux/Mac)</option>
              <option value="Backslash">Backslash (Windows)</option>
            </select>
          </div>

          <!-- Extended Info Toggle -->
          <div class="flex flex-col justify-end">
             <div 
              class="w-full bg-gray-700 border border-gray-600 text-white rounded px-3 py-2 text-sm flex items-center justify-between cursor-pointer hover:bg-gray-600/50 transition-colors h-[38px]" 
              @click="!creating && (useExtendedInfo = !useExtendedInfo)"
            >
              <span class="text-sm text-gray-300 select-none">Métadonnées (EXTM3U)</span>
              <button 
                type="button"
                :class="useExtendedInfo ? 'bg-blue-600' : 'bg-gray-600'"
                class="relative inline-flex h-5 w-9 items-center rounded-full transition-colors focus:outline-none"
                :disabled="creating"
              >
                <span
                  :class="useExtendedInfo ? 'translate-x-4' : 'translate-x-1'"
                  class="inline-block h-3 w-3 transform rounded-full bg-white transition-transform"
                />
              </button>
            </div>
          </div>
        </div>

        <!-- Track Preview (Compact) -->
        <div class="mt-4">
          <label class="block text-xs font-medium text-gray-400 mb-1">Aperçu des pistes ({{ album.tracks.length }})</label>
          <div class="bg-gray-800 rounded border border-gray-700 max-h-32 overflow-y-auto p-2 text-xs text-gray-400 font-mono">
            <div v-for="(track, i) in album.tracks" :key="track.path" class="truncate">
              {{ i + 1 }}. {{ track.title || track.filename }}
            </div>
          </div>
        </div>

      </div>

      <!-- Footer -->
      <div class="p-4 border-t border-gray-700 bg-gray-800 flex justify-end gap-3">
        <button 
          @click="close" 
          class="px-4 py-2 text-sm text-gray-300 hover:text-white transition-colors"
          :disabled="creating"
        >
          Annuler
        </button>
        
        <button 
          @click="createPlaylist"
          :disabled="creating || !filename"
          class="px-6 py-2 bg-blue-600 hover:bg-blue-500 text-white text-sm font-bold rounded shadow-lg transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
        >
          <span v-if="creating" class="animate-spin">↻</span>
          {{ creating ? 'Création...' : 'Créer la playlist' }}
        </button>
      </div>

    </div>
  </div>
</template>
