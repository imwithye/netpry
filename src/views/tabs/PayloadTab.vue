<template>
  <el-scrollbar height="100%">
    <Section title="Query String Parameters" v-if="queryStringParameters.length > 0">
      <div class="flex align-top" v-for="(param, idx) in queryStringParameters" :key="idx">
        <div class="key">{{ param.key }}:</div>
        <div class="value">{{ param.value }}</div>
      </div>
    </Section>
    <Section title="Request Body" v-if="body">
      <div class="w-full h-[320px]">
        <MonacoEditor :value="body" />
      </div>
    </Section>
  </el-scrollbar>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRequestDetailsStore } from '../../store/request_details_store'
import Section from '../../components/Section.vue'
import MonacoEditor from '../../components/MonacoEditor.vue'

const requestDetailsStore = useRequestDetailsStore()

const requestDetails = computed(() => {
  return requestDetailsStore.activatedRequestDetails
})

const queryStringParameters = computed(() => {
  const uri = requestDetails.value?.uri
  if (!uri) return new Array<{ key: string; value: string }>()
  const uriObject = new URL(uri)
  const searchParams = new URLSearchParams(uriObject.search)
  const queryParams = new Array<{ key: string; value: string }>()
  searchParams.forEach((value, key) => {
    queryParams.push({ key, value })
  })
  return queryParams
})

const body = computed(() => {
  const details = requestDetailsStore.activatedRequestDetails
  if (!details) {
    return ''
  }
  const encodedBody = details.request_body || ''
  return atob(encodedBody)
})
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
