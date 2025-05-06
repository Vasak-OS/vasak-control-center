<template>
  <div class="bg-white/50 dark:bg-black/50 rounded-xl p-4 flex items-center gap-4 w-full">
    <div class="relative w-16 h-16">
      <img 
        :src="userInfo.avatar_data" 
        :alt="userInfo.full_name"
        class="w-full h-full rounded-full object-cover"
      />
    </div>
    <div class="flex flex-col flex-1">
      <h2 class="text-lg font-semibold">{{ userInfo.full_name }}</h2>
      <p class="text-sm opacity-75">{{ userInfo.username }}</p>
    </div>
    <div class="text-right">
      <div class="text-2xl font-medium">{{ currentTime }}</div>
      <div class="text-sm opacity-75">{{ currentDate }}</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { getUserData, type UserInfo } from '@vasakgroup/plugin-user-data';

const userInfo = ref<UserInfo>({
  username: '',
  full_name: '',
  avatar_data: '',
});

const currentTime = ref('');
const currentDate = ref('');

const updateDateTime = () => {
  const now = new Date();
  currentTime.value = now.toLocaleTimeString('es-ES', { 
    hour: '2-digit', 
    minute: '2-digit' 
  });
  currentDate.value = now.toLocaleDateString('es-ES', { 
    weekday: 'long', 
    day: 'numeric', 
    month: 'long' 
  });
};

const getUserInfo = async () => {
  try {
    const info = await getUserData();
    userInfo.value = info as UserInfo;
  } catch (error) {
    console.error('Error getting user info:', error);
  }
};

let timeInterval: number;

onMounted(async () => {
  await getUserInfo();
  updateDateTime();
  timeInterval = window.setInterval(updateDateTime, 1000);
});

onUnmounted(() => {
  if (timeInterval) {
    clearInterval(timeInterval);
  }
});
</script> 