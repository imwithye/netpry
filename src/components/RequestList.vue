<template>
  <div
    class="h-full border-t border-b overflow-hidden"
    style="border-color: var(--el-border-color)"
  >
    <el-table
      class="w-full"
      height="100%"
      :data="requestDetailsStore.requestDetailsList"
      :row-class-name="tableRowClassName"
      @row-click="(row: RequestDetails) => setActivatedRequestDetails(row)"
    >
      <el-table-column prop="method" label="Method" width="80" min-width="80">
        <template #default="scope">
          <MethodTag :method="scope.row.method" :status_code="scope.row.status_code" />
        </template>
      </el-table-column>
      <el-table-column prop="uri" label="URL">
        <template #default="scope">
          <URI :uri="scope.row.uri" />
        </template>
      </el-table-column>
      <el-table-column prop="status_code" label="Status" width="80" min-width="80">
        <template #default="scope">
          <div class="text-xs font-mono">{{ scope.row.status_code }}</div>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>

<script lang="ts" setup>
import { RequestDetails, useRequestDetailsStore } from '../store/request_details_store.ts'
import MethodTag from './MethodTag.vue'
import URI from './URI.vue'
import { computed } from 'vue'

const requestDetailsStore = useRequestDetailsStore()
const requestDetails = computed(() => requestDetailsStore.activatedRequestDetails)

const setActivatedRequestDetails = (row: RequestDetails) => {
  const req: RequestDetails = JSON.parse(JSON.stringify(row))
  requestDetailsStore.setActiveRequestDetails(req)
}

const tableRowClassName = ({ row }: { row: RequestDetails }) => {
  if (requestDetails.value && requestDetails.value.uri == row.uri)
    return 'cursor-pointer active-row'
  return 'cursor-pointer'
}
</script>

<style scoped>
:deep(.active-row) {
  background-color: var(--el-table-current-row-bg-color);
}
</style>
