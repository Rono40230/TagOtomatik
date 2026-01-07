<script setup lang="ts">
import { ref } from 'vue'
import { usePlayerStore } from '../stores/player'

defineProps<{
  isPlaying: boolean
}>()

const player = usePlayerStore()
const showPopup = ref(false)

function togglePopup() {
  showPopup.value = !showPopup.value
}

function closePopup() {
  showPopup.value = false
}

function updateEq() {
  player.setEq(player.eqSettings.bass, player.eqSettings.mid, player.eqSettings.treble)
}
</script>

<template>
  <div class="relative">
    <!-- Visualizer (Clickable) -->
    <div 
      @click="togglePopup"
      class="flex items-end gap-[2px] h-6 w-12 opacity-80 cursor-pointer hover:scale-105 transition-transform" 
      :class="{ 'opacity-30': !isPlaying }"
    >
      <div class="bar bg-blue-400 w-1.5 rounded-t-sm" :class="{ 'animate-eq-1': isPlaying }"></div>
      <div class="bar bg-blue-500 w-1.5 rounded-t-sm" :class="{ 'animate-eq-2': isPlaying }"></div>
      <div class="bar bg-indigo-500 w-1.5 rounded-t-sm" :class="{ 'animate-eq-3': isPlaying }"></div>
      <div class="bar bg-purple-500 w-1.5 rounded-t-sm" :class="{ 'animate-eq-4': isPlaying }"></div>
      <div class="bar bg-pink-500 w-1.5 rounded-t-sm" :class="{ 'animate-eq-5': isPlaying }"></div>
    </div>

    <!-- Interface Popup -->
    <div v-if="showPopup" class="absolute bottom-full mb-4 left-1/2 transform -translate-x-1/2 bg-gray-800 border border-gray-600 p-4 rounded-xl shadow-2xl z-[60] w-48 flex flex-col gap-4">
      <div class="flex justify-between items-center border-b border-gray-700 pb-2">
        <span class="text-xs font-bold text-gray-300">Égaliseur</span>
        <button @click="closePopup" class="text-gray-400 hover:text-white text-xs">✕</button>
      </div>
      
      <!-- Sliders -->
      <div class="flex justify-between items-end h-32 px-2">
        <!-- Bass -->
        <div class="flex flex-col items-center gap-2 h-full">
            <input 
              type="range" 
              min="-10" 
              max="10" 
              v-model.number="player.eqSettings.bass"
              @input="updateEq"
              class="h-full w-1 bg-gray-600 rounded-lg appearance-none cursor-pointer vertical-slider"
            />
            <span class="text-[10px] text-gray-400">Bass</span>
        </div>

        <!-- Mid -->
        <div class="flex flex-col items-center gap-2 h-full">
            <input 
              type="range" 
              min="-10" 
              max="10" 
              v-model.number="player.eqSettings.mid"
              @input="updateEq"
              class="h-full w-1 bg-gray-600 rounded-lg appearance-none cursor-pointer vertical-slider"
            />
            <span class="text-[10px] text-gray-400">Mid</span>
        </div>

        <!-- Treble -->
        <div class="flex flex-col items-center gap-2 h-full">
            <input 
              type="range" 
              min="-10" 
              max="10" 
              v-model.number="player.eqSettings.treble"
              @input="updateEq"
              class="h-full w-1 bg-gray-600 rounded-lg appearance-none cursor-pointer vertical-slider accent-pink-500"
            />
            <span class="text-[10px] text-gray-400">Treb</span>
        </div>
      </div>
      
      <!-- Presets (Optional decoration) -->
      <div class="flex justify-center gap-2">
        <button @click="player.setEq(0,0,0)" class="text-[10px] bg-gray-700 hover:bg-gray-600 px-2 py-1 rounded text-gray-300">Reset</button>
      </div>

       <!-- Petit triangle en bas -->
       <div class="absolute -bottom-2 left-1/2 -translate-x-1/2 w-0 h-0 border-l-[8px] border-l-transparent border-r-[8px] border-r-transparent border-t-[8px] border-t-gray-600"></div>
    </div>
    
    <!-- Backdrop invisible pour fermer le popup -->
    <div v-if="showPopup" class="fixed inset-0 z-[55] cursor-default" @click="closePopup"></div>
  </div>
</template>

<style scoped>
.bar {
  height: 20%;
  transition: all 0.2s ease;
}

/* Vertical Slider Magic */
.vertical-slider {
  writing-mode: bt-lr; /* IE */
  -webkit-appearance: slider-vertical; /* WebKit */
  width: 20px;
}

@keyframes eq {
  0% { height: 20%; }
  50% { height: 100%; }
  100% { height: 20%; }
}

.animate-eq-1 { animation: eq 0.6s ease-in-out infinite alternate; }
.animate-eq-2 { animation: eq 0.7s ease-in-out infinite alternate 0.1s; }
.animate-eq-3 { animation: eq 0.5s ease-in-out infinite alternate 0.2s; }
.animate-eq-4 { animation: eq 0.8s ease-in-out infinite alternate 0.15s; }
.animate-eq-5 { animation: eq 0.6s ease-in-out infinite alternate 0.05s; }
</style>
