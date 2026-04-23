import { ref } from 'vue'

export interface Toast {
  id: number
  type: 'success' | 'error' | 'warning' | 'info'
  message: string
}

const toasts = ref<Toast[]>([])
let toastId = 0

export function useToast() {
  function show(message: string, type: Toast['type'] = 'info', duration = 3000) {
    const id = ++toastId
    toasts.value.push({ id, type, message })

    setTimeout(() => {
      remove(id)
    }, duration)
  }

  function success(message: string, duration?: number) {
    show(message, 'success', duration)
  }

  function error(message: string, duration?: number) {
    show(message, 'error', duration)
  }

  function warning(message: string, duration?: number) {
    show(message, 'warning', duration)
  }

  function info(message: string, duration?: number) {
    show(message, 'info', duration)
  }

  function remove(id: number) {
    const index = toasts.value.findIndex((t) => t.id === id)
    if (index !== -1) {
      toasts.value.splice(index, 1)
    }
  }

  return {
    toasts,
    show,
    success,
    error,
    warning,
    info,
    remove,
  }
}
