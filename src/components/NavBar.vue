<template>
  <div class="w-full h-[32px] flex justify-between items-center px-4 py-1 font-mono">
    <div class="w-[128px]"></div>
    <div class="flex justify-between items-center">
      <el-icon class="mr-2">
        <Histogram />
      </el-icon>
      <span>netpry - network inspector</span>
    </div>
    <div class="w-[128px] flex justify-end items-center">
      <el-switch class="theme-switch mr-2" v-model="isDark" @change="toggleDark">
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
      <el-icon>
        <Setting />
      </el-icon>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useDark, useToggle } from '@vueuse/core'
import { Histogram, Sunny, Moon, Setting } from '@element-plus/icons-vue'

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
