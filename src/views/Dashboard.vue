<script setup lang="ts">
import { ref } from 'vue';
import { open } from '@tauri-apps/plugin-dialog';
import { useLibraryStore } from '../stores/library';
import { useToastStore } from '../stores/toast';
import { useRouter } from 'vue-router';
import DashboardTitle from '../components/DashboardTitle.vue';
import SettingsModal from '../components/SettingsModal.vue';
import ImportCard from '../components/ImportCard.vue';

const libraryStore = useLibraryStore();
const toastStore = useToastStore();
const router = useRouter();
const isScanning = ref(false);
const isSettingsOpen = ref(false);

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

    <div class="grid grid-cols-1 md:grid-cols-3 gap-8 max-w-7xl w-full z-10">
      
      <!-- Card 0: Settings (Analog Control Panel) -->
      <div 
        @click="isSettingsOpen = true"
        class="group relative h-96 rounded-[2rem] bg-gray-900/40 border border-white/5 p-8 cursor-pointer backdrop-blur-xl transition-all duration-500 hover:bg-gray-900/60 hover:border-amber-500/30 hover:shadow-[0_0_40px_-10px_rgba(245,158,11,0.3)] overflow-hidden"
      >
        <!-- Hover Gradient Overlay -->
        <div class="absolute inset-0 bg-gradient-to-br from-amber-500/10 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-500"></div>
        
        <div class="relative h-full flex flex-col justify-between z-10">
          <div class="space-y-2">
            <h2 class="text-3xl font-bold text-white group-hover:text-amber-400 transition-colors">Paramètres</h2>
            <p class="text-gray-400 group-hover:text-gray-300 transition-colors">Configuration & Préférences</p>
          </div>

          <!-- Animated Control Panel Icon -->
          <div class="self-center relative w-48 h-48 flex items-center justify-center">
             <!-- Frame -->
             <div class="absolute w-40 h-40 bg-gray-800/50 rounded-sm border border-amber-300 shadow-2xl backdrop-blur-sm flex items-center justify-center transform group-hover:scale-105 transition-transform duration-500 overflow-hidden">
               <!-- Gears SVG -->
               <svg viewBox="0 0 100 100" class="w-full h-full opacity-90">
                 <!-- Gear 1 (Bottom Left) -->
                 <g transform="translate(30, 70)">
                   <g class="gear-spin-cw">
                     <!-- Teeth -->
                     <circle r="26" fill="none" stroke="#f59e0b" stroke-width="6" stroke-dasharray="5 3" />
                     <!-- Rim -->
                     <circle r="22" fill="#1f2937" stroke="#f59e0b" stroke-width="1" />
                     <!-- Spokes -->
                     <path d="M-20 0 H20 M0 -20 V20 M-14 -14 L14 14 M-14 14 L14 -14" stroke="#f59e0b" stroke-width="1" />
                     <!-- Hub -->
                     <circle r="5" fill="#f59e0b" />
                   </g>
                 </g>

                 <!-- Gear 2 (Center Right) -->
                 <g transform="translate(70, 50)">
                   <g class="gear-spin-ccw">
                     <!-- Teeth -->
                     <circle r="18" fill="none" stroke="#d97706" stroke-width="5" stroke-dasharray="4 3" />
                     <!-- Rim -->
                     <circle r="15" fill="#1f2937" stroke="#d97706" stroke-width="1" />
                     <!-- Spokes -->
                     <path d="M-13 0 H13 M0 -13 V13" stroke="#d97706" stroke-width="1" />
                     <!-- Hub -->
                     <circle r="4" fill="#d97706" />
                   </g>
                 </g>

                 <!-- Gear 3 (Top Left) -->
                 <g transform="translate(30, 30)">
                   <g class="gear-spin-cw-fast">
                     <!-- Teeth -->
                     <circle r="12" fill="none" stroke="#fbbf24" stroke-width="4" stroke-dasharray="3 2" />
                     <!-- Rim -->
                     <circle r="9" fill="#1f2937" stroke="#fbbf24" stroke-width="1" />
                     <!-- Hub -->
                     <circle r="3" fill="#fbbf24" />
                   </g>
                 </g>
               </svg>
             </div>
          </div>

          <div class="flex items-center text-sm font-medium text-amber-400 opacity-60 group-hover:opacity-100 transition-opacity">
            <span class="mr-2">Configurer</span>
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 group-hover:translate-x-1 transition-transform" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
            </svg>
          </div>
        </div>
      </div>

      <!-- Card 1: Import (Vinyl/Waveform Theme) -->
      <ImportCard @click="openFolder" />

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
          <div class="self-center relative w-48 h-48 flex items-center justify-center">
            <!-- Frame -->
            <div class="w-40 h-40 rounded-sm border border-emerald-300 bg-black/40 flex items-end justify-center gap-2 pb-8 shadow-2xl backdrop-blur-sm transition-transform duration-500 group-hover:scale-105">
              <!-- EQ Bars -->
              <div class="w-4 bg-emerald-500/40 rounded-t-md h-8 animate-eq-1 transition-all"></div>
              <div class="w-4 bg-emerald-500/60 rounded-t-md h-16 animate-eq-2 transition-all"></div>
              <div class="w-4 bg-emerald-500/80 rounded-t-md h-12 animate-eq-3 transition-all"></div>
              <div class="w-4 bg-emerald-500/60 rounded-t-md h-20 animate-eq-4 transition-all"></div>
              <div class="w-4 bg-emerald-500/40 rounded-t-md h-10 animate-eq-5 transition-all"></div>
            </div>
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

    <!-- Settings Modal -->
    <SettingsModal :is-open="isSettingsOpen" @close="isSettingsOpen = false" />
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

/* Gear Animations (Paused by default, running on hover) */
.gear-spin-cw {
  animation: spin-slow 12s linear infinite;
  animation-play-state: paused;
}
.gear-spin-ccw {
  animation: spin-slow 8s linear infinite reverse;
  animation-play-state: paused;
}
.gear-spin-cw-fast {
  animation: spin-slow 6s linear infinite;
  animation-play-state: paused;
}

.group:hover .gear-spin-cw,
.group:hover .gear-spin-ccw,
.group:hover .gear-spin-cw-fast {
  animation-play-state: running;
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
