<template>
  <button 
    @click="toggleTheme" 
    class="p-2 rounded-xl bg-white/50 dark:bg-black/50 hover:bg-white/70 dark:hover:bg-black/70 transition-colors h-[70px] w-[70px]"
  >
    <img 
      :src="icon" 
      :alt="theme === 'dark' ? 'Cambiar a tema claro' : 'Cambiar a tema oscuro'"
      class="m-auto w-[50px] h-[50px]"
    />
  </button>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import dark from '@/assets/dark.png';
import light from '@/assets/light.png';

const theme = ref('');

const icon = computed(() => {
  return theme.value === 'dark' ? light : dark;
});

const toggleTheme = async () => {
  try {
    await invoke('toggle_system_theme'); // Llama a la función de Rust
    // Cambia el icono según el tema
    theme.value = theme.value === 'dark' 
      ? 'light' 
      : 'dark';
  } catch (error) {
    console.error('Error toggling system theme:', error);
  }
};

onMounted(async () => {
  if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
    theme.value = 'dark';
  } else {
    theme.value = 'light';
  }
});

</script>

<style scoped>
/* Estilos adicionales si es necesario */
</style> 