<script setup lang="ts">
import { onMounted, onUnmounted, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useSigninStore } from '../../stores/signin'
import { useAuthStore } from '../../stores/auth'
import { useToast } from '../../composables/useToast'

const { t } = useI18n()
const authStore = useAuthStore()
const signinStore = useSigninStore()
const toast = useToast()

async function handleCheckin() {
  if (!authStore.isAuthenticated) {
    toast.info(t('auth.loginRequired'))
    return
  }

  try {
    const data = await signinStore.checkin()
    toast.success(t('signin.checkinSuccess', { points: data.points_earned }))
  } catch (err: any) {
    toast.error(err.response?.data?.error?.message || t('signin.checkinFailed'))
  }
}

function handleUserLoggedIn() {
  signinStore.fetchSigninStatus()
}

onMounted(() => {
  if (authStore.isAuthenticated) {
    signinStore.fetchSigninStatus()
  }
  window.addEventListener('user-logged-in', handleUserLoggedIn)
})

onUnmounted(() => {
  window.removeEventListener('user-logged-in', handleUserLoggedIn)
})

watch(() => authStore.isAuthenticated, (isAuth) => {
  if (isAuth) {
    signinStore.fetchSigninStatus()
  }
})
</script>

<template>
  <div v-if="authStore.isAuthenticated" class="card p-4">
    <div class="flex items-center justify-between">
      <div class="flex items-center space-x-4">
        <div class="w-12 h-12 rounded-full bg-gradient-to-br from-amber-400 to-orange-500 flex items-center justify-center">
          <svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
          </svg>
        </div>
        <div>
          <h3 class="font-medium text-slate-900">{{ t('signin.dailyCheckin') }}</h3>
          <p class="text-sm text-slate-500">
            {{ signinStore.signinStatus?.consecutive_days || 0 }} {{ t('signin.consecutiveDays') }}
            <span v-if="signinStore.signinStatus?.total_points" class="text-amber-500">
              ({{ t('signin.totalPoints') }}: {{ signinStore.signinStatus.total_points }})
            </span>
          </p>
        </div>
      </div>

      <button
        @click="handleCheckin"
        :disabled="signinStore.loading || signinStore.todayChecked"
        class="px-6 py-2 rounded-lg font-medium transition-all"
        :class="signinStore.todayChecked
          ? 'bg-green-100 text-green-700 cursor-default'
          : 'bg-gradient-to-r from-amber-500 to-orange-500 text-white hover:from-amber-600 hover:to-orange-600 shadow-lg shadow-amber-200'"
      >
        <span v-if="signinStore.loading">{{ t('signin.checkingIn') }}</span>
        <span v-else-if="signinStore.todayChecked">{{ t('signin.checkedIn') }}</span>
        <span v-else>{{ t('signin.checkIn') }}</span>
      </button>
    </div>

    <div v-if="signinStore.signinStatus?.consecutive_days >= 7" class="mt-3 flex items-center space-x-1">
      <span v-for="i in 7" :key="i" class="text-amber-500">
        <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
          <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z" />
        </svg>
      </span>
    </div>
  </div>
</template>