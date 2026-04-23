<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import api from '../composables/useApi'
import { useToast } from '../composables/useToast'

const router = useRouter()
const toast = useToast()

const loading = ref(false)
const username = ref('')
const bio = ref('')
const avatarUrl = ref('')

async function fetchProfile() {
  try {
    const response = await api.get('/users/me')
    username.value = response.data.username
    bio.value = response.data.bio || ''
    avatarUrl.value = response.data.avatar_url || ''
  } catch (error) {
    toast.error('Failed to load profile')
  }
}

async function handleSubmit() {
  if (!username.value.trim()) {
    toast.error('Username is required')
    return
  }

  loading.value = true
  try {
    await api.patch('/users/me', {
      username: username.value,
      bio: bio.value,
      avatar_url: avatarUrl.value || null,
    })
    toast.success('Profile updated')
    router.push(`/profile/${username.value}`)
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
    <h1 class="text-2xl font-bold text-slate-900 mb-8">Edit Profile</h1>

    <form @submit.prevent="handleSubmit" class="space-y-6">
      <div>
        <label class="block text-sm font-medium text-slate-700 mb-2">Username</label>
        <input
          v-model="username"
          type="text"
          class="w-full px-4 py-2 border border-slate-300 rounded-lg focus:border-primary focus:ring-2 focus:ring-primary/20 outline-none"
          placeholder="Username"
        />
      </div>

      <div>
        <label class="block text-sm font-medium text-slate-700 mb-2">Bio</label>
        <textarea
          v-model="bio"
          rows="4"
          class="w-full px-4 py-2 border border-slate-300 rounded-lg focus:border-primary focus:ring-2 focus:ring-primary/20 outline-none resize-none"
          placeholder="Tell us about yourself"
        ></textarea>
      </div>

      <div>
        <label class="block text-sm font-medium text-slate-700 mb-2">Avatar URL</label>
        <input
          v-model="avatarUrl"
          type="url"
          class="w-full px-4 py-2 border border-slate-300 rounded-lg focus:border-primary focus:ring-2 focus:ring-primary/20 outline-none"
          placeholder="https://example.com/avatar.jpg"
        />
      </div>

      <div class="flex items-center justify-end space-x-4">
        <button
          type="button"
          @click="router.back()"
          class="btn btn-secondary"
        >
          Cancel
        </button>
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
</template>