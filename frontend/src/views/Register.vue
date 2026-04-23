<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'
import { useToast } from '../composables/useToast'

const router = useRouter()
const authStore = useAuthStore()
const toast = useToast()

const username = ref('')
const email = ref('')
const password = ref('')
const loading = ref(false)
const error = ref('')

async function handleSubmit() {
  error.value = ''
  loading.value = true

  try {
    await authStore.register(username.value, email.value, password.value)
    toast.success('Registration successful! Please check your email to verify.')
    router.push('/login')
  } catch (err: any) {
    error.value = err.response?.data?.error?.message || 'Registration failed'
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="min-h-[calc(100vh-4rem)] flex items-center justify-center px-4 py-12">
    <div class="w-full max-w-md">
      <div class="card p-8">
        <div class="text-center mb-8">
          <h1 class="text-2xl font-bold text-slate-900">Create an account</h1>
          <p class="text-slate-600 mt-2">Join our community</p>
        </div>

        <form @submit.prevent="handleSubmit" class="space-y-6">
          <div v-if="error" class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded-lg text-sm">
            {{ error }}
          </div>

          <div>
            <label for="username" class="block text-sm font-medium text-slate-700 mb-2">
              Username
            </label>
            <input
              id="username"
              v-model="username"
              type="text"
              required
              class="input"
              placeholder="johndoe"
            />
          </div>

          <div>
            <label for="email" class="block text-sm font-medium text-slate-700 mb-2">
              Email
            </label>
            <input
              id="email"
              v-model="email"
              type="email"
              required
              class="input"
              placeholder="you@example.com"
            />
          </div>

          <div>
            <label for="password" class="block text-sm font-medium text-slate-700 mb-2">
              Password
            </label>
            <input
              id="password"
              v-model="password"
              type="password"
              required
              minlength="8"
              class="input"
              placeholder="At least 8 characters"
            />
            <p class="mt-1 text-xs text-slate-500">
              Must contain at least 8 characters, including uppercase, lowercase, and numbers
            </p>
          </div>

          <button
            type="submit"
            :disabled="loading"
            class="btn btn-primary w-full"
          >
            {{ loading ? 'Creating account...' : 'Create account' }}
          </button>
        </form>

        <p class="mt-6 text-center text-sm text-slate-600">
          Already have an account?
          <RouterLink to="/login" class="text-primary hover:text-primary-hover font-medium">
            Sign in
          </RouterLink>
        </p>
      </div>
    </div>
  </div>
</template>
