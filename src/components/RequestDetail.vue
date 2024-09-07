<template>
  <el-tabs type="border-card">
    <el-tab-pane class="no-padding" label="Headers">
      <HeadersTab v-if="requestDetails" />
    </el-tab-pane>
    <el-tab-pane class="no-padding" label="Payload">
      <PayloadTab v-if="requestDetails" />
    </el-tab-pane>
    <el-tab-pane class="no-padding" label="Preview">
      <PreviewTab v-if="requestDetails" />
    </el-tab-pane>
    <el-tab-pane class="no-padding" label="Response">
      <ResponseTab v-if="requestDetails" />
    </el-tab-pane>
  </el-tabs>
</template>

<script lang="ts" setup>
import HeadersTab from '../views/tabs/HeadersTab.vue'
import PayloadTab from '../views/tabs/PayloadTab.vue'
import PreviewTab from '../views/tabs/PreviewTab.vue'
import ResponseTab from '../views/tabs/ResponseTab.vue'

import { computed } from 'vue'
import { useRequestDetailsStore } from '../store/request_details_store'

const requestDetailsStore = useRequestDetailsStore()
const requestDetails = computed(() => requestDetailsStore.activatedRequestDetails)
</script>

<style scoped>
.no-padding {
  margin: -15px;
}
</style>
