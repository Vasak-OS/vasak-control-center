<template>
  <button @click="toggleCurrentNetwork"
    class="p-2 rounded-xl bg-white/50 dark:bg-black/50 hover:bg-white/70 dark:hover:bg-black/70 transition-colors h-[70px] w-[70px]"
    :disabled="isLoading">
    <img :src="networkIconSrc" :alt="networkAlt" class="m-auto w-[50px] h-[50px]" />
  </button>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { getIconSource } from '@vasakgroup/plugin-vicons';
import { listen } from '@tauri-apps/api/event';
import { getCurrentNetworkState, type NetworkInfo, toggleNetwork, WiFiSecurityType } from '@vasakgroup/plugin-network-manager';

let ulisten: Function | null = null;

const networkState = ref<NetworkInfo>(
  {
    name: "Unknown",
    ssid: "Unknown",
    connection_type: "Unknown",
    icon: "network-offline-symbolic",
    ip_address: "0.0.0.0",
    mac_address: "00:00:00:00:00:00",
    signal_strength: 0,
    security_type: WiFiSecurityType.NONE,
    is_connected: false
  }
);

const networkIconSrc = ref('');

const isLoading = ref(false);

const networkAlt = computed(() => {
  return networkState.value.is_connected
    ? `Conectado a ${networkState.value.connection_type} ${networkState.value.ssid}`
    : 'Conectado a red desconocida';
});

const toggleCurrentNetwork = async () => {
  if (isLoading.value) return;

  isLoading.value = true;
  try {
    await toggleNetwork(!networkState.value.is_connected);
  } catch (error) {
    console.error('Error toggling network:', error);
  } finally {
    isLoading.value = false;
  }
};

const getCurrentNetwork = async () => {
  try {
    networkState.value = await getCurrentNetworkState();
    networkIconSrc.value = await getIconSource(networkState.value.icon);
    return networkState;
  } catch (error) {
    networkIconSrc.value = await getIconSource('network-offline-symbolic');
    console.error('Error getting current network state:', error);
    return null;
  }
};

onMounted(async () => {
  await getCurrentNetwork();
  ulisten = await listen<NetworkInfo>('network-changed', async (event) => {
    console.log('Network changed', event.payload);
    networkState.value = event.payload;
    networkIconSrc.value = await getIconSource(event.payload.icon);
  });
});

onUnmounted(() => {
  if (ulisten !== null) {
    ulisten();
  }
});
</script>