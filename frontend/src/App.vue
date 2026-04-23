<script setup lang="ts">
import { onMounted } from 'vue'
import { useAuthStore } from './stores/auth'
import Navbar from './components/layout/Navbar.vue'
import Toast from './components/common/Toast.vue'
import AnnouncementBanner from './components/common/AnnouncementBanner.vue'

const authStore = useAuthStore()

onMounted(() => {
  authStore.checkAuth()
})
</script>

<template>
  <div class="min-h-screen flex flex-col">
    <Navbar />
    <AnnouncementBanner />
    <main class="flex-1">
      <router-view v-slot="{ Component, route }">
        <transition name="fade-slide" mode="out-in">
          <keep-alive include="Home,Tags">
            <component :is="Component" :key="route.path" />
          </keep-alive>
        </transition>
      </router-view>
    </main>
    <Toast />
  </div>
</template>

<style>
.fade-slide-enter-active,
.fade-slide-leave-active {
  transition: all 0.15s ease;
}

.fade-slide-enter-from {
  opacity: 0;
}

.fade-slide-leave-to {
  opacity: 0;
}
</style>
