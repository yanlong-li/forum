<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { useRoute, RouterLink } from 'vue-router'
import { useI18n } from 'vue-i18n'
import api from '../composables/useApi'
import { useAuthStore } from '../stores/auth'
import { useToast } from '../composables/useToast'
import { formatDistanceToNow } from '../utils/time'
import CommentList from '../components/comment/CommentList.vue'
import Loading from '../components/common/Loading.vue'
import MarkdownRenderer from '../components/editor/MarkdownRenderer.vue'
import MarkdownEditor from '../components/editor/MarkdownEditor.vue'
import ReportModal from '../components/report/ReportModal.vue'
import LevelBadge from '../components/user/LevelBadge.vue'
import ShareButtons from '../components/common/ShareButtons.vue'

const route = useRoute()
const authStore = useAuthStore()
const toast = useToast()
const { t } = useI18n()

const post = ref<any>(null)
const loading = ref(true)
const error = ref('')
const commentContent = ref('')
const submitting = ref(false)
const showReportModal = ref(false)
const commentRefreshKey = ref(0)

async function fetchPost() {
  try {
    const response = await api.get(`/posts/${route.params.id}`)
    post.value = response.data
  } catch (err: any) {
    error.value = err.response?.data?.error?.message || 'Failed to load post'
  } finally {
    loading.value = false
  }
}

async function handleVote(value: number) {
  if (!authStore.isAuthenticated) {
    toast.info(t('error.loginRequired'))
    return
  }

  try {
    await api.post('/votes', {
      post_id: post.value.id,
      value,
    })
    await fetchPost()
  } catch (err: any) {
    toast.error(t('error.vote.failed'))
  }
}

async function handleBookmark() {
  if (!authStore.isAuthenticated) {
    toast.info(t('error.loginRequired'))
    return
  }

  try {
    if (post.value.is_bookmarked) {
      await api.delete(`/bookmarks/${post.value.id}`)
    } else {
      await api.post('/bookmarks', { post_id: post.value.id })
    }
    post.value.is_bookmarked = !post.value.is_bookmarked
    toast.success(post.value.is_bookmarked ? t('post.bookmarked') : t('post.removedFromBookmarks'))
  } catch (err: any) {
    toast.error(t('error.bookmark.failed'))
  }
}

function handleReport() {
  if (!authStore.isAuthenticated) {
    toast.info(t('error.loginRequired'))
    return
  }
  showReportModal.value = true
}

async function handleComment() {
  if (!commentContent.value.trim()) return

  submitting.value = true
  try {
    await api.post(`/comments/posts/${route.params.id}/comments`, {
      content: commentContent.value,
    })
    commentContent.value = ''
    toast.success(t('post.comment') + ' ' + t('common.success'))
    commentRefreshKey.value++
    await fetchPost()
  } catch (err: any) {
    toast.error(t('error.comment.addFailed'))
  } finally {
    submitting.value = false
  }
}

function handleNewComment(event: CustomEvent) {
  const payload = event.detail
  if (payload.post_id === route.params.id) {
    commentRefreshKey.value++
    if (post.value) {
      post.value.comment_count = (post.value.comment_count || 0) + 1
    }
  }
}

onMounted(() => {
  fetchPost()
  window.addEventListener('new-comment', handleNewComment as EventListener)
})

onUnmounted(() => {
  window.removeEventListener('new-comment', handleNewComment as EventListener)
})
</script>

