<script setup>
import { ref, onMounted, onUnmounted, reactive } from "vue";
import { createDir, exists, currentDir } from "./utils/fs";
import { projectsDir } from "./stores/app";
async function initProjects() {
  if (await exists(projectsDir)) {
    return;
  }
  await createDir(projectsDir);
}
onMounted(async () => {
  await initProjects();
});
onUnmounted(() => {});
</script>
<template>
  <router-view></router-view>
</template>
