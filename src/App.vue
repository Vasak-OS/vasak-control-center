<script setup lang="ts">
import { onMounted } from "vue";

import { setWindowPorperties } from "@/commons/window";
import VolumeControl from "@/components/VolumeControl.vue";
import BrightnessControl from "@/components/BrightnessControl.vue";
import NotificationCenter from "@/components/NotificationCenter.vue";
import NetworkControl from "@/components/NetworkControl.vue";
import BluetoothControl from "@/components/BluetoothControl.vue";
import UserProfile from "@/components/UserProfile.vue";
import ThemeToggle from "@/components/ThemeToggle.vue";




onMounted(() => {
  setWindowPorperties();
});
</script>

<template>
  <main class="bg-white/70 dark:bg-black/70 text-black dark:text-white h-screen w-screen rounded-xl flex flex-row flex-wrap justify-between">
    <div class="flex flex-col w-full gap-2 p-2">
      <UserProfile />
      <NotificationCenter />
    </div>
    <div class="flex flex-wrap w-full justify-around items-end p-2">
      <NetworkControl />
      <BluetoothControl />
      <ThemeToggle />
      <div class="flex flex-col gap-2 w-full max-w-xs">
        <BrightnessControl />
        <VolumeControl />
      </div>
    </div>
  </main>
</template>

<style scoped>
</style>