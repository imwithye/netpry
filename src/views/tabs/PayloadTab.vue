<template>
  <Section title="Query String Parameters"> </Section>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRequestDetailsStore } from '../../store/request_details_store'
import Section from '../../components/Section.vue'

const requestDetailsStore = useRequestDetailsStore()
const requestDetails = computed(() => {
  const details = requestDetailsStore.activatedRequestDetails
  console.log(details)
  return details
})
</script>
