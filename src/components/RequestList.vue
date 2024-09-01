<template>
  <div class="w-full h-full overflow-hidden">
    <el-table
      class="w-full"
      :data="requestDetailsStore.requestDetailsList"
      row-class-name="cursor-pointer"
      @row-click="(row: RequestDetails) => setActivatedRequestDetails(row)"
    >
      <el-table-column prop="method" label="Method" width="80" min-width="80">
        <template #default="scope">
          <el-tag>{{ scope.row.method }}</el-tag>
        </template>
      </el-table-column>
      <el-table-column prop="url" label="URL">
        <template #default="scope">
          <div>{{ scope.row.url }}</div>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>

<script lang="ts" setup>
import { RequestDetails, useRequestDetailsStore } from '../store/request_details_store.ts'

const requestDetailsStore = useRequestDetailsStore()

const setActivatedRequestDetails = (row: RequestDetails) => {
  const req: RequestDetails = JSON.parse(JSON.stringify(row))
  requestDetailsStore.setActiveRequestDetails(req)
}
</script>
