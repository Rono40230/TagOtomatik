<script setup lang="ts">
import { useToastStore } from '../stores/toast'
import { storeToRefs } from 'pinia'

const store = useToastStore()
const { toasts } = storeToRefs(store)

function remove(id: number) {
  store.remove(id)
}
</script>

<template>
  <div class="fixed bottom-4 right-4 z-50 flex flex-col gap-2 pointer-events-none">
    <transition-group name="toast">
      <div 
        v-for="toast in toasts" 
        :key="toast.id"
        class="pointer-events-auto min-w-[300px] max-w-md p-4 rounded-lg shadow-lg text-white flex items-center justify-between gap-4 transform transition-all duration-300"
        :class="{
          'bg-green-600': toast.type === 'success',
          'bg-red-600': toast.type === 'error',
          'bg-blue-600': toast.type === 'info',
          'bg-yellow-600': toast.type === 'warning'
        }"
      >
        <p class="text-sm font-medium">{{ toast.message }}</p>
        <button @click="remove(toast.id)" class="text-white/80 hover:text-white">
          ✕
        </button>
      </div>
    </transition-group>
  </div>
</template>

<style scoped>
.toast-enter-active,
.toast-leave-active {
  transition: all 0.3s ease;
}
.toast-enter-from {
  opacity: 0;
  transform: translateX(30px);
}
.toast-leave-to {
  opacity: 0;
  transform: translateX(30px);
}
</style>
