<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import api from '../../composables/useApi'
import Loading from '../../components/common/Loading.vue'

interface User {
  id: string
  username: string
  email: string
  avatar_url: string | null
  bio: string | null
  is_admin: boolean
  is_locked: boolean
  created_at: string
}

const { t } = useI18n()
const users = ref<User[]>([])
const loading = ref(true)
const page = ref(1)
const hasMore = ref(true)

async function fetchUsers() {
  try {
    const response = await api.get('/admin/users', { params: { page: page.value, limit: 20 } })
    if (page.value === 1) {
      users.value = response.data.users
    } else {
      users.value.push(...response.data.users)
    }
    hasMore.value = users.value.length < response.data.total
  } catch (error) {
    console.error(t('admin.loadFailed'), error)
  } finally {
    loading.value = false
  }
}

function loadMore() {
  page.value++
  fetchUsers()
}

onMounted(() => {
  fetchUsers()
})
</script>

<template>
  <div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    <div class="flex items-center justify-between mb-8">
      <h1 class="text-2xl font-bold text-slate-900">{{ t('admin.userManagement') }}</h1>
      <RouterLink to="/admin" class="btn btn-secondary">
        {{ t('admin.backToDashboard') }}
      </RouterLink>
    </div>

    <Loading v-if="loading" />

    <div v-else class="card overflow-hidden">
      <table class="w-full">
        <thead class="bg-slate-50 border-b border-slate-200">
          <tr>
            <th class="px-6 py-3 text-left text-xs font-medium text-slate-500 uppercase tracking-wider">{{ t('admin.user') }}</th>
            <th class="px-6 py-3 text-left text-xs font-medium text-slate-500 uppercase tracking-wider">{{ t('admin.email') }}</th>
            <th class="px-6 py-3 text-left text-xs font-medium text-slate-500 uppercase tracking-wider">{{ t('admin.status') }}</th>
            <th class="px-6 py-3 text-left text-xs font-medium text-slate-500 uppercase tracking-wider">{{ t('admin.joined') }}</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-slate-200">
          <tr v-for="user in users" :key="user.id" class="hover:bg-slate-50">
            <td class="px-6 py-4 whitespace-nowrap">
              <div class="flex items-center">
                <img
                  v-if="user.avatar_url"
                  :src="user.avatar_url"
                  :alt="user.username"
                  class="w-10 h-10 rounded-full object-cover"
                  loading="lazy"
                />
                <div
                  v-else
                  class="w-10 h-10 rounded-full bg-primary text-white flex items-center justify-center font-medium"
                >
                  {{ user.username.charAt(0).toUpperCase() }}
                </div>
                <div class="ml-4">
                  <div class="font-medium text-slate-900">{{ user.username }}</div>
                  <div v-if="user.is_admin" class="text-xs text-primary">{{ t('admin.admin') }}</div>
                </div>
              </div>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-slate-500">{{ user.email }}</td>
            <td class="px-6 py-4 whitespace-nowrap">
              <span v-if="user.is_locked" class="badge badge-error">{{ t('admin.locked') }}</span>
              <span v-else class="badge badge-success">{{ t('admin.activeStatus') }}</span>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-slate-500 text-sm">
              {{ new Date(user.created_at).toLocaleDateString() }}
            </td>
          </tr>
        </tbody>
      </table>

      <div v-if="hasMore" class="p-4 border-t border-slate-200">
        <button @click="loadMore" class="btn btn-secondary w-full">
          {{ t('admin.loadMore') }}
        </button>
      </div>
    </div>
  </div>
</template>