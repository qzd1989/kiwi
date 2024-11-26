<script setup>
import { ref, onMounted, onUnmounted } from "vue";
import { message } from "@tauri-apps/plugin-dialog";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { getAllWindows } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";
import Install from "./views/Install.vue";
const isInstalled = ref(true);
const hasPermission = ref(false);
async function closeOtherWindows() {
  const windows = await getAllWindows();
  for (const window of windows) {
    if (window.label != "main") {
      await window.close();
    }
  }
}
onMounted(async () => {
  const currentWindow = await getCurrentWindow();

  //install check
  if (currentWindow.label != "main") {
    return;
  }
  isInstalled.value = await invoke("is_installed");

  //has permission
  hasPermission.value = await invoke("has_permission");
  if (!hasPermission.value) {
    await message("Please run as admin", "error");
    await currentWindow.close();
    return;
  }

  //close other windows if close main window
  currentWindow.onCloseRequested(async () => {
    await closeOtherWindows();
  });

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
