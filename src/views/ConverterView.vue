<script setup lang="ts">
import { ref } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core'
import { useToastStore } from '../stores/toast'
import { useRouter } from 'vue-router'

const files = ref<string[]>([])
const converting = ref(false)
const progress = ref(0)
const toast = useToastStore()
const router = useRouter()

async function selectFiles() {
  try {
    const selected = await open({
      multiple: true,
      filters: [{ name: 'Audio', extensions: ['flac', 'wav', 'm4a', 'ogg'] }]
    })
    if (selected) {
      // @ts-ignore
      files.value = Array.isArray(selected) ? selected : [selected]
    }
  } catch (e) {
    toast.error(`Erreur sélection: ${e}`)
  }
}

async function convert() {
  if (files.value.length === 0) return
  
  converting.value = true
  progress.value = 0
  let successCount = 0
  
  for (let i = 0; i < files.value.length; i++) {
    try {
      await invoke('convert_file', { 
        inputPath: files.value[i], 
        bitrate: '320k' 
      })
      successCount++
    } catch (e) {
      toast.error(`Erreur conversion ${files.value[i]}: ${e}`)
    }
    progress.value = ((i + 1) / files.value.length) * 100
  }
  
  toast.success(`${successCount}/${files.value.length} fichiers convertis.`)
  converting.value = false
  files.value = []
}

function goBack() {
  router.push('/')
}
</script>

<template>
  <div class="min-h-screen bg-gray-900 flex flex-col text-white">
    <!-- Header -->
    <header class="bg-gray-800 shadow-sm px-6 py-4 flex items-center justify-between border-b border-gray-700">
      <div class="flex items-center gap-4">
        <button @click="goBack" class="p-2 hover:bg-gray-700 rounded-full transition-colors text-gray-300">
          ⬅️
        </button>
        <h1 class="text-xl font-bold text-white">Convertisseur MP3</h1>
      </div>
    </header>

    <main class="flex-1 max-w-4xl w-full mx-auto p-8">
      <div class="bg-gray-800 rounded-xl shadow-lg p-8 text-center border border-gray-700">
        <div class="mb-8">
          <div class="w-24 h-24 bg-blue-900/30 text-blue-400 rounded-full flex items-center justify-center mx-auto mb-4 text-4xl">
            🔄
          </div>
          <h2 class="text-2xl font-bold text-white">Convertir en MP3 (320kbps)</h2>
          <p class="text-gray-400 mt-2">Sélectionnez vos fichiers FLAC, WAV ou M4A</p>
        </div>

        <button 
          @click="selectFiles"
          :disabled="converting"
          class="bg-blue-600 text-white px-8 py-3 rounded-xl font-semibold hover:bg-blue-700 transition-all transform hover:scale-105 disabled:opacity-50 disabled:cursor-not-allowed shadow-lg shadow-blue-600/20"
        >
          Choisir des fichiers
        </button>

        <div v-if="files.length > 0" class="mt-8 text-left">
          <h3 class="font-medium text-gray-300 mb-2">Fichiers sélectionnés ({{ files.length }})</h3>
          <div class="bg-gray-900 rounded-lg p-4 max-h-60 overflow-y-auto border border-gray-700">
            <div v-for="file in files" :key="file" class="text-sm text-gray-400 py-1 truncate">
              {{ file }}
            </div>
          </div>
          
          <div class="mt-6 flex justify-center">
            <button 
              @click="convert"
              :disabled="converting"
              class="bg-green-600 text-white px-8 py-3 rounded-xl font-semibold hover:bg-green-700 transition-all shadow-lg shadow-green-600/20 flex items-center gap-2"
            >
              <span v-if="converting" class="animate-spin">⏳</span>
              {{ converting ? 'Conversion en cours...' : 'Lancer la conversion' }}
            </button>
          </div>
        </div>

        <!-- Progress Bar -->
        <div v-if="converting" class="mt-8">
          <div class="w-full bg-gray-700 rounded-full h-2.5">
            <div class="bg-green-600 h-2.5 rounded-full transition-all duration-300" :style="{ width: progress + '%' }"></div>
          </div>
          <p class="text-sm text-gray-400 mt-2">{{ Math.round(progress) }}%</p>
        </div>
      </div>
    </main>
  </div>
</template>
