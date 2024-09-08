<template>
  <el-tooltip
    effect="dark"
    placement="top"
    :content="`<div class='break-all' style='max-width: 80vw'>${uri}</div>`"
    raw-content
    :visible="visible"
  >
    <div
      style="max-width: 80vw"
      class="truncate text-xs font-mono"
      @mouseenter="visible = true"
      @mouseleave="visible = false"
    >
      {{ uri }}
    </div>
  </el-tooltip>
</template>

<script setup lang="ts">
import { ref } from 'vue'

defineProps({
  uri: {
    type: String,
    required: true
  }
})

const visible = ref(false)
</script>
