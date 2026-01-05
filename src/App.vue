<script setup lang="ts">
import ToastContainer from './components/ToastContainer.vue'
import PlayerBar from './components/PlayerBar.vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { onMounted } from 'vue'
import { useLibraryStore } from './stores/library'

const appWindow = getCurrentWindow()
const libraryStore = useLibraryStore()

onMounted(() => {
  libraryStore.loadLibrary()
})

function startDrag() {
  appWindow.startDragging()
}

function minimize() {
  appWindow.minimize()
}

function toggleMaximize() {
  appWindow.toggleMaximize()
}

function closeApp() {
  appWindow.close()
}
</script>

<template>
  <div class="h-screen w-screen flex flex-col bg-app-bg text-slate-100 overflow-hidden">
    <!-- Custom Window Controls -->
    <div data-tauri-drag-region @dblclick="toggleMaximize" class="fixed top-0 left-0 right-0 h-8 z-40 cursor-move"></div>
    
    <div class="fixed top-2 right-2 z-50 flex gap-2 bg-black/20 backdrop-blur-md rounded-lg p-1 border border-white/5">
      <button @click="minimize" class="p-1.5 hover:bg-white/10 rounded-md transition-colors text-gray-400 hover:text-white">
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="5" y1="12" x2="19" y2="12"></line></svg>
      </button>
      <button @click="toggleMaximize" class="p-1.5 hover:bg-white/10 rounded-md transition-colors text-gray-400 hover:text-white">
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect></svg>
      </button>
      <button @click="closeApp" class="p-1.5 hover:bg-red-500/80 hover:text-white rounded-md transition-colors text-gray-400">
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
      </button>
    </div>

    <!-- Main Content Area -->
    <div class="flex-1 overflow-hidden relative">
      <router-view v-slot="{ Component }">
        <transition name="fade" mode="out-in">
          <component :is="Component" />
        </transition>
      </router-view>
    </div>

    <PlayerBar />
    <ToastContainer />
  </div>
</template>
