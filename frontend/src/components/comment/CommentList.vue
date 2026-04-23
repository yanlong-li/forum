<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { RouterLink } from 'vue-router'
import { useI18n } from 'vue-i18n'
import api from '../../composables/useApi'
import { useAuthStore } from '../../stores/auth'
import { useToast } from '../../composables/useToast'
import { formatDistanceToNow } from '../../utils/time'
import Loading from '../common/Loading.vue'
import MarkdownRenderer from '../editor/MarkdownRenderer.vue'
import MarkdownEditor from '../editor/MarkdownEditor.vue'
import ReportModal from '../report/ReportModal.vue'

const { t } = useI18n()

const props = defineProps<{
  postId: string
  postAuthorId?: string
  refreshKey?: number
}>()

const authStore = useAuthStore()
const toast = useToast()

const comments = ref<any[]>([])
const loading = ref(true)
const page = ref(1)
const hasMore = ref(true)
const replyingTo = ref<string | null>(null)
const replyContent = ref('')
const submitting = ref(false)
const showReportModal = ref(false)
const reportingCommentId = ref<string | undefined>(undefined)

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
    toast.error(t('error.comment.loadFailed'))
  } finally {
    loading.value = false
  }
}

async function handleReply(parentId: string) {
  replyingTo.value = parentId
  replyContent.value = ''
}

async function submitReply(parentId: string) {
  if (!replyContent.value.trim()) return

  submitting.value = true
  try {
    await api.post(`/comments/posts/${props.postId}/comments`, {
      content: replyContent.value,
      parent_id: parentId,
    })
    toast.success(t('comment.replyAdded'))
    replyContent.value = ''
    replyingTo.value = null
    await fetchComments()
  } catch (err: any) {
    toast.error(t('error.comment.addFailed'))
  } finally {
    submitting.value = false
  }
}

function cancelReply() {
  replyingTo.value = null
  replyContent.value = ''
}

function openReportModal(commentId: string) {
  if (!authStore.isAuthenticated) {
    toast.info(t('auth.loginRequired'))
    return
  }
  reportingCommentId.value = commentId
  showReportModal.value = true
}

async function handleAccept(commentId: string) {
  if (!authStore.isAuthenticated) {
    toast.info(t('auth.loginRequired'))
    return
  }

  try {
    await api.post(`/comments/${commentId}/accept`)
    toast.success(t('comment.acceptedBestAnswer'))
    await fetchComments()
  } catch (err: any) {
    toast.error(t('comment.acceptFailed'))
  }
}

async function handleVote(commentId: string, value: number) {
  if (!authStore.isAuthenticated) {
    toast.info(t('auth.loginRequired'))
    return
  }

  try {
    await api.post('/votes', {
      comment_id: commentId,
      value,
    })
    await fetchComments()
  } catch (err: any) {
    toast.error(t('error.vote.failed'))
  }
}

function loadMore() {
  page.value++
  fetchComments()
}

onMounted(() => {
  fetchComments()
})

watch(() => props.postId, () => {
  page.value = 1
  fetchComments()
})

watch(() => props.refreshKey, () => {
  page.value = 1
  fetchComments()
})
</script>

<template>
  <div class="space-y-6">
    <Loading v-if="loading" />

    <div v-else-if="comments.length === 0" class="card p-6 text-center text-slate-500">
      {{ t('comment.noComments') }}
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
                ({{ t('comment.edited') }})
              </span>
            </div>

            <div v-if="!comment.is_deleted" class="mt-2 text-slate-700">
              <MarkdownRenderer :content="comment.content" />
            </div>
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
                {{ t('comment.reply') }}
              </button>

              <button
                v-if="authStore.isAuthenticated && !comment.is_deleted"
                @click="openReportModal(comment.id)"
                class="text-sm text-slate-500 hover:text-red-500 transition-colors"
              >
                {{ t('comment.report') }}
              </button>

              <button
                v-if="authStore.user?.id === postAuthorId && !comment.is_deleted"
                @click="handleAccept(comment.id)"
                class="flex items-center space-x-1 text-sm transition-colors"
                :class="comment.is_accepted ? 'text-green-600' : 'text-slate-500 hover:text-green-600'"
              >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                </svg>
                <span>{{ comment.is_accepted ? t('comment.accepted') : t('comment.accept') }}</span>
              </button>
            </div>

            <div v-if="replyingTo === comment.id" class="mt-4 p-4 bg-slate-50 rounded-lg">
              <MarkdownEditor
                v-model="replyContent"
                :placeholder="t('comment.writeReplyPlaceholder')"
                min-height="100px"
              />
              <div class="mt-3 flex justify-end space-x-2">
                <button
                  @click="cancelReply"
                  class="btn btn-secondary"
                >
                  {{ t('common.cancel') }}
                </button>
                <button
                  @click="submitReply(comment.id)"
                  :disabled="submitting || !replyContent.trim()"
                  class="btn btn-primary"
                >
                  {{ submitting ? t('comment.posting') : t('comment.postReply') }}
                </button>
              </div>
            </div>

            <div v-if="comment.reply_count > 0" class="mt-4 ml-4 space-y-4 border-l-2 border-slate-100 pl-4">
              <p class="text-sm text-slate-500">{{ comment.reply_count }} {{ t('comment.replies') }}</p>
            </div>
          </div>
        </div>
      </div>

      <div v-if="hasMore" class="flex justify-center mt-6">
        <button @click="loadMore" class="btn btn-secondary">
          {{ t('common.loadMore') }}
        </button>
      </div>
    </div>

    <ReportModal
      :visible="showReportModal"
      :comment-id="reportingCommentId"
      @close="showReportModal = false; reportingCommentId = undefined"
    />
  </div>
</template>
