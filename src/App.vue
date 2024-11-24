<script setup>
import { ref, onMounted, onUnmounted } from "vue";
import { message } from "@tauri-apps/plugin-dialog";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";
import Install from "./views/Install.vue";
const isInstalled = ref(true);
const hasPermission = ref(false);
onMounted(async () => {
  //install check
  if ((await getCurrentWindow().label) != "main") {
    return;
  }
  isInstalled.value = await invoke("is_installed");

  //has permission
  hasPermission.value = await invoke("has_permission");
  console.log("hasPermission", hasPermission.value);
  if (!hasPermission.value) {
    await message("Permission denied, please run as admin", "error");
    await getCurrentWindow().close();
    return;
  }

  //init
  if (isInstalled.value) {
    await invoke("init");
  }
});
onUnmounted(() => {});
</script>
<template>
  <div v-show="!isInstalled">
    <Install @finished="isInstalled = true" />
  </div>
  <router-view v-show="isInstalled"></router-view>
</template>
