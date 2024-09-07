<template>
  <el-tooltip effect="dark" placement="top" :content="uri" :visible="visible">
    <div
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
