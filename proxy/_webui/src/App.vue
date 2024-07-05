<template>
  <el-container style="width: 100%; height: 100%">
    <el-header class="header"> netpry </el-header>

    <el-main class="main">
      <RouterView />
    </el-main>

    <el-footer class="footer text-small"> netpry - Advanced traffic inspection tool </el-footer>
  </el-container>
</template>

<script setup lang="ts">
import { RouterView } from 'vue-router'
</script>

<style scoped>
.header {
  border-bottom: 0.3px solid #ffffff32;
  height: 48px;
  display: flex;
  justify-content: center;
  align-items: center;
}

.main {
  padding: 0px !important;
}

.footer {
  text-align: center;
  border-top: 0.3px solid #ffffff32;
  height: 32px;
  display: flex;
  justify-content: center;
  align-items: center;
}
</style>
