<script setup lang="ts">
import { computed } from 'vue'
import { RouterLink } from 'vue-router'
import type { PostSummary } from '../../types'
import { formatDistanceToNow } from '../../utils/time'
import LevelBadge from '../user/LevelBadge.vue'

const props = defineProps<{
  post: PostSummary
}>()

const timeAgo = computed(() => {
  return formatDistanceToNow(new Date(props.post.created_at))
})
</script>

<template>
  <article class="card card-hover p-6">
    <div class="flex items-start space-x-4">
      <RouterLink :to="`/profile/${post.author.username}`" class="flex-shrink-0">
        <img
          v-if="post.author.avatar_url"
          :src="post.author.avatar_url"
          :alt="post.author.username"
          class="w-10 h-10 rounded-full object-cover"
        />
        <div
          v-else
          class="w-10 h-10 rounded-full bg-primary text-white flex items-center justify-center font-medium"
        >
          {{ post.author.username.charAt(0).toUpperCase() }}
        </div>
      </RouterLink>

      <div class="flex-1 min-w-0">
        <div class="flex items-center space-x-2 text-sm">
          <span v-if="post.is_pinned" class="text-red-500 font-medium">📌 Pinned</span>
          <span v-if="post.is_featured" class="text-amber-500 font-medium">⭐ Featured</span>
          <RouterLink
            :to="`/profile/${post.author.username}`"
            class="font-medium text-slate-900 hover:text-primary"
          >
            {{ post.author.username }}
          </RouterLink>
          <LevelBadge :level="post.author.level" :points="post.author.points" />
          <span class="text-slate-400">·</span>
          <span class="text-slate-500">{{ timeAgo }}</span>
        </div>

        <RouterLink :to="`/post/${post.id}`" class="block mt-1">
          <h2 class="text-lg font-semibold text-slate-900 hover:text-primary transition-colors line-clamp-2">
            {{ post.title }}
          </h2>
        </RouterLink>

        <p class="mt-2 text-slate-600 line-clamp-2">
          {{ post.content }}
        </p>

        <div class="mt-4 flex items-center space-x-4">
          <div class="flex items-center space-x-1 text-slate-500 text-sm">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" />
            </svg>
            <span>{{ post.like_count }}</span>
          </div>

          <div class="flex items-center space-x-1 text-slate-500 text-sm">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
            </svg>
            <span>{{ post.comment_count }}</span>
          </div>

          <div class="flex items-center space-x-1 text-slate-500 text-sm">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
            </svg>
            <span>{{ post.view_count || 0 }}</span>
          </div>

          <div class="flex-1"></div>

          <div class="flex items-center space-x-2">
            <RouterLink
              v-for="tag in post.tags.slice(0, 3)"
              :key="tag"
              :to="`/tags/${tag}`"
              class="badge badge-primary hover:bg-primary-hover"
            >
              {{ tag }}
            </RouterLink>
          </div>
        </div>
      </div>
    </div>
  </article>
</template>
