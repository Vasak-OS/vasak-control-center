<template>
  <div class="bg-white/50 dark:bg-black/50 rounded-xl flex flex-row items-center gap-2 justify-between w-full h-auto p-4">
    <button class="w-8 h-8 flex items-center justify-center">
      <img 
        :src="currentIcon" 
        :alt="'Brillo: ' + brightnessPercentage + '%'"
        class="w-6 h-6"
      />
    </button>
    <input 
      type="range" 
      :min="brightnessInfo.min" 
      :max="brightnessInfo.max" 
      v-model="currentBrightness"
      @input="updateBrightness"
      class="flex-1"
    />
    <span class="w-12 text-right">{{ brightnessPercentage }}%</span>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { getIcon, getImageType } from '@/common/icons';

interface BrightnessInfo {
  current: number;
  min: number;
  max: number;
}

const brightnessInfo = ref<BrightnessInfo>({
  current: 100,
  min: 0,
  max: 100,
});

const currentBrightness = ref(100);
const currentIcon = ref('');

async function updateIcon() {
  try {
    const iconName = 
      brightnessPercentage.value > 66 ? 'display-brightness-high-symbolic' :
      brightnessPercentage.value > 33 ? 'display-brightness-medium-symbolic' :
      'display-brightness-low-symbolic';

    const icon = await getIcon(iconName);
    currentIcon.value = `data:${getImageType(icon)};base64,${icon}`;
  } catch (error) {
    console.error('Error loading brightness icon:', error);
  }
}

const brightnessPercentage = computed(() => {
  return Math.round(currentBrightness.value);
});

async function getBrightnessInfo() {
  try {
    const info = await invoke('get_brightness');
    brightnessInfo.value = info;
    currentBrightness.value = info.current;
    await updateIcon();
  } catch (error) {
    console.error('Error getting brightness:', error);
  }
}

async function updateBrightness() {
  try {
    await invoke('set_brightness', { 
      brightness: Number(currentBrightness.value)
    });
    await updateIcon();
  } catch (error) {
    console.error('Error setting brightness:', error);
  }
}

onMounted(async () => {
  await getBrightnessInfo();
});
</script> 