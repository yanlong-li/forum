<script setup lang="ts">
import { ref, onMounted } from 'vue'
import api from '../../composables/useApi'

interface Announcement {
  id: string
  title: string
  content: string
  is_active: boolean
  created_at: string
}

const announcement = ref<Announcement | null>(null)

async function fetchAnnouncement() {
  try {
    const response = await api.get('/announcements')
    if (response.data.length > 0) {
      announcement.value = response.data[0]
    }
  } catch (error) {
    console.error('Failed to fetch announcement:', error)
  }
}

function dismiss() {
  announcement.value = null
}

onMounted(() => {
  fetchAnnouncement()
})
</script>

<template>
  <div
    v-if="announcement"
    class="bg-gradient-to-r from-amber-500 to-orange-500 text-white"
  >
    <div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 py-3">
      <div class="flex items-center justify-between">
        <div class="flex items-center space-x-3">
          <svg class="w-5 h-5 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5.882V19.24a1.76 1.76 0 01-3.417.592l-2.147-6.15M18 13a3 3 0 100-6M5.436 13.683A4.001 4.001 0 017 6h1.832c4.1 0 7.625-1.234 9.168-3v14c-1.543-1.766-5.067-3-9.168-3H7a3.988 3.988 0 01-1.564-.317z" />
          </svg>
          <div>
            <span class="font-medium">{{ announcement.title }}</span>
            <span class="mx-2">-</span>
            <span class="text-amber-100">{{ announcement.content }}</span>
          </div>
        </div>
        <button
          @click="dismiss"
          class="flex-shrink-0 text-amber-100 hover:text-white transition-colors"
        >
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>
    </div>
  </div>
</template>