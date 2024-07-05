<template>
  <el-container style="width: 100%; height: 100%">
    <el-aside class="side">Aside</el-aside>
    <el-main class="main">
      <div class="content">
        <div class="request-list">
          <RequestListPanel />
        </div>
        <div class="request-detail">
          <div class="request-detail-request">
            <RequestDetailPanel />
          </div>
          <div class="request-detail-response">
            <ResponseDetailPanel />
          </div>
        </div>
      </div>
    </el-main>
  </el-container>
</template>

<script lang="ts" setup>
import RequestListPanel from '@/views/HomePanels/RequestListPanel.vue'
import RequestDetailPanel from '@/views/HomePanels/RequestDetailPanel.vue'
import ResponseDetailPanel from '@/views/HomePanels/ResponseDetailPanel.vue'
</script>

<style scoped>
.side {
  height: 100%;
  border-right: 0.3px solid #ffffff32;
  padding: 0px !important;
}

.main {
  height: 100%;
  padding: 0px !important;
}

.content {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
}

.request-list {
  height: 50%;
  border-bottom: 0.3px solid #ffffff32;
}

.request-detail {
  flex-grow: 1;
  display: flex;
}

.request-detail-request {
  width: 50%;
  border-right: 0.3px solid #ffffff32;
}

.request-detail-response {
  flex-grow: 1;
}
</style>
