<script setup lang="ts">
import { ref, watch, nextTick } from 'vue'
import MarkdownRenderer from './MarkdownRenderer.vue'

const props = defineProps<{
  modelValue: string
  placeholder?: string
  minHeight?: string
}>()

const emit = defineEmits<{
  'update:modelValue': [value: string]
}>()

const textareaRef = ref<HTMLTextAreaElement | null>(null)
const isPreview = ref(false)
const localValue = ref(props.modelValue)

watch(() => props.modelValue, (newVal) => {
  localValue.value = newVal
})

function insertText(before: string, after: string = '') {
  const textarea = textareaRef.value
  if (!textarea) return

  const start = textarea.selectionStart
  const end = textarea.selectionEnd
  const selected = localValue.value.substring(start, end)
  const newText = before + selected + after

  localValue.value =
    localValue.value.substring(0, start) +
    newText +
    localValue.value.substring(end)

  emit('update:modelValue', localValue.value)

  nextTick(() => {
    textarea.focus()
    textarea.setSelectionRange(start + before.length, start + before.length + selected.length)
  })
}

function handleKeydown(e: KeyboardEvent) {
  if (e.key === 'Tab') {
    e.preventDefault()
    insertText('  ')
  }
}

const insertActions: { icon: string; title: string; action: () => void }[] = [
  { icon: 'B', title: 'Bold', action: () => insertText('**', '**') },
  { icon: 'I', title: 'Italic', action: () => insertText('*', '*') },
  { icon: 'S', title: 'Strikethrough', action: () => insertText('~~', '~~') },
  { icon: '`', title: 'Inline Code', action: () => insertText('`', '`') },
  { icon: '```', title: 'Code Block', action: () => insertText('\n```\n', '\n```\n') },
  { icon: 'H', title: 'Heading', action: () => insertText('\n## ', '') },
  { icon: '❝', title: 'Quote', action: () => insertText('\n> ', '') },
  { icon: '•', title: 'List', action: () => insertText('\n- ', '') },
  { icon: '1.', title: 'Numbered List', action: () => insertText('\n1. ', '') },
  { icon: '🔗', title: 'Link', action: () => insertText('[', '](url)') },
  { icon: '🖼', title: 'Image', action: () => insertText('![alt](', ')') },
  { icon: '---', title: 'Divider', action: () => insertText('\n---\n', '') },
]

function handleInput(e: Event) {
  const target = e.target as HTMLTextAreaElement
  localValue.value = target.value
  emit('update:modelValue', localValue.value)
}
</script>

<template>
  <div class="markdown-editor border border-slate-300 rounded-lg overflow-hidden bg-white">
    <div class="flex items-center justify-between px-3 py-2 bg-slate-50 border-b border-slate-300">
      <div class="flex items-center space-x-1">
        <button
          v-for="action in insertActions"
          :key="action.title"
          type="button"
          @click="action.action"
          class="w-8 h-8 flex items-center justify-center rounded text-slate-600 hover:bg-slate-200 hover:text-slate-900 transition-colors font-mono text-sm font-bold"
          :title="action.title"
        >
          {{ action.icon }}
        </button>
      </div>

      <div class="flex items-center space-x-2">
        <button
          type="button"
          @click="isPreview = false"
          class="px-3 py-1 rounded text-sm font-medium transition-colors"
          :class="!isPreview ? 'bg-primary text-white' : 'text-slate-600 hover:bg-slate-200'"
        >
          Write
        </button>
        <button
          type="button"
          @click="isPreview = true"
          class="px-3 py-1 rounded text-sm font-medium transition-colors"
          :class="isPreview ? 'bg-primary text-white' : 'text-slate-600 hover:bg-slate-200'"
        >
          Preview
        </button>
      </div>
    </div>

    <div class="relative" :style="{ minHeight: minHeight || '300px' }">
      <textarea
        v-show="!isPreview"
        ref="textareaRef"
        :value="localValue"
        @input="handleInput($event)"
        @keydown="handleKeydown"
        class="w-full p-4 resize-none focus:outline-none font-mono text-sm"
        :style="{ minHeight: minHeight || '300px' }"
        :placeholder="placeholder || 'Write your content here... (Supports Markdown)'"
      ></textarea>

      <div
        v-show="isPreview"
        class="p-4 overflow-auto"
        :style="{ minHeight: minHeight || '300px' }"
      >
        <MarkdownRenderer v-if="localValue" :content="localValue" />
        <p v-else class="text-slate-400 italic">Nothing to preview</p>
      </div>
    </div>
  </div>
</template>

<style scoped>
.markdown-editor {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
}
</style>
