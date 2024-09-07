<template>
  <div>
    <div
      class="w-full h-[24px] cursor-pointer flex items-center px-4 py-1 text-xs font-mono section"
      @click="isOpen = !isOpen"
    >
      <el-icon class="mr-2">
        <CaretRight v-if="!isOpen" />
        <CaretBottom v-else />
      </el-icon>
      <div>{{ title }}</div>
    </div>
    <div v-if="isOpen" class="px-4 py-1 text-xs font-mono">
      <slot />
    </div>
  </div>
</template>

<script setup lang="ts">
import { CaretBottom, CaretRight } from '@element-plus/icons-vue'
import { ref } from 'vue'

defineProps({
  title: {
    type: String,
    required: true
  }
})

const isOpen = ref(true)
</script>

<style scoped>
.section {
  background-color: var(--el-bg-color-page);
  color: var(--el-text-color-secondary);
}
</style>
