<template>
    <button 
      @click="toggleNetwork"
      class="p-2 rounded-xl bg-white/50 dark:bg-black/50 hover:bg-white/70 dark:hover:bg-black/70 transition-colors h-[70px] w-[70px]"
      :class="{ 'opacity-50': !networkState.enabled }"
      :disabled="isLoading"
    >
      <img 
        :src="networkIcon" 
        :alt="networkAlt"
        class="m-auto w-[50px] h-[50px]"
      />
    </button>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { getIcon, getImageType } from '@/common/icons';

interface NetworkState {
  connected: boolean;
  network_type: 'Ethernet' | 'Wifi' | 'Disconnected';
  interface_name: string;
  enabled: boolean;
}

const networkState = ref<NetworkState>({
  connected: false,
  network_type: 'Disconnected',
  interface_name: '',
  enabled: false,
});

const networkIcon = ref('');
const isLoading = ref(false);

const getNetworkIcon = async () => {
  try {
    const iconName = networkState.value.enabled 
      ? networkState.value.network_type === 'Wifi'
        ? 'network-wireless-signal-good-symbolic'
        : networkState.value.network_type === 'Ethernet'
          ? 'network-wired-symbolic'
          : 'network-offline-symbolic'
      : 'network-offline-symbolic';
    
    const icon = await getIcon(iconName);
    networkIcon.value = `data:${getImageType(icon)};base64,${icon}`;
  } catch (error) {
    console.error('Error loading network icon:', error);
  }
};

const networkAlt = computed(() => {
  if (!networkState.value.enabled) return 'Red deshabilitada';
  return networkState.value.connected 
    ? `Conectado a ${networkState.value.interface_name}`
    : 'Desconectado';
});

const updateNetworkState = async () => {
  try {
    networkState.value = await invoke('get_network_state');
    await getNetworkIcon();
  } catch (error) {
    console.error('Error getting network state:', error);
  }
};

const toggleNetwork = async () => {
  if (isLoading.value) return;
  
  isLoading.value = true;
  try {
    await invoke('toggle_network', { 
      enable: !networkState.value.enabled 
    });
    await updateNetworkState();
  } catch (error) {
    console.error('Error toggling network:', error);
  } finally {
    isLoading.value = false;
  }
};

let interval: number;

onMounted(async () => {
  await updateNetworkState();
  interval = window.setInterval(updateNetworkState, 5000);
});

onUnmounted(() => {
  if (interval) {
    clearInterval(interval);
  }
});
</script> 