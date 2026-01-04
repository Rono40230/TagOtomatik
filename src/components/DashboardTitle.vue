<script setup lang="ts">
// No props needed for now
</script>

<template>
  <div class="z-10 text-center mb-12 relative group-title cursor-default select-none">
    <h1 class="text-7xl md:text-8xl font-black tracking-tighter text-white glitch-title relative inline-block" data-text="TagOtomatik">
      TagOtomatik
    </h1>
    <div class="h-1 w-32 mx-auto my-6 bg-gradient-to-r from-cyan-500 to-emerald-500 rounded-full shadow-[0_0_15px_rgba(6,182,212,0.5)]"></div>
    <div class="overflow-hidden inline-block border-r-2 pr-1 animate-typewriter">
      <p class="text-xl md:text-2xl text-gray-300 font-light tracking-widest uppercase">
        Vos métadonnées, au diapason.
      </p>
    </div>
  </div>
</template>

<style scoped>
/* Glitch Effect */
.group-title:hover .glitch-title {
  animation: glitch-skew 0.3s cubic-bezier(0.25, 0.46, 0.45, 0.94) both infinite;
  color: rgba(255, 255, 255, 0.9);
}

.group-title:hover .glitch-title::before,
.group-title:hover .glitch-title::after {
  content: attr(data-text);
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: #0f172a; /* Match bg-app-bg roughly */
  opacity: 0.7;
}

.group-title:hover .glitch-title::before {
  left: 3px;
  text-shadow: -2px 0 #06b6d4;
  clip-path: polygon(0 0, 100% 0, 100% 35%, 0 35%);
  animation: glitch-anim-1 2s infinite linear alternate-reverse;
}

.group-title:hover .glitch-title::after {
  left: -3px;
  text-shadow: -2px 0 #10b981;
  clip-path: polygon(0 65%, 100% 65%, 100% 100%, 0 100%);
  animation: glitch-anim-2 3s infinite linear alternate-reverse;
}

@keyframes glitch-skew {
  0% { transform: skew(0deg); }
  20% { transform: skew(-1deg); }
  40% { transform: skew(1deg); }
  60% { transform: skew(-0.5deg); }
  80% { transform: skew(0.5deg); }
  100% { transform: skew(0deg); }
}

@keyframes glitch-anim-1 {
  0% { clip-path: inset(20% 0 80% 0); transform: translate(-2px, 1px); }
  20% { clip-path: inset(60% 0 10% 0); transform: translate(2px, -1px); }
  40% { clip-path: inset(40% 0 50% 0); transform: translate(-2px, 2px); }
  60% { clip-path: inset(80% 0 5% 0); transform: translate(2px, -2px); }
  80% { clip-path: inset(10% 0 70% 0); transform: translate(-1px, 1px); }
  100% { clip-path: inset(30% 0 50% 0); transform: translate(1px, -1px); }
}

@keyframes glitch-anim-2 {
  0% { clip-path: inset(10% 0 60% 0); transform: translate(2px, -1px); }
  20% { clip-path: inset(80% 0 5% 0); transform: translate(-2px, 2px); }
  40% { clip-path: inset(30% 0 20% 0); transform: translate(2px, 1px); }
  60% { clip-path: inset(10% 0 80% 0); transform: translate(-1px, -2px); }
  80% { clip-path: inset(50% 0 30% 0); transform: translate(1px, 2px); }
  100% { clip-path: inset(70% 0 10% 0); transform: translate(-2px, -1px); }
}

/* Typewriter Animation */
.animate-typewriter {
  animation: 
    typing 2.5s steps(30, end),
    blink-caret 0.75s step-end 4 forwards;
  white-space: nowrap;
  max-width: 100%;
  border-right-color: transparent;
}

@keyframes typing {
  from { max-width: 0; }
  to { max-width: 100%; }
}

@keyframes blink-caret {
  from, to { border-color: transparent; }
  50% { border-color: #34d399; }
}
</style>
