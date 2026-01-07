<script setup lang="ts">
import { usePlayerStore } from '../stores/player'
import { computed, ref, watch, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import EqualizerWidget from './EqualizerWidget.vue'

const player = usePlayerStore()

const trackTitle = computed(() => {
  if (!player.currentTrack) return 'Aucune lecture'
  const num = player.currentTrack.track_number
  const prefix = num ? `${num.toString().padStart(2, '0')} - ` : ''
  return `${prefix}${player.currentTrack.title}`
})
const trackArtist = computed(() => player.currentTrack?.artist || '')
const coverUrl = ref<string | null>(null)

// Format time mm:ss
function formatTime(seconds: number) {
  const m = Math.floor(seconds / 60)
  const s = Math.floor(seconds % 60)
  return `${m}:${s.toString().padStart(2, '0')}`
}

async function loadCover() {
  if (!player.currentCoverPath) {
    coverUrl.value = null
    return
  }
  try {
    const contents = await invoke<number[]>('read_cover', { path: player.currentCoverPath })
    const uint8Array = new Uint8Array(contents)
    const blob = new Blob([uint8Array])
    const url = URL.createObjectURL(blob)
    
    if (coverUrl.value && coverUrl.value.startsWith('blob:')) {
      URL.revokeObjectURL(coverUrl.value)
    }
    coverUrl.value = url
  } catch (e) {
    coverUrl.value = null
  }
}

watch(() => player.currentCoverPath, loadCover)

onUnmounted(() => {
  if (coverUrl.value) URL.revokeObjectURL(coverUrl.value)
})

function togglePlay() {
  if (player.currentTrack) {
    player.play(player.currentTrack, player.currentCoverPath)
  }
}

function handleVolume(event: Event) {
  const val = parseFloat((event.target as HTMLInputElement).value)
  player.setVolume(val)
}

// ---- SEEK LOGIC ----
const isDragging = ref(false)
const internalTime = ref(player.currentTime) 

// Initial sync
watch(() => player.currentTime, (newVal) => {
  if (!isDragging.value) {
    internalTime.value = newVal
  }
}, { immediate: true })

function onDragStart() {
  isDragging.value = true
}

function onDragEnd() {
  isDragging.value = false
  // Le seek est géré par @change, mais on assure le flag ici aussi
}

// Called continuously while dragging: updates UI only
function onSeekInput(event: Event) {
  isDragging.value = true // Safety
  internalTime.value = parseFloat((event.target as HTMLInputElement).value)
}

// Called on drop: updates actual player
function onSeekChange(event: Event) {
  const val = parseFloat((event.target as HTMLInputElement).value)
  player.seek(val)
  isDragging.value = false
}
</script>


<template>
  <div v-if="player.currentTrack" class="h-24 bg-gray-900 border-t border-gray-700 flex items-center justify-center z-50 text-white shadow-2xl relative">
    
    <!-- Background blurred for aesthetic (optional) -> removed for clean look -->
    
    <!-- GAUCHE : Cover + Infos (Absolute) -->
    <div class="absolute left-4 flex items-center gap-4">
      <!-- Cover -->
      <div class="w-16 h-16 bg-gray-800 rounded-md shadow-lg overflow-hidden flex-shrink-0 relative group">
        <img v-if="coverUrl" :src="coverUrl" class="w-full h-full object-cover" />
        <div v-else class="w-full h-full flex items-center justify-center text-2xl">🎵</div>
      </div>
      
      <!-- Infos -->
      <div class="flex flex-col min-w-0 max-w-[200px] mr-4">
        <span class="text-sm font-bold text-white truncate" :title="trackTitle">{{ trackTitle }}</span>
        <span class="text-xs text-gray-400 truncate" :title="trackArtist">{{ trackArtist }}</span>
      </div>
    </div>

    <!-- CENTRE : Contrôles + Progress Bar -->
    <div class="flex items-center gap-6">
       <!-- Contrôles Compacts -->
      <div class="flex items-center gap-3">
        <button 
          @click="player.previous()"
          class="text-gray-400 hover:text-white hover:bg-gray-800 p-1 rounded-full transition-all"
          title="Précédent"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="currentColor" viewBox="0 0 24 24"><path d="M6 6h2v12H6zm3.5 6l8.5 6V6z"/></svg>
        </button>
        <button 
          @click="togglePlay"
          class="w-10 h-10 rounded-full bg-white text-black flex items-center justify-center hover:scale-110 hover:shadow-lg transition-all"
        >
          <svg v-if="player.isPlaying" xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor"><path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zM7 8a1 1 0 012 0v4a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v4a1 1 0 102 0V8a1 1 0 00-1-1z" clip-rule="evenodd" /></svg>
          <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 ml-0.5" viewBox="0 0 20 20" fill="currentColor"><path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clip-rule="evenodd" /></svg>
        </button>
        <button 
          @click="player.next()"
          :disabled="player.queue.length > 0 && player.currentIndex >= player.queue.length - 1"
          class="text-gray-400 hover:text-white hover:bg-gray-800 p-1 rounded-full transition-all disabled:opacity-30 disabled:cursor-not-allowed"
          title="Suivant"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="currentColor" viewBox="0 0 24 24"><path d="M6 18l8.5-6L6 6v12zM16 6v12h2V6h-2z"/></svg>
        </button>
      </div>

      <!-- Time + Progress Bar + Duration -->
      <div class="flex items-center gap-3 w-[25vw]">
        <span class="text-xs text-gray-400 font-mono w-10 text-right">{{ formatTime(internalTime) }}</span>
        
        <div class="relative flex-1 h-1.5 group cursor-pointer">
             <!-- Background Track -->
            <div class="absolute inset-0 bg-gray-700 rounded-full"></div>
            <!-- Progress Fill -->
            <div 
                class="absolute left-0 top-0 bottom-0 bg-gradient-to-r from-blue-500 to-indigo-500 rounded-full transition-all duration-300 pointer-events-none"
                :style="{ width: `${(internalTime / (player.duration || 1)) * 100}%` }"
            ></div>
            <!-- Input Thumb (Invisible but interactive) -->
            <input 
                type="range" 
                min="0" 
                :max="player.duration || 100" 
                v-model.number="internalTime"
                @mousedown="onDragStart"
                @touchstart="onDragStart"
                @mouseup="onDragEnd"
                @touchend="onDragEnd"
                @input="onSeekInput"
                @change="onSeekChange"
                class="absolute inset-0 w-full h-full opacity-0 cursor-pointer z-10"
            />
            <!-- Visual Thumb on Hover -->
            <div 
                class="absolute top-1/2 -mt-2 h-4 w-4 bg-white rounded-full shadow-md opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none transform -translate-x-2"
                :style="{ left: `${(internalTime / (player.duration || 1)) * 100}%` }"
            ></div>
        </div>

        <span class="text-xs text-gray-400 font-mono w-10">{{ formatTime(player.duration) }}</span>
      </div>
    </div>

    <!-- DROITE : Volume (Absolute) -->
    <div class="absolute right-4 flex items-center justify-end gap-3 min-w-[120px]">
      
      <!-- Equalizer Widget -->
      <EqualizerWidget :is-playing="player.isPlaying" class="mr-4 cursor-pointer hover:opacity-100 transition-opacity" title="Égaliseur (Bientôt disponible)" />

      <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.536 8.464a5 5 0 010 7.072m2.828-9.9a9 9 0 010 12.728M5.586 15H4a1 1 0 01-1-1v-4a1 1 0 011-1h1.586l4.707-4.707C10.923 3.663 12 4.109 12 5v14c0 .891-1.077 1.337-1.707.707L5.586 15z" />
      </svg>
      <input 
        type="range" 
        min="0" 
        max="1" 
        step="0.01" 
        :value="player.volume"
        @input="handleVolume"
        class="w-24 h-1 bg-gray-600 rounded-lg appearance-none cursor-pointer accent-indigo-500 hover:accent-indigo-400"
      />
    </div>
  </div>
</template>

<style scoped>
/* Pour styliser la range input chrome/safari */
input[type="range"]::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 12px;
  height: 12px;
  background: white;
  border-radius: 50%;
  cursor: pointer;
}
</style>
