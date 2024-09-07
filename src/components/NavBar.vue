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
      <el-switch class="theme-switch" v-model="isDark" @change="toggleDark">
        <template #active-action>
          <el-icon>
            <Moon class="text-gray-200" />
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
import { useDark, useToggle } from '@vueuse/core'
import { Histogram, Sunny, Moon } from '@element-plus/icons-vue'

const isDark = useDark({
  disableTransition: false
})
const toggleDark = useToggle(isDark)
</script>

<style scoped>
.theme-switch {
  --el-switch-on-color: #2c2c2c;
  --el-switch-off-color: #f2f2f2;
  --el-switch-border-color: var(--el-border-color);
  --el-color-white: transparent;
}
</style>
