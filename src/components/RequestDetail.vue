<template>
  <el-tabs type="border-card">
    <el-tab-pane class="no-padding" label="Headers">
      <HeadersTab />
    </el-tab-pane>
    <el-tab-pane class="no-padding" label="Payload">
      <PayloadTab />
    </el-tab-pane>
    <el-tab-pane class="no-padding" label="Preview">
      <PreviewTab />
    </el-tab-pane>
    <el-tab-pane class="no-padding" label="Response">Response</el-tab-pane>
  </el-tabs>
</template>

<script lang="ts" setup>
import HeadersTab from '../views/tabs/HeadersTab.vue'
import PayloadTab from '../views/tabs/PayloadTab.vue'
import PreviewTab from '../views/tabs/PreviewTab.vue'
</script>

<style scoped>
.no-padding {
  margin: -15px;
}
</style>
