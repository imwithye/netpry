import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

export const useProxyStore = defineStore('proxy', () => {
  const isProxyRunning = ref(false)

  async function run(ingress: string, backend: string) {
    return invoke('cmd_proxy_run', { ingress, backend })
  }

  function setProxyRunning(value: boolean) {
    isProxyRunning.value = value
  }

  return {
    isProxyRunning,
    run,
    setProxyRunning
  }
})
