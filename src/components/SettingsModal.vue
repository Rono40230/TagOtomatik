<script setup lang="ts">
import { ref } from 'vue';
import { MusicalNoteIcon, ListBulletIcon, ExclamationTriangleIcon } from '@heroicons/vue/24/outline';
import SettingsAudioTab from './settings/SettingsAudioTab.vue';
import SettingsPlaylistTab from './settings/SettingsPlaylistTab.vue';
import SettingsExceptionsTab from './settings/SettingsExceptionsTab.vue';

defineProps<{
  isOpen: boolean;
}>();

defineEmits<{
  (e: 'close'): void;
}>();

const activeTab = ref<'audio' | 'playlist' | 'exceptions'>('audio');
</script>

<template>
  <div v-if="isOpen" class="fixed inset-0 z-50 flex items-center justify-center">
    <!-- Backdrop -->
    <div class="absolute inset-0 bg-black/80 backdrop-blur-sm" @click="$emit('close')"></div>

    <!-- Modal Window -->
    <div class="relative w-full max-w-4xl h-[80vh] bg-gray-900 border border-gray-700 rounded-2xl shadow-2xl flex overflow-hidden animate-fade-in">
      
      <!-- Sidebar -->
      <div class="w-64 bg-gray-950 border-r border-gray-800 flex flex-col">
        <div class="p-6 border-b border-gray-800">
          <h2 class="text-xl font-bold text-white tracking-wider">PARAMÈTRES</h2>
          <p class="text-xs text-gray-500 mt-1">CONFIGURATION GLOBALE</p>
        </div>
        
        <nav class="flex-1 p-4 space-y-2">
          <button 
            @click="activeTab = 'audio'"
            :class="[
              'w-full flex items-center px-4 py-3 rounded-lg text-sm font-medium transition-all duration-200',
              activeTab === 'audio' ? 'bg-cyan-500/10 text-cyan-400 border border-cyan-500/20' : 'text-gray-400 hover:bg-gray-900 hover:text-white'
            ]"
          >
            <MusicalNoteIcon class="w-5 h-5 mr-3" />
            Audio & Conversion
          </button>

          <button 
            @click="activeTab = 'playlist'"
            :class="[
              'w-full flex items-center px-4 py-3 rounded-lg text-sm font-medium transition-all duration-200',
              activeTab === 'playlist' ? 'bg-purple-500/10 text-purple-400 border border-purple-500/20' : 'text-gray-400 hover:bg-gray-900 hover:text-white'
            ]"
          >
            <ListBulletIcon class="w-5 h-5 mr-3" />
            Playlists
          </button>

          <button 
            @click="activeTab = 'exceptions'"
            :class="[
              'w-full flex items-center px-4 py-3 rounded-lg text-sm font-medium transition-all duration-200',
              activeTab === 'exceptions' ? 'bg-amber-500/10 text-amber-400 border border-amber-500/20' : 'text-gray-400 hover:bg-gray-900 hover:text-white'
            ]"
          >
            <ExclamationTriangleIcon class="w-5 h-5 mr-3" />
            Exceptions
          </button>
        </nav>

        <div class="p-4 border-t border-gray-800">
          <button @click="$emit('close')" class="w-full py-2 px-4 bg-gray-800 hover:bg-gray-700 text-white rounded-lg transition-colors text-sm">
            Fermer
          </button>
        </div>
      </div>

      <!-- Content Area -->
      <div class="flex-1 bg-gray-900 overflow-y-auto p-8">
        <SettingsAudioTab v-if="activeTab === 'audio'" />
        <SettingsPlaylistTab v-if="activeTab === 'playlist'" />
        <SettingsExceptionsTab v-if="activeTab === 'exceptions'" />
      </div>
    </div>
  </div>
</template>

<style scoped>
.animate-fade-in {
  animation: fadeIn 0.2s ease-out;
}

@keyframes fadeIn {
  from { opacity: 0; transform: scale(0.95); }
  to { opacity: 1; transform: scale(1); }
}
</style>
