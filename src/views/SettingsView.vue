<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useExceptionsStore } from '../stores/exceptions'
import { useRouter } from 'vue-router'

const store = useExceptionsStore()
const router = useRouter()

const form = ref({
  original: '',
  corrected: '',
  category: 'global'
})

onMounted(() => {
  store.chargerExceptions()
})

async function handleSubmit() {
  if (!form.value.original || !form.value.corrected) return
  
  await store.ajouterException(
    form.value.original,
    form.value.corrected,
    form.value.category
  )
  
  // Reset form (keep category)
  form.value.original = ''
  form.value.corrected = ''
}

function goBack() {
  router.back()
}
</script>

<template>
  <div class="container mx-auto p-6">
    <div class="flex items-center justify-between mb-6">
      <h1 class="text-2xl font-bold text-gray-800">Gestion des Exceptions</h1>
      <button @click="goBack" class="text-gray-600 hover:text-gray-900">
        &larr; Retour
      </button>
    </div>

    <!-- Formulaire d'ajout -->
    <div class="bg-white rounded-lg shadow p-6 mb-8">
      <h2 class="text-lg font-semibold mb-4">Ajouter une exception</h2>
      <form @submit.prevent="handleSubmit" class="flex flex-wrap gap-4 items-end">
        <div class="flex-1 min-w-[200px]">
          <label class="block text-sm font-medium text-gray-700 mb-1">Original (Incorrect)</label>
          <input 
            v-model="form.original"
            type="text" 
            class="w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 border p-2"
            placeholder="ex: AC/DC"
          >
        </div>
        
        <div class="flex-1 min-w-[200px]">
          <label class="block text-sm font-medium text-gray-700 mb-1">Correction</label>
          <input 
            v-model="form.corrected"
            type="text" 
            class="w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 border p-2"
            placeholder="ex: AC/DC"
          >
        </div>

        <div class="w-40">
          <label class="block text-sm font-medium text-gray-700 mb-1">Catégorie</label>
          <select 
            v-model="form.category"
            class="w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 border p-2"
          >
            <option value="global">Global</option>
            <option value="artist">Artiste</option>
            <option value="album">Album</option>
          </select>
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
    <div class="bg-white rounded-lg shadow overflow-hidden">
      <table class="min-w-full divide-y divide-gray-200">
        <thead class="bg-gray-50">
          <tr>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Original</th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Correction</th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Catégorie</th>
            <th class="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">Actions</th>
          </tr>
        </thead>
        <tbody class="bg-white divide-y divide-gray-200">
          <tr v-if="store.exceptions.length === 0">
            <td colspan="4" class="px-6 py-4 text-center text-gray-500">
              Aucune exception définie.
            </td>
          </tr>
          <tr v-for="ex in store.exceptions" :key="ex.id">
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">{{ ex.original }}</td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 font-medium text-green-600">{{ ex.corrected }}</td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
              <span class="px-2 inline-flex text-xs leading-5 font-semibold rounded-full"
                :class="{
                  'bg-blue-100 text-blue-800': ex.category === 'global',
                  'bg-purple-100 text-purple-800': ex.category === 'artist',
                  'bg-pink-100 text-pink-800': ex.category === 'album'
                }"
              >
                {{ ex.category }}
              </span>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
              <button 
                @click="ex.id && store.supprimerException(ex.id)"
                class="text-red-600 hover:text-red-900"
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
