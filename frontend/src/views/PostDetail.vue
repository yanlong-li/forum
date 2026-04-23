<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute, RouterLink } from 'vue-router'
import api from '../composables/useApi'
import { useAuthStore } from '../stores/auth'
import { useToast } from '../composables/useToast'
import { formatDistanceToNow } from '../utils/time'
import CommentList from '../components/comment/CommentList.vue'
import Loading from '../components/common/Loading.vue'

const route = useRoute()
const authStore = useAuthStore()
const toast = useToast()

const post = ref<any>(null)
const loading = ref(true)
const error = ref('')
const commentContent = ref('')
const submitting = ref(false)

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
    toast.info('Please login to vote')
    return
  }

  try {
    await api.post('/votes', {
      post_id: post.value.id,
      value,
    })
    await fetchPost()
  } catch (err: any) {
    toast.error('Failed to vote')
  }
}

async function handleBookmark() {
  if (!authStore.isAuthenticated) {
    toast.info('Please login to bookmark')
    return
  }

  try {
    if (post.value.is_bookmarked) {
      await api.delete(`/bookmarks/${post.value.id}`)
    } else {
      await api.post('/bookmarks', { post_id: post.value.id })
    }
    post.value.is_bookmarked = !post.value.is_bookmarked
    toast.success(post.value.is_bookmarked ? 'Bookmarked' : 'Removed from bookmarks')
  } catch (err: any) {
    toast.error('Failed to bookmark')
  }
}

async function handleComment() {
  if (!commentContent.value.trim()) return

  submitting.value = true
  try {
    await api.post(`/comments/posts/${route.params.id}/comments`, {
      content: commentContent.value,
    })
    commentContent.value = ''
    toast.success('Comment added')
    await fetchPost()
  } catch (err: any) {
    toast.error('Failed to add comment')
  } finally {
    submitting.value = false
  }
}

onMounted(() => {
  fetchPost()
})
</script>

<template>
  <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    <Loading v-if="loading" />

    <div v-else-if="error" class="card p-8 text-center">
      <p class="text-red-500">{{ error }}</p>
      <RouterLink to="/" class="btn btn-primary mt-4">Go Home</RouterLink>
    </div>

    <article v-else class="card p-8">
      <header class="mb-8">
        <div class="flex items-center space-x-4 mb-4">
          <RouterLink :to="`/profile/${post.author.username}`" class="flex-shrink-0">
            <img
              v-if="post.author.avatar_url"
              :src="post.author.avatar_url"
              :alt="post.author.username"
              class="w-12 h-12 rounded-full object-cover"
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
            <p class="text-sm text-slate-500">
              {{ formatDistanceToNow(new Date(post.created_at)) }}
            </p>
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
        <p class="whitespace-pre-wrap">{{ post.content }}</p>
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
          <span>{{ post.is_bookmarked ? 'Bookmarked' : 'Bookmark' }}</span>
        </button>

        <div class="flex-1"></div>

        <span class="text-sm text-slate-500">
          {{ post.comment_count }} comments
        </span>
      </footer>
    </article>

    <section class="mt-8">
      <h2 class="text-xl font-semibold text-slate-900 mb-4">Comments</h2>

      <div v-if="authStore.isAuthenticated" class="card p-6 mb-6">
        <form @submit.prevent="handleComment">
          <textarea
            v-model="commentContent"
            class="input min-h-[100px] resize-none"
            placeholder="Write a comment..."
          ></textarea>
          <div class="mt-4 flex justify-end">
            <button
              type="submit"
              :disabled="submitting || !commentContent.trim()"
              class="btn btn-primary"
            >
              {{ submitting ? 'Posting...' : 'Post Comment' }}
            </button>
          </div>
        </form>
      </div>

      <CommentList :post-id="post.id" />
    </section>
  </div>
</template>
