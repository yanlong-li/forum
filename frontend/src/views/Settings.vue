<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useAuthStore } from '../stores/auth'
import { useToast } from '../composables/useToast'
import api from '../composables/useApi'

const authStore = useAuthStore()
const toast = useToast()

const username = ref('')
const bio = ref('')
const avatarUrl = ref('')
const loading = ref(false)

async function fetchProfile() {
  try {
    const response = await api.get(`/users/me`)
    username.value = response.data.username
    bio.value = response.data.bio || ''
    avatarUrl.value = response.data.avatar_url || ''
  } catch (err) {
    console.error('Failed to fetch profile:', err)
  }
}

async function handleSubmit() {
  loading.value = true
  try {
    await api.patch('/users/me', {
      username: username.value,
      bio: bio.value,
      avatar_url: avatarUrl.value || null,
    })
    authStore.setUser({
      ...authStore.user!,
      username: username.value,
      avatar_url: avatarUrl.value || null,
      bio: bio.value || null,
    })
    toast.success('Profile updated')
  } catch (err: any) {
    toast.error(err.response?.data?.error?.message || 'Failed to update profile')
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  fetchProfile()
})
</script>

<template>
  <div class="max-w-2xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    <h1 class="text-2xl font-bold text-slate-900 mb-8">Settings</h1>

    <div class="card p-8">
      <form @submit.prevent="handleSubmit" class="space-y-6">
        <div>
          <label for="avatar" class="block text-sm font-medium text-slate-700 mb-2">
            Avatar URL
          </label>
          <input
            id="avatar"
            v-model="avatarUrl"
            type="url"
            class="input"
            placeholder="https://example.com/avatar.jpg"
          />
        </div>

        <div>
          <label for="username" class="block text-sm font-medium text-slate-700 mb-2">
            Username
          </label>
          <input
            id="username"
            v-model="username"
            type="text"
            class="input"
            placeholder="your_username"
          />
        </div>

        <div>
          <label for="bio" class="block text-sm font-medium text-slate-700 mb-2">
            Bio
          </label>
          <textarea
            id="bio"
            v-model="bio"
            class="input min-h-[100px] resize-none"
            placeholder="Tell us about yourself..."
          ></textarea>
        </div>

        <div class="flex justify-end">
          <button
            type="submit"
            :disabled="loading"
            class="btn btn-primary"
          >
            {{ loading ? 'Saving...' : 'Save Changes' }}
          </button>
        </div>
      </form>
    </div>
  </div>
</template>
