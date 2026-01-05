<script setup lang="ts">
import { useLibraryStore } from '../stores/library';
import { useToastStore } from '../stores/toast';
import AlbumCard from '../components/AlbumCard.vue';
import ImportCard from '../components/ImportCard.vue';
import ConfirmationModal from '../components/ConfirmationModal.vue';
import { useRouter } from 'vue-router';
import { ref } from 'vue';
import { open } from '@tauri-apps/plugin-dialog';

const libraryStore = useLibraryStore();
const toast = useToastStore();
const router = useRouter();
const selectedAlbumIds = ref<Set<string>>(new Set());

// Modal State
const isModalOpen = ref(false);
const modalTitle = ref('');
const modalMessage = ref('');
const modalAction = ref<() => void>(() => {});

function goBack() {
  router.push('/');
}

async function addFolder() {
  try {
    const selected = await open({
      directory: true,
      multiple: true,
    });

    if (selected) {
      const paths = Array.isArray(selected) ? selected : [selected];
      for (const path of paths) {
         await libraryStore.scanDirectory(path);
      }
    }
  } catch (err) {
    toast.error(`Erreur lors de l'ajout du dossier: ${err}`);
  }
}

function toggleSelection(id: string, selected: boolean) {
  if (selected) {
    selectedAlbumIds.value.add(id);
  } else {
    selectedAlbumIds.value.delete(id);
  }
}

function openDeleteModal(action: () => void, title: string, message: string) {
  modalAction.value = action;
  modalTitle.value = title;
  modalMessage.value = message;
  isModalOpen.value = true;
}

function confirmAction() {
  modalAction.value();
  isModalOpen.value = false;
}

function deleteAlbum(id: string) {
  openDeleteModal(
    () => {
      libraryStore.removeAlbum(id);
      selectedAlbumIds.value.delete(id);
    },
    'Supprimer l\'album',
    'Voulez-vous vraiment retirer cet album de la bibliothèque ? Cette action ne supprime pas les fichiers du disque.'
  );
}

function deleteSelected() {
  if (selectedAlbumIds.value.size === 0) return;
  openDeleteModal(
    () => {
      const ids = Array.from(selectedAlbumIds.value);
      ids.forEach(id => libraryStore.removeAlbum(id));
      selectedAlbumIds.value.clear();
    },
    'Supprimer la sélection',
    `Voulez-vous retirer ${selectedAlbumIds.value.size} albums de la bibliothèque ?`
  );
}

function openSelected() {
  if (selectedAlbumIds.value.size === 0) return;
  const ids = Array.from(selectedAlbumIds.value).join(',');
  router.push({ name: 'MultiAlbumEdit', query: { ids } });
}
</script>

<template>
  <div class="min-h-screen bg-gray-900 text-white">
    <!-- Header -->
    <header class="bg-gray-800 shadow-sm sticky top-0 z-10 border-b border-gray-700">
      <div class="w-full mx-auto px-4 sm:px-6 lg:px-8 h-16 grid grid-cols-3 items-center">
        <div class="flex items-center justify-start gap-4">
          <button 
            @click="goBack" 
            class="group p-2 rounded-lg bg-gray-800/50 border border-gray-700 hover:border-cyan-500/50 hover:bg-gray-800 hover:shadow-[0_0_15px_-3px_rgba(6,182,212,0.4)] transition-all duration-300"
            title="Retour"
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-gray-400 group-hover:text-cyan-400 transition-colors" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
            </svg>
          </button>
          <h1 class="text-xl font-bold text-white">Bibliothèque</h1>
          <span class="bg-blue-900 text-blue-200 text-xs font-medium px-2.5 py-0.5 rounded-full">
            {{ libraryStore.albums.length }} albums
          </span>
        </div>
        
        <div class="flex items-center justify-center gap-2">
          <template v-if="selectedAlbumIds.size > 0">
            <button 
              @click="openSelected"
              class="px-4 py-2 bg-blue-900/50 border border-blue-700 text-blue-200 rounded-lg hover:bg-blue-900 text-sm font-medium transition-colors flex items-center gap-2"
            >
              ✏️ Ouvrir ({{ selectedAlbumIds.size }})
            </button>
            <button 
              @click="deleteSelected"
              class="px-4 py-2 bg-red-900/50 border border-red-700 text-red-200 rounded-lg hover:bg-red-900 text-sm font-medium transition-colors flex items-center gap-2"
            >
              🗑️ Supprimer ({{ selectedAlbumIds.size }})
            </button>
          </template>
          <template v-else>
            <button 
              @click="addFolder"
              class="px-4 py-2 bg-green-600 hover:bg-green-500 text-white rounded-lg text-sm font-medium transition-colors flex items-center gap-2 border border-green-500 shadow-lg shadow-green-900/20"
            >
              <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
              </svg>
              Ajouter des albums
            </button>

            <router-link 
              to="/settings"
              class="px-4 py-2 bg-blue-600 hover:bg-blue-500 text-white rounded-lg text-sm font-medium transition-colors flex items-center gap-2 border border-blue-500 shadow-lg shadow-blue-900/20"
            >
              <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
              </svg>
              Gérer les exceptions
            </router-link>
          </template>
        </div>

        <div class="flex items-center justify-end gap-2">
          <!-- Actions globales (Filtrer, Trier...) -->
        </div>
      </div>
    </header>

    <!-- Grid -->
    <main class="w-full mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <div v-if="libraryStore.albums.length === 0" class="flex justify-center py-20">
        <div class="w-full max-w-md">
          <ImportCard @click="addFolder" />
        </div>
      </div>
      
      <div v-else class="grid grid-cols-[repeat(auto-fill,minmax(300px,1fr))] gap-6">
        <AlbumCard 
          v-for="album in libraryStore.albums"  
          :key="album.id" 
          :album="album"
          :selected="selectedAlbumIds.has(album.id)"
          @toggle-selection="toggleSelection"
          @delete="deleteAlbum"
          @refresh="libraryStore.refreshAlbum"
        />
      </div>
    </main>

    <ConfirmationModal
      :is-open="isModalOpen"
      :title="modalTitle"
      :message="modalMessage"
      confirm-text="Supprimer"
      type="danger"
      @confirm="confirmAction"
      @cancel="isModalOpen = false"
    />
  </div>
</template>
