<template>
  <el-dialog :model-value="visible" title="Setting" width="400" @close="() => $emit('close')">
    <el-form label-width="auto">
      <el-form-item label="Ingress Endpoint">
        <el-input />
      </el-form-item>
      <el-form-item label="Backend Endpoint">
        <el-input />
      </el-form-item>
    </el-form>

    <template #footer>
      <div class="dialog-footer">
        <el-button @click="() => $emit('close')">Cancel</el-button>
        <el-button type="primary">Confirm</el-button>
      </div>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
defineProps({
  visible: {
    type: Boolean
  }
})
</script>
