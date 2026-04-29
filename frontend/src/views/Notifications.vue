<script setup lang="ts">
import { onMounted } from 'vue'
import { RouterLink } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { useNotificationStore } from '../stores/notification'
import { useToast } from '../composables/useToast'
import { formatDistanceToNow } from '../utils/time'
import Loading from '../components/common/Loading.vue'

const notificationStore = useNotificationStore()
const toast = useToast()
const { t } = useI18n()

async function handleMarkAllRead() {
  try {
    await notificationStore.markAllAsRead()
    toast.success(t('notification.allMarkedAsRead'))
  } catch {
    toast.error(t('notification.markAllAsReadFailed'))
  }
}

async function handleMarkRead(id: string) {
  try {
    await notificationStore.markAsRead(id)
  } catch {
    toast.error(t('notification.markAsReadFailed'))
  }
}

function getNotificationText(notification: any) {
  switch (notification.type) {
    case 'comment':
      return t('notification.commentedOnYourPost')
    case 'follow':
      return t('notification.startedFollowingYou')
    case 'like':
      return t('notification.likedYourPost')
    case 'reply':
      return t('notification.repliedToYourComment')
    case 'mention':
      return t('notification.mentionedYou')
    default:
      return t('notification.interactedWithYou')
  }
}

onMounted(() => {
  notificationStore.fetchNotifications()
})
</script>

<template>
  <div class="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    <div class="flex items-center justify-between mb-8">
      <h1 class="text-2xl font-bold text-slate-900">{{ t('notification.notifications') }}</h1>

      <button
        v-if="notificationStore.hasUnread"
        @click="handleMarkAllRead"
        class="btn btn-secondary text-sm"
      >
        {{ t('notification.markAllRead') }}
      </button>
    </div>

    <Loading v-if="notificationStore.notifications.length === 0" />

    <div v-else class="space-y-3">
      <RouterLink
        v-for="notification in notificationStore.notifications"
        :key="notification.id"
        :to="notification.post_id ? `/post/${notification.post_id}` : '/'"
        class="card p-4 block hover:shadow-md transition-shadow"
        :class="{ 'bg-blue-50': !notification.is_read }"
        @click="handleMarkRead(notification.id)"
      >
        <div class="flex items-start space-x-4">
          <RouterLink :to="`/profile/${notification.actor.username}`" @click.stop>
            <img
              v-if="notification.actor.avatar_url"
              :src="notification.actor.avatar_url"
              :alt="notification.actor.username"
              class="w-10 h-10 rounded-full object-cover"
              loading="lazy"
            />
            <div
              v-else
              class="w-10 h-10 rounded-full bg-primary text-white flex items-center justify-center font-medium"
            >
              {{ notification.actor.username.charAt(0).toUpperCase() }}
            </div>
          </RouterLink>

          <div class="flex-1 min-w-0">
            <p class="text-sm text-slate-700">
              <RouterLink
                :to="`/profile/${notification.actor.username}`"
                class="font-medium text-slate-900 hover:text-primary"
                @click.stop
              >
                {{ notification.actor.username }}
              </RouterLink>
              {{ getNotificationText(notification) }}
            </p>
            <p class="text-xs text-slate-500 mt-1">
              {{ formatDistanceToNow(new Date(notification.created_at)) }}
            </p>
          </div>

          <div
            v-if="!notification.is_read"
            class="w-2 h-2 bg-primary rounded-full flex-shrink-0"
          ></div>
        </div>
      </RouterLink>
    </div>
  </div>
</template>
