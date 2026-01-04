<script setup lang="ts">
import { useSettingsStore } from '../../stores/settings';

const settingsStore = useSettingsStore();
</script>

<template>
  <div class="space-y-8 animate-slide-up">
    <div class="space-y-2">
      <h3 class="text-2xl font-bold text-white">Conversion Audio</h3>
      <p class="text-gray-400">Paramètres par défaut pour la conversion de format.</p>
    </div>

    <div class="grid grid-cols-1 gap-6">
      <!-- Format -->
      <div class="bg-gray-800/50 p-6 rounded-xl border border-gray-700">
        <label class="block text-sm font-medium text-gray-300 mb-4">Format de Sortie</label>
        <div class="flex space-x-4">
          <button 
            v-for="fmt in ['mp3', 'flac']" 
            :key="fmt"
            @click="settingsStore.conversion.format = fmt as any"
            :class="[
              'px-6 py-3 rounded-lg border transition-all uppercase font-bold',
              settingsStore.conversion.format === fmt 
                ? 'bg-cyan-500 text-white border-cyan-400 shadow-[0_0_15px_rgba(6,182,212,0.3)]' 
                : 'bg-gray-900 text-gray-500 border-gray-700 hover:border-gray-500'
            ]"
          >
            {{ fmt }}
          </button>
        </div>
      </div>

      <!-- Bitrate -->
      <div class="bg-gray-800/50 p-6 rounded-xl border border-gray-700" v-if="settingsStore.conversion.format === 'mp3'">
        <label class="block text-sm font-medium text-gray-300 mb-4">Qualité / Bitrate</label>
        <select 
          v-model="settingsStore.conversion.bitrate"
          class="w-full bg-gray-900 border border-gray-700 rounded-lg px-4 py-3 text-white focus:ring-2 focus:ring-cyan-500 focus:border-transparent outline-none"
        >
          <option value="320k">320 kbps (CBR) - Haute Qualité</option>
          <option value="V0">V0 (VBR) - Meilleur compromis</option>
          <option value="192k">192 kbps (CBR)</option>
          <option value="128k">128 kbps (CBR)</option>
        </select>
      </div>

      <!-- Options -->
      <div class="bg-gray-800/50 p-6 rounded-xl border border-gray-700">
        <label class="flex items-center space-x-3 cursor-pointer">
          <input type="checkbox" v-model="settingsStore.conversion.normalize" class="form-checkbox h-5 w-5 text-cyan-500 rounded bg-gray-900 border-gray-700">
          <span class="text-gray-300">Normaliser le volume (ReplayGain)</span>
        </label>
      </div>
    </div>
  </div>
</template>

<style scoped>
.animate-slide-up {
  animation: slideUp 0.3s ease-out;
}

@keyframes slideUp {
  from { opacity: 0; transform: translateY(10px); }
  to { opacity: 1; transform: translateY(0); }
}
</style>
