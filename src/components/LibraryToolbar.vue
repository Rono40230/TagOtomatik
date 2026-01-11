<script setup lang="ts">
import { defineProps, defineEmits } from 'vue';

defineProps<{
  albumCount: number;
  selectedCount: number;
  viewMode: 'grid' | 'list';
  sortOption: string;
}>();

const emit = defineEmits<{
  (e: 'go-back'): void;
  (e: 'add-folder'): void;
  (e: 'delete-selected'): void;
  (e: 'edit-selected'): void;
  (e: 'update:viewMode', mode: 'grid' | 'list'): void;
  (e: 'update:sortOption', sort: string): void;
  (e: 'open-settings'): void;
  (e: 'select-all'): void;
  (e: 'clear-selection'): void;
}>();
</script>

<template>
    <header class="bg-gray-800 shadow-sm sticky top-0 z-10 border-b border-gray-700">
      <div class="w-full mx-auto px-4 sm:px-6 lg:px-8 h-16 grid grid-cols-3 items-center">
        <!-- Left: Navigation & Title -->
        <div class="flex items-center justify-start gap-4">
          <button 
            @click="emit('go-back')" 
            class="group p-2 rounded-lg bg-gray-700 border border-gray-600 hover:bg-gray-600"
            title="Retour"
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-gray-200" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
            </svg>
          </button>
          <h1 class="text-xl font-bold text-white">Bibliothèque</h1>
          <span class="bg-blue-900 text-blue-200 text-xs font-medium px-2.5 py-0.5 rounded-full">
            {{ albumCount }} albums
          </span>
        </div>
        
        <!-- Center: Actions for selection or Global -->
        <div class="flex justify-center gap-2">
            <template v-if="selectedCount > 0">
                <button 
                    @click="emit('clear-selection')"
                    class="p-2 bg-gray-600 hover:bg-gray-500 text-white rounded-lg flex items-center justify-center border border-gray-500 shadow-lg"
                    title="Annuler la sélection"
                >
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                    </svg>
                </button>
                 <button 
                    @click="emit('delete-selected')"
                    class="px-3 py-2 bg-red-600/20 hover:bg-red-600/30 text-red-400 hover:text-red-300 border border-red-900/50 rounded-lg text-sm font-medium transition-colors flex items-center gap-1 whitespace-nowrap"
                >
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                    </svg>
                    <span>Supprimer ({{ selectedCount }})</span>
                </button>
                <button 
                    @click="emit('edit-selected')"
                    class="px-3 py-2 bg-cyan-600 hover:bg-cyan-500 text-white border border-cyan-500 rounded-lg text-sm font-medium transition-colors flex items-center gap-1 shadow-[0_0_15px_-3px_rgba(6,182,212,0.5)] whitespace-nowrap"
                >
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                    </svg>
                    <span>Éditer ({{ selectedCount }})</span>
                </button>
            </template>
            <template v-else>
                <button 
                    @click="emit('add-folder')"
                    class="px-3 py-2 bg-green-600 hover:bg-green-500 text-white rounded-lg text-sm font-medium flex items-center gap-1 shadow-lg shadow-green-900/20 whitespace-nowrap"
                >
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M10 3a1 1 0 011 1v5h5a1 1 0 110 2h-5v5a1 1 0 11-2 0v-5H4a1 1 0 110-2h5V4a1 1 0 011-1z" clip-rule="evenodd" />
                    </svg>
                    <span>Ajouter dossier</span>
                </button>

                <button 
                    @click="emit('select-all')"
                    class="px-3 py-2 bg-gray-600 hover:bg-gray-500 text-white rounded-lg text-sm font-medium flex items-center gap-1 shadow-lg shadow-gray-900/20 whitespace-nowrap"
                >
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                    </svg>
                    <span>Tout sélectionner</span>
                </button>

                <!-- View Toggle -->
                <div class="flex bg-gray-700/50 rounded-lg p-0.5 border border-gray-600">
                    <button 
                        @click="emit('update:viewMode', 'grid')"
                        :class="[
                            'p-1.5 rounded-md',
                            viewMode === 'grid' ? 'bg-gray-600 text-white shadow' : 'text-gray-400 hover:text-gray-200'
                        ]"
                        title="Vue Grille"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z" />
                        </svg>
                    </button>
                    <button 
                        @click="emit('update:viewMode', 'list')"
                        :class="[
                            'p-1.5 rounded-md',
                            viewMode === 'list' ? 'bg-gray-600 text-white shadow' : 'text-gray-400 hover:text-gray-200'
                        ]"
                        title="Vue Liste"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
                        </svg>
                    </button>
                </div>

                <!-- Sort Dropdown -->
                <select 
                    :value="sortOption" 
                    @change="emit('update:sortOption', ($event.target as HTMLSelectElement).value)"
                    class="bg-gray-700 text-white text-xs rounded-lg focus:ring-blue-500 focus:border-blue-500 block p-2 border border-gray-600"
                >
                    <option value="default">Défaut</option>
                    <option value="artist">Artiste</option>
                    <option value="year">Année ↓</option>
                    <option value="year_asc">Année ↑</option>
                </select>

                <button
                    @click="emit('open-settings')"
                    class="px-3 py-2 bg-blue-600 hover:bg-blue-500 text-white rounded-lg text-sm font-medium flex items-center gap-1 border border-blue-500 shadow-lg shadow-blue-900/20 whitespace-nowrap"
                >
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                    </svg>
                    <span>Gérer exceptions</span>
                </button>
            </template>
        </div>

        <!-- Right: View Options, Sort & Settings (Empty) -->
        <div class="hidden md:block"></div>
      </div>
    </header>
</template>