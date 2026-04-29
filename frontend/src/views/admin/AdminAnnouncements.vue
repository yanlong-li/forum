<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import api from '../../composables/useApi'
import { useToast } from '../../composables/useToast'
import Loading from '../../components/common/Loading.vue'

interface Announcement {
  id: string
  title: string
  content: string
  is_active: boolean
  created_at: string
}

const { t } = useI18n()
const announcements = ref<Announcement[]>([])
const loading = ref(true)
const toast = useToast()
const showModal = ref(false)
const editing = ref<Announcement | null>(null)
const form = ref({ title: '', content: '' })
const saving = ref(false)

async function fetchAnnouncements() {
  try {
    const response = await api.get('/announcements')
    announcements.value = response.data
  } catch (error) {
    console.error(t('admin.loadFailed'), error)
  } finally {
    loading.value = false
  }
}

function openCreate() {
  editing.value = null
  form.value = { title: '', content: '' }
  showModal.value = true
}

function openEdit(announcement: Announcement) {
  editing.value = announcement
  form.value = { title: announcement.title, content: announcement.content }
  showModal.value = true
}

async function save() {
  if (!form.value.title.trim() || !form.value.content.trim()) {
    toast.error(t('admin.titleRequired'))
    return
  }

  saving.value = true
  try {
    if (editing.value) {
      await api.patch(`/announcements/${editing.value.id}`, {
        title: form.value.title,
        content: form.value.content,
        is_active: true
      })
      toast.success(t('admin.announcementUpdated'))
    } else {
      await api.post('/announcements', {
        title: form.value.title,
        content: form.value.content
      })
      toast.success(t('admin.announcementCreated'))
    }
    showModal.value = false
    fetchAnnouncements()
  } catch (error) {
    toast.error(t('admin.saveAnnouncementFailed'))
  } finally {
    saving.value = false
  }
}

async function deleteAnnouncement(id: string) {
  if (!confirm(t('admin.confirmDelete'))) return

  try {
    await api.delete(`/announcements/${id}`)
    toast.success(t('admin.announcementDeleted'))
    announcements.value = announcements.value.filter(a => a.id !== id)
  } catch (error) {
    toast.error(t('admin.deleteAnnouncementFailed'))
  }
}

onMounted(() => {
  fetchAnnouncements()
})
</script>

<template>
  <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    <div class="flex items-center justify-between mb-8">
      <h1 class="text-2xl font-bold text-slate-900">{{ t('admin.announcements') }}</h1>
      <RouterLink to="/admin" class="btn btn-secondary">
        {{ t('admin.backToDashboard') }}
      </RouterLink>
    </div>

    <div class="mb-6">
      <button @click="openCreate" class="btn btn-primary">
        {{ t('admin.createAnnouncement') }}
      </button>
    </div>

    <Loading v-if="loading" />

    <div v-else-if="announcements.length === 0" class="card p-8 text-center">
      <p class="text-slate-500">{{ t('admin.noAnnouncements') }}</p>
    </div>

    <div v-else class="space-y-4">
      <div v-for="announcement in announcements" :key="announcement.id" class="card p-6">
        <div class="flex items-start justify-between">
          <div class="flex-1">
            <h3 class="font-semibold text-slate-900">{{ announcement.title }}</h3>
            <p class="text-slate-600 mt-1">{{ announcement.content }}</p>
            <p class="text-xs text-slate-400 mt-2">
              {{ new Date(announcement.created_at).toLocaleString() }}
            </p>
          </div>
          <div class="flex space-x-2 ml-4">
            <button @click="openEdit(announcement)" class="btn btn-secondary">
              {{ t('admin.edit') }}
            </button>
            <button @click="deleteAnnouncement(announcement.id)" class="btn btn-primary">
              {{ t('admin.delete') }}
            </button>
          </div>
        </div>
      </div>
    </div>

    <div v-if="showModal" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div class="bg-white rounded-lg p-6 w-full max-w-md">
        <h2 class="text-xl font-bold mb-4">
          {{ editing ? t('admin.editAnnouncement') : t('admin.createAnnouncement') }}
        </h2>
        <div class="space-y-4">
          <div>
            <label class="block text-sm font-medium text-slate-700 mb-1">{{ t('admin.title') }}</label>
            <input v-model="form.title" type="text" class="input w-full" :placeholder="t('admin.title')" />
          </div>
          <div>
            <label class="block text-sm font-medium text-slate-700 mb-1">{{ t('admin.content') }}</label>
            <textarea v-model="form.content" rows="4" class="input w-full" :placeholder="t('admin.content')"></textarea>
          </div>
        </div>
        <div class="flex justify-end space-x-2 mt-6">
          <button @click="showModal = false" class="btn btn-secondary">{{ t('admin.cancel') }}</button>
          <button @click="save" :disabled="saving" class="btn btn-primary">
            {{ saving ? t('admin.saving') : t('admin.save') }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>