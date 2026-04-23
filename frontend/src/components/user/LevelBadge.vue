<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  level?: number
  points?: number
  showPoints?: boolean
}>()

const levelConfig = computed(() => {
  const configs: Record<number, { color: string; bg: string; label: string }> = {
    1: { color: 'text-slate-600', bg: 'bg-slate-100', label: 'L1' },
    2: { color: 'text-green-600', bg: 'bg-green-100', label: 'L2' },
    3: { color: 'text-blue-600', bg: 'bg-blue-100', label: 'L3' },
    4: { color: 'text-purple-600', bg: 'bg-purple-100', label: 'L4' },
    5: { color: 'text-amber-600', bg: 'bg-amber-100', label: 'L5' },
    6: { color: 'text-red-600', bg: 'bg-red-100', label: 'L6' },
  }
  return configs[props.level || 1] || configs[1]
})
</script>

<template>
  <div class="flex items-center space-x-1">
    <span
      class="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium"
      :class="[levelConfig.color, levelConfig.bg]"
    >
      {{ levelConfig.label }}
    </span>
    <span v-if="showPoints && points !== undefined" class="text-xs text-slate-500">
      {{ points }} pts
    </span>
  </div>
</template>