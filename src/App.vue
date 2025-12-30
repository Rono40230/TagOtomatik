<script setup lang="ts">
import ToastContainer from './components/ToastContainer.vue'
</script>

<template>
  <main class="min-h-screen bg-gray-100 text-gray-900">
    <router-view />
    <ToastContainer />
  </main>
</template>
