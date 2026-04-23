<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import api from '../composables/useApi'
import PostCard from '../components/post/PostCard.vue'
import Loading from '../components/common/Loading.vue'

const route = useRoute()

const posts = ref<any[]>([])
const tag = ref<any>(null)
const loading = ref(true)
const page = ref(1)
const hasMore = ref(true)

async function fetchTagPosts() {
  try {
    const [postsResponse, tagResponse] = await Promise.all([
      api.get(`/tags/${route.params.name}/posts`, { params: { page: page.value, limit: 20 } }),
      api.get(`/tags/${route.params.name}`),
    ])

    const postsData = postsResponse.data
    if (page.value === 1) {
      posts.value = postsData.posts
    } else {
      posts.value.push(...postsData.posts)
    }

    hasMore.value = posts.value.length < postsData.total
    tag.value = tagResponse.data
  } catch (error) {
    console.error('Failed to fetch posts:', error)
  } finally {
    loading.value = false
  }
}

function loadMore() {
  page.value++
  fetchTagPosts()
}

onMounted(() => {
  fetchTagPosts()
})
</script>

<template>
  <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    <div class="mb-8">
      <span class="badge badge-primary text-2xl px-4 py-2">{{ tag?.name || route.params.name }}</span>
      <p class="mt-2 text-slate-600">{{ tag?.post_count || 0 }} posts</p>
    </div>

    <Loading v-if="loading" />

    <div v-else class="space-y-4">
      <PostCard v-for="post in posts" :key="post.id" :post="post" />

      <div v-if="posts.length === 0" class="card p-8 text-center text-slate-500">
        No posts with this tag yet.
      </div>

      <div v-if="hasMore" class="flex justify-center mt-6">
        <button @click="loadMore" class="btn btn-secondary">
          Load More
        </button>
      </div>
    </div>
  </div>
</template>
