<script setup>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

onMounted(async () => {
  console.log("begin");
  invoke("snapshot").then((frame) => console.log(frame));
  console.log("end");
  console.log("size begin");
  invoke("display_size").then((size) => console.log(size));
  console.log("size end");
});
</script>
<template>
  <router-view></router-view>
</template>
