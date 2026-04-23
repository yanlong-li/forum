<script setup lang="ts">
import { RouterLink } from 'vue-router'
</script>

<template>
  <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    <h1 class="text-2xl font-bold text-slate-900 mb-8">Settings</h1>

    <div class="space-y-4">
      <RouterLink
        to="/drafts"
        class="card p-4 block hover:border-primary transition-colors"
      >
        <h2 class="font-medium text-slate-900">My Drafts</h2>
        <p class="text-sm text-slate-500 mt-1">Manage your saved drafts</p>
      </RouterLink>

      <RouterLink
        to="/settings/blocks"
        class="card p-4 block hover:border-primary transition-colors"
      >
        <h2 class="font-medium text-slate-900">Blocked Users</h2>
        <p class="text-sm text-slate-500 mt-1">Manage users you have blocked</p>
      </RouterLink>

      <RouterLink
        to="/settings/profile"
        class="card p-4 block hover:border-primary transition-colors"
      >
        <h2 class="font-medium text-slate-900">Edit Profile</h2>
        <p class="text-sm text-slate-500 mt-1">Update your profile information</p>
      </RouterLink>
    </div>
  </div>
</template>