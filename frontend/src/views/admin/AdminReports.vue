<script setup lang="ts">
import { ref, onMounted } from 'vue'
import api from '../../composables/useApi'
import { useToast } from '../../composables/useToast'
import Loading from '../../components/common/Loading.vue'

interface Report {
  id: string
  reporter_id: string
  reporter_username: string
  reported_user_id: string
  reported_username: string
  post_id: string | null
  comment_id: string | null
  reason: string
  status: string
  created_at: string
}

const reports = ref<Report[]>([])
const loading = ref(true)
const processing = ref<string | null>(null)
const toast = useToast()

async function fetchReports() {
  try {
    const response = await api.get('/admin/reports', { params: { page: 1, limit: 50 } })
    reports.value = response.data.reports
  } catch (error) {
    console.error('Failed to fetch reports:', error)
  } finally {
    loading.value = false
  }
}

async function processReport(reportId: string, action: 'dismiss' | 'delete_content') {
  processing.value = reportId
  try {
    await api.patch(`/admin/reports/${reportId}`, { action })
    toast.success(`Report ${action === 'dismiss' ? 'dismissed' : 'content deleted'}`)
    reports.value = reports.value.filter(r => r.id !== reportId)
  } catch (error) {
    toast.error('Failed to process report')
  } finally {
    processing.value = null
  }
}

onMounted(() => {
  fetchReports()
})
</script>

<template>
  <div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    <div class="flex items-center justify-between mb-8">
      <h1 class="text-2xl font-bold text-slate-900">Content Moderation</h1>
      <RouterLink to="/admin" class="btn btn-secondary">
        Back to Dashboard
      </RouterLink>
    </div>

    <Loading v-if="loading" />

    <div v-else-if="reports.length === 0" class="card p-8 text-center">
      <p class="text-slate-500">No pending reports</p>
    </div>

    <div v-else class="space-y-4">
      <div v-for="report in reports" :key="report.id" class="card p-6">
        <div class="flex items-start justify-between">
          <div class="flex-1">
            <div class="flex items-center space-x-2 mb-2">
              <span class="badge badge-error">{{ report.reason }}</span>
              <span class="text-sm text-slate-500">Reported by {{ report.reporter_username }}</span>
            </div>
            <p class="text-slate-700 mb-2">
              <strong>{{ report.reported_username }}</strong> reported for {{ report.reason }}
            </p>
            <div v-if="report.post_id" class="text-sm">
              <RouterLink :to="`/post/${report.post_id}`" class="text-primary hover:underline">
                View Post
              </RouterLink>
            </div>
            <div v-if="report.comment_id" class="text-sm">
              <span class="text-slate-500">Comment ID: {{ report.comment_id }}</span>
            </div>
            <p class="text-xs text-slate-400 mt-2">
              {{ new Date(report.created_at).toLocaleString() }}
            </p>
          </div>
          <div class="flex space-x-2 ml-4">
            <button
              @click="processReport(report.id, 'dismiss')"
              :disabled="processing === report.id"
              class="btn btn-secondary"
            >
              Dismiss
            </button>
            <button
              @click="processReport(report.id, 'delete_content')"
              :disabled="processing === report.id"
              class="btn btn-primary"
            >
              Delete Content
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>