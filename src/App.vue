<script setup lang="ts">
import ToastContainer from './components/ToastContainer.vue'
import PlayerBar from './components/PlayerBar.vue'
</script>

<template>
  <div class="h-screen w-screen flex flex-col bg-app-bg text-slate-100 overflow-hidden">
    <!-- Top Bar (Drag Region) - REMOVED -->


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
