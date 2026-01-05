<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useExceptionsStore } from '../../stores/exceptions';

const exceptionsStore = useExceptionsStore();

const newOriginal = ref('');
const newCorrected = ref('');

onMounted(() => {
  exceptionsStore.chargerExceptions();
});

async function addException() {
  if (newOriginal.value && newOriginal.value.trim()) {
    await exceptionsStore.ajouterException(
      newOriginal.value.trim(), 
      newCorrected.value ? newCorrected.value.trim() : '', 
      'global'
    );
    newOriginal.value = '';
    newCorrected.value = '';
  }
}
</script>

<template>
  <div class="space-y-8 animate-slide-up">
    <div class="space-y-2">
      <h3 class="text-2xl font-bold text-white">Exceptions de Correction</h3>
      <p class="text-gray-400">Règles spécifiques pour le correcteur de casse.</p>
    </div>

    <!-- Add New -->
    <div class="bg-gray-800/50 p-6 rounded-xl border border-gray-700">
      <h4 class="text-sm font-bold text-gray-300 mb-4 uppercase tracking-wider">Ajouter une règle</h4>
      <div class="grid grid-cols-12 gap-4 items-end">
        <div class="col-span-5">
          <label class="block text-xs text-gray-500 mb-1">Original (Incorrect)</label>
          <input v-model="newOriginal" type="text" class="w-full bg-gray-900 border border-gray-700 rounded px-3 py-2 text-white text-sm" placeholder="ex: ac/dc">
        </div>
        <div class="col-span-5">
          <label class="block text-xs text-gray-500 mb-1">Correction (Voulu)</label>
          <input v-model="newCorrected" type="text" class="w-full bg-gray-900 border border-gray-700 rounded px-3 py-2 text-white text-sm" placeholder="ex: AC/DC">
        </div>
        <div class="col-span-2">
          <button 
            @click="addException" 
            :disabled="!newOriginal || !newOriginal.trim()"
            :class="[
              'w-full rounded py-2 flex items-center justify-center transition-colors',
              (!newOriginal || !newOriginal.trim()) 
                ? 'bg-gray-700 text-gray-500 cursor-not-allowed' 
                : 'bg-amber-600 hover:bg-amber-500 text-white'
            ]"
          >
            +
          </button>
        </div>
      </div>
    </div>

    <!-- List -->
    <div class="bg-gray-900 rounded-xl border border-gray-800 overflow-hidden">
      <table class="w-full text-left text-sm">
        <thead class="bg-gray-950 text-gray-400 uppercase text-xs">
          <tr>
            <th class="px-6 py-3">Original</th>
            <th class="px-6 py-3">Correction</th>
            <th class="px-6 py-3 text-right">Action</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-gray-800">
          <tr v-for="ex in exceptionsStore.exceptions" :key="ex.id" class="hover:bg-gray-800/50">
            <td class="px-6 py-3 text-red-400 font-mono">{{ ex.original }}</td>
            <td class="px-6 py-3 text-green-400 font-mono">
              <span v-if="ex.corrected === ''" class="text-gray-500 italic">(suppression)</span>
              <span v-else>{{ ex.corrected }}</span>
            </td>
            <td class="px-6 py-3 text-right">
              <button 
                @click="ex.id && exceptionsStore.supprimerException(ex.id)"
                class="text-gray-600 hover:text-red-500 transition-colors"
              >
                Supprimer
              </button>
            </td>
          </tr>
          <tr v-if="exceptionsStore.exceptions.length === 0">
            <td colspan="3" class="px-6 py-8 text-center text-gray-600 italic">Aucune exception définie.</td>
          </tr>
        </tbody>
      </table>
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
