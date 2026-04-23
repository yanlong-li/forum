<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted } from 'vue'
import api from '../../composables/useApi'

const props = defineProps<{
  visible: boolean
  query: string
  position: { top: number; left: number }
}>()

const emit = defineEmits<{
  select: [username: string]
  close: []
}>()

interface User {
  id: string
  username: string
  avatar_url: string | null
}

const users = ref<User[]>([])
const loading = ref(false)
const selectedIndex = ref(0)

async function searchUsers(query: string) {
  if (!query || query.length < 1) {
    users.value = []
    return
  }

  loading.value = true
  try {
    const response = await api.get('/users/search', { params: { q: query, limit: 5 } })
    users.value = response.data.users || []
    selectedIndex.value = 0
  } catch (error) {
    console.error('Failed to search users:', error)
    users.value = []
  } finally {
    loading.value = false
  }
}

watch(() => props.query, (newQuery) => {
  searchUsers(newQuery)
})

function selectUser(user: User) {
  emit('select', user.username)
}

function handleKeydown(e: KeyboardEvent) {
  if (e.key === 'ArrowDown') {
    e.preventDefault()
    selectedIndex.value = Math.min(selectedIndex.value + 1, users.value.length - 1)
  } else if (e.key === 'ArrowUp') {
    e.preventDefault()
    selectedIndex.value = Math.max(selectedIndex.value - 1, 0)
  } else if (e.key === 'Enter' && users.value.length > 0) {
    e.preventDefault()
    selectUser(users.value[selectedIndex.value])
  } else if (e.key === 'Escape') {
    emit('close')
  }
}

onMounted(() => {
  document.addEventListener('keydown', handleKeydown)
})

onUnmounted(() => {
  document.removeEventListener('keydown', handleKeydown)
})
</script>

<template>
  <div
    v-if="visible && users.length > 0"
    class="fixed z-50 bg-white rounded-lg shadow-lg border border-slate-200 w-64 max-h-64 overflow-y-auto"
    :style="{ top: position.top + 'px', left: position.left + 'px' }"
  >
    <div v-if="loading" class="p-3 text-center text-slate-500 text-sm">
      Searching...
    </div>
    <div v-else>
      <button
        v-for="(user, index) in users"
        :key="user.id"
        @click="selectUser(user)"
        class="w-full px-3 py-2 flex items-center space-x-3 hover:bg-slate-50 transition-colors"
        :class="{ 'bg-slate-100': index === selectedIndex }"
      >
        <img
          v-if="user.avatar_url"
          :src="user.avatar_url"
          class="w-8 h-8 rounded-full object-cover"
        />
        <div
          v-else
          class="w-8 h-8 rounded-full bg-primary text-white flex items-center justify-center text-sm font-medium"
        >
          {{ user.username.charAt(0).toUpperCase() }}
        </div>
        <span class="font-medium text-slate-900">{{ user.username }}</span>
      </button>
    </div>
  </div>
</template>