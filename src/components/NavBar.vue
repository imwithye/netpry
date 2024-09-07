<template>
  <div class="w-full h-[32px] flex justify-between items-center px-4 py-1 font-mono">
    <div></div>
    <div>
      <el-icon class="mr-2">
        <Histogram />
      </el-icon>
      <span>netpry - network inspection</span>
    </div>
    <div>
      <el-switch class="theme-switch" v-model="darkMode">
        <template #active-action>
          <el-icon>
            <Moon class="text-gray-800" />
          </el-icon>
        </template>
        <template #inactive-action>
          <el-icon>
            <Sunny class="text-gray-800" />
          </el-icon>
        </template>
      </el-switch>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Histogram, Sunny, Moon } from '@element-plus/icons-vue'
import { ref } from 'vue'

const darkMode = ref(false)
</script>

<style scoped>
.theme-switch {
  --el-switch-on-color: #f2f2f2;
  --el-switch-off-color: #f2f2f2;
  --el-switch-border-color: var(--el-border-color);
}
</style>
