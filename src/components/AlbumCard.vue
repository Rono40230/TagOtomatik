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
    case 'Clean': return 'bg-green-900 text-green-200';
    case 'Dirty': return 'bg-yellow-900 text-yellow-200';
    case 'Incomplete': return 'bg-red-900 text-red-200';
    case 'Processing': return 'bg-blue-900 text-blue-200';
    default: return 'bg-gray-700 text-gray-300';
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
    class="bg-gray-800 rounded-lg shadow-md overflow-hidden hover:shadow-lg transition-shadow cursor-pointer border border-gray-700"
  >
    <div class="aspect-square bg-gray-700 flex items-center justify-center relative">
      <img v-if="coverUrl" :src="coverUrl" alt="Cover" class="w-full h-full object-cover" />
      <div v-else class="flex flex-col items-center justify-center text-gray-400 h-full w-full">
        <span class="text-sm font-medium">Cover absente</span>
      </div>
      
      <div class="absolute top-2 right-2">
        <span :class="['text-xs font-bold px-2 py-1 rounded-full', statusColor]">
          {{ album.status }}
        </span>
      </div>
    </div>
    
    <div class="p-4">
      <h3 class="font-bold text-lg truncate text-white" :title="album.title">{{ album.title || 'Sans titre' }}</h3>
      <p class="text-gray-300 truncate" :title="album.artist">{{ album.artist || 'Artiste inconnu' }}</p>
      
      <div class="mt-3 flex justify-between items-center text-xs text-gray-400">
        <span>{{ album.year || '????' }}</span>
        <span class="bg-gray-700 px-2 py-0.5 rounded">{{ formatBadge }}</span>
        <span>{{ album.tracks.length }} pistes</span>
      </div>
    </div>
  </div>
</template>
