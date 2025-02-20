<script setup lang="ts">
import { onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import VolumeControl from "@/components/VolumeControl.vue";
import BrightnessControl from "@/components/BrightnessControl.vue";
import NotificationCenter from "@/components/NotificationCenter.vue";
import NetworkControl from "@/components/NetworkControl.vue";
import BluetoothControl from "@/components/BluetoothControl.vue";

onMounted(() => {
  invoke('initialize_window')
});
</script>

<template>
  <main class="bg-white/70 dark:bg-black/70 text-black dark:text-white h-screen w-screen rounded-xl flex flex-row flex-wrap justify-between">
    <div class="flex w-full justify-center items-start p-2">
      <NotificationCenter />
    </div>
    <div class="flex flex-wrap w-full justify-around items-end gap-2 p-2">
      <NetworkControl />
      <BluetoothControl />
      <div class="flex flex-col gap-2 w-full max-w-xs">
        <BrightnessControl />
        <VolumeControl />
      </div>
    </div>
  </main>
</template>

<style scoped>
</style>