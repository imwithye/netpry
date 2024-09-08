<template>
  <div class="w-full h-full">
    <MonacoEditor :value="body" language="html" readonly />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRequestDetailsStore } from '../../store/request_details_store'
import MonacoEditor from '../../components/MonacoEditor.vue'

const requestDetailsStore = useRequestDetailsStore()
const body = computed(() => {
  const details = requestDetailsStore.activatedRequestDetails
  if (!details) {
    return ''
  }
  const encodedBody = details.response_body || ''
  return atob(encodedBody)
})
</script>
