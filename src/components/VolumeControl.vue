<template>
  <div class="flex flex-row items-center gap-2 justify-between w-full h-auto p-4">
    <button @click="toggleMute" class="w-8 h-8 flex items-center justify-center">
      <img 
        :src="currentIcon" 
        :alt="volumeInfo.is_muted ? 'Unmute' : 'Mute'"
        class="w-6 h-6"
      />
    </button>
    <input 
      type="range" 
      :min="volumeInfo.min" 
      :max="volumeInfo.max" 
      v-model="currentVolume"
      @input="updateVolume"
      class="flex-1"
    />
    <span class="w-12 text-right">{{ volumePercentage }}%</span>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { getIcon, getImageType } from '@/common/icons'
const volumeInfo = ref({
  current: 0,
  min: 0,
  max: 100,
  is_muted: false
})

const currentVolume = ref(0)
const currentIcon = ref('')

async function updateIcon() {
  const getIconName = () => {
    if (volumeInfo.value.is_muted) return 'audio-volume-muted-symbolic'
    
    const percentage = volumePercentage.value
    if (percentage <= 0) return 'audio-volume-muted-symbolic'
    if (percentage <= 33) return 'audio-volume-low-symbolic'
    if (percentage <= 66) return 'audio-volume-medium-symbolic'
    return 'audio-volume-high-symbolic'
  }

  try {
    const base64Icon = await getIcon(getIconName())
    currentIcon.value = `data:${getImageType(base64Icon)};base64,${base64Icon}`
  } catch (error) {
    console.error('Error loading icon:', error)
  }
}

const volumePercentage = computed(() => {
  const range = volumeInfo.value.max - volumeInfo.value.min
  const current = currentVolume.value - volumeInfo.value.min
  return Math.round((current / range) * 100)
})

// Observar cambios en el volumen y estado de mute
watch([() => volumeInfo.value.is_muted, volumePercentage], updateIcon, { immediate: true })

async function getVolumeInfo() {
  try {
    const info = await invoke('get_volume')
    volumeInfo.value = info
    currentVolume.value = info.current
    await updateIcon()
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