import { createApp } from 'vue'

import { createPinia } from 'pinia'
import router from './router.ts'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import 'element-plus/theme-chalk/dark/css-vars.css'
import * as ElementPlusIconsVue from '@element-plus/icons-vue'

import App from './App.vue'

import './assets/style.css'

import { listen } from '@tauri-apps/api/event'
import { RequestDetails, useRequestDetailsStore } from './store/request_details_store.ts'

const pinia = createPinia()
const app = createApp(App)

app.use(ElementPlus)
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
  app.component(key, component)
}
app.use(pinia)
app.use(router)
app.mount('#app')

const requestDetailsStore = useRequestDetailsStore()
await listen('update', (event) => {
  requestDetailsStore.addRequestDetails(event.payload as RequestDetails)
})
