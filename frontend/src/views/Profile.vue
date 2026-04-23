<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { useRoute, RouterLink } from 'vue-router'
import { useI18n } from 'vue-i18n'
import api from '../composables/useApi'
import { useAuthStore } from '../stores/auth'
import { useToast } from '../composables/useToast'
import { formatDate } from '../utils/time'
import PostCard from '../components/post/PostCard.vue'
import Loading from '../components/common/Loading.vue'

const route = useRoute()
const authStore = useAuthStore()
const toast = useToast()
const { t } = useI18n()

const profile = ref<any>(null)
const posts = ref<any[]>([])
const comments = ref<any[]>([])
const followers = ref<any[]>([])
const following = ref<any[]>([])
const badges = ref<any[]>([])
const loading = ref(true)
const activeTab = ref<'posts' | 'comments' | 'followers' | 'following' | 'badges'>('posts')
const followLoading = ref(false)
const listLoading = ref(false)

async function fetchProfile() {
  try {
    const response = await api.get(`/users/${route.params.username}`)
    profile.value = response.data
  } catch (err: any) {
    toast.error(t('user.loadingFailed'))
  } finally {
    loading.value = false
  }
}

async function fetchPosts() {
  try {
    const response = await api.get(`/users/${route.params.username}/posts`)
    posts.value = response.data.posts
  } catch (err: any) {
    toast.error(t('user.loadPostsFailed'))
  }
}

async function fetchFollowers() {
  listLoading.value = true
  try {
    const response = await api.get(`/users/${route.params.username}/followers`)
    followers.value = response.data.users
  } catch (err: any) {
    toast.error(t('user.loadFollowersFailed'))
  } finally {
    listLoading.value = false
  }
}

async function fetchFollowing() {
  listLoading.value = true
  try {
    const response = await api.get(`/users/${route.params.username}/following`)
    following.value = response.data.users
  } catch (err: any) {
    toast.error(t('user.loadFollowingFailed'))
  } finally {
    listLoading.value = false
  }
}

async function fetchBadges() {
  if (!profile.value) return
  listLoading.value = true
  try {
    const response = await api.get(`/badges/user/${profile.value.id}`)
    badges.value = response.data
  } catch (err: any) {
    toast.error(t('user.loadBadgesFailed'))
  } finally {
    listLoading.value = false
  }
}

async function fetchComments() {
  if (!profile.value) return
  listLoading.value = true
  try {
    const response = await api.get(`/comments/user/${profile.value.id}`, { params: { page: 1, limit: 20 } })
    comments.value = response.data.comments
  } catch (err: any) {
    toast.error(t('user.loadCommentsFailed'))
  } finally {
    listLoading.value = false
  }
}

async function handleFollow() {
  if (!authStore.isAuthenticated) {
    toast.info(t('error.loginRequired'))
    return
  }

  followLoading.value = true
  try {
    if (profile.value.is_following) {
      await api.delete(`/follow/${profile.value.id}`)
    } else {
      await api.post(`/follow/${profile.value.id}`)
    }
    profile.value.is_following = !profile.value.is_following
    profile.value.follower_count += profile.value.is_following ? 1 : -1
    toast.success(profile.value.is_following ? t('user.follow') : t('user.unfollow'))
  } catch (err: any) {
    toast.error(t('user.followStatusUpdateFailed'))
  } finally {
    followLoading.value = false
  }
}

watch(activeTab, (newTab) => {
  if (newTab === 'followers' && followers.value.length === 0) {
    fetchFollowers()
  } else if (newTab === 'following' && following.value.length === 0) {
    fetchFollowing()
  } else if (newTab === 'badges' && badges.value.length === 0) {
    fetchBadges()
  } else if (newTab === 'comments' && comments.value.length === 0) {
    fetchComments()
  }
})

watch(() => route.params.username, () => {
  fetchProfile()
  fetchPosts()
  activeTab.value = 'posts'
  followers.value = []
  following.value = []
  badges.value = []
  comments.value = []
})

onMounted(() => {
  fetchProfile()
  fetchPosts()
})
</script>

<template>
  <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    <Loading v-if="loading" />

    <div v-else class="card p-8 mb-8">
      <div class="flex items-start space-x-6">
        <img
          v-if="profile.avatar_url"
          :src="profile.avatar_url"
          :alt="profile.username"
          class="w-24 h-24 rounded-full object-cover"
        />
        <div
          v-else
          class="w-24 h-24 rounded-full bg-primary text-white flex items-center justify-center text-3xl font-medium"
        >
          {{ profile.username.charAt(0).toUpperCase() }}
        </div>

        <div class="flex-1">
          <div class="flex items-center justify-between">
            <h1 class="text-2xl font-bold text-slate-900">{{ profile.username }}</h1>

            <button
              v-if="authStore.user?.username !== profile.username"
              @click="handleFollow"
              :disabled="followLoading"
              class="btn"
              :class="profile.is_following ? 'btn-secondary' : 'btn-primary'"
            >
              {{ profile.is_following ? t('user.followingLower') : t('user.follow') }}
            </button>
          </div>

          <p v-if="profile.bio" class="mt-2 text-slate-600">{{ profile.bio }}</p>

          <div class="mt-4 flex items-center space-x-6 text-sm">
            <div class="flex items-center space-x-1">
              <span class="font-semibold text-slate-900">{{ profile.post_count }}</span>
              <span class="text-slate-500">{{ t('user.posts') }}</span>
            </div>
            <RouterLink :to="`/profile/${profile.username}?tab=followers`" class="flex items-center space-x-1 hover:text-primary">
              <span class="font-semibold text-slate-900">{{ profile.follower_count }}</span>
              <span class="text-slate-500">{{ t('user.followersLower') }}</span>
            </RouterLink>
            <RouterLink :to="`/profile/${profile.username}?tab=following`" class="flex items-center space-x-1 hover:text-primary">
              <span class="font-semibold text-slate-900">{{ profile.following_count }}</span>
              <span class="text-slate-500">{{ t('user.following') }}</span>
            </RouterLink>
          </div>

          <p class="mt-4 text-sm text-slate-500">
            {{ t('user.joined') }} {{ formatDate(new Date(profile.created_at)) }}
          </p>
        </div>
      </div>
    </div>

    <div class="mb-6">
      <div class="border-b border-slate-200">
        <nav class="flex space-x-8">
          <button
            @click="activeTab = 'posts'"
            class="py-4 px-1 border-b-2 font-medium text-sm transition-colors"
            :class="activeTab === 'posts' ? 'border-primary text-primary' : 'border-transparent text-slate-500 hover:text-slate-700'"
          >
            {{ t('user.posts') }}
          </button>
          <button
            @click="activeTab = 'comments'"
            class="py-4 px-1 border-b-2 font-medium text-sm transition-colors"
            :class="activeTab === 'comments' ? 'border-primary text-primary' : 'border-transparent text-slate-500 hover:text-slate-700'"
          >
            {{ t('user.comments') }}
          </button>
          <button
            @click="activeTab = 'followers'"
            class="py-4 px-1 border-b-2 font-medium text-sm transition-colors"
            :class="activeTab === 'followers' ? 'border-primary text-primary' : 'border-transparent text-slate-500 hover:text-slate-700'"
          >
            {{ t('user.followers') }}
          </button>
          <button
            @click="activeTab = 'following'"
            class="py-4 px-1 border-b-2 font-medium text-sm transition-colors"
            :class="activeTab === 'following' ? 'border-primary text-primary' : 'border-transparent text-slate-500 hover:text-slate-700'"
          >
            {{ t('user.following') }}
          </button>
          <button
            @click="activeTab = 'badges'"
            class="py-4 px-1 border-b-2 font-medium text-sm transition-colors"
            :class="activeTab === 'badges' ? 'border-primary text-primary' : 'border-transparent text-slate-500 hover:text-slate-700'"
          >
            {{ t('user.badges') }}
          </button>
        </nav>
      </div>
    </div>

    <div v-if="activeTab === 'posts'" class="space-y-4">
      <PostCard v-for="post in posts" :key="post.id" :post="post" />

      <div v-if="posts.length === 0" class="card p-8 text-center text-slate-500">
        {{ t('user.noPostsYet') }}
      </div>
    </div>

    <div v-else-if="activeTab === 'comments'" class="space-y-4">
      <div v-if="comments.length === 0" class="card p-8 text-center text-slate-500">
        {{ t('user.noCommentsYet') }}
      </div>
      <div v-else class="card p-6" v-for="comment in comments" :key="comment.id">
        <p class="text-slate-700 mb-2">{{ comment.content }}</p>
        <RouterLink :to="`/post/${comment.post_id}`" class="text-sm text-primary hover:underline">
          {{ t('user.viewPost') }}
        </RouterLink>
        <p class="text-xs text-slate-500 mt-1">{{ formatDate(new Date(comment.created_at)) }}</p>
      </div>
    </div>

    <div v-else-if="activeTab === 'followers'">
      <Loading v-if="listLoading" />
      <div v-else-if="followers.length === 0" class="card p-8 text-center text-slate-500">
        {{ t('user.noFollowersYet') }}
      </div>
      <div v-else class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div v-for="user in followers" :key="user.id" class="card p-4 flex items-center space-x-4">
          <RouterLink :to="`/profile/${user.username}`">
            <img
              v-if="user.avatar_url"
              :src="user.avatar_url"
              :alt="user.username"
              class="w-12 h-12 rounded-full object-cover"
            />
            <div
              v-else
              class="w-12 h-12 rounded-full bg-primary text-white flex items-center justify-center font-medium"
            >
              {{ user.username.charAt(0).toUpperCase() }}
            </div>
          </RouterLink>
          <div class="flex-1 min-w-0">
            <RouterLink :to="`/profile/${user.username}`" class="font-medium text-slate-900 hover:text-primary">
              {{ user.username }}
            </RouterLink>
            <p v-if="user.bio" class="text-sm text-slate-500 truncate">{{ user.bio }}</p>
          </div>
        </div>
      </div>
    </div>

    <div v-else-if="activeTab === 'following'">
      <Loading v-if="listLoading" />
      <div v-else-if="following.length === 0" class="card p-8 text-center text-slate-500">
        {{ t('user.notFollowingAnyoneYet') }}
      </div>
      <div v-else class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div v-for="user in following" :key="user.id" class="card p-4 flex items-center space-x-4">
          <RouterLink :to="`/profile/${user.username}`">
            <img
              v-if="user.avatar_url"
              :src="user.avatar_url"
              :alt="user.username"
              class="w-12 h-12 rounded-full object-cover"
            />
            <div
              v-else
              class="w-12 h-12 rounded-full bg-primary text-white flex items-center justify-center font-medium"
            >
              {{ user.username.charAt(0).toUpperCase() }}
            </div>
          </RouterLink>
          <div class="flex-1 min-w-0">
            <RouterLink :to="`/profile/${user.username}`" class="font-medium text-slate-900 hover:text-primary">
              {{ user.username }}
            </RouterLink>
            <p v-if="user.bio" class="text-sm text-slate-500 truncate">{{ user.bio }}</p>
          </div>
        </div>
      </div>
    </div>

    <div v-else-if="activeTab === 'badges'">
      <Loading v-if="listLoading" />
      <div v-else-if="badges.length === 0" class="card p-8 text-center text-slate-500">
        {{ t('user.noBadgesEarnedYet') }}
      </div>
      <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        <div v-for="badge in badges" :key="badge.id" class="card p-6 text-center">
          <div class="w-16 h-16 mx-auto mb-4 rounded-full bg-gradient-to-br from-amber-400 to-orange-500 flex items-center justify-center text-3xl">
            {{ badge.icon || '🏅' }}
          </div>
          <h3 class="font-semibold text-slate-900">{{ badge.name }}</h3>
          <p v-if="badge.description" class="text-sm text-slate-500 mt-1">{{ badge.description }}</p>
          <p class="text-xs text-slate-400 mt-2">Earned {{ formatDate(new Date(badge.earned_at)) }}</p>
        </div>
      </div>
    </div>
  </div>
</template>
