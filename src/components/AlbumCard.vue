<script setup lang="ts">
import type { Album } from '../types';
import { computed, ref, onMounted, watch } from 'vue';
import { useRouter } from 'vue-router';
import { readFile } from '@tauri-apps/plugin-fs';

const props = defineProps<{
  album: Album
}>();

const router = useRouter();
const coverUrl = ref<string | null>(null);

async function loadCover() {
  if (!props.album.cover_path) {
    coverUrl.value = null;
    return;
  }

  try {
    const contents = await readFile(props.album.cover_path);
    const blob = new Blob([contents]);
    coverUrl.value = URL.createObjectURL(blob);
  } catch {
    coverUrl.value = null;
  }
}

onMounted(loadCover);
watch(() => props.album.cover_path, loadCover);

const statusColor = computed(() => {
  switch (props.album.status) {
    case 'Clean': return 'bg-green-100 text-green-800';
    case 'Dirty': return 'bg-yellow-100 text-yellow-800';
    case 'Incomplete': return 'bg-red-100 text-red-800';
    case 'Processing': return 'bg-blue-100 text-blue-800';
    default: return 'bg-gray-100 text-gray-800';
  }
});

const formatBadge = computed(() => {
  const formats = new Set(props.album.tracks.map(t => t.format.toUpperCase()));
  return Array.from(formats).join('/');
});

function openDetail() {
  router.push(`/album/${props.album.id}`);
}
</script>

<template>
  <div 
    @click="openDetail"
    class="bg-white rounded-lg shadow-md overflow-hidden hover:shadow-lg transition-shadow cursor-pointer border border-gray-200"
  >
    <div class="aspect-square bg-gray-200 flex items-center justify-center relative">
      <img v-if="coverUrl" :src="coverUrl" alt="Cover" class="w-full h-full object-cover" />
      <div v-else class="flex flex-col items-center justify-center text-gray-500 h-full w-full">
        <span class="text-sm font-medium">Cover absente</span>
      </div>
      
      <div class="absolute top-2 right-2">
        <span :class="['text-xs font-bold px-2 py-1 rounded-full', statusColor]">
          {{ album.status }}
        </span>
      </div>
    </div>
    
    <div class="p-4">
      <h3 class="font-bold text-lg truncate text-black" :title="album.title">{{ album.title || 'Sans titre' }}</h3>
      <p class="text-gray-600 truncate" :title="album.artist">{{ album.artist || 'Artiste inconnu' }}</p>
      
      <div class="mt-3 flex justify-between items-center text-xs text-gray-500">
        <span>{{ album.year || '????' }}</span>
        <span class="bg-gray-100 px-2 py-0.5 rounded">{{ formatBadge }}</span>
        <span>{{ album.tracks.length }} pistes</span>
      </div>
    </div>
  </div>
</template>
