<template>
  <el-tabs type="border-card">
    <el-tab-pane class="no-padding" label="Headers">
      <HeadersTab />
    </el-tab-pane>
    <el-tab-pane class="no-padding" label="Payload">Config</el-tab-pane>
    <el-tab-pane class="no-padding" label="Preview">Preview</el-tab-pane>
    <el-tab-pane class="no-padding" label="Response">Response</el-tab-pane>
  </el-tabs>
</template>

<script lang="ts" setup>
import HeadersTab from '../views/tabs/HeadersTab.vue'
</script>

<style scoped>
.no-padding {
  margin: -15px;
}
</style>
