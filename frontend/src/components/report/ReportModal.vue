<script setup lang="ts">
import { ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import api from '../../composables/useApi'
import { useToast } from '../../composables/useToast'

const { t } = useI18n()

const props = defineProps<{
  postId?: string
  commentId?: string
  visible: boolean
}>()

const emit = defineEmits<{
  close: []
}>()

const toast = useToast()
const submitting = ref(false)
const reason = ref('')
const selectedReason = ref('')

const reasonOptions = [
  { value: 'spam', label: t('report.spam') },
  { value: 'harassment', label: t('report.harassment') },
  { value: 'hate_speech', label: t('report.hateSpeech') },
  { value: 'misinformation', label: t('report.misinformation') },
  { value: 'inappropriate', label: t('report.inappropriate') },
  { value: 'other', label: t('report.other') },
]

async function handleSubmit() {
  const finalReason = selectedReason.value === 'other' ? reason.value : selectedReason.value
  if (!finalReason) {
    toast.error(t('report.selectReason'))
    return
  }

  submitting.value = true
  try {
    await api.post('/reports', {
      post_id: props.postId,
      reason: finalReason,
    })
    toast.success(t('report.thankYou'))
    emit('close')
    reason.value = ''
    selectedReason.value = ''
  } catch (err: any) {
    toast.error(err.response?.data?.error?.message || t('report.submitFailed'))
  } finally {
    submitting.value = false
  }
}

watch(() => props.visible, (val) => {
  if (!val) {
    reason.value = ''
    selectedReason.value = ''
  }
})
</script>

<template>
  <Teleport to="body">
    <div
      v-if="visible"
      class="fixed inset-0 z-50 flex items-center justify-center"
      @click.self="emit('close')"
    >
      <div class="fixed inset-0 bg-black/50 backdrop-blur-sm"></div>

      <div class="relative bg-white rounded-xl shadow-2xl w-full max-w-md mx-4 overflow-hidden">
        <div class="bg-gradient-to-r from-red-500 to-rose-500 px-6 py-4">
          <h3 class="text-lg font-semibold text-white">{{ t('report.reportContent') }}</h3>
          <p class="text-red-100 text-sm mt-1">{{ t('report.helpCommunity') }}</p>
        </div>

        <div class="p-6">
          <div class="space-y-4">
            <div>
              <label class="block text-sm font-medium text-slate-700 mb-2">{{ t('report.selectReason') }}</label>
              <div class="grid grid-cols-1 gap-2">
                <button
                  v-for="option in reasonOptions"
                  :key="option.value"
                  @click="selectedReason = option.value"
                  class="px-4 py-3 text-left rounded-lg border transition-all duration-200"
                  :class="selectedReason === option.value
                    ? 'border-red-500 bg-red-50 text-red-700'
                    : 'border-slate-200 hover:border-slate-300 hover:bg-slate-50'"
                >
                  {{ option.label }}
                </button>
              </div>
            </div>

            <div v-if="selectedReason === 'other'">
              <label class="block text-sm font-medium text-slate-700 mb-2">{{ t('report.describeIssue') }}</label>
              <textarea
                v-model="reason"
                rows="3"
                class="w-full px-4 py-3 rounded-lg border border-slate-200 focus:border-red-500 focus:ring-2 focus:ring-red-200 outline-none transition-all resize-none"
                :placeholder="t('report.provideDetails')"
              ></textarea>
            </div>
          </div>

          <div class="flex items-center justify-end gap-3 mt-6 pt-4 border-t border-slate-100">
            <button
              @click="emit('close')"
              class="px-4 py-2 text-slate-600 hover:text-slate-800 font-medium transition-colors"
            >
              {{ t('common.cancel') }}
            </button>
            <button
              @click="handleSubmit"
              :disabled="submitting || (selectedReason === 'other' && !reason.trim())"
              class="px-6 py-2 bg-gradient-to-r from-red-500 to-rose-500 text-white font-medium rounded-lg hover:from-red-600 hover:to-rose-600 transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed shadow-lg shadow-red-200"
            >
              {{ submitting ? t('report.submitting') : t('report.submitReport') }}
            </button>
          </div>
        </div>
      </div>
    </div>
  </Teleport>
</template>