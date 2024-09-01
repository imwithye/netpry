<template>
  <el-tabs type="border-card">
    <el-tab-pane label="Headers"
      >{{ requestDetails?.method }} {{ requestDetails?.uri }}</el-tab-pane
    >
    <el-tab-pane label="Payload">Config</el-tab-pane>
    <el-tab-pane label="Preview">Preview</el-tab-pane>
    <el-tab-pane label="Response">Response</el-tab-pane>
  </el-tabs>
</template>

<script lang="ts" setup>
import { useRequestDetailsStore } from '../store/request_details_store'
import { computed } from 'vue'

const requestDetailsStore = useRequestDetailsStore()
const requestDetails = computed(() => requestDetailsStore.activatedRequestDetails)
console.log(requestDetails)
</script>
