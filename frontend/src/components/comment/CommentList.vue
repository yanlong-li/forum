<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { RouterLink } from 'vue-router'
import api from '../../composables/useApi'
import { useAuthStore } from '../../stores/auth'
import { useToast } from '../../composables/useToast'
import { formatDistanceToNow } from '../../utils/time'
import Loading from '../common/Loading.vue'

const props = defineProps<{
  postId: string
}>()

const authStore = useAuthStore()
const toast = useToast()

const comments = ref<any[]>([])
const loading = ref(true)
const page = ref(1)
const hasMore = ref(true)

async function fetchComments() {
  try {
    const response = await api.get(`/comments/posts/${props.postId}/comments`, {
      params: { page: page.value, limit: 20 },
    })
    const data = response.data

    if (page.value === 1) {
      comments.value = data.comments
    } else {
      comments.value.push(...data.comments)
    }

    hasMore.value = comments.value.length < data.total
  } catch (err: any) {
    toast.error('Failed to load comments')
  } finally {
    loading.value = false
  }
}

async function handleReply(parentId: string) {
  const content = prompt('Enter your reply:')
  if (!content?.trim()) return

  try {
    await api.post(`/comments/posts/${props.postId}/comments`, {
      content,
      parent_id: parentId,
    })
    toast.success('Reply added')
    await fetchComments()
  } catch (err: any) {
    toast.error('Failed to add reply')
  }
}

async function handleVote(commentId: string, value: number) {
  if (!authStore.isAuthenticated) {
    toast.info('Please login to vote')
    return
  }

  try {
    await api.post('/votes', {
      comment_id: commentId,
      value,
    })
    await fetchComments()
  } catch (err: any) {
    toast.error('Failed to vote')
  }
}

function loadMore() {
  page.value++
  fetchComments()
}

onMounted(() => {
  fetchComments()
})
</script>

<template>
  <div class="space-y-6">
    <Loading v-if="loading" />

    <div v-else-if="comments.length === 0" class="card p-6 text-center text-slate-500">
      No comments yet. Be the first to comment!
    </div>

    <div v-else>
      <div
        v-for="comment in comments"
        :key="comment.id"
        class="card p-6"
      >
        <div class="flex items-start space-x-4">
          <RouterLink :to="`/profile/${comment.author.username}`" class="flex-shrink-0">
            <img
              v-if="comment.author.avatar_url"
              :src="comment.author.avatar_url"
              :alt="comment.author.username"
              class="w-10 h-10 rounded-full object-cover"
            />
            <div
              v-else
              class="w-10 h-10 rounded-full bg-primary text-white flex items-center justify-center font-medium"
            >
              {{ comment.author.username.charAt(0).toUpperCase() }}
            </div>
          </RouterLink>

          <div class="flex-1">
            <div class="flex items-center space-x-2">
              <RouterLink
                :to="`/profile/${comment.author.username}`"
                class="font-medium text-slate-900 hover:text-primary"
              >
                {{ comment.author.username }}
              </RouterLink>
              <span class="text-slate-400">·</span>
              <span class="text-slate-500 text-sm">
                {{ formatDistanceToNow(new Date(comment.created_at)) }}
              </span>
              <span v-if="comment.updated_at !== comment.created_at" class="text-slate-400 text-sm">
                (edited)
              </span>
            </div>

            <p v-if="!comment.is_deleted" class="mt-2 text-slate-700">
              {{ comment.content }}
            </p>
            <p v-else class="mt-2 text-slate-400 italic">[deleted]</p>

            <div class="mt-3 flex items-center space-x-4">
              <button
                @click="handleVote(comment.id, 1)"
                class="flex items-center space-x-1 text-sm transition-colors"
                :class="comment.is_liked ? 'text-primary' : 'text-slate-500 hover:text-primary'"
              >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 10h4.764a2 2 0 011.789 2.894l-3.5 7A2 2 0 0115.263 21h-4.017c-.163 0-.326-.02-.485-.06L7 20m7-10V5a2 2 0 00-2-2h-.095c-.5 0-.905.405-.905.905 0 .714-.211 1.412-.608 2.006L7 11v9m7-10h-2M7 20H5a2 2 0 01-2-2v-6a2 2 0 012-2h2.5" />
                </svg>
                <span>{{ comment.like_count }}</span>
              </button>

              <button
                v-if="authStore.isAuthenticated && !comment.is_deleted"
                @click="handleReply(comment.id)"
                class="text-sm text-slate-500 hover:text-primary transition-colors"
              >
                Reply
              </button>
            </div>

            <div v-if="comment.reply_count > 0" class="mt-4 ml-4 space-y-4 border-l-2 border-slate-100 pl-4">
              <p class="text-sm text-slate-500">{{ comment.reply_count }} replies</p>
            </div>
          </div>
        </div>
      </div>

      <div v-if="hasMore" class="flex justify-center mt-6">
        <button @click="loadMore" class="btn btn-secondary">
          Load More
        </button>
      </div>
    </div>
  </div>
</template>
