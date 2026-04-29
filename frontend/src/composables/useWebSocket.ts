import { ref, onUnmounted, watch } from 'vue'
import { useAuthStore } from '../stores/auth'
import { useNotificationStore } from '../stores/notification'
import { useToast } from './useToast'

export function useWebSocket() {
  const ws = ref<WebSocket | null>(null)
  const isConnected = ref(false)
  const reconnectAttempts = ref(0)
  const maxReconnectAttempts = 5
  const reconnectInterval = 3000
  let reconnectTimeout: ReturnType<typeof setTimeout> | null = null

  const authStore = useAuthStore()
  const notificationStore = useNotificationStore()
  const toast = useToast()

  function connect() {
    if (!authStore.accessToken) return
    if (ws.value?.readyState === WebSocket.OPEN) return

    const wsUrl = `wss://3000-b7492d738b4a6566.monkeycode-ai.online/api/v1/ws?token=${authStore.accessToken}`

    try {
      ws.value = new WebSocket(wsUrl)

      ws.value.onopen = () => {
        isConnected.value = true
        reconnectAttempts.value = 0
        console.log('WebSocket connected')
      }

      ws.value.onmessage = (event) => {
        try {
          const message = JSON.parse(event.data)
          handleMessage(message)
        } catch {
          console.error('Failed to parse WebSocket message')
        }
      }

      ws.value.onclose = () => {
        isConnected.value = false
        scheduleReconnect()
      }

      ws.value.onerror = () => {
        isConnected.value = false
      }
    } catch (error) {
      console.error('WebSocket connection error:', error)
      scheduleReconnect()
    }
  }

  function handleMessage(message: any) {
    switch (message.type) {
      case 'notification':
        notificationStore.addNotification(message.payload)
        toast.info(message.payload.content || 'You have a new notification')
        break
      case 'new_post':
        window.dispatchEvent(new CustomEvent('new-post', { detail: message.payload }))
        break
      case 'new_comment':
        window.dispatchEvent(new CustomEvent('new-comment', { detail: message.payload }))
        break
      case 'new_mention':
        notificationStore.addNotification(message.payload)
        toast.info(message.payload.content || 'You were mentioned in a comment')
        break
      default:
        break
    }
  }

  function scheduleReconnect() {
    if (reconnectTimeout) {
      clearTimeout(reconnectTimeout)
    }
    if (reconnectAttempts.value >= maxReconnectAttempts) {
      console.log('Max reconnect attempts reached')
      return
    }

    reconnectAttempts.value++
    reconnectTimeout = setTimeout(() => {
      connect()
    }, reconnectInterval * Math.min(reconnectAttempts.value, 3))
  }

  function disconnect() {
    if (reconnectTimeout) {
      clearTimeout(reconnectTimeout)
      reconnectTimeout = null
    }
    if (ws.value) {
      ws.value.close()
      ws.value = null
      isConnected.value = false
    }
  }

  function send(message: any) {
    if (ws.value && ws.value.readyState === WebSocket.OPEN) {
      ws.value.send(JSON.stringify(message))
    }
  }

  watch(() => authStore.accessToken, (token) => {
    if (token) {
      connect()
    } else {
      disconnect()
    }
  }, { immediate: true })

  onUnmounted(() => {
    disconnect()
  })

  return {
    isConnected,
    connect,
    disconnect,
    send,
  }
}