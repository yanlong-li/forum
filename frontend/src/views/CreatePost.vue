<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import api from '../composables/useApi'
import { useToast } from '../composables/useToast'

const router = useRouter()
const toast = useToast()

const title = ref('')
const content = ref('')
const tagInput = ref('')
const tags = ref<string[]>([])
const submitting = ref(false)

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
    toast.error('Title is required')
    return
  }
  if (content.value.trim().length < 10) {
    toast.error('Content must be at least 10 characters')
    return
  }
  if (tags.value.length === 0) {
    toast.error('At least one tag is required')
    return
  }

  submitting.value = true
  try {
    const response = await api.post('/posts', {
      title: title.value,
      content: content.value,
      tags: tags.value,
    })
    toast.success('Post created!')
    router.push(`/post/${response.data.id}`)
  } catch (err: any) {
    toast.error(err.response?.data?.error?.message || 'Failed to create post')
  } finally {
    submitting.value = false
  }
}
</script>

<template>
  <div class="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    <h1 class="text-2xl font-bold text-slate-900 mb-8">Create New Post</h1>

    <form @submit.prevent="handleSubmit" class="space-y-6">
      <div>
        <label for="title" class="block text-sm font-medium text-slate-700 mb-2">
          Title
        </label>
        <input
          id="title"
          v-model="title"
          type="text"
          class="input text-lg"
          placeholder="Enter an interesting title..."
          maxlength="200"
        />
        <p class="mt-1 text-sm text-slate-500">{{ title.length }}/200</p>
      </div>

      <div>
        <label for="content" class="block text-sm font-medium text-slate-700 mb-2">
          Content
        </label>
        <textarea
          id="content"
          v-model="content"
          class="input min-h-[300px] resize-none font-mono text-sm"
          placeholder="Write your post content here..."
        ></textarea>
        <p class="mt-1 text-sm text-slate-500">Supports Markdown formatting</p>
      </div>

      <div>
        <label for="tags" class="block text-sm font-medium text-slate-700 mb-2">
          Tags
        </label>
        <div class="flex items-center space-x-2">
          <input
            id="tags"
            v-model="tagInput"
            type="text"
            class="input"
            placeholder="Add a tag..."
            @keydown.enter.prevent="addTag"
          />
          <button
            type="button"
            @click="addTag"
            class="btn btn-secondary"
          >
            Add
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
        <p class="mt-2 text-sm text-slate-500">Up to 5 tags</p>
      </div>

      <div class="flex justify-end space-x-4">
        <RouterLink to="/" class="btn btn-secondary">
          Cancel
        </RouterLink>
        <button
          type="submit"
          :disabled="submitting"
          class="btn btn-primary"
        >
          {{ submitting ? 'Publishing...' : 'Publish Post' }}
        </button>
      </div>
    </form>
  </div>
</template>
