import { createI18n } from 'vue-i18n'
import en from './locales/en.json'
import zhCN from './locales/zh-CN.json'

type MessageSchema = typeof en

const savedLocale = localStorage.getItem('locale') || 'en'

const i18n = createI18n<[MessageSchema], 'en' | 'zh-CN'>({
  legacy: false,
  locale: savedLocale,
  fallbackLocale: 'en',
  messages: {
    'en': en,
    'zh-CN': zhCN
  }
})

export default i18n