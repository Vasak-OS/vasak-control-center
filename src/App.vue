<script setup lang="ts">
import { onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

onMounted(() => {
  invoke('initialize_window')
  console.info("greetMsg");
});
</script>

<template>
  <main class="bg-white/70 dark:bg-black/70 text-black dark:text-white h-screen w-screen rounded-xl flex flex-row flex-wrap justify-between">
    <div class="flex w-full justify-center items-start p-2">
      <h1>Vasak Control Center</h1>
    </div>
    <div class="flex w-full justify-center items-end p-2">
      <h1>Vasak Control Center</h1>
    </div>
  </main>
</template>

<style scoped>
</style>