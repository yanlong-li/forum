<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute, RouterLink } from 'vue-router'
import api from '../composables/useApi'
import { useAuthStore } from '../stores/auth'
import { useToast } from '../composables/useToast'
import { formatDate } from '../utils/time'
import PostCard from '../components/post/PostCard.vue'
import Loading from '../components/common/Loading.vue'

const route = useRoute()
const authStore = useAuthStore()
const toast = useToast()

const profile = ref<any>(null)
const posts = ref<any[]>([])
const loading = ref(true)
const activeTab = ref<'posts' | 'followers' | 'following'>('posts')
const followLoading = ref(false)

async function fetchProfile() {
  try {
    const response = await api.get(`/users/${route.params.username}`)
    profile.value = response.data
  } catch (err: any) {
    toast.error('Failed to load profile')
  } finally {
    loading.value = false
  }
}

async function fetchPosts() {
  try {
    const response = await api.get(`/users/${route.params.username}/posts`)
    posts.value = response.data.posts
  } catch (err: any) {
    toast.error('Failed to load posts')
  }
}

async function handleFollow() {
  if (!authStore.isAuthenticated) {
    toast.info('Please login to follow')
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
    toast.success(profile.value.is_following ? 'Followed' : 'Unfollowed')
  } catch (err: any) {
    toast.error('Failed to update follow status')
  } finally {
    followLoading.value = false
  }
}

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
              {{ profile.is_following ? 'Following' : 'Follow' }}
            </button>
          </div>

          <p v-if="profile.bio" class="mt-2 text-slate-600">{{ profile.bio }}</p>

          <div class="mt-4 flex items-center space-x-6 text-sm">
            <div class="flex items-center space-x-1">
              <span class="font-semibold text-slate-900">{{ profile.post_count }}</span>
              <span class="text-slate-500">posts</span>
            </div>
            <RouterLink :to="`/profile/${profile.username}?tab=followers`" class="flex items-center space-x-1 hover:text-primary">
              <span class="font-semibold text-slate-900">{{ profile.follower_count }}</span>
              <span class="text-slate-500">followers</span>
            </RouterLink>
            <RouterLink :to="`/profile/${profile.username}?tab=following`" class="flex items-center space-x-1 hover:text-primary">
              <span class="font-semibold text-slate-900">{{ profile.following_count }}</span>
              <span class="text-slate-500">following</span>
            </RouterLink>
          </div>

          <p class="mt-4 text-sm text-slate-500">
            Joined {{ formatDate(new Date(profile.created_at)) }}
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
            Posts
          </button>
          <button
            @click="activeTab = 'followers'"
            class="py-4 px-1 border-b-2 font-medium text-sm transition-colors"
            :class="activeTab === 'followers' ? 'border-primary text-primary' : 'border-transparent text-slate-500 hover:text-slate-700'"
          >
            Followers
          </button>
          <button
            @click="activeTab = 'following'"
            class="py-4 px-1 border-b-2 font-medium text-sm transition-colors"
            :class="activeTab === 'following' ? 'border-primary text-primary' : 'border-transparent text-slate-500 hover:text-slate-700'"
          >
            Following
          </button>
        </nav>
      </div>
    </div>

    <div v-if="activeTab === 'posts'" class="space-y-4">
      <PostCard v-for="post in posts" :key="post.id" :post="post" />

      <div v-if="posts.length === 0" class="card p-8 text-center text-slate-500">
        No posts yet.
      </div>
    </div>

    <div v-else-if="activeTab === 'followers'" class="card p-8 text-center text-slate-500">
      Followers list coming soon.
    </div>

    <div v-else class="card p-8 text-center text-slate-500">
      Following list coming soon.
    </div>
  </div>
</template>
