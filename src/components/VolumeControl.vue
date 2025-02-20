<template>
  <div class="flex flex-row items-center gap-2 justify-between w-full h-auto p-4">
    <button @click="toggleMute">
      {{ volumeInfo.is_muted ? 'Unmute' : 'Mute' }}
    </button>
    <input 
      type="range" 
      :min="volumeInfo.min" 
      :max="volumeInfo.max" 
      v-model="currentVolume"
      @input="updateVolume"
    />
    <span>{{ volumePercentage }}%</span>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const volumeInfo = ref({
  current: 0,
  min: 0,
  max: 100,
  is_muted: false
})

const currentVolume = ref(0)

const volumePercentage = computed(() => {
  const range = volumeInfo.value.max - volumeInfo.value.min
  const current = currentVolume.value - volumeInfo.value.min
  return Math.round((current / range) * 100)
})

async function getVolumeInfo() {
  try {
    const info = await invoke('get_volume')
    volumeInfo.value = info
    currentVolume.value = info.current
  } catch (error) {
    console.error('Error getting volume:', error)
  }
}

async function updateVolume() {
  try {
    await invoke('set_volume', { 
      volume: Number(currentVolume.value) 
    })
  } catch (error) {
    console.error('Error setting volume:', error)
  }
}

async function toggleMute() {
  try {
    const isUnmuted = await invoke('toggle_mute')
    volumeInfo.value.is_muted = !isUnmuted
  } catch (error) {
    console.error('Error toggling mute:', error)
  }
}

onMounted(async () => {
  await getVolumeInfo()
})
</script>

<style scoped>

</style> 