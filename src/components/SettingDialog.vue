<template>
  <el-dialog :model-value="visible" title="Setting" width="400" @close="() => $emit('close')">
    <el-form label-width="auto">
      <el-form-item label="Ingress Endpoint">
        <el-input v-model="ingress" />
      </el-form-item>
      <el-form-item label="Backend Endpoint">
        <el-input v-model="backend" />
      </el-form-item>
    </el-form>

    <template #footer>
      <div class="dialog-footer">
        <el-button @click="() => $emit('close')">Cancel</el-button>
        <el-button type="primary" @click="onConfirm">Confirm</el-button>
      </div>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { useProxyStore } from '../store/proxy_store'
import { ref } from 'vue'

defineProps({
  visible: {
    type: Boolean
  }
})

const $emit = defineEmits(['close'])

const ingress = ref('')
const backend = ref('')

const proxyStatusStore = useProxyStore()

const onConfirm = async () => {
  await proxyStatusStore.run(ingress.value, backend.value)
  $emit('close')
}
</script>
