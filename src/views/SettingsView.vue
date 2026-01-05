<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useExceptionsStore } from '../stores/exceptions'
import { useRouter } from 'vue-router'

const store = useExceptionsStore()
const router = useRouter()

const form = ref({
  original: '',
  corrected: ''
})

onMounted(() => {
  store.chargerExceptions()
})

async function handleSubmit() {
  if (!form.value.original || !form.value.original.trim()) return
  
  await store.ajouterException(
    form.value.original.trim(),
    form.value.corrected ? form.value.corrected.trim() : ''
  )
  
  // Reset form
  form.value.original = ''
  form.value.corrected = ''
}

function goBack() {
  router.back()
}
</script>

<template>
  <div class="container mx-auto p-6 bg-gray-900 min-h-screen text-white">
    <div class="flex items-center justify-between mb-6">
      <h1 class="text-2xl font-bold text-white">Gestion des Exceptions</h1>
      <button @click="goBack" class="text-gray-400 hover:text-white">
        &larr; Retour
      </button>
    </div>

    <!-- Formulaire d'ajout -->
    <div class="bg-gray-800 rounded-lg shadow p-6 mb-8 border border-gray-700 w-1/2 mx-auto">
      <h2 class="text-lg font-semibold mb-4 text-white">Ajouter une exception</h2>
      <form @submit.prevent="handleSubmit" class="flex flex-wrap gap-4 items-end">
        <div class="flex-1 min-w-[200px]">
          <label class="block text-sm font-medium text-gray-400 mb-1">Original (Incorrect)</label>
          <input 
            v-model="form.original"
            type="text" 
            class="w-full rounded-md border-gray-600 bg-gray-700 text-white shadow-sm focus:border-indigo-500 focus:ring-indigo-500 border p-2"
            placeholder="ex: AC/DC"
          >
        </div>
        
        <div class="flex-1 min-w-[200px]">
          <label class="block text-sm font-medium text-gray-400 mb-1">Correction</label>
          <input 
            v-model="form.corrected"
            type="text" 
            class="w-full rounded-md border-gray-600 bg-gray-700 text-white shadow-sm focus:border-indigo-500 focus:ring-indigo-500 border p-2"
            placeholder="ex: AC/DC"
          >
        </div>

        <button 
          type="submit"
          class="bg-indigo-600 text-white px-4 py-2 rounded-md hover:bg-indigo-700 disabled:opacity-50"
          :disabled="store.loading"
        >
          {{ store.loading ? '...' : 'Ajouter' }}
        </button>
      </form>
    </div>

    <!-- Liste des exceptions -->
    <div class="bg-gray-800 rounded-lg shadow overflow-hidden border border-gray-700 w-1/2 mx-auto">
      <table class="min-w-full divide-y divide-gray-700">
        <thead class="bg-gray-900">
          <tr>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">Original</th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">Correction</th>
            <th class="px-6 py-3 text-right text-xs font-medium text-gray-400 uppercase tracking-wider">Actions</th>
          </tr>
        </thead>
        <tbody class="bg-gray-800 divide-y divide-gray-700">
          <tr v-if="store.exceptions.length === 0">
            <td colspan="3" class="px-6 py-4 text-center text-gray-500">
              Aucune exception définie.
            </td>
          </tr>
          <tr v-for="ex in store.exceptions" :key="ex.id">
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-300">{{ ex.original }}</td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-300">{{ ex.corrected }}</td>
            <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
              <button 
                @click="store.supprimerException(ex.id!)"
                class="text-red-400 hover:text-red-300"
              >
                Supprimer
              </button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>
