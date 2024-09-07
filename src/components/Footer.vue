<template>
  <div class="w-full h-[24px] flex items-center px-4 py-1 font-mono text-xs">
    <div>{{ requestCount }} {{ requestCount > 1 ? 'requests' : 'request' }}</div>
    <div class="grow"></div>
    <div>
      <el-link type="info" href="https://github.com/imwithye/netpry" target="_blank">
        netpry - https://github.com/imwithye/netpry
      </el-link>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { useRequestDetailsStore } from '../store/request_details_store.ts'
import { computed } from 'vue'

const requestDetailsStore = useRequestDetailsStore()
const requestCount = computed(() => requestDetailsStore.requestDetailsList.length)
</script>
