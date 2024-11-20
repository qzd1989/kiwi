<script setup>
import { ref, onMounted, onUnmounted } from "vue";
import { type, arch } from "@tauri-apps/plugin-os";
import { createDir, exists } from "./utils/fs";
import { projectsDir } from "./stores/app";
import { message } from "@tauri-apps/plugin-dialog";
import { msgSuccess, msgError } from "./utils/msg";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";
import Install from "./views/Install.vue";
const isInstalled = ref(true);
onMounted(async () => {
  //install check
  if ((await getCurrentWindow().label) != "main") {
    return;
  }
  isInstalled.value = await invoke("is_installed");
  //install check end
});
onUnmounted(() => {});
</script>
<template>
  <div v-show="!isInstalled">
    <Install @finished="isInstalled = true" />
  </div>
  <router-view v-show="isInstalled"></router-view>
</template>
