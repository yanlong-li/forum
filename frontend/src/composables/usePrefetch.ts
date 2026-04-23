import { ref } from 'vue'
import api from './useApi'

const prefetchedData = ref<Record<string, any>>({})
const prefetchedRoutes = ref<Set<string>>(new Set())

export function usePrefetch() {
  async function prefetchTags() {
    if (prefetchedRoutes.value.has('tags')) return
    prefetchedRoutes.value.add('tags')
    try {
      const response = await api.get('/tags', { params: { page: 1, limit: 50, sort: 'popular' } })
      prefetchedData.value['tags'] = response.data
    } catch {}
  }

  async function prefetchHome() {
    if (prefetchedRoutes.value.has('home')) return
    prefetchedRoutes.value.add('home')
    try {
      const response = await api.get('/posts', { params: { page: 1, limit: 20 } })
      prefetchedData.value['home'] = response.data
    } catch {}
  }

  function getPrefetchedData(key: string) {
    const data = prefetchedData.value[key]
    delete prefetchedData.value[key]
    prefetchedRoutes.value.delete(key)
    return data
  }

  function hasPrefetchedData(key: string) {
    return prefetchedRoutes.value.has(key)
  }

  function prefetchOnHover(route: string) {
    if (route === '/tags' || route === '/tags/') {
      prefetchTags()
    } else if (route === '/' || route === '') {
      prefetchHome()
    }
  }

  return {
    prefetchTags,
    prefetchHome,
    prefetchOnHover,
    getPrefetchedData,
    hasPrefetchedData,
  }
}
