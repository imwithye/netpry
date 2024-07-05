<template>
  <div class="tab">
    <div class="header">
      <div style="font-weight: bold; margin-right: 20px">{{ tabName }}</div>
      <div class="header-button active">Header</div>
      <div class="header-button">Query</div>
      <div class="header-button">Body</div>
      <div class="header-button">Raw</div>
    </div>
    <div class="content"></div>
  </div>
</template>

<script setup lang="ts">
defineProps({
  tabName: String
})
</script>

<style scoped>
.tab {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.header {
  border-bottom: 0.3px solid #ffffff32;
  height: 32px;
  display: flex;
  align-items: center;
  padding-left: 10px;
  padding-right: 10px;
}

.header-button {
  margin-left: 10px;
  margin-right: 10px;
}

.active {
  color: var(--el-color-primary);
}

.content {
  flex-grow: 1;
  overflow: hidden;
}
</style>
