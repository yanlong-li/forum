<script setup lang="ts">
import { ref, onMounted } from 'vue'
import api from '../../composables/useApi'
import Loading from '../../components/common/Loading.vue'

const stats = ref<any>(null)
const loading = ref(true)

async function fetchStats() {
  try {
    const response = await api.get('/admin/stats')
    stats.value = response.data
  } catch (error) {
    console.error('Failed to fetch stats:', error)
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  fetchStats()
})
</script>

<template>
  <div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    <h1 class="text-2xl font-bold text-slate-900 mb-8">Admin Dashboard</h1>

    <Loading v-if="loading" />

    <div v-else>
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
        <div class="card p-6">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-slate-500">Daily Active Users</p>
              <p class="text-3xl font-bold text-slate-900">{{ stats?.daily_active_users || 0 }}</p>
            </div>
            <div class="w-12 h-12 bg-blue-100 rounded-lg flex items-center justify-center">
              <svg class="w-6 h-6 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
              </svg>
            </div>
          </div>
        </div>

        <div class="card p-6">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-slate-500">Total Posts</p>
              <p class="text-3xl font-bold text-slate-900">{{ stats?.total_posts || 0 }}</p>
            </div>
            <div class="w-12 h-12 bg-emerald-100 rounded-lg flex items-center justify-center">
              <svg class="w-6 h-6 text-emerald-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
              </svg>
            </div>
          </div>
        </div>

        <div class="card p-6">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-slate-500">Total Comments</p>
              <p class="text-3xl font-bold text-slate-900">{{ stats?.total_comments || 0 }}</p>
            </div>
            <div class="w-12 h-12 bg-amber-100 rounded-lg flex items-center justify-center">
              <svg class="w-6 h-6 text-amber-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
              </svg>
            </div>
          </div>
        </div>

        <div class="card p-6">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-slate-500">Total Users</p>
              <p class="text-3xl font-bold text-slate-900">{{ stats?.total_users || 0 }}</p>
            </div>
            <div class="w-12 h-12 bg-purple-100 rounded-lg flex items-center justify-center">
              <svg class="w-6 h-6 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z" />
              </svg>
            </div>
          </div>
        </div>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
        <div class="card p-6">
          <h2 class="text-lg font-semibold text-slate-900 mb-4">Quick Actions</h2>
          <div class="space-y-3">
            <RouterLink to="/admin/users" class="block p-4 rounded-lg border border-slate-200 hover:border-primary hover:bg-slate-50 transition-colors">
              <h3 class="font-medium text-slate-900">User Management</h3>
              <p class="text-sm text-slate-500">View and manage user accounts</p>
            </RouterLink>
            <RouterLink to="/admin/reports" class="block p-4 rounded-lg border border-slate-200 hover:border-primary hover:bg-slate-50 transition-colors">
              <h3 class="font-medium text-slate-900">Content Moderation</h3>
              <p class="text-sm text-slate-500">Review reported posts and comments</p>
            </RouterLink>
            <RouterLink to="/admin/announcements" class="block p-4 rounded-lg border border-slate-200 hover:border-primary hover:bg-slate-50 transition-colors">
              <h3 class="font-medium text-slate-900">Announcements</h3>
              <p class="text-sm text-slate-500">Manage site announcements</p>
            </RouterLink>
          </div>
        </div>

        <div class="card p-6">
          <h2 class="text-lg font-semibold text-slate-900 mb-4">System Status</h2>
          <div class="space-y-4">
            <div class="flex items-center justify-between">
              <span class="text-slate-600">Database</span>
              <span class="badge badge-success">Connected</span>
            </div>
            <div class="flex items-center justify-between">
              <span class="text-slate-600">API Status</span>
              <span class="badge badge-success">Operational</span>
            </div>
            <div class="flex items-center justify-between">
              <span class="text-slate-600">WebSocket</span>
              <span class="badge badge-success">Active</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
