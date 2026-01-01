<script setup lang="ts">
defineProps<{
  isOpen: boolean
  title: string
  message: string
  confirmText?: string
  cancelText?: string
  type?: 'danger' | 'info' | 'warning'
}>();

const emit = defineEmits<{
  (e: 'confirm'): void
  (e: 'cancel'): void
}>();
</script>

<template>
  <div v-if="isOpen" class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/70 backdrop-blur-sm">
    <div class="bg-gray-800 border border-gray-700 rounded-xl shadow-2xl max-w-md w-full transform transition-all scale-100">
      <!-- Header -->
      <div class="p-6 pb-4">
        <h3 class="text-xl font-bold text-white mb-2">{{ title }}</h3>
        <p class="text-gray-300 text-sm leading-relaxed">
          {{ message }}
        </p>
      </div>

      <!-- Actions -->
      <div class="p-6 pt-2 flex justify-end gap-3">
        <button 
          @click="emit('cancel')"
          class="px-4 py-2 rounded-lg text-sm font-medium text-gray-300 hover:bg-gray-700 hover:text-white transition-colors"
        >
          {{ cancelText || 'Annuler' }}
        </button>
        <button 
          @click="emit('confirm')"
          :class="[
            'px-4 py-2 rounded-lg text-sm font-medium text-white shadow-lg transition-all transform active:scale-95',
            type === 'danger' ? 'bg-red-600 hover:bg-red-500 shadow-red-900/20' : 'bg-blue-600 hover:bg-blue-500 shadow-blue-900/20'
          ]"
        >
          {{ confirmText || 'Confirmer' }}
        </button>
      </div>
    </div>
  </div>
</template>
