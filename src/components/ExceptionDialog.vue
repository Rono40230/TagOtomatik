<script setup lang="ts">
import { ref, watch } from 'vue';

const props = defineProps<{
  isOpen: boolean;
  proposal: {
    original: string;
    corrected: string;
    category: string;
  } | null;
}>();

const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'confirm', data: { original: string; corrected: string; category: string }): void;
}>();

const category = ref('global');

// Reset category when proposal changes
watch(() => props.proposal, (newVal) => {
  if (newVal) {
    // Default to global, but user can switch to specific category
    category.value = 'global';
  }
});

function confirm() {
  if (props.proposal) {
    emit('confirm', {
      original: props.proposal.original,
      corrected: props.proposal.corrected,
      category: category.value
    });
  }
}

function cancel() {
  emit('close');
}
</script>

<template>
  <div v-if="isOpen && proposal" class="fixed inset-0 z-50 flex items-center justify-center bg-black/70 backdrop-blur-sm">
    <div class="bg-gray-800 p-6 rounded-xl shadow-2xl border border-gray-700 w-96 transform transition-all scale-100">
      <h3 class="text-xl font-bold text-white mb-4 flex items-center gap-2">
        <span>✨</span> Créer une règle ?
      </h3>
      
      <div class="bg-gray-900/50 p-4 rounded-lg mb-6 border border-gray-700/50">
        <p class="text-gray-300 text-sm mb-2">Voulez-vous remplacer automatiquement :</p>
        <div class="flex items-center gap-3 text-base">
          <span class="text-red-400 line-through decoration-red-500/50">{{ proposal.original }}</span> 
          <span class="text-gray-500">➜</span> 
          <span class="text-green-400 font-medium">{{ proposal.corrected }}</span>
        </div>
      </div>
      
      <div class="mb-6">
        <label class="block text-xs text-gray-400 uppercase font-bold mb-2">Portée de la règle</label>
        <select 
          v-model="category" 
          class="w-full bg-gray-900 text-white p-2.5 rounded-lg border border-gray-700 focus:border-blue-500 focus:ring-1 focus:ring-blue-500 outline-none transition-colors"
        >
          <option value="global">🌍 Partout (Global)</option>
          <option :value="proposal.category">📂 Seulement dans {{ proposal.category }}</option>
        </select>
      </div>

      <div class="flex justify-end gap-3">
        <button 
          @click="cancel" 
          class="px-4 py-2 text-gray-400 hover:text-white hover:bg-gray-700 rounded-lg transition-colors text-sm font-medium"
        >
          Non, juste cette fois
        </button>
        <button 
          @click="confirm" 
          class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-500 shadow-lg shadow-blue-900/20 transition-all transform hover:scale-105 text-sm font-medium"
        >
          Oui, mémoriser
        </button>
      </div>
    </div>
  </div>
</template>
