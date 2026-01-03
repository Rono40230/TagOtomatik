<script setup lang="ts">
defineProps<{
  stats: { success: number; total: number; errors: number }
}>()

const emit = defineEmits<{
  (e: 'close'): void
}>()
</script>

<template>
  <div class="flex flex-col items-center justify-center p-6 text-center animate-fade-in w-full">
    <div 
      class="w-16 h-16 rounded-full flex items-center justify-center mb-4 shadow-lg"
      :class="stats.errors === 0 ? 'bg-green-500/20 text-green-500' : 'bg-red-500/20 text-red-500'"
    >
      <svg v-if="stats.errors === 0" xmlns="http://www.w3.org/2000/svg" class="h-8 w-8" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="3">
        <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
      </svg>
      <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-8 w-8" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="3">
        <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
      </svg>
    </div>
    
    <h3 class="text-xl font-bold text-white mb-2">
      {{ stats.errors === 0 ? 'Conversion réussie !' : 'Conversion terminée avec erreurs' }}
    </h3>
    
    <p class="text-gray-400 mb-6 text-sm">
      {{ stats.success }} fichiers convertis sur {{ stats.total }}
      <span v-if="stats.errors > 0" class="text-red-400 block mt-1 font-medium">
        {{ stats.errors }} erreur{{ stats.errors > 1 ? 's' : '' }} détectée{{ stats.errors > 1 ? 's' : '' }}
      </span>
    </p>
    
    <div class="flex gap-3">
      <button 
        @click="emit('close')"
        class="px-4 py-2 bg-gray-700 hover:bg-gray-600 text-white text-sm rounded-lg font-medium transition-colors"
      >
        Rester ici
      </button>
      <button 
        @click="emit('close')"
        class="px-4 py-2 bg-blue-600 hover:bg-blue-500 text-white text-sm rounded-lg font-bold shadow-lg hover:shadow-blue-500/25 transition-all"
      >
        Retour à la bibliothèque
      </button>
    </div>
  </div>
</template>

<style scoped>
.animate-fade-in {
  animation: fadeIn 0.3s ease-out;
}
@keyframes fadeIn {
  from { opacity: 0; transform: scale(0.95); }
  to { opacity: 1; transform: scale(1); }
}
</style>
