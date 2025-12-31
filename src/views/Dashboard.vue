<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';
import { useLibraryStore } from '../stores/library';
import { useToastStore } from '../stores/toast';
import { useRouter } from 'vue-router';

const libraryStore = useLibraryStore();
const toastStore = useToastStore();
const router = useRouter();
const isScanning = ref(false);
const history = ref<string[]>([]);

onMounted(async () => {
  try {
    history.value = await invoke('get_scan_history');
  } catch (e) {
    toastStore.error(String(e));
  }
});

async function openHistory(path: string) {
  isScanning.value = true;
  await libraryStore.scanDirectory(path);
  isScanning.value = false;
  
  if (libraryStore.albums.length > 0) {
    router.push('/library');
  }
}

async function openFolder() {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
    });

    if (selected && typeof selected === 'string') {
      isScanning.value = true;
      await libraryStore.scanDirectory(selected);
      isScanning.value = false;
      
      if (libraryStore.albums.length > 0) {
        router.push('/library');
      }
    }
  } catch (err) {
    const message = err instanceof Error ? err.message : String(err);
    toastStore.error(message);
    isScanning.value = false;
  }
}
</script>

<template>
  <div class="h-full flex flex-col items-center justify-center p-8 relative overflow-hidden">
    <!-- Background Gradients -->
    <div class="absolute top-0 left-0 w-full h-full overflow-hidden -z-10 pointer-events-none">
      <div class="absolute top-[-10%] left-[-10%] w-[40%] h-[40%] bg-primary/20 rounded-full blur-[120px]"></div>
      <div class="absolute bottom-[-10%] right-[-10%] w-[40%] h-[40%] bg-purple-600/20 rounded-full blur-[120px]"></div>
    </div>

    <div class="max-w-2xl w-full flex flex-col items-center text-center z-10">
      <!-- Logo / Icon -->
      <div class="mb-8 p-6 bg-app-surface/50 backdrop-blur-xl rounded-3xl border border-app-border shadow-2xl ring-1 ring-white/10">
        <span class="text-6xl filter drop-shadow-lg">🎵</span>
      </div>

      <h1 class="mb-4 text-5xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-white to-slate-400 tracking-tight">
        TagOtomatik
      </h1>
      
      <p class="mb-10 text-lg text-slate-400 max-w-md leading-relaxed">
        L'outil ultime pour nettoyer, organiser et taguer votre bibliothèque musicale locale.
      </p>
      
      <!-- Main Action -->
      <button 
        @click="openFolder" 
        :disabled="isScanning || libraryStore.isLoading"
        class="group relative w-full max-w-sm py-4 px-8 bg-primary hover:bg-primary-hover text-white rounded-xl font-semibold text-lg transition-all duration-300 shadow-lg hover:shadow-primary/25 hover:-translate-y-0.5 disabled:opacity-70 disabled:cursor-not-allowed disabled:hover:translate-y-0 overflow-hidden"
      >
        <div class="absolute inset-0 bg-gradient-to-r from-transparent via-white/10 to-transparent translate-x-[-100%] group-hover:translate-x-[100%] transition-transform duration-1000"></div>
        <div class="flex items-center justify-center gap-3">
          <span v-if="isScanning || libraryStore.isLoading" class="animate-spin">⏳</span>
          <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 19a2 2 0 01-2-2V7a2 2 0 012-2h4l2 2h4a2 2 0 012 2v1M5 19h14a2 2 0 002-2v-5a2 2 0 00-2-2H9a2 2 0 00-2 2v5a2 2 0 01-2 2z" />
          </svg>
          <span>{{ isScanning || libraryStore.isLoading ? 'Analyse en cours...' : 'Ouvrir un dossier' }}</span>
        </div>
      </button>

      <!-- Secondary Actions -->
      <div class="mt-8 flex gap-4">
        <router-link 
          to="/converter"
          class="px-4 py-2 text-slate-400 hover:text-white text-sm font-medium transition-colors flex items-center gap-2 hover:bg-white/5 rounded-lg"
        >
          <span class="text-lg">🔄</span>
          Convertisseur
        </router-link>

        <router-link 
          to="/settings"
          class="px-4 py-2 text-slate-400 hover:text-white text-sm font-medium transition-colors flex items-center gap-2 hover:bg-white/5 rounded-lg"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
          </svg>
          Exceptions
        </router-link>
      </div>
      
      <div v-if="history.length > 0" class="mt-12 w-full max-w-md animate-fade-in-up">
        <h3 class="text-xs font-bold text-slate-500 uppercase tracking-widest mb-4">Récemment ouverts</h3>
        <div class="bg-white/5 rounded-xl overflow-hidden border border-white/10 backdrop-blur-sm">
          <button 
            v-for="path in history" 
            :key="path"
            @click="openHistory(path)"
            class="w-full text-left px-4 py-3 text-sm text-slate-300 hover:bg-white/10 hover:text-white transition-all border-b border-white/5 last:border-0 truncate flex items-center gap-3 group"
            :title="path"
          >
            <span class="opacity-50 group-hover:opacity-100 transition-opacity">📁</span>
            <span class="truncate flex-1">{{ path }}</span>
            <span class="opacity-0 group-hover:opacity-100 text-xs text-primary transition-opacity">Ouvrir →</span>
          </button>
        </div>
      </div>
      
      <div v-if="libraryStore.error" class="mt-6 p-4 bg-red-500/10 border border-red-500/20 text-red-400 rounded-xl text-sm backdrop-blur-sm animate-pulse">
        {{ libraryStore.error }}
      </div>
    </div>
    
    <!-- Footer / Recent -->
    <div class="absolute bottom-8 w-full max-w-2xl px-4">
      <div v-if="history.length > 0" class="text-center">
        <p class="text-xs text-slate-600 font-medium tracking-wider uppercase mb-3">Récemment ouverts</p>
        <div class="flex flex-wrap justify-center gap-2">
          <button 
            v-for="path in history.slice(0, 3)" 
            :key="path"
            @click="openHistory(path)"
            class="px-3 py-1 bg-white/10 hover:bg-white/20 text-slate-400 hover:text-white text-xs rounded-full transition-colors truncate max-w-[200px]"
            :title="path"
          >
            {{ path.split('/').pop() }}
          </button>
        </div>
      </div>
      <div v-else class="text-center text-xs text-slate-600 font-medium tracking-wider uppercase">
        Prêt à scanner
      </div>
    </div>
  </div>
</template>
