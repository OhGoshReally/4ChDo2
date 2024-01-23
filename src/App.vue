<template>
  <div class="container pt-4">
    <h1 class="display-4 fw-bold">4ChDo 2.0</h1>

    <p class="m-0">4chan downloader</p>
    <p>The easy way to download images from 4chan</p>

    <Greet />
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>

<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Greet from "./components/Greet.vue";
</script>