<template>
  <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    <Loading v-if="loading" />

    <div v-else-if="error" class="card p-8 text-center">
      <p class="text-red-500">{{ error }}</p>
      <RouterLink to="/" class="btn btn-primary mt-4">{{ t('post.goHome') }}</RouterLink>
    </div>

    <article v-if="post" class="card p-8">
      <header class="mb-8">
        <div class="flex items-center space-x-4 mb-4">
          <RouterLink :to="`/profile/${post.author.username}`" class="flex-shrink-0">
            <img
              v-if="post.author.avatar_url"
              :src="post.author.avatar_url"
              :alt="post.author.username"
              class="w-12 h-12 rounded-full object-cover"
              loading="lazy"
            />
            <div
              v-else
              class="w-12 h-12 rounded-full bg-primary text-white flex items-center justify-center text-lg font-medium"
            >
              {{ post.author.username.charAt(0).toUpperCase() }}
            </div>
          </RouterLink>

          <div>
            <RouterLink
              :to="`/profile/${post.author.username}`"
              class="font-medium text-slate-900 hover:text-primary"
            >
              {{ post.author.username }}
            </RouterLink>
            <div class="flex items-center space-x-2">
              <LevelBadge :level="post.author.level" :points="post.author.points" show-points />
              <span class="text-slate-400">·</span>
              <p class="text-sm text-slate-500">
                {{ formatDistanceToNow(new Date(post.created_at)) }}
              </p>
            </div>
          </div>
        </div>

        <h1 class="text-3xl font-bold text-slate-900 mb-4">{{ post.title }}</h1>

        <div class="flex items-center space-x-2">
          <RouterLink
            v-for="tag in post.tags"
            :key="tag"
            :to="`/tags/${tag}`"
            class="badge badge-primary hover:bg-primary-hover"
          >
            {{ tag }}
          </RouterLink>
        </div>
      </header>

      <div class="prose prose-slate max-w-none mb-8">
        <MarkdownRenderer :content="post.content" />
      </div>

      <footer class="flex items-center space-x-6 border-t border-slate-200 pt-6">
        <button
          @click="handleVote(1)"
          class="flex items-center space-x-2 px-3 py-2 rounded-lg transition-colors"
          :class="post.is_liked ? 'text-primary bg-blue-50' : 'text-slate-500 hover:bg-slate-100'"
        >
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 10h4.764a2 2 0 011.789 2.894l-3.5 7A2 2 0 0115.263 21h-4.017c-.163 0-.326-.02-.485-.06L7 20m7-10V5a2 2 0 00-2-2h-.095c-.5 0-.905.405-.905.905 0 .714-.211 1.412-.608 2.006L7 11v9m7-10h-2M7 20H5a2 2 0 01-2-2v-6a2 2 0 012-2h2.5" />
          </svg>
          <span>{{ post.like_count }}</span>
        </button>

        <button
          @click="handleBookmark"
          class="flex items-center space-x-2 px-3 py-2 rounded-lg transition-colors"
          :class="post.is_bookmarked ? 'text-accent bg-amber-50' : 'text-slate-500 hover:bg-slate-100'"
        >
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 5a2 2 0 012-2h10a2 2 0 012 2v16l-7-3.5L5 21V5z" />
          </svg>
          <span>{{ post.is_bookmarked ? t('post.bookmarked') : t('post.bookmark') }}</span>
        </button>

        <button
          @click="handleReport"
          class="flex items-center space-x-2 px-3 py-2 rounded-lg text-slate-500 hover:bg-slate-100 transition-colors"
        >
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
          </svg>
          <span>{{ t('post.report') }}</span>
        </button>

        <ShareButtons :title="post.title" />

        <div class="flex-1"></div>

        <span class="text-sm text-slate-500">
          {{ post.comment_count }} {{ t('post.comments') }}
        </span>
      </footer>
    </article>

    <section v-if="post" class="mt-8">
      <h2 class="text-xl font-semibold text-slate-900 mb-4">{{ t('post.comments') }}</h2>

      <div v-if="authStore.isAuthenticated" class="card p-6 mb-6">
        <form @submit.prevent="handleComment">
          <MarkdownEditor
            v-model="commentContent"
            :placeholder="t('post.writeCommentPlaceholder')"
            min-height="100px"
          />
          <div class="mt-4 flex justify-end">
            <button
              type="submit"
              :disabled="submitting || !commentContent.trim()"
              class="btn btn-primary"
            >
              {{ submitting ? t('post.postingComment') : t('post.postComment') }}
            </button>
          </div>
        </form>
      </div>

      <CommentList :post-id="post.id" :post-author-id="post.author_id" :refresh-key="commentRefreshKey" />
    </section>

    <ReportModal
      v-if="post"
      :visible="showReportModal"
      :post-id="post.id"
      @close="showReportModal = false"
    />
  </div>
</template>
