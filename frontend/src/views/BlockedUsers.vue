<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { RouterLink } from 'vue-router'
import { useI18n } from 'vue-i18n'
import api from '../composables/useApi'
import { useToast } from '../composables/useToast'
import Loading from '../components/common/Loading.vue'

interface BlockedUser {
  id: string
  username: string
  avatar_url: string | null
  bio: string | null
  created_at: string
}

const blockedUsers = ref<BlockedUser[]>([])
const loading = ref(true)
const toast = useToast()
const { t } = useI18n()

async function fetchBlockedUsers() {
  try {
    const response = await api.get('/blocks')
    blockedUsers.value = response.data
  } catch (error) {
    console.error(t('block.loadBlockedUsersFailed'), error)
  } finally {
    loading.value = false
  }
}

async function unblockUser(userId: string) {
  try {
    await api.delete(`/blocks/${userId}`)
    blockedUsers.value = blockedUsers.value.filter(u => u.id !== userId)
    toast.success(t('block.userUnblocked'))
  } catch (error) {
    toast.error(t('block.unblockUserFailed'))
  }
}

onMounted(() => {
  fetchBlockedUsers()
})
</script>

<template>
  <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    <div class="flex items-center justify-between mb-8">
      <h1 class="text-2xl font-bold text-slate-900">{{ t('block.blockedUsers') }}</h1>
      <RouterLink to="/settings" class="btn btn-secondary">
        {{ t('block.backToSettings') }}
      </RouterLink>
    </div>

    <Loading v-if="loading" />

    <div v-else-if="blockedUsers.length === 0" class="card p-8 text-center">
      <p class="text-slate-500">{{ t('block.noBlockedUsers') }}</p>
    </div>

    <div v-else class="space-y-4">
      <div v-for="user in blockedUsers" :key="user.id" class="card p-4">
        <div class="flex items-center space-x-4">
          <RouterLink :to="`/profile/${user.username}`" class="flex-shrink-0">
            <img
              v-if="user.avatar_url"
              :src="user.avatar_url"
              :alt="user.username"
              class="w-10 h-10 rounded-full object-cover"
            />
            <div
              v-else
              class="w-10 h-10 rounded-full bg-primary text-white flex items-center justify-center font-medium"
            >
              {{ user.username.charAt(0).toUpperCase() }}
            </div>
          </RouterLink>

          <div class="flex-1 min-w-0">
            <RouterLink
              :to="`/profile/${user.username}`"
              class="font-medium text-slate-900 hover:text-primary"
            >
              {{ user.username }}
            </RouterLink>
            <p v-if="user.bio" class="text-sm text-slate-500 truncate">{{ user.bio }}</p>
          </div>

          <button
            @click="unblockUser(user.id)"
            class="btn btn-secondary"
          >
            {{ t('block.unblock') }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>