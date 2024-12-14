<script setup>
import { ref, onMounted, onUnmounted, watchEffect } from "vue";
import { message } from "@tauri-apps/plugin-dialog";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { getCurrentWebview } from "@tauri-apps/api/webview";
import { getAllWindows } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";
import { useStore } from "vuex";
import { minZoomFactor, maxZoomFactor } from "./stores/app";
import Install from "./views/Install.vue";
const store = useStore();
const isInstalled = ref(true);
const hasPermission = ref(false);
const currentZoomFactor = ref(1);
async function closeOtherWindows() {
  const windows = await getAllWindows();
  for (const window of windows) {
    if (window.label != "main") {
      await window.close();
    }
  }
}
async function shortcutZoom(event) {
  if (
    (event.key === "=" && event.ctrlKey) ||
    (event.key === "=" && event.metaKey)
  ) {
    event.preventDefault();
    store.commit(
      "zoomFactor",
      Math.min(store.getters.zoomFactor + 0.1, maxZoomFactor)
    );
  }
  if (
    (event.key === "-" && event.ctrlKey) ||
    (event.key === "-" && event.metaKey)
  ) {
    event.preventDefault();
    store.commit(
      "zoomFactor",
      Math.max(store.getters.zoomFactor - 0.1, minZoomFactor)
    );
  }
  if (
    (event.key === "0" && event.ctrlKey) ||
    (event.key === "0" && event.metaKey)
  ) {
    event.preventDefault();
    store.commit("zoomFactor", 1);
  }
}
watchEffect(async () => {
  if (store.getters.zoomFactor != currentZoomFactor.value) {
    currentZoomFactor.value = store.getters.zoomFactor;
    await getCurrentWebview().setZoom(currentZoomFactor.value);
  }
});
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

  //zoom
  window.addEventListener("keyup", shortcutZoom);

  //init
  if (isInstalled.value) {
    await invoke("init");
  }
});
onUnmounted(() => {
  //cancel zoom
  window.removeEventListener("keyup", shortcutZoom);
});
</script>
<template>
  <div v-show="!isInstalled">
    <Install @finished="isInstalled = true" />
  </div>
  <router-view v-show="isInstalled"></router-view>
</template>
