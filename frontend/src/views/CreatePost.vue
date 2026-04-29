<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import api from '../composables/useApi'
import { useToast } from '../composables/useToast'
import { useDraft } from '../composables/useDraft'
import MarkdownEditor from '../components/editor/MarkdownEditor.vue'

const router = useRouter()
const toast = useToast()
const { t } = useI18n()

const title = ref('')
const content = ref('')
const tagInput = ref('')
const tags = ref<string[]>([])
const submitting = ref(false)
const showDraftModal = ref(false)
const draftLastSaved = ref<string>('')

const { saveDraft, loadDraft, clearDraft, getTimeSinceLastSaved, AUTO_SAVE_INTERVAL } = useDraft()

let autoSaveTimer: ReturnType<typeof setInterval> | null = null

function addTag() {
  const tag = tagInput.value.trim().toLowerCase()
  if (tag && !tags.value.includes(tag) && tags.value.length < 5) {
    tags.value.push(tag)
    tagInput.value = ''
  }
}

function removeTag(tag: string) {
  tags.value = tags.value.filter((t) => t !== tag)
}

async function handleSubmit() {
  if (!title.value.trim()) {
    toast.error(t('error.posts.titleRequired'))
    return
  }
  if (content.value.trim().length < 10) {
    toast.error(t('error.posts.contentTooShort'))
    return
  }
  if (tags.value.length === 0) {
    toast.error(t('error.posts.tagsRequired'))
    return
  }

  submitting.value = true
  try {
    const response = await api.post('/posts', {
      title: title.value,
      content: content.value,
      tags: tags.value,
    })
    clearDraft()
    toast.success(t('error.posts.createSuccess'))
    router.push(`/post/${response.data.id}`)
  } catch (err: any) {
    toast.error(err.response?.data?.error?.message || t('error.posts.createFailed'))
  } finally {
    submitting.value = false
  }
}

function handleSaveDraft() {
  if (title.value || content.value || tags.value.length) {
    saveDraft({
      title: title.value,
      content: content.value,
      tags: tags.value,
      updatedAt: Date.now(),
    })
    draftLastSaved.value = getTimeSinceLastSaved()
    toast.info('Draft saved')
  }
}

function restoreDraft() {
  const draft = loadDraft()
  if (draft) {
    title.value = draft.title
    content.value = draft.content
    tags.value = draft.tags || []
    showDraftModal.value = false
    toast.success('Draft restored')
  }
}

function discardDraft() {
  clearDraft()
  showDraftModal.value = false
  toast.info('Draft discarded')
}

function setupAutoSave() {
  if (autoSaveTimer) {
    clearInterval(autoSaveTimer)
  }

  autoSaveTimer = setInterval(() => {
    if (title.value || content.value || tags.value.length) {
      saveDraft({
        title: title.value,
        content: content.value,
        tags: tags.value,
        updatedAt: Date.now(),
      })
      draftLastSaved.value = getTimeSinceLastSaved()
    }
  }, AUTO_SAVE_INTERVAL)
}

watch([title, content, tags], () => {
  if (title.value || content.value || tags.value.length) {
    draftLastSaved.value = 'Unsaved'
  }
})

onMounted(() => {
  const draft = loadDraft()
  if (draft) {
    showDraftModal.value = true
  }
  setupAutoSave()
})

onUnmounted(() => {
  if (autoSaveTimer) {
    clearInterval(autoSaveTimer)
  }
})
</script>

<template>
  <div class="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    <div class="flex items-center justify-between mb-8">
      <h1 class="text-2xl font-bold text-slate-900">{{ t('post.createNewPost') }}</h1>
      <div v-if="draftLastSaved" class="flex items-center space-x-2 text-sm text-slate-500">
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
        </svg>
        <span>Draft saved {{ draftLastSaved }}</span>
        <button @click="handleSaveDraft" class="text-primary hover:text-primary-hover">
          Save now
        </button>
      </div>
    </div>

    <form @submit.prevent="handleSubmit" class="space-y-6">
      <div>
        <label for="title" class="block text-sm font-medium text-slate-700 mb-2">
          {{ t('post.title') }}
        </label>
        <input
          id="title"
          v-model="title"
          type="text"
          class="input text-lg"
          :placeholder="t('post.titlePlaceholder')"
          maxlength="200"
        />
        <p class="mt-1 text-sm text-slate-500">{{ title.length }}/200</p>
      </div>

      <div>
        <label for="content" class="block text-sm font-medium text-slate-700 mb-2">
          {{ t('post.content') }}
        </label>
        <MarkdownEditor
          v-model="content"
          :placeholder="t('post.writeCommentPlaceholder')"
          min-height="300px"
        />
        <p class="mt-1 text-sm text-slate-500">{{ t('post.supportsMarkdown') }}</p>
      </div>

      <div>
        <label for="tags" class="block text-sm font-medium text-slate-700 mb-2">
          {{ t('post.tags') }}
        </label>
        <div class="flex items-center space-x-2">
          <input
            id="tags"
            v-model="tagInput"
            type="text"
            class="input"
            :placeholder="t('post.addTagPlaceholder')"
            @keydown.enter.prevent="addTag"
          />
          <button
            type="button"
            @click="addTag"
            class="btn btn-secondary"
          >
            {{ t('post.addTag') }}
          </button>
        </div>
        <div class="mt-3 flex flex-wrap gap-2">
          <span
            v-for="tag in tags"
            :key="tag"
            class="badge badge-primary pr-1"
          >
            {{ tag }}
            <button
              type="button"
              @click="removeTag(tag)"
              class="ml-1 hover:text-white"
            >
              <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
              </svg>
            </button>
          </span>
        </div>
        <p class="mt-2 text-sm text-slate-500">{{ t('post.upTo5Tags') }}</p>
      </div>

      <div class="flex justify-end space-x-4">
        <RouterLink to="/" class="btn btn-secondary">
          {{ t('common.cancel') }}
        </RouterLink>
        <button
          type="submit"
          :disabled="submitting"
          class="btn btn-primary"
        >
          {{ submitting ? t('post.publishing') : t('post.publish') }}
        </button>
      </div>
    </form>

    <div
      v-if="showDraftModal"
      class="fixed inset-0 z-50 flex items-center justify-center bg-black/50"
      @click.self="showDraftModal = false"
    >
      <div class="bg-white rounded-lg shadow-xl p-6 max-w-md w-full mx-4">
        <h3 class="text-lg font-semibold text-slate-900 mb-2">Restore previous draft?</h3>
        <p class="text-slate-600 mb-6">
          You have an unsaved draft from a previous session. Would you like to restore it?
        </p>
        <div class="flex space-x-4 justify-end">
          <button
            @click="discardDraft"
            class="btn btn-secondary"
          >
            Discard
          </button>
          <button
            @click="restoreDraft"
            class="btn btn-primary"
          >
            Restore
          </button>
        </div>
      </div>
    </div>
  </div>
</template>