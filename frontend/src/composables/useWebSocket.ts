import { ref, onUnmounted } from 'vue'
import { useAuthStore } from '../stores/auth'
import { useNotificationStore } from '../stores/notification'

export function useWebSocket() {
  const ws = ref<WebSocket | null>(null)
  const isConnected = ref(false)
  const reconnectAttempts = ref(0)
  const maxReconnectAttempts = 5
  const reconnectInterval = 2000

  const authStore = useAuthStore()

  function connect() {
    if (!authStore.accessToken) return

    const wsUrl = `wss://3000-b7492d738b4a6566.monkeycode-ai.online/api/v1/ws?token=${authStore.accessToken}`

    ws.value = new WebSocket(wsUrl)

    ws.value.onopen = () => {
      isConnected.value = true
      reconnectAttempts.value = 0
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
      reconnect()
    }

    ws.value.onerror = () => {
      isConnected.value = false
    }
  }

  function handleMessage(message: any) {
    const notificationStore = useNotificationStore()

    switch (message.type) {
      case 'notification':
        notificationStore.addNotification(message.payload)
        break
      case 'new_post':
        break
      default:
        break
    }
  }

  function reconnect() {
    if (reconnectAttempts.value >= maxReconnectAttempts) {
      return
    }

    reconnectAttempts.value++

    setTimeout(() => {
      connect()
    }, reconnectInterval)
  }

  function disconnect() {
    if (ws.value) {
      ws.value.close()
      ws.value = null
      isConnected.value = false
    }
  }

  function send(message: any) {
    if (ws.value && isConnected.value) {
      ws.value.send(JSON.stringify(message))
    }
  }

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
