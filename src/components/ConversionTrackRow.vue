<script setup lang="ts">
import type { Track } from '../types'

defineProps<{
  track: Track
  index: number
  status: 'pending' | 'converting' | 'success' | 'error'
  quality: string
}>()
</script>

<template>
  <div 
    class="flex items-center justify-between p-2 rounded hover:bg-gray-800 transition-colors border border-transparent"
    :class="{'border-blue-500/30 bg-blue-900/10': status === 'converting'}"
  >
    <div class="flex items-center gap-3 overflow-hidden flex-1">
      <!-- Status Icon / Track Number -->
      <div class="w-8 flex justify-center font-mono text-sm shrink-0">
        <span v-if="status === 'pending'" class="text-gray-500">{{ track.track_number || index + 1 }}</span>
        <span v-else-if="status === 'converting'" class="text-blue-400">
          <!-- Indeterminate Progress Bar for current track -->
          <div class="w-4 h-4 relative flex items-center justify-center">
              <div class="absolute inset-0 border-2 border-blue-500/30 rounded-full"></div>
              <div class="absolute inset-0 border-2 border-blue-500 border-t-transparent rounded-full animate-spin"></div>
          </div>
        </span>
        <span v-else-if="status === 'success'" class="text-green-500">✅</span>
        <span v-else-if="status === 'error'" class="text-red-500">❌</span>
      </div>
      
      <div class="flex flex-col truncate w-full">
        <div class="flex justify-between items-center">
          <span class="text-sm text-gray-200 truncate">{{ track.title || track.filename }}</span>
        </div>
        
        <!-- Track Progress Bar (Indeterminate) when converting -->
        <div v-if="status === 'converting'" class="w-full h-1 bg-gray-700 rounded-full mt-1 overflow-hidden relative">
          <div class="absolute inset-0 bg-blue-500/50 animate-pulse w-full h-full origin-left"></div>
          <div class="absolute top-0 bottom-0 left-0 bg-blue-400 w-1/3 animate-[shimmer_1.5s_infinite]"></div>
        </div>
        <div v-else class="text-xs text-gray-500">
          {{ track.format }} • {{ (track.size / 1024 / 1024).toFixed(1) }} Mo
        </div>
      </div>
    </div>

    <!-- Estimated Size (Mockup logic for now) -->
    <div class="text-xs text-gray-500 whitespace-nowrap">
      <span v-if="status === 'success'">Converti</span>
      <span v-else>~{{ (track.duration_sec * (parseInt(quality) || 320) / 8 / 1024).toFixed(1) }} Mo</span>
    </div>
  </div>
</template>

<style scoped>
@keyframes shimmer {
  0% { transform: translateX(-100%); }
  100% { transform: translateX(300%); }
}
</style>
