import { ref } from 'vue'
import api from './useApi'

const prefetchedData = ref<Record<string, any>>({})
const prefetchedRoutes = ref<Set<string>>(new Set())
const prefetchTimestamp = ref<Record<string, number>>({})
const PREFETCH_DURATION = 30000

export function usePrefetch() {
  async function prefetchTags() {
    if (prefetchedRoutes.value.has('tags')) {
      if (Date.now() - (prefetchTimestamp.value['tags'] || 0) < PREFETCH_DURATION) {
        return
      }
    }
    prefetchedRoutes.value.add('tags')
    prefetchTimestamp.value['tags'] = Date.now()
    try {
      const response = await api.get('/tags', { params: { page: 1, limit: 50, sort: 'popular' } })
      prefetchedData.value['tags'] = response.data
    } catch {}
  }

  async function prefetchHome() {
    if (prefetchedRoutes.value.has('home')) {
      if (Date.now() - (prefetchTimestamp.value['home'] || 0) < PREFETCH_DURATION) {
        return
      }
    }
    prefetchedRoutes.value.add('home')
    prefetchTimestamp.value['home'] = Date.now()
    try {
      const [latestResponse, hotResponse] = await Promise.allSettled([
        api.get('/posts', { params: { page: 1, limit: 20 } }),
        api.get('/posts/hot', { params: { page: 1, limit: 20 } }),
      ])

      if (latestResponse.status === 'fulfilled') {
        prefetchedData.value['home-latest'] = latestResponse.value.data
      }
      if (hotResponse.status === 'fulfilled') {
        prefetchedData.value['home-hot'] = hotResponse.value.data
      }
    } catch {}
  }

  async function prefetchTagPosts(tagName: string) {
    const key = `tag-${tagName}`
    if (prefetchedRoutes.value.has(key)) {
      if (Date.now() - (prefetchTimestamp.value[key] || 0) < PREFETCH_DURATION) {
        return
      }
    }
    prefetchedRoutes.value.add(key)
    prefetchTimestamp.value[key] = Date.now()
    try {
      const response = await api.get(`/tags/${tagName}/posts`, { params: { page: 1, limit: 20 } })
      prefetchedData.value[key] = response.data
    } catch {}
  }

  function getPrefetchedData(key: string) {
    const data = prefetchedData.value[key]
    delete prefetchedData.value[key]
    prefetchedRoutes.value.delete(key)
    delete prefetchTimestamp.value[key]
    return data
  }

  function hasPrefetchedData(key: string) {
    return prefetchedRoutes.value.has(key) && !!prefetchedData.value[key]
  }

  function prefetchOnHover(route: string) {
    if (route === '/tags' || route === '/tags/') {
      prefetchTags()
    } else if (route === '/' || route === '') {
      prefetchHome()
    } else if (route.startsWith('/tags/')) {
      const tagName = route.split('/tags/')[1]
      if (tagName) {
        prefetchTagPosts(tagName)
      }
    }
  }

  return {
    prefetchTags,
    prefetchHome,
    prefetchTagPosts,
    prefetchOnHover,
    getPrefetchedData,
    hasPrefetchedData,
  }
}