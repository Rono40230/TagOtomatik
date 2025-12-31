<script setup lang="ts">
import ToastContainer from './components/ToastContainer.vue'
import PlayerBar from './components/PlayerBar.vue'
</script>

<template>
  <div class="h-screen w-screen flex flex-col bg-app-bg text-slate-100 overflow-hidden">
    <!-- Top Bar (Drag Region) -->
    <div data-tauri-drag-region class="h-8 bg-app-bg/80 backdrop-blur-md flex items-center px-4 select-none border-b border-app-border z-50">
      <span class="text-xs font-medium text-slate-400 tracking-wide">TagOtomatik</span>
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
