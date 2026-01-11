<script setup lang="ts">
import { useLibraryStore } from '../stores/library';
import { useToastStore } from '../stores/toast';
import AlbumCard from '../components/AlbumCard.vue';
import AlbumListRow from '../components/AlbumListRow.vue';
import LibraryToolbar from '../components/LibraryToolbar.vue';
import ImportCard from '../components/ImportCard.vue';
import ConfirmationModal from '../components/ConfirmationModal.vue';
import { useRouter } from 'vue-router';
import { ref, onActivated, computed } from 'vue';
import { open } from '@tauri-apps/plugin-dialog';

const libraryStore = useLibraryStore();
const toast = useToastStore();
const router = useRouter();
const selectedAlbumIds = ref<Set<string>>(new Set());
const sortOption = ref('default');
const viewMode = ref<'grid' | 'list'>('grid'); // Default to grid

const sortedAlbums = computed(() => {
    const list = [...libraryStore.albums];
    switch (sortOption.value) {
        case 'artist':
            return list.sort((a, b) => a.artist.localeCompare(b.artist) || a.title.localeCompare(b.title));
        case 'year':
            // Plus récent en premier
            return list.sort((a, b) => {
                const yA = a.yearMax || a.year || 0;
                const yB = b.yearMax || b.year || 0;
                return yB - yA; // Descending
            });
        case 'year_asc':
            // Plus vieux en premier
            return list.sort((a, b) => {
                const yA = a.yearMin || a.year || 0;
                const yB = b.yearMin || b.year || 0;
                return yA - yB; 
            });
        default:
            // Par défaut on trie par ID (souvent lié au path) pour garder une cohérence
            // ou on ne fait rien si la liste est déjà ordonnée par le backend
            return list;
    }
});

// Refresh library when view is activated (returned to from home/other views)
// This ensures moved/deleted files are cleaned up from the view
onActivated(() => {
  libraryStore.loadLibrary();
});

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

function selectAll() {
  sortedAlbums.value.forEach(a => selectedAlbumIds.value.add(a.id));
}

function clearSelection() {
  selectedAlbumIds.value.clear();
}

function openSelected() {
  if (selectedAlbumIds.value.size === 0) return;
  const ids = Array.from(selectedAlbumIds.value).join(',');
  router.push({ name: 'MultiAlbumEdit', query: { ids } });
}
</script>

<template>
  <div class="h-full overflow-y-auto bg-gray-900 text-white scrollbar-thin scrollbar-thumb-gray-700 scrollbar-track-transparent">
    <!-- Header -->
    <LibraryToolbar
      :album-count="libraryStore.albums.length"
      :selected-count="selectedAlbumIds.size"
      :view-mode="viewMode"
      :sort-option="sortOption"
      @go-back="goBack"
      @add-folder="addFolder"
      @delete-selected="deleteSelected"
      @edit-selected="openSelected"
      @select-all="selectAll"
      @clear-selection="clearSelection"
      @update:viewMode="viewMode = $event"
      @update:sortOption="sortOption = $event"
      @open-settings="router.push('/settings')"
    />

    <!-- Grid -->
    <main class="w-full mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <div v-if="libraryStore.albums.length === 0" class="flex justify-center py-20">
        <div class="w-full max-w-md">
          <ImportCard @click="addFolder" />
        </div>
      </div>
      
      <div v-else>
         <div v-if="viewMode === 'grid'" class="grid grid-cols-[repeat(auto-fill,minmax(300px,1fr))] gap-6">
            <AlbumCard 
              v-for="album in sortedAlbums"  
              :key="album.id" 
              :album="album"
              :selected="selectedAlbumIds.has(album.id)"
              @toggle-selection="toggleSelection"
              @delete="deleteAlbum"
              @refresh="libraryStore.refreshAlbum"
            />
         </div>
         <div v-else class="flex flex-col gap-2">
            <AlbumListRow
              v-for="album in sortedAlbums"
              :key="album.id"
              :album="album"
              :selected="selectedAlbumIds.has(album.id)"
              @toggle-selection="toggleSelection"
              @delete="deleteAlbum"
              @refresh="libraryStore.refreshAlbum"
            />
         </div>
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
