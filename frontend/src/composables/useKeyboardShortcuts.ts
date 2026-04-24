import { onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'

export interface KeyboardShortcut {
  key: string
  ctrl?: boolean
  alt?: boolean
  shift?: boolean
  handler: () => void
  description: string
}

const shortcuts: KeyboardShortcut[] = []

function isInputElement(element: HTMLElement): boolean {
  const tagName = element.tagName.toLowerCase()
  return (
    tagName === 'input' ||
    tagName === 'textarea' ||
    tagName === 'select' ||
    element.isContentEditable
  )
}

function handleKeyDown(event: KeyboardEvent) {
  if (isInputElement(event.target as HTMLElement)) return

  const shortcut = shortcuts.find((s) => {
    const keyMatch = s.key.toLowerCase() === event.key.toLowerCase()
    const ctrlMatch = !!s.ctrl === (event.ctrlKey || event.metaKey)
    const altMatch = !!s.alt === event.altKey
    const shiftMatch = !!s.shift === event.shiftKey
    return keyMatch && ctrlMatch && altMatch && shiftMatch
  })

  if (shortcut) {
    event.preventDefault()
    shortcut.handler()
  }
}

export function useKeyboardShortcuts() {
  function registerShortcut(shortcut: KeyboardShortcut) {
    shortcuts.push(shortcut)
  }

  function unregisterShortcut(key: string, ctrl?: boolean) {
    const index = shortcuts.findIndex(
      (s) => s.key === key && (ctrl ? s.ctrl : !s.ctrl)
    )
    if (index !== -1) {
      shortcuts.splice(index, 1)
    }
  }

  onMounted(() => {
    window.addEventListener('keydown', handleKeyDown)
  })

  onUnmounted(() => {
    window.removeEventListener('keydown', handleKeyDown)
  })

  return {
    registerShortcut,
    unregisterShortcut,
    shortcuts,
  }
}

export function setupGlobalShortcuts(router: ReturnType<typeof useRouter>) {
  shortcuts.length = 0

  shortcuts.push(
    {
      key: 'j',
      handler: () => {
        const focusedElement = document.querySelector('.focused-post')
        if (focusedElement) {
          focusedElement.classList.remove('focused-post')
          const next = focusedElement.nextElementSibling
          if (next) {
            next.classList.add('focused-post')
            next.scrollIntoView({ behavior: 'smooth', block: 'center' })
          }
        } else {
          const firstPost = document.querySelector('.post-card')
          if (firstPost) {
            firstPost.classList.add('focused-post')
          }
        }
      },
      description: '下一帖'
    },
    {
      key: 'k',
      handler: () => {
        const focusedElement = document.querySelector('.focused-post')
        if (focusedElement) {
          focusedElement.classList.remove('focused-post')
          const prev = focusedElement.previousElementSibling
          if (prev) {
            prev.classList.add('focused-post')
            prev.scrollIntoView({ behavior: 'smooth', block: 'center' })
          }
        } else {
          const firstPost = document.querySelector('.post-card')
          if (firstPost) {
            firstPost.classList.add('focused-post')
          }
        }
      },
      description: '上一帖'
    },
    {
      key: 'Enter',
      handler: () => {
        const focusedElement = document.querySelector('.focused-post a.post-title')
        if (focusedElement instanceof HTMLAnchorElement) {
          router.push(focusedElement.href)
        }
      },
      description: '打开帖子'
    },
    {
      key: 'n',
      handler: () => {
        router.push('/create')
      },
      description: '新帖子'
    },
    {
      key: 'c',
      handler: () => {
        const commentInput = document.querySelector('.comment-input textarea') as HTMLElement
        if (commentInput) {
          commentInput.focus()
          commentInput.scrollIntoView({ behavior: 'smooth', block: 'center' })
        } else if (router.currentRoute.value.name !== 'login') {
          router.push('/login')
        }
      },
      description: '评论'
    },
    {
      key: 'r',
      handler: () => {
        window.scrollTo({ top: 0, behavior: 'smooth' })
      },
      description: '返回顶部'
    },
    {
      key: '/',
      handler: () => {
        const searchInput = document.querySelector('.search-input input') as HTMLElement
        if (searchInput) {
          searchInput.focus()
        }
      },
      description: '搜索'
    },
    {
      key: 'g',
      handler: () => {
        router.push('/')
      },
      description: '首页'
    },
    {
      key: 'n',
      shift: true,
      handler: () => {
        router.push('/notifications')
      },
      description: '通知'
    }
  )

  onMounted(() => {
    window.addEventListener('keydown', handleKeyDown)
  })

  onUnmounted(() => {
    window.removeEventListener('keydown', handleKeyDown)
  })
}