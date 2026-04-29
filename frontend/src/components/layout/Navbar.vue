<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { RouterLink, useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { useAuthStore } from '../../stores/auth'
import { useNotificationStore } from '../../stores/notification'
import { usePrefetch } from '../../composables/usePrefetch'

const { locale, t } = useI18n()
const router = useRouter()
const authStore = useAuthStore()
const notificationStore = useNotificationStore()
const { prefetchTags, prefetchHome } = usePrefetch()

const isUserMenuOpen = ref(false)
const isLangMenuOpen = ref(false)

const showMobileMenu = ref(false)

const navigation = [
  { name: 'nav.home', href: '/' },
  { name: 'nav.posts', href: '/tags' },
]

const languages = [
  { code: 'en', name: 'English' },
  { code: 'zh-CN', name: '简体中文' }
]

async function handleLogout() {
  await authStore.logout()
  router.push('/')
}

function setLocale(langCode: string) {
  locale.value = langCode
  localStorage.setItem('locale', langCode)
  isLangMenuOpen.value = false
}

onMounted(() => {
  if (authStore.isAuthenticated) {
    notificationStore.fetchNotifications()
  }
})
</script>

<template>
  <nav class="bg-white border-b border-slate-200 sticky top-0 z-50">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="flex justify-between h-16">
        <div class="flex items-center">
          <RouterLink to="/" class="flex-shrink-0 flex items-center">
            <span class="text-2xl font-bold text-primary">Forum</span>
          </RouterLink>

          <div class="hidden sm:ml-8 sm:flex sm:space-x-4">
            <RouterLink
              v-for="item in navigation"
              :key="item.name"
              :to="item.href"
              class="text-slate-600 hover:text-slate-900 px-3 py-2 rounded-lg text-sm font-medium transition-colors"
              @mouseenter="item.href === '/tags' ? prefetchTags() : item.href === '/' ? prefetchHome() : null"
            >
              {{ t(item.name) }}
            </RouterLink>
          </div>
        </div>

        <div class="flex items-center space-x-4">
          <div class="relative">
            <button
              @click="isLangMenuOpen = !isLangMenuOpen"
              class="flex items-center space-x-1 px-2 py-1 rounded-lg hover:bg-slate-100 transition-colors text-sm"
            >
              <svg class="w-4 h-4 text-slate-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 5h12M9 3v2m1.048 9.5A18.022 18.022 0 016.412 9m6.088 9h7M11 21l5-10 5 10M12.751 5C11.783 10.77 8.07 15.61 3 18.129" />
              </svg>
              <span class="text-slate-600">{{ languages.find(l => l.code === locale)?.name }}</span>
              <svg class="w-3 h-3 text-slate-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
              </svg>
            </button>

            <transition
              enter-active-class="transition ease-out duration-100"
              enter-from-class="transform opacity-0 scale-95"
              enter-to-class="transform opacity-100 scale-100"
              leave-active-class="transition ease-in duration-75"
              leave-from-class="transform opacity-100 scale-100"
              leave-to-class="transform opacity-0 scale-95"
            >
              <div
                v-if="isLangMenuOpen"
                class="absolute right-0 mt-2 w-36 bg-white rounded-lg shadow-lg border border-slate-200 py-1"
              >
                <button
                  v-for="lang in languages"
                  :key="lang.code"
                  @click="setLocale(lang.code)"
                  class="w-full text-left px-4 py-2 text-sm text-slate-700 hover:bg-slate-100"
                  :class="{ 'bg-slate-100': locale === lang.code }"
                >
                  {{ lang.name }}
                </button>
              </div>
            </transition>
          </div>

          <template v-if="authStore.isAuthenticated">
            <RouterLink
              to="/notifications"
              class="relative p-2 text-slate-600 hover:text-slate-900 rounded-lg hover:bg-slate-100 transition-colors"
            >
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9" />
              </svg>
              <span
                v-if="notificationStore.hasUnread"
                class="absolute top-1 right-1 w-2 h-2 bg-red-500 rounded-full"
              />
            </RouterLink>

            <RouterLink
              to="/create"
              class="btn btn-primary hidden sm:flex"
            >
              {{ t('post.createPost') }}
            </RouterLink>

            <div class="relative">
              <button
                @click="isUserMenuOpen = !isUserMenuOpen"
                class="flex items-center space-x-2 p-2 rounded-lg hover:bg-slate-100 transition-colors"
              >
                <img
                  v-if="authStore.user?.avatar_url"
                  :src="authStore.user.avatar_url"
                  :alt="authStore.user.username"
                  class="w-8 h-8 rounded-full object-cover"
                />
                <div v-else class="w-8 h-8 rounded-full bg-primary text-white flex items-center justify-center font-medium">
                  {{ authStore.user?.username?.charAt(0).toUpperCase() }}
                </div>
                <svg class="w-4 h-4 text-slate-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
                </svg>
              </button>

              <transition
                enter-active-class="transition ease-out duration-100"
                enter-from-class="transform opacity-0 scale-95"
                enter-to-class="transform opacity-100 scale-100"
                leave-active-class="transition ease-in duration-75"
                leave-from-class="transform opacity-100 scale-100"
                leave-to-class="transform opacity-0 scale-95"
              >
                <div
                  v-if="isUserMenuOpen"
                  class="absolute right-0 mt-2 w-48 bg-white rounded-lg shadow-lg border border-slate-200 py-1"
                >
                  <RouterLink
                    :to="`/profile/${authStore.user?.username}`"
                    class="block px-4 py-2 text-sm text-slate-700 hover:bg-slate-100"
                    @click="isUserMenuOpen = false"
                  >
                    {{ t('user.profile') }}
                  </RouterLink>
                  <RouterLink
                    to="/settings"
                    class="block px-4 py-2 text-sm text-slate-700 hover:bg-slate-100"
                    @click="isUserMenuOpen = false"
                  >
                    {{ t('settings.settings') }}
                  </RouterLink>
                  <RouterLink
                    to="/bookmarks"
                    class="block px-4 py-2 text-sm text-slate-700 hover:bg-slate-100"
                    @click="isUserMenuOpen = false"
                  >
                    {{ t('post.bookmark') }}
                  </RouterLink>
                  <RouterLink
                    v-if="authStore.isAdmin"
                    to="/admin"
                    class="block px-4 py-2 text-sm text-slate-700 hover:bg-slate-100"
                    @click="isUserMenuOpen = false"
                  >
                    {{ t('nav.admin') }}
                  </RouterLink>
                  <button
                    @click="handleLogout"
                    class="w-full text-left px-4 py-2 text-sm text-red-600 hover:bg-slate-100"
                  >
                    {{ t('common.logout') }}
                  </button>
                </div>
              </transition>
            </div>
          </template>

          <template v-else>
            <RouterLink
              to="/login"
              class="text-slate-600 hover:text-slate-900 px-3 py-2 rounded-lg text-sm font-medium"
            >
              {{ t('auth.login') }}
            </RouterLink>
            <RouterLink
              to="/register"
              class="btn btn-primary"
            >
              {{ t('auth.register') }}
            </RouterLink>
          </template>

          <button
            @click="showMobileMenu = !showMobileMenu"
            class="sm:hidden p-2 rounded-lg hover:bg-slate-100"
          >
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path v-if="!showMobileMenu" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
              <path v-else stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>
      </div>
    </div>

    <transition
      enter-active-class="transition ease-out duration-100"
      enter-from-class="transform opacity-0 scale-95"
      enter-to-class="transform opacity-100 scale-100"
      leave-active-class="transition ease-in duration-75"
      leave-from-class="transform opacity-100 scale-100"
      leave-to-class="transform opacity-0 scale-95"
    >
      <div v-if="showMobileMenu" class="sm:hidden border-t border-slate-200 bg-white">
        <div class="px-4 py-3 space-y-2">
          <RouterLink
            v-for="item in navigation"
            :key="item.name"
            :to="item.href"
            class="block px-3 py-2 rounded-lg text-slate-700 hover:bg-slate-100"
            @click="showMobileMenu = false"
            @mouseenter="item.href === '/tags' ? prefetchTags() : item.href === '/' ? prefetchHome() : null"
          >
            {{ item.name }}
          </RouterLink>
          <RouterLink
            v-if="authStore.isAuthenticated"
            to="/create"
            class="block px-3 py-2 rounded-lg bg-primary text-white text-center"
            @click="showMobileMenu = false"
          >
            {{ t('post.createPost') }}
          </RouterLink>
        </div>
      </div>
    </transition>
  </nav>
</template>
