import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

export const useProxyStore = defineStore('proxy', () => {
  const isRunning = ref(false)

  async function run(ingress: string, backend: string) {
    try {
      await invoke('cmd_proxy_run', { ingress, backend })
      isRunning.value = true
    } catch (error) {
      throw error
    }
  }

  return {
    isRunning,
    run
  }
})
