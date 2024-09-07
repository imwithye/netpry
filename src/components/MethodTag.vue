<template>
  <el-tag :type="tagType" class="uppercase font-mono">{{ method }}</el-tag>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps({
  method: {
    type: String,
    required: true
  },
  status_code: {
    type: Number,
    default: 0
  }
})

const tagType = computed(() => {
  const status_code = props.status_code

  if (status_code >= 200 && status_code < 300) {
    return 'success'
  }
  if (status_code >= 300 && status_code < 400) {
    return 'info'
  }
  if (status_code >= 400 && status_code < 500) {
    return status_code === 404 ? 'warning' : 'danger'
  }
  if (status_code >= 500) {
    return 'danger'
  }
  return 'info'
})
</script>
