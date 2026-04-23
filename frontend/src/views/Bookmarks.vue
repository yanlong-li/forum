<script setup lang="ts">
import { ref, onMounted } from 'vue'
import api from '../composables/useApi'
import { useToast } from '../composables/useToast'
import PostCard from '../components/post/PostCard.vue'
import Loading from '../components/common/Loading.vue'

const toast = useToast()
const posts = ref<any[]>([])
const loading = ref(true)
const page = ref(1)
const hasMore = ref(true)

async function fetchBookmarks() {
  try {
    const response = await api.get('/bookmarks', { params: { page: page.value, limit: 20 } })
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

function loadMore() {
  page.value++
  fetchBookmarks()
}

onMounted(() => {
  fetchBookmarks()
})
</script>

<template>
  <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    <h1 class="text-2xl font-bold text-slate-900 mb-8">My Bookmarks</h1>

    <Loading v-if="loading" />

    <div v-else-if="posts.length === 0" class="card p-8 text-center text-slate-500">
      <p>No bookmarks yet.</p>
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
  </div>
</template>
