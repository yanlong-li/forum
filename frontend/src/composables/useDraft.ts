import { ref } from 'vue'

const DRAFT_KEY = 'draft_post'
const AUTO_SAVE_INTERVAL = 30000

export interface Draft {
  title: string
  content: string
  tags: string[]
  updatedAt: number
}

export function useDraft() {
  const hasDraft = ref(false)
  const lastSaved = ref<Date | null>(null)

  function saveDraft(draft: Draft) {
    try {
      localStorage.setItem(DRAFT_KEY, JSON.stringify(draft))
      hasDraft.value = true
      lastSaved.value = new Date()
    } catch (error) {
      console.error('Failed to save draft:', error)
    }
  }

  function loadDraft(): Draft | null {
    try {
      const saved = localStorage.getItem(DRAFT_KEY)
      if (saved) {
        const draft = JSON.parse(saved) as Draft
        if (draft.title || draft.content || draft.tags?.length) {
          hasDraft.value = true
          return draft
        }
      }
      return null
    } catch (error) {
      console.error('Failed to load draft:', error)
      return null
    }
  }

  function clearDraft() {
    try {
      localStorage.removeItem(DRAFT_KEY)
      hasDraft.value = false
      lastSaved.value = null
    } catch (error) {
      console.error('Failed to clear draft:', error)
    }
  }

  function getTimeSinceLastSaved(): string {
    if (!lastSaved.value) return ''
    const seconds = Math.floor((Date.now() - lastSaved.value.getTime()) / 1000)
    if (seconds < 60) return `${seconds}s ago`
    const minutes = Math.floor(seconds / 60)
    if (minutes < 60) return `${minutes}m ago`
    const hours = Math.floor(minutes / 60)
    return `${hours}h ago`
  }

  return {
    hasDraft,
    lastSaved,
    saveDraft,
    loadDraft,
    clearDraft,
    getTimeSinceLastSaved,
    DRAFT_KEY,
    AUTO_SAVE_INTERVAL,
  }
}