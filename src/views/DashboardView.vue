<template>
  <div class="w-full h-full overflow-hidden flex flex-col">
    <NavBar />

    <div class="flex-grow w-full overflow-hidden">
      <div class="w-full h-full flex">
        <RequestList class="w-1/3 h-full" />
        <RequestDetail class="w-2/3 h-full" />
      </div>
    </div>

    <Footer />
  </div>
</template>

<script lang="ts" setup>
import NavBar from '../components/NavBar.vue'
import Footer from '../components/Footer.vue'
import RequestList from '../components/RequestList.vue'
import RequestDetail from '../components/RequestDetail.vue'
</script>
