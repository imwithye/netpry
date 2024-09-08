<template>
  <div class="w-full h-[24px] flex items-center px-4 py-1 font-mono text-xs">
    <div class="w-[128px] flex items-center">
      {{ requestCount }} {{ requestCount > 1 ? 'requests' : 'request' }}
    </div>
    <div class="grow flex justify-center items-center">
      <div
        class="mr-2"
        style="border-radius: 50%; width: 10px; height: 10px"
        :style="{
          backgroundColor: isProxyRunning ? 'var(--el-color-success)' : 'var(--el-color-danger)'
        }"
      ></div>
      Proxy {{ isProxyRunning ? '' : 'Not' }} Running
    </div>
    <div class="w-[128px] flex justify-end items-center">
      <el-link type="info" href="https://github.com/imwithye/netpry" target="_blank">
        netpry
      </el-link>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { useProxyStore } from '../store/proxy_store'
import { useRequestDetailsStore } from '../store/request_details_store'
import { computed } from 'vue'

const proxyStore = useProxyStore()
const isProxyRunning = computed(() => proxyStore.isRunning)
const requestDetailsStore = useRequestDetailsStore()
const requestCount = computed(() => requestDetailsStore.requestDetailsList.length)
</script>
