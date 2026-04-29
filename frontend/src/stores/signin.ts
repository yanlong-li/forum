import { defineStore } from 'pinia'
import { ref } from 'vue'
import api from '../composables/useApi'

export const useSigninStore = defineStore('signin', () => {
  const signinStatus = ref<any>(null)
  const todayChecked = ref(false)
  const loading = ref(false)

  async function fetchSigninStatus() {
    try {
      const response = await api.get('/signin/status')
      signinStatus.value = response.data
      todayChecked.value = response.data.signed_in_today
    } catch (error) {
      console.error('Failed to fetch signin status:', error)
    }
  }

  async function checkin() {
    loading.value = true
    try {
      const response = await api.post('/signin/checkin')
      const data = response.data
      todayChecked.value = true
      signinStatus.value = {
        ...signinStatus.value,
        consecutive_days: data.consecutive_days,
        total_points: data.total_points
      }
      return data
    } finally {
      loading.value = false
    }
  }

  return {
    signinStatus,
    todayChecked,
    loading,
    fetchSigninStatus,
    checkin,
  }
})