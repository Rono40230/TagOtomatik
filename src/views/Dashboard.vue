<script setup lang="ts">
import { ref } from 'vue';
import { open } from '@tauri-apps/plugin-dialog';
import { useLibraryStore } from '../stores/library';
import { useToastStore } from '../stores/toast';
import { useRouter } from 'vue-router';
import DashboardTitle from '../components/DashboardTitle.vue';

const libraryStore = useLibraryStore();
const toastStore = useToastStore();
const router = useRouter();
const isScanning = ref(false);

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
    toastStore.error(`Erreur: ${message}`);
    isScanning.value = false;
  }
}

function goToLibrary() {
  router.push('/library');
}
</script>

<template>
  <div class="min-h-screen flex flex-col items-center justify-center p-8 relative overflow-hidden">
    <!-- Background Ambient Glow -->
    <div class="absolute top-0 left-1/4 w-96 h-96 bg-cyan-500/20 rounded-full blur-[128px] pointer-events-none"></div>
    <div class="absolute bottom-0 right-1/4 w-96 h-96 bg-emerald-500/20 rounded-full blur-[128px] pointer-events-none"></div>

    <!-- Title Section (Glitch Equalizer) -->
    <DashboardTitle />

    <div class="grid grid-cols-1 md:grid-cols-2 gap-8 max-w-5xl w-full z-10">
      
      <!-- Card 1: Import (Vinyl/Waveform Theme) -->
      <div 
        @click="openFolder"
        class="group relative h-96 rounded-[2rem] bg-gray-900/40 border border-white/5 p-8 cursor-pointer backdrop-blur-xl transition-all duration-500 hover:bg-gray-900/60 hover:border-cyan-500/30 hover:shadow-[0_0_40px_-10px_rgba(6,182,212,0.3)] overflow-hidden"
      >
        <!-- Hover Gradient Overlay -->
        <div class="absolute inset-0 bg-gradient-to-br from-cyan-500/10 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-500"></div>
        
        <div class="relative h-full flex flex-col justify-between z-10">
          <div class="space-y-2">
            <h2 class="text-3xl font-bold text-white group-hover:text-cyan-400 transition-colors">Importer</h2>
            <p class="text-gray-400 group-hover:text-gray-300 transition-colors">Scanner un nouveau dossier d'albums</p>
          </div>

          <!-- Animated Vinyl Icon -->
          <div class="self-center relative w-48 h-48 flex items-center justify-center">
            <!-- Sleeve (Decorated - Abstract Fluid) -->
            <div class="absolute w-40 h-40 rounded-sm shadow-2xl z-20 flex flex-col items-center justify-center overflow-hidden bg-black border border-blue-300 group-hover:-translate-x-1 transition-transform duration-500">
               <!-- Fluid Background -->
               <div class="absolute inset-0 bg-slate-900"></div>
               
               <!-- Fluid Shapes SVG -->
               <div class="absolute inset-0 flex items-center justify-center overflow-hidden">
                 <svg viewBox="0 0 100 100" class="w-full h-full opacity-90">
                   <defs>
                     <linearGradient id="fluidGrad1" x1="0%" y1="0%" x2="100%" y2="100%">
                       <stop offset="0%" stop-color="#06b6d4" /> <!-- Cyan -->
                       <stop offset="100%" stop-color="#3b82f6" /> <!-- Blue -->
                     </linearGradient>
                     <linearGradient id="fluidGrad2" x1="100%" y1="0%" x2="0%" y2="100%">
                       <stop offset="0%" stop-color="#ec4899" /> <!-- Pink -->
                       <stop offset="100%" stop-color="#8b5cf6" /> <!-- Purple -->
                     </linearGradient>
                     <filter id="blurFilter">
                       <feGaussianBlur in="SourceGraphic" stdDeviation="3" />
                     </filter>
                   </defs>
                   
                   <!-- Blob 1 -->
                   <path d="M20 30 Q40 10 60 30 T90 50 T60 80 T20 60 T20 30" fill="url(#fluidGrad1)" opacity="0.8" filter="url(#blurFilter)" />
                   
                   <!-- Blob 2 -->
                   <path d="M70 20 Q90 40 70 70 T30 80 T10 50 T40 10 T70 20" fill="url(#fluidGrad2)" opacity="0.7" filter="url(#blurFilter)" style="mix-blend-mode: screen;" />
                   
                   <!-- Fine Lines -->
                   <path d="M10 50 Q30 20 50 50 T90 50" fill="none" stroke="white" stroke-width="0.5" opacity="0.3" />
                   <path d="M10 60 Q40 90 70 60 T90 30" fill="none" stroke="white" stroke-width="0.5" opacity="0.2" />
                 </svg>
               </div>
               
               <!-- Typography -->
               <div class="relative z-10 flex flex-col items-center justify-center">
                 <span class="text-[1.5rem] font-light text-white tracking-[0.2em] mix-blend-overlay">ARTIST</span>
                 <span class="text-[0.7rem] text-cyan-200 tracking-[0.4em] font-medium opacity-80">ALBUM</span>
               </div>
            </div>

            <!-- Vinyl Disc (Slides out) -->
            <div class="absolute w-36 h-36 rounded-full bg-black border-4 border-gray-900 shadow-2xl flex items-center justify-center z-10 transition-transform duration-700 ease-out group-hover:translate-x-20 group-hover:rotate-180">
              <!-- Grooves -->
              <div class="absolute inset-1 rounded-full border border-gray-800/40"></div>
              <div class="absolute inset-3 rounded-full border border-gray-800/40"></div>
              <div class="absolute inset-5 rounded-full border border-gray-800/40"></div>
              <div class="absolute inset-7 rounded-full border border-gray-800/40"></div>
              <!-- Label -->
              <div class="w-12 h-12 rounded-full bg-gradient-to-tr from-cyan-600 to-blue-600 flex items-center justify-center">
                <div class="w-2 h-2 bg-black rounded-full"></div>
              </div>
              <!-- Shine -->
              <div class="absolute inset-0 rounded-full bg-gradient-to-tr from-white/5 to-transparent pointer-events-none"></div>
            </div>
          </div>

          <div class="flex items-center text-sm font-medium text-cyan-400 opacity-60 group-hover:opacity-100 transition-opacity">
            <span class="mr-2">Démarrer le scan</span>
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 group-hover:translate-x-1 transition-transform" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3" />
            </svg>
          </div>
        </div>
      </div>

      <!-- Card 2: Library (Equalizer/Stack Theme) -->
      <div 
        @click="goToLibrary"
        class="group relative h-96 rounded-[2rem] bg-gray-900/40 border border-white/5 p-8 cursor-pointer backdrop-blur-xl transition-all duration-500 hover:bg-gray-900/60 hover:border-emerald-500/30 hover:shadow-[0_0_40px_-10px_rgba(16,185,129,0.3)] overflow-hidden"
      >
        <!-- Hover Gradient Overlay -->
        <div class="absolute inset-0 bg-gradient-to-bl from-emerald-500/10 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-500"></div>

        <div class="relative h-full flex flex-col justify-between z-10">
          <div class="space-y-2">
            <h2 class="text-3xl font-bold text-white group-hover:text-emerald-400 transition-colors">Ma Collection</h2>
            <p class="text-gray-400 group-hover:text-gray-300 transition-colors">Explorer et gérer vos albums</p>
          </div>

          <!-- Animated Equalizer Icon -->
          <div class="self-center relative w-48 h-32 flex items-end justify-center gap-2 pb-4">
            <!-- EQ Bars -->
            <div class="w-4 bg-emerald-500/40 rounded-t-md h-8 animate-eq-1 transition-all"></div>
            <div class="w-4 bg-emerald-500/60 rounded-t-md h-16 animate-eq-2 transition-all"></div>
            <div class="w-4 bg-emerald-500/80 rounded-t-md h-12 animate-eq-3 transition-all"></div>
            <div class="w-4 bg-emerald-500/60 rounded-t-md h-20 animate-eq-4 transition-all"></div>
            <div class="w-4 bg-emerald-500/40 rounded-t-md h-10 animate-eq-5 transition-all"></div>
          </div>

          <div class="flex items-center text-sm font-medium text-emerald-400 opacity-60 group-hover:opacity-100 transition-opacity">
            <span class="mr-2">Ouvrir la bibliothèque</span>
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 group-hover:translate-x-1 transition-transform" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3" />
            </svg>
          </div>
        </div>
      </div>

    </div>
    
    <!-- Loading Overlay -->
    <div v-if="isScanning" class="absolute inset-0 bg-black/80 backdrop-blur-sm z-50 flex flex-col items-center justify-center">
      <div class="w-16 h-16 border-4 border-cyan-500/30 border-t-cyan-500 rounded-full animate-spin mb-4"></div>
      <p class="text-cyan-400 font-medium animate-pulse">Analyse en cours...</p>
    </div>
  </div>
</template>

<style scoped>
/* Custom Animations */
@keyframes spin-slow {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}
.animate-spin-slow {
  animation: spin-slow 8s linear infinite;
}

/* Equalizer Animations */
@keyframes eq-bounce {
  0%, 100% { height: 20%; }
  50% { height: 90%; }
}

.group:hover .animate-eq-1 { animation: eq-bounce 0.8s ease-in-out infinite; }
.group:hover .animate-eq-2 { animation: eq-bounce 1.2s ease-in-out infinite 0.1s; }
.group:hover .animate-eq-3 { animation: eq-bounce 1.0s ease-in-out infinite 0.2s; }
.group:hover .animate-eq-4 { animation: eq-bounce 1.4s ease-in-out infinite 0.1s; }
.group:hover .animate-eq-5 { animation: eq-bounce 0.9s ease-in-out infinite 0.3s; }
</style>
