<script setup lang="ts">
import { usePlayerStore } from '../stores/player'
import { computed } from 'vue'

const player = usePlayerStore()

const trackTitle = computed(() => player.currentTrack?.title || 'Aucune lecture')
const trackArtist = computed(() => player.currentTrack?.artist || '')

function togglePlay() {
  if (player.currentTrack) {
    player.play(player.currentTrack)
  }
}

function handleVolume(event: Event) {
  const val = parseFloat((event.target as HTMLInputElement).value)
  player.setVolume(val)
}
</script>

<template>
  <div v-if="player.currentTrack" class="h-20 bg-app-surface border-t border-app-border flex items-center px-6 justify-between z-50">
    <!-- Info Piste -->
    <div class="flex items-center gap-4 w-1/3">
      <div class="w-12 h-12 bg-slate-700 rounded-md flex items-center justify-center overflow-hidden">
        <span class="text-2xl">🎵</span>
      </div>
      <div class="flex flex-col overflow-hidden">
        <span class="text-sm font-semibold text-white truncate">{{ trackTitle }}</span>
        <span class="text-xs text-slate-400 truncate">{{ trackArtist }}</span>
      </div>
    </div>

    <!-- Contrôles -->
    <div class="flex flex-col items-center gap-2 w-1/3">
      <div class="flex items-center gap-4">
        <button class="text-slate-400 hover:text-white transition-colors">
          ⏮️
        </button>
        <button 
          @click="togglePlay"
          class="w-10 h-10 rounded-full bg-white text-black flex items-center justify-center hover:scale-105 transition-transform"
        >
          <span v-if="player.isPlaying">⏸️</span>
          <span v-else>▶️</span>
        </button>
        <button class="text-slate-400 hover:text-white transition-colors">
          ⏭️
        </button>
      </div>
    </div>

    <!-- Volume -->
    <div class="flex items-center justify-end gap-2 w-1/3">
      <span class="text-xs">🔊</span>
      <input 
        type="range" 
        min="0" 
        max="1" 
        step="0.01" 
        :value="player.volume"
        @input="handleVolume"
        class="w-24 h-1 bg-slate-600 rounded-lg appearance-none cursor-pointer accent-primary"
      />
    </div>
  </div>
</template>
