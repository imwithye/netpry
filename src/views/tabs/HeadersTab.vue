<template>
  <Section title="General" />
  <Section title="Request Headers" />
  <Section title="Response Headers" />
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRequestDetailsStore } from '../../store/request_details_store'
import Section from '../../components/Section.vue'

const requestDetailsStore = useRequestDetailsStore()
const requestDetails = computed(() => requestDetailsStore.activatedRequestDetails)
</script>
