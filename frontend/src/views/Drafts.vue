<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { RouterLink } from 'vue-router'
import api from '../composables/useApi'
import { useToast } from '../composables/useToast'
import Loading from '../components/common/Loading.vue'
import { formatDistanceToNow } from '../utils/time'

interface Draft {
  id: string
  draft_type: string
  title: string | null
  content: string | null
  updated_at: string
}

const drafts = ref<Draft[]>([])
const loading = ref(true)
const toast = useToast()

async function fetchDrafts() {
  try {
    const response = await api.get('/drafts')
    drafts.value = response.data
  } catch (error) {
    console.error('Failed to fetch drafts:', error)
  } finally {
    loading.value = false
  }
}

async function deleteDraft(id: string) {
  try {
    await api.delete(`/drafts/${id}`)
    drafts.value = drafts.value.filter(d => d.id !== id)
    toast.success('Draft deleted')
  } catch (error) {
    toast.error('Failed to delete draft')
  }
}

async function continueDraft(draft: Draft) {
  if (draft.draft_type === 'post') {
    window.location.href = `/create?draft=${draft.id}`
  }
}

onMounted(() => {
  fetchDrafts()
})
</script>

<template>
  <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    <div class="flex items-center justify-between mb-8">
      <h1 class="text-2xl font-bold text-slate-900">Drafts</h1>
      <RouterLink to="/create" class="btn btn-primary">
        New Post
      </RouterLink>
    </div>

    <Loading v-if="loading" />

    <div v-else-if="drafts.length === 0" class="card p-8 text-center">
      <p class="text-slate-500">No drafts saved</p>
      <RouterLink to="/create" class="btn btn-primary mt-4">
        Create New Post
      </RouterLink>
    </div>

    <div v-else class="space-y-4">
      <div v-for="draft in drafts" :key="draft.id" class="card p-4">
        <div class="flex items-start justify-between">
          <div class="flex-1 min-w-0 cursor-pointer" @click="continueDraft(draft)">
            <h3 class="font-medium text-slate-900 truncate">
              {{ draft.title || 'Untitled' }}
            </h3>
            <p class="text-sm text-slate-500 mt-1 line-clamp-2">
              {{ draft.content || 'No content' }}
            </p>
            <p class="text-xs text-slate-400 mt-2">
              Last edited {{ formatDistanceToNow(new Date(draft.updated_at)) }}
            </p>
          </div>
          <button
            @click.stop="deleteDraft(draft.id)"
            class="ml-4 text-slate-400 hover:text-red-500 transition-colors"
          >
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
            </svg>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>