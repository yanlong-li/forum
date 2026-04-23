<script setup lang="ts">
import { RouterLink } from 'vue-router'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
</script>

<template>
  <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    <h1 class="text-2xl font-bold text-slate-900 mb-8">{{ t('settings.settings') }}</h1>

    <div class="space-y-4">
      <RouterLink
        to="/drafts"
        class="card p-4 block hover:border-primary transition-colors"
      >
        <h2 class="font-medium text-slate-900">{{ t('settings.myDrafts') }}</h2>
        <p class="text-sm text-slate-500 mt-1">{{ t('settings.manageYourDrafts') }}</p>
      </RouterLink>

      <RouterLink
        to="/settings/blocks"
        class="card p-4 block hover:border-primary transition-colors"
      >
        <h2 class="font-medium text-slate-900">{{ t('settings.blockedUsers') }}</h2>
        <p class="text-sm text-slate-500 mt-1">{{ t('settings.manageBlockedUsers') }}</p>
      </RouterLink>

      <RouterLink
        to="/settings/profile"
        class="card p-4 block hover:border-primary transition-colors"
      >
        <h2 class="font-medium text-slate-900">{{ t('settings.editProfile') }}</h2>
        <p class="text-sm text-slate-500 mt-1">{{ t('settings.updateProfileInfo') }}</p>
      </RouterLink>
    </div>
  </div>
</template>