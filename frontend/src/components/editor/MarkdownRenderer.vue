<script setup lang="ts">
import { computed } from 'vue'
import MarkdownIt from 'markdown-it'
import { useRouter } from 'vue-router'

const props = defineProps<{
  content: string
}>()

const router = useRouter()

const md = new MarkdownIt({
  html: false,
  linkify: true,
  typographer: true,
  breaks: true,
})

function processMentions(text: string): string {
  return text.replace(/@(\w+)/g, '<a href="/profile/$1" class="mention text-primary hover:underline">@$1</a>')
}

function processTags(text: string): string {
  return text.replace(/#(\w+)/g, '<a href="/tags/$1" class="tag-link text-primary hover:underline">#$1</a>')
}

function preprocessContent(content: string): string {
  let result = content
  result = processMentions(result)
  result = processTags(result)
  return result
}

const renderedContent = computed(() => {
  if (!props.content) return ''

  const processedContent = preprocessContent(props.content)

  let html = md.render(processedContent)

  html = html.replace(/<a\s+href="([^"]*)"/g, (_match: string, href: string) => {
    if (href.startsWith('/') || href.startsWith('#')) {
      return `<a href="${href}"`
    }
    return `<a href="${href}" target="_blank" rel="noopener noreferrer"`
  })

  return html
})

function handleClick(e: MouseEvent) {
  const target = e.target as HTMLElement
  if (target.tagName === 'A') {
    e.preventDefault()
    const href = (target as HTMLAnchorElement).href
    const url = new URL(href)
    if (url.pathname.startsWith('/')) {
      router.push(url.pathname)
    } else {
      window.open(href, '_blank')
    }
  }
}
</script>

<template>
  <div
    class="prose prose-slate max-w-none markdown-body"
    @click="handleClick"
    v-html="renderedContent"
  ></div>
</template>

<style>
.markdown-body {
  line-height: 1.7;
}

.markdown-body h1,
.markdown-body h2,
.markdown-body h3,
.markdown-body h4,
.markdown-body h5,
.markdown-body h6 {
  font-weight: 600;
  margin-top: 1.5em;
  margin-bottom: 0.5em;
  color: #1e293b;
}

.markdown-body h1 { font-size: 1.75em; }
.markdown-body h2 { font-size: 1.5em; }
.markdown-body h3 { font-size: 1.25em; }
.markdown-body h4 { font-size: 1.1em; }

.markdown-body p {
  margin-bottom: 1em;
}

.markdown-body a {
  color: #3b82f6;
  text-decoration: none;
}

.markdown-body a:hover {
  text-decoration: underline;
}

.markdown-body code {
  background: #f1f5f9;
  padding: 0.2em 0.4em;
  border-radius: 4px;
  font-family: 'Fira Code', 'SF Mono', Consolas, monospace;
  font-size: 0.9em;
  color: #e11d48;
}

.markdown-body pre {
  background: #1e293b;
  color: #e2e8f0;
  padding: 1em;
  border-radius: 8px;
  overflow-x: auto;
  margin: 1em 0;
}

.markdown-body pre code {
  background: transparent;
  color: inherit;
  padding: 0;
  font-size: 0.875em;
}

.markdown-body blockquote {
  border-left: 4px solid #3b82f6;
  padding-left: 1em;
  margin-left: 0;
  color: #64748b;
  font-style: italic;
}

.markdown-body ul,
.markdown-body ol {
  padding-left: 1.5em;
  margin-bottom: 1em;
}

.markdown-body li {
  margin-bottom: 0.25em;
}

.markdown-body hr {
  border: none;
  border-top: 2px solid #e2e8f0;
  margin: 2em 0;
}

.markdown-body img {
  max-width: 100%;
  border-radius: 8px;
  margin: 1em 0;
}

.markdown-body table {
  width: 100%;
  border-collapse: collapse;
  margin: 1em 0;
}

.markdown-body th,
.markdown-body td {
  border: 1px solid #e2e8f0;
  padding: 0.5em 1em;
  text-align: left;
}

.markdown-body th {
  background: #f8fafc;
  font-weight: 600;
}

.markdown-body .mention {
  color: #3b82f6;
  font-weight: 500;
}

.markdown-body .tag-link {
  color: #8b5cf6;
  font-weight: 500;
}
</style>
