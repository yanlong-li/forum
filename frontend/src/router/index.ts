import { createRouter, createWebHistory } from 'vue-router'
import { useAuthStore } from '../stores/auth'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'home',
      component: () => import('../views/Home.vue'),
    },
    {
      path: '/login',
      name: 'login',
      component: () => import('../views/Login.vue'),
      meta: { guest: true },
    },
    {
      path: '/register',
      name: 'register',
      component: () => import('../views/Register.vue'),
      meta: { guest: true },
    },
    {
      path: '/post/:id',
      name: 'post',
      component: () => import('../views/PostDetail.vue'),
    },
    {
      path: '/create',
      name: 'create',
      component: () => import('../views/CreatePost.vue'),
      meta: { requiresAuth: true },
    },
    {
      path: '/profile/:username',
      name: 'profile',
      component: () => import('../views/Profile.vue'),
    },
    {
      path: '/settings',
      name: 'settings',
      component: () => import('../views/Settings.vue'),
      meta: { requiresAuth: true },
    },
    {
      path: '/notifications',
      name: 'notifications',
      component: () => import('../views/Notifications.vue'),
      meta: { requiresAuth: true },
    },
    {
      path: '/bookmarks',
      name: 'bookmarks',
      component: () => import('../views/Bookmarks.vue'),
      meta: { requiresAuth: true },
    },
    {
      path: '/tags',
      name: 'tags',
      component: () => import('../views/Tags.vue'),
    },
    {
      path: '/tags/:name',
      name: 'tag-posts',
      component: () => import('../views/TagPosts.vue'),
    },
    {
      path: '/admin',
      name: 'admin',
      component: () => import('../views/admin/Dashboard.vue'),
      meta: { requiresAuth: true, requiresAdmin: true },
    },
    {
      path: '/admin/users',
      name: 'admin-users',
      component: () => import('../views/admin/AdminUsers.vue'),
      meta: { requiresAuth: true, requiresAdmin: true },
    },
    {
      path: '/admin/reports',
      name: 'admin-reports',
      component: () => import('../views/admin/AdminReports.vue'),
      meta: { requiresAuth: true, requiresAdmin: true },
    },
    {
      path: '/admin/announcements',
      name: 'admin-announcements',
      component: () => import('../views/admin/AdminAnnouncements.vue'),
      meta: { requiresAuth: true, requiresAdmin: true },
    },
    {
      path: '/settings',
      name: 'settings',
      component: () => import('../views/Settings.vue'),
      meta: { requiresAuth: true },
    },
    {
      path: '/settings/blocks',
      name: 'blocked-users',
      component: () => import('../views/BlockedUsers.vue'),
      meta: { requiresAuth: true },
    },
    {
      path: '/drafts',
      name: 'drafts',
      component: () => import('../views/Drafts.vue'),
      meta: { requiresAuth: true },
    },
    {
      path: '/settings/profile',
      name: 'edit-profile',
      component: () => import('../views/EditProfile.vue'),
      meta: { requiresAuth: true },
    },
  ],
  scrollBehavior(_to, _from, savedPosition) {
    if (savedPosition) {
      return savedPosition
    } else {
      return { top: 0 }
    }
  },
})

router.beforeEach((to, _from, next) => {
  const authStore = useAuthStore()

  if (to.meta.requiresAuth && !authStore.isAuthenticated) {
    next({ name: 'login', query: { redirect: to.fullPath } })
  } else if (to.meta.guest && authStore.isAuthenticated) {
    next({ name: 'home' })
  } else if (to.meta.requiresAdmin && !authStore.isAdmin) {
    next({ name: 'home' })
  } else {
    next()
  }
})

export default router
