import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import api from '../composables/useApi'

export interface User {
  id: string
  username: string
  email?: string
  avatar_url: string | null
  bio: string | null
  is_admin: boolean
}

export const useAuthStore = defineStore('auth', () => {
  const user = ref<User | null>(null)
  const accessToken = ref<string | null>(localStorage.getItem('access_token'))
  const refreshToken = ref<string | null>(localStorage.getItem('refresh_token'))

  const isAuthenticated = computed(() => !!accessToken.value)
  const isAdmin = computed(() => user.value?.is_admin ?? false)

  async function login(email: string, password: string) {
    const response = await api.post('/auth/login', { email, password })
    const data = response.data

    accessToken.value = data.access_token
    refreshToken.value = data.refresh_token
    user.value = data.user

    localStorage.setItem('access_token', data.access_token)
    localStorage.setItem('refresh_token', data.refresh_token)

    window.dispatchEvent(new CustomEvent('user-logged-in'))
  }

  async function register(username: string, email: string, password: string) {
    await api.post('/auth/register', { username, email, password })
  }

  async function logout() {
    user.value = null
    accessToken.value = null
    refreshToken.value = null
    localStorage.removeItem('access_token')
    localStorage.removeItem('refresh_token')
  }

  async function checkAuth() {
    if (!accessToken.value) return

    try {
      const response = await api.get('/users/me')
      user.value = response.data
    } catch {
      logout()
    }
  }

  async function refreshAccessToken(): Promise<boolean> {
    if (!refreshToken.value) return false

    try {
      const response = await api.post('/auth/refresh', {
        refresh_token: refreshToken.value,
      })
      const data = response.data

      accessToken.value = data.access_token
      refreshToken.value = data.refresh_token

      localStorage.setItem('access_token', data.access_token)
      localStorage.setItem('refresh_token', data.refresh_token)
      return true
    } catch {
      logout()
      return false
    }
  }

  function setUser(userData: User) {
    user.value = userData
  }

  return {
    user,
    accessToken,
    refreshToken,
    isAuthenticated,
    isAdmin,
    login,
    register,
    logout,
    checkAuth,
    refreshAccessToken,
    setUser,
  }
})
