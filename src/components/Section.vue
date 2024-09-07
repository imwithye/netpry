<template>
  <div class="w-full h-[24px] flex items-center px-4 py-1 text-xs font-mono section">
    <el-icon class="mr-2">
      <CaretRight />
    </el-icon>
    <div>{{ title }}</div>
  </div>
</template>

<script setup lang="ts">
import { CaretRight } from '@element-plus/icons-vue'

defineProps({
  title: {
    type: String,
    required: true
  }
})
</script>

<style scoped>
.section {
  background-color: var(--el-bg-color);
  color: var(--el-text-color-secondary);
}
</style>
