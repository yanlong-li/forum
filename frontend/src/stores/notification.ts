import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import api from '../composables/useApi'

export interface Notification {
  id: string
  type: string
  actor: {
    id: string
    username: string
    avatar_url: string | null
  }
  post_id: string | null
  comment_id: string | null
  is_read: boolean
  created_at: string
}

export const useNotificationStore = defineStore('notification', () => {
  const notifications = ref<Notification[]>([])
  const unreadCount = ref(0)

  const hasUnread = computed(() => unreadCount.value > 0)

  async function fetchNotifications() {
    const response = await api.get('/notifications')
    const data = response.data
    notifications.value = data.notifications
    unreadCount.value = data.unread_count
  }

  async function markAsRead(id: string) {
    await api.patch(`/notifications/${id}/read`)
    const notification = notifications.value.find((n) => n.id === id)
    if (notification && !notification.is_read) {
      notification.is_read = true
      unreadCount.value = Math.max(0, unreadCount.value - 1)
    }
  }

  async function markAllAsRead() {
    await api.patch('/notifications/read-all')
    notifications.value.forEach((n) => {
      n.is_read = true
    })
    unreadCount.value = 0
  }

  function addNotification(notification: Notification) {
    notifications.value.unshift(notification)
    if (!notification.is_read) {
      unreadCount.value++
    }
  }

  return {
    notifications,
    unreadCount,
    hasUnread,
    fetchNotifications,
    markAsRead,
    markAllAsRead,
    addNotification,
  }
})
