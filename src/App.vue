<template>
  <div class="w-full h-full flex flex-col">
    <NavBar />
    <Content class="flex-grow"/>
    <Footer />
  </div>
</template>

<script lang="ts" setup>
import NavBar from "./components/NavBar.vue";
import Content from "./components/Content.vue";
import Footer from "./components/Footer.vue";
</script>