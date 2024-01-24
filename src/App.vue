<template>
  <div class="container-fluid text-center pt-4">
    <h1 class="main-title mb-1">4ChDo 2.0</h1>

    <p class="sub-title m-0">4chan downloader</p>
    <p class="sub-title">The easy way to download images from 4chan</p>

    <Main />
  </div>
</template>

<style scoped>
  .main-title {
    font-size: calc(1.25rem + 1.5vw);
    line-height: 1.2;
    font-weight: bold;
  }

  .sub-title {
    font-size: 15px;
  }
</style>

<script setup lang="ts">
  import Main from "./components/Main.vue";
</script>
