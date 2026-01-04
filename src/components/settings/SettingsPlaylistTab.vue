<script setup lang="ts">
import { useSettingsStore } from '../../stores/settings';

const settingsStore = useSettingsStore();
</script>

<template>
  <div class="space-y-8 animate-slide-up">
    <div class="space-y-2">
      <h3 class="text-2xl font-bold text-white">Playlists</h3>
      <p class="text-gray-400">Configuration de la génération automatique de playlists.</p>
    </div>

    <div class="space-y-6">
      <div class="bg-gray-800/50 p-6 rounded-xl border border-gray-700">
        <label class="block text-sm font-medium text-gray-300 mb-2">Modèle de nom par défaut</label>
        <div class="flex flex-col gap-3">
          <input 
            v-model="settingsStore.playlist.defaultNamePattern"
            type="text" 
            class="w-full bg-gray-900 border border-gray-700 rounded-lg px-4 py-3 text-white focus:ring-2 focus:ring-purple-500 outline-none font-mono"
            placeholder="{artist} - {album}.m3u"
          >
          <div class="flex flex-wrap gap-2">
            <button 
              v-for="tag in ['{artist}', '{album}', '{year}']" 
              :key="tag"
              @click="settingsStore.playlist.defaultNamePattern += tag"
              class="px-3 py-1 bg-gray-700 hover:bg-gray-600 text-xs text-gray-300 rounded-md border border-gray-600 transition-colors"
            >
              + {{ tag }}
            </button>
          </div>
        </div>
        <p class="text-xs text-gray-500 mt-2">Utilisez les balises ci-dessus pour personnaliser le nom. N'oubliez pas l'extension .m3u</p>
      </div>

      <div class="bg-gray-800/50 p-6 rounded-xl border border-gray-700 space-y-4">
        <div class="space-y-3">
          <label class="flex items-center space-x-3 cursor-pointer">
            <input type="radio" :value="true" v-model="settingsStore.playlist.useRelativePaths" class="form-radio h-5 w-5 text-purple-500 bg-gray-900 border-gray-700 focus:ring-purple-500">
            <span class="text-gray-300">Utiliser des chemins relatifs</span>
          </label>
          
          <label class="flex items-center space-x-3 cursor-pointer">
            <input type="radio" :value="false" v-model="settingsStore.playlist.useRelativePaths" class="form-radio h-5 w-5 text-purple-500 bg-gray-900 border-gray-700 focus:ring-purple-500">
            <span class="text-gray-300">Utiliser des chemins absolus</span>
          </label>
        </div>

        <div class="border-t border-gray-700 pt-4">
          <label class="flex items-center space-x-3 cursor-pointer">
            <input type="checkbox" v-model="settingsStore.playlist.autoCreate" class="form-checkbox h-5 w-5 text-purple-500 rounded bg-gray-900 border-gray-700">
            <span class="text-gray-300">Créer automatiquement après import</span>
          </label>
        </div>
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
