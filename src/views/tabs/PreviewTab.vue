<template>
  <div>Hello World</div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRequestDetailsStore } from '../../store/request_details_store'

const requestDetailsStore = useRequestDetailsStore()
const requestDetails = computed(() => {
  const details = requestDetailsStore.activatedRequestDetails
  console.log(details)
  return details
})
</script>
