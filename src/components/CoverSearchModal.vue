<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useToastStore } from '../stores/toast'

const props = defineProps<{
  artist: string
  album: string
  isOpen: boolean
}>()

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'select', url: string): void
}>()

interface CoverResult {
  url: string
  source: string
  size?: [number, number]
}

const results = ref<CoverResult[]>([])
const loading = ref(false)
const toast = useToastStore()

async function search() {
  loading.value = true
  results.value = []
  try {
    results.value = await invoke('search_cover', { 
      artist: props.artist, 
      album: props.album 
    })
    if (results.value.length === 0) {
      toast.info('Aucune pochette trouvée.')
    }
  } catch (e) {
    toast.error(`Erreur recherche: ${e}`)
  } finally {
    loading.value = false
  }
}

function select(url: string) {
  emit('select', url)
  emit('close')
}

// Auto-search on open
import { watch } from 'vue'
watch(() => props.isOpen, (newVal) => {
  if (newVal) {
    search()
  }
})
</script>

<template>
  <div v-if="isOpen" class="fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-sm">
    <div class="bg-app-surface border border-app-border rounded-xl w-full max-w-3xl max-h-[80vh] flex flex-col shadow-2xl">
      <!-- Header -->
      <div class="p-4 border-b border-app-border flex justify-between items-center">
        <h3 class="text-lg font-semibold text-white">Recherche de pochette</h3>
        <button @click="$emit('close')" class="text-slate-400 hover:text-white">✕</button>
      </div>

      <!-- Content -->
      <div class="p-6 overflow-y-auto flex-1">
        <div v-if="loading" class="flex flex-col items-center justify-center h-64">
          <div class="animate-spin rounded-full h-12 w-12 border-4 border-primary border-t-transparent"></div>
          <p class="mt-4 text-slate-400">Recherche sur MusicBrainz...</p>
        </div>

        <div v-else-if="results.length > 0" class="grid grid-cols-3 gap-4">
          <div 
            v-for="res in results" 
            :key="res.url"
            class="group relative aspect-square bg-slate-800 rounded-lg overflow-hidden cursor-pointer border-2 border-transparent hover:border-primary transition-all"
            @click="select(res.url)"
          >
            <img :src="res.url" class="w-full h-full object-cover" loading="lazy" />
            <div class="absolute inset-0 bg-black/50 opacity-0 group-hover:opacity-100 flex items-center justify-center transition-opacity">
              <span class="bg-primary text-white px-3 py-1 rounded-full text-sm font-medium">Choisir</span>
            </div>
            <div class="absolute bottom-0 left-0 right-0 bg-black/70 p-2 text-xs text-slate-300 truncate">
              {{ res.source }}
            </div>
          </div>
        </div>

        <div v-else class="flex flex-col items-center justify-center h-64 text-slate-500">
          <span class="text-4xl mb-2">🖼️</span>
          <p>Aucun résultat trouvé pour "{{ album }}"</p>
        </div>
      </div>
    </div>
  </div>
</template>
