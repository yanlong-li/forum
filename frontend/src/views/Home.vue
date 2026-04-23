<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { RouterLink } from 'vue-router'
import { useI18n } from 'vue-i18n'
import api from '../composables/useApi'
import PostCard from '../components/post/PostCard.vue'
import { usePrefetch } from '../composables/usePrefetch'
import SigninCard from '../components/user/SigninCard.vue'

defineOptions({ name: 'Home' })

const { t } = useI18n()

const posts = ref<any[]>([])
const tags = ref<any[]>([])
const loading = ref(true)
const page = ref(1)
const hasMore = ref(true)
const activeTab = ref<'latest' | 'hot'>('latest')
const { getPrefetchedData, hasPrefetchedData } = usePrefetch()

async function fetchPosts() {
  if (page.value === 1 && hasPrefetchedData('home') && activeTab.value === 'latest') {
    const data = getPrefetchedData('home')
    posts.value = data.posts
    hasMore.value = posts.value.length < data.total
    loading.value = false
    return
  }

  try {
    const endpoint = activeTab.value === 'hot' ? '/posts/hot' : '/posts'
    const response = await api.get(endpoint, { params: { page: page.value, limit: 20 } })
    const data = response.data

    if (page.value === 1) {
      posts.value = data.posts
    } else {
      posts.value.push(...data.posts)
    }

    hasMore.value = posts.value.length < data.total
  } catch (error) {
    console.error('Failed to fetch posts:', error)
  } finally {
    loading.value = false
  }
}

async function fetchTags() {
  if (hasPrefetchedData('tags')) {
    tags.value = getPrefetchedData('tags').tags.slice(0, 10)
    return
  }

  try {
    const response = await api.get('/tags', { params: { limit: 10, sort: 'popular' } })
    tags.value = response.data.tags
  } catch (error) {
    console.error('Failed to fetch tags:', error)
  }
}

const announcement = ref<string | null>(null)

async function fetchAnnouncement() {
  try {
    const response = await api.get('/announcements')
    if (response.data.length > 0) {
      announcement.value = response.data[0].content
    }
  } catch (error) {
    console.error('Failed to fetch announcement:', error)
  }
}

function loadMore() {
  page.value++
  fetchPosts()
}

watch(activeTab, () => {
  page.value = 1
  posts.value = []
  fetchPosts()
})

onMounted(() => {
  fetchPosts()
  fetchTags()
  fetchAnnouncement()
})
</script>

<template>
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    <div class="lg:grid lg:grid-cols-12 lg:gap-8">
      <div class="lg:col-span-8">
        <div class="flex items-center justify-between mb-6">
          <div class="flex items-center space-x-4">
            <button
              @click="activeTab = 'latest'"
              class="text-lg font-medium transition-colors"
              :class="activeTab === 'latest' ? 'text-primary' : 'text-slate-500 hover:text-slate-700'"
            >
              {{ t('home.latest') }}
            </button>
            <button
              @click="activeTab = 'hot'"
              class="text-lg font-medium transition-colors"
              :class="activeTab === 'hot' ? 'text-primary' : 'text-slate-500 hover:text-slate-700'"
            >
              {{ t('home.hot') }} 🔥
            </button>
          </div>
        </div>

        <div v-if="loading" class="space-y-4">
          <div v-for="i in 5" :key="i" class="card p-6">
            <div class="skeleton h-6 w-3/4 mb-4"></div>
            <div class="skeleton h-4 w-full mb-2"></div>
            <div class="skeleton h-4 w-2/3"></div>
          </div>
        </div>

        <div v-else-if="posts.length === 0" class="card p-12 text-center">
          <p class="text-slate-500">{{ t('home.noPostsYet') }}</p>
          <RouterLink to="/create" class="btn btn-primary mt-4">
            {{ t('post.createPost') }}
          </RouterLink>
        </div>

        <div v-else class="space-y-4">
          <PostCard
            v-for="post in posts"
            :key="post.id"
            :post="post"
          />

          <div v-if="hasMore" class="flex justify-center mt-6">
            <button
              @click="loadMore"
              class="btn btn-secondary"
            >
              {{ t('home.loadMore') }}
            </button>
          </div>
        </div>
      </div>

      <aside class="hidden lg:block lg:col-span-4">
        <div class="sticky top-24 space-y-6">
          <SigninCard />

          <div class="card p-6">
            <h2 class="text-lg font-semibold text-slate-900 mb-4">{{ t('home.popularTags') }}</h2>
            <div class="flex flex-wrap gap-2">
              <RouterLink
                v-for="tag in tags"
                :key="tag.id"
                :to="`/tags/${tag.name}`"
                class="badge badge-primary hover:bg-primary-hover hover:text-white"
              >
                {{ tag.name }}
                <span class="ml-1 text-xs opacity-75">{{ tag.post_count }}</span>
              </RouterLink>
            </div>
          </div>

          <div class="card p-6">
            <h2 class="text-lg font-semibold text-slate-900 mb-4">{{ t('home.about') }}</h2>
            <p class="text-slate-600 text-sm">
              {{ announcement || t('home.welcome') }}
            </p>
          </div>
        </div>
      </aside>
    </div>
  </div>
</template>
