<template>
  <div class="w-full h-full overflow-hidden">
    <el-table class="w-full" :data="requests">
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
import { ref } from 'vue'

const requests = ref([
  {
    method: 'GET',
    url: 'https://google.com',
    status: 200
  },
  {
    method: 'GET',
    url: 'https://google.com',
    status: 200
  },
  {
    method: 'GET',
    url: 'https://google.com',
    status: 200
  }
])
</script>
