<template>
  <div class="w-full h-full flex flex-col">
    <NavBar />

    <div class="flex-grow">Hello World</div>

    <Footer />
  </div>
</template>

<script lang="ts" setup>
import NavBar from '../components/NavBar.vue'
import Footer from '../components/Footer.vue'
</script>
