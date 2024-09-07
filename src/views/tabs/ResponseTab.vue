<template>
  <div>{{ body }}</div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRequestDetailsStore } from '../../store/request_details_store'

const requestDetailsStore = useRequestDetailsStore()
const body = computed(() => {
  const details = requestDetailsStore.activatedRequestDetails
  if (!details) {
    return null
  }
  return atob(details.response_body)
})
</script>
