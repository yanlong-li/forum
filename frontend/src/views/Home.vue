<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch, nextTick } from 'vue'
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
const loadingMore = ref(false)
const page = ref(1)
const hasMore = ref(true)
const activeTab = ref<'latest' | 'hot'>('latest')
const { getPrefetchedData, hasPrefetchedData, prefetchHome } = usePrefetch()
const loadMoreTrigger = ref<HTMLElement | null>(null)
let observer: IntersectionObserver | null = null

const CACHE_DURATION = 5000
const tabCache = ref<{
  latest: { posts: any[], total: number, timestamp: number } | null
  hot: { posts: any[], total: number, timestamp: number } | null
}>({ latest: null, hot: null })

function getCachedPosts(tab: 'latest' | 'hot') {
  const cached = tabCache.value[tab]
  if (!cached) return null
  if (Date.now() - cached.timestamp > CACHE_DURATION) {
    tabCache.value[tab] = null
    return null
  }
  return cached
}

function setCachedPosts(tab: 'latest' | 'hot', data: { posts: any[], total: number }) {
  tabCache.value[tab] = {
    posts: data.posts,
    total: data.total,
    timestamp: Date.now()
  }
}

async function fetchPosts(useCache = true) {
  const cached = page.value === 1 && useCache ? getCachedPosts(activeTab.value) : null

  if (cached) {
    posts.value = cached.posts
    hasMore.value = posts.value.length < cached.total
    loading.value = false
    return
  }

  if (page.value === 1 && activeTab.value === 'latest' && hasPrefetchedData('home-latest')) {
    const data = getPrefetchedData('home-latest')
    posts.value = data.posts
    hasMore.value = posts.value.length < data.total
    loading.value = false
    setCachedPosts('latest', data)
    prefetchHome()
    return
  }

  if (page.value === 1 && activeTab.value === 'hot' && hasPrefetchedData('home-hot')) {
    const data = getPrefetchedData('home-hot')
    posts.value = data.posts
    hasMore.value = posts.value.length < data.total
    loading.value = false
    setCachedPosts('hot', data)
    prefetchHome()
    return
  }

  try {
    const endpoint = activeTab.value === 'hot' ? '/posts/hot' : '/posts'
    const response = await api.get(endpoint, { params: { page: page.value, limit: 20 } })
    const data = response.data

    if (page.value === 1) {
      posts.value = data.posts
      setCachedPosts(activeTab.value, data)
      prefetchHome()
    } else {
      posts.value.push(...data.posts)
    }

    hasMore.value = posts.value.length < data.total
  } catch (error) {
    console.error('Failed to fetch posts:', error)
  } finally {
    loading.value = false
    loadingMore.value = false
  }
}

async function loadMore() {
  if (loadingMore.value || !hasMore.value) return
  loadingMore.value = true
  page.value++
  await fetchPosts(false)
}

function setupInfiniteScroll() {
  if (observer) {
    observer.disconnect()
  }

  observer = new IntersectionObserver(
    (entries) => {
      if (entries[0].isIntersecting && hasMore.value && !loadingMore.value && !loading.value) {
        loadMore()
      }
    },
    { rootMargin: '200px' }
  )

  if (loadMoreTrigger.value) {
    observer.observe(loadMoreTrigger.value)
  }
}

function handleNewPost() {
  if (activeTab.value === 'latest') {
    const cached = getCachedPosts('latest')
    if (cached) {
      cached.timestamp = 0
    }
    fetchPosts()
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

watch(activeTab, () => {
  page.value = 1
  const cached = getCachedPosts(activeTab.value)
  if (cached) {
    posts.value = cached.posts
    hasMore.value = posts.value.length < cached.total
    loading.value = false
  } else {
    fetchPosts()
  }
  nextTick(() => {
    setupInfiniteScroll()
  })
})

onMounted(() => {
  fetchPosts()
  fetchTags()
  fetchAnnouncement()
  window.addEventListener('new-post', handleNewPost)
  nextTick(() => {
    setupInfiniteScroll()
  })
})

onUnmounted(() => {
  window.removeEventListener('new-post', handleNewPost)
  if (observer) {
    observer.disconnect()
  }
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

          <div
            ref="loadMoreTrigger"
            class="h-10 flex items-center justify-center"
          >
            <div v-if="loadingMore" class="flex items-center space-x-2 text-slate-500">
              <svg class="animate-spin h-5 w-5" fill="none" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
              </svg>
              <span>Loading more...</span>
            </div>
            <span v-else-if="!hasMore && posts.length > 0" class="text-slate-400">
              No more posts
            </span>
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
