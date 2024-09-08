<template>
  <div
    class="h-full border-t border-b overflow-hidden"
    style="border-color: var(--el-border-color)"
  >
    <div
      class="flex items-center w-full px-2 border-b"
      style="border-color: var(--el-border-color)"
    >
      <el-icon>
        <Filter />
      </el-icon>
      <el-input v-model="filter" class="no-border-input" placeholder="Filter..." clearable />
    </div>

    <el-table
      class="w-full"
      height="100%"
      :data="filteredRequestDetailsList"
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
import { computed, ref } from 'vue'
import { Filter } from '@element-plus/icons-vue'

const filter = ref<string>('')

const requestDetailsStore = useRequestDetailsStore()

const filteredRequestDetailsList = computed(() => {
  const list = requestDetailsStore.requestDetailsList
  if (!filter.value) return list
  return list.filter((item) => item.uri.indexOf(filter.value) > -1)
})

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

.no-border-input :deep(.el-input__inner) {
  border: none;
  outline: none;
  box-shadow: none;
}

.no-border-input :deep(.el-input__wrapper) {
  border: none;
  outline: none;
  box-shadow: none;
}
</style>
