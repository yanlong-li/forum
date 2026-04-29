<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import api from '../composables/useApi'
import { useAuthStore } from '../stores/auth'
import { useToast } from '../composables/useToast'

const router = useRouter()
const authStore = useAuthStore()
const toast = useToast()
const { t } = useI18n()

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
    toast.error(t('settings.loadProfileFailed'))
  }
}

async function handleSubmit() {
  if (!username.value.trim()) {
    toast.error(t('settings.usernameRequired'))
    return
  }

  loading.value = true
  try {
    const response = await api.patch('/users/me', {
      username: username.value,
      bio: bio.value,
      avatar_url: avatarUrl.value || null,
    })
    const updatedUser = response.data
    authStore.setUser({
      ...authStore.user,
      ...updatedUser,
      username: updatedUser.username
    })
    toast.success(t('settings.profileUpdated'))
    router.push(`/profile/${username.value}`)
  } catch (err: any) {
    toast.error(err.response?.data?.error?.message || t('settings.updateProfileFailed'))
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
    <h1 class="text-2xl font-bold text-slate-900 mb-8">{{ t('settings.editProfile') }}</h1>

    <form @submit.prevent="handleSubmit" class="space-y-6">
      <div>
        <label class="block text-sm font-medium text-slate-700 mb-2">{{ t('settings.username') }}</label>
        <input
          v-model="username"
          type="text"
          class="w-full px-4 py-2 border border-slate-300 rounded-lg focus:border-primary focus:ring-2 focus:ring-primary/20 outline-none"
          :placeholder="t('settings.username')"
        />
      </div>

      <div>
        <label class="block text-sm font-medium text-slate-700 mb-2">{{ t('settings.bio') }}</label>
        <textarea
          v-model="bio"
          rows="4"
          class="w-full px-4 py-2 border border-slate-300 rounded-lg focus:border-primary focus:ring-2 focus:ring-primary/20 outline-none resize-none"
          :placeholder="t('settings.bioPlaceholder')"
        ></textarea>
      </div>

      <div>
        <label class="block text-sm font-medium text-slate-700 mb-2">{{ t('settings.avatarUrl') }}</label>
        <input
          v-model="avatarUrl"
          type="url"
          class="w-full px-4 py-2 border border-slate-300 rounded-lg focus:border-primary focus:ring-2 focus:ring-primary/20 outline-none"
          :placeholder="t('settings.avatarUrlPlaceholder')"
        />
      </div>

      <div class="flex items-center justify-end space-x-4">
        <button
          type="button"
          @click="router.back()"
          class="btn btn-secondary"
        >
          {{ t('settings.cancel') }}
        </button>
        <button
          type="submit"
          :disabled="loading"
          class="btn btn-primary"
        >
          {{ loading ? t('settings.saving') : t('settings.saveChanges') }}
        </button>
      </div>
    </form>
  </div>
</template>