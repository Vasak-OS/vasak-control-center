<template>
  <button 
    @click="toggleBluetooth"
    class="p-2 rounded-xl bg-white/50 dark:bg-black/50 hover:bg-white/70 dark:hover:bg-black/70 transition-colors h-[70px] w-[70px]"
    :class="{ 'opacity-50': !bluetoothState.enabled }"
    :disabled="isLoading"
  >
    <img 
      :src="bluetoothIcon" 
      :alt="bluetoothAlt"
      class="p-auto w-[50px] h-[50px]"
    />
  </button>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { getIcon, getImageType } from '@/common/icons';

interface BluetoothState {
  enabled: boolean;
  connected_devices: string[];
}

const bluetoothState = ref<BluetoothState>({
  enabled: false,
  connected_devices: [],
});

const bluetoothIcon = ref('');
const isLoading = ref(false);

const getBluetoothIcon = async () => {
  try {
    const iconName = bluetoothState.value.enabled
      ? bluetoothState.value.connected_devices.length > 0
        ? 'bluetooth-active-symbolic'
        : 'bluetooth-symbolic'
      : 'bluetooth-disabled-symbolic';
    
    const icon = await getIcon(iconName);
    bluetoothIcon.value = `data:${getImageType(icon)};base64,${icon}`;
  } catch (error) {
    console.error('Error loading bluetooth icon:', error);
  }
};

const bluetoothAlt = computed(() => {
  if (!bluetoothState.value.enabled) return 'Bluetooth deshabilitado';
  return bluetoothState.value.connected_devices.length > 0
    ? `${bluetoothState.value.connected_devices.length} dispositivos conectados`
    : 'Bluetooth activo';
});

const updateBluetoothState = async () => {
  try {
    bluetoothState.value = await invoke('get_bluetooth_state');
    await getBluetoothIcon();
  } catch (error) {
    console.error('Error getting bluetooth state:', error);
  }
};

const toggleBluetooth = async () => {
  if (isLoading.value) return;
  
  isLoading.value = true;
  try {
    await invoke('toggle_bluetooth', { 
      enable: !bluetoothState.value.enabled 
    });
    await updateBluetoothState();
  } catch (error) {
    console.error('Error toggling bluetooth:', error);
  } finally {
    isLoading.value = false;
  }
};

let interval: number;

onMounted(async () => {
  await updateBluetoothState();
  interval = window.setInterval(updateBluetoothState, 5000);
});

onUnmounted(() => {
  if (interval) {
    clearInterval(interval);
  }
});
</script> 