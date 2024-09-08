<template>
  <el-scrollbar height="100%">
    <Section title="General">
      <div class="flex align-top">
        <div class="key">Request URL:</div>
        <div class="value">{{ requestDetails?.uri }}</div>
      </div>
      <div class="flex align-top">
        <div class="key">Request Method:</div>
        <div class="value">{{ requestDetails?.method }}</div>
      </div>
      <div class="flex align-top">
        <div class="key">Status Code:</div>
        <div class="value">{{ requestDetails?.status_code }}</div>
      </div>
    </Section>
    <Section title="Request Headers">
      <div
        class="flex align-top"
        v-for="(key, idx) in Object.keys(requestDetails?.request_headers || [])"
        :key="idx"
      >
        <div class="key">{{ key }}:</div>
        <div class="value">{{ requestDetails?.request_headers?.[key] }}</div>
      </div>
    </Section>
    <Section title="Response Headers">
      <div
        class="flex align-top"
        v-for="(key, idx) in Object.keys(requestDetails?.response_headers || [])"
        :key="idx"
      >
        <div class="key">{{ key }}:</div>
        <div class="value">{{ requestDetails?.response_headers?.[key] }}</div>
      </div>
    </Section>
  </el-scrollbar>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRequestDetailsStore } from '../../store/request_details_store'
import Section from '../../components/Section.vue'

const requestDetailsStore = useRequestDetailsStore()
const requestDetails = computed(() => requestDetailsStore.activatedRequestDetails)
</script>

<style scoped>
.key {
  min-width: 200px;
  width: 200px;
}

.value {
  word-break: break-all;
}
</style>
