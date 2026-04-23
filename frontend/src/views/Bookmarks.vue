<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import api from '../composables/useApi'
import { useToast } from '../composables/useToast'
import PostCard from '../components/post/PostCard.vue'
import Loading from '../components/common/Loading.vue'

interface Folder {
  id: string
  name: string
  created_at: string
}

const toast = useToast()
const posts = ref<any[]>([])
const folders = ref<Folder[]>([])
const loading = ref(true)
const page = ref(1)
const hasMore = ref(true)
const activeFolder = ref<string | null>(null)
const showNewFolderModal = ref(false)
const newFolderName = ref('')
const creating = ref(false)

async function fetchFolders() {
  try {
    const response = await api.get('/folders')
    folders.value = response.data
  } catch {
    toast.error('Failed to load folders')
  }
}

async function fetchBookmarks() {
  try {
    const params: any = { page: page.value, limit: 20 }
    if (activeFolder.value) {
      params.folder_id = activeFolder.value
    }
    const response = await api.get('/bookmarks', { params })
    const data = response.data

    if (page.value === 1) {
      posts.value = data.posts
    } else {
      posts.value.push(...data.posts)
    }

    hasMore.value = posts.value.length < data.total
  } catch {
    toast.error('Failed to load bookmarks')
  } finally {
    loading.value = false
  }
}

async function createFolder() {
  if (!newFolderName.value.trim()) {
    toast.error('Folder name is required')
    return
  }

  creating.value = true
  try {
    await api.post('/folders', { name: newFolderName.value })
    toast.success('Folder created')
    showNewFolderModal.value = false
    newFolderName.value = ''
    await fetchFolders()
  } catch {
    toast.error('Failed to create folder')
  } finally {
    creating.value = false
  }
}

async function deleteFolder(folderId: string) {
  try {
    await api.delete(`/folders/${folderId}`)
    toast.success('Folder deleted')
    if (activeFolder.value === folderId) {
      activeFolder.value = null
    }
    await fetchFolders()
    page.value = 1
    fetchBookmarks()
  } catch {
    toast.error('Failed to delete folder')
  }
}

function selectFolder(folderId: string | null) {
  activeFolder.value = folderId
  page.value = 1
  posts.value = []
  loading.value = true
  fetchBookmarks()
}

function loadMore() {
  page.value++
  fetchBookmarks()
}

watch(activeFolder, () => {
  page.value = 1
  posts.value = []
  loading.value = true
  fetchBookmarks()
})

onMounted(() => {
  fetchFolders()
  fetchBookmarks()
})
</script>

<template>
  <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    <div class="flex items-center justify-between mb-8">
      <h1 class="text-2xl font-bold text-slate-900">My Bookmarks</h1>
      <button @click="showNewFolderModal = true" class="btn btn-primary">
        New Folder
      </button>
    </div>

    <div class="flex space-x-4 mb-8">
      <button
        @click="selectFolder(null)"
        class="px-4 py-2 rounded-lg transition-colors"
        :class="activeFolder === null ? 'bg-primary text-white' : 'bg-slate-100 text-slate-700 hover:bg-slate-200'"
      >
        All
      </button>
      <button
        v-for="folder in folders"
        :key="folder.id"
        @click="selectFolder(folder.id)"
        class="px-4 py-2 rounded-lg transition-colors flex items-center space-x-2 group"
        :class="activeFolder === folder.id ? 'bg-primary text-white' : 'bg-slate-100 text-slate-700 hover:bg-slate-200'"
      >
        <span>{{ folder.name }}</span>
        <span
          @click.stop="deleteFolder(folder.id)"
          class="text-xs opacity-0 group-hover:opacity-100 ml-1"
          :class="activeFolder === folder.id ? 'text-white/70 hover:text-white' : 'text-slate-500 hover:text-slate-700'"
        >
          x
        </span>
      </button>
    </div>

    <Loading v-if="loading" />

    <div v-else-if="posts.length === 0" class="card p-8 text-center text-slate-500">
      <p>No bookmarks{{ activeFolder ? ' in this folder' : '' }}.</p>
      <RouterLink to="/" class="btn btn-primary mt-4">
        Explore Posts
      </RouterLink>
    </div>

    <div v-else class="space-y-4">
      <PostCard v-for="post in posts" :key="post.id" :post="post" />

      <div v-if="hasMore" class="flex justify-center mt-6">
        <button @click="loadMore" class="btn btn-secondary">
          Load More
        </button>
      </div>
    </div>

    <div v-if="showNewFolderModal" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div class="bg-white rounded-lg p-6 w-full max-w-md">
        <h2 class="text-xl font-bold mb-4">Create New Folder</h2>
        <input
          v-model="newFolderName"
          type="text"
          placeholder="Folder name"
          class="w-full px-4 py-2 border border-slate-300 rounded-lg mb-4"
          @keyup.enter="createFolder"
        />
        <div class="flex justify-end space-x-2">
          <button @click="showNewFolderModal = false" class="btn btn-secondary">
            Cancel
          </button>
          <button @click="createFolder" :disabled="creating" class="btn btn-primary">
            {{ creating ? 'Creating...' : 'Create' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>