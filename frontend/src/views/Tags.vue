<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { RouterLink } from 'vue-router'
import { useI18n } from 'vue-i18n'
import api from '../composables/useApi'
import Loading from '../components/common/Loading.vue'
import { usePrefetch } from '../composables/usePrefetch'

defineOptions({ name: 'Tags' })

const { t } = useI18n()
const tags = ref<any[]>([])
const loading = ref(true)
const page = ref(1)
const hasMore = ref(true)
const { getPrefetchedData, hasPrefetchedData } = usePrefetch()

async function fetchTags() {
  if (page.value === 1 && hasPrefetchedData('tags')) {
    const data = getPrefetchedData('tags')
    tags.value = data.tags
    hasMore.value = tags.value.length < data.total
    loading.value = false
    return
  }

  try {
    const response = await api.get('/tags', { params: { page: page.value, limit: 50, sort: 'popular' } })
    const data = response.data

    if (page.value === 1) {
      tags.value = data.tags
    } else {
      tags.value.push(...data.tags)
    }

    hasMore.value = tags.value.length < data.total
  } catch (error) {
    console.error(t('tag.loadFailed'), error)
  } finally {
    loading.value = false
  }
}

function loadMore() {
  page.value++
  fetchTags()
}

onMounted(() => {
  fetchTags()
})
</script>

<template>
  <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    <h1 class="text-2xl font-bold text-slate-900 mb-8">{{ t('tag.tags') }}</h1>

    <Loading v-if="loading" />

    <div v-else class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-4">
      <RouterLink
        v-for="tag in tags"
        :key="tag.id"
        :to="`/tags/${tag.name}`"
        class="card p-4 hover:shadow-md hover:border-primary transition-all group"
      >
        <div class="flex items-center justify-between">
          <span class="badge badge-primary text-lg px-3 py-1 group-hover:bg-primary-hover group-hover:text-white">
            {{ tag.name }}
          </span>
          <span class="text-sm text-slate-500">
            {{ tag.post_count }} {{ t('tag.posts') }}
          </span>
        </div>
      </RouterLink>

      <div v-if="tags.length === 0" class="col-span-full card p-8 text-center text-slate-500">
        {{ t('tag.noTagsYet') }}
      </div>
    </div>

    <div v-if="hasMore" class="flex justify-center mt-8">
      <button @click="loadMore" class="btn btn-secondary">
        {{ t('tag.loadMore') }}
      </button>
    </div>
  </div>
</template>
