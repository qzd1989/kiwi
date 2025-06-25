<script setup>
import "element-plus/dist/index.css";
import { onMounted, onUnmounted, watchEffect, ref } from "vue";
import { join } from "@tauri-apps/api/path";
import { useStateStore } from "@utils/state-store";
import { getLocalValue, setLocalValue } from "@utils/local-store";
import { listen } from "@tauri-apps/api/event";
import { msgError } from "@utils/msg";
import { openWebsocket, isWebsocketAlive } from "@utils/common";
import { invoke } from "@tauri-apps/api/core";
import { platform } from "@tauri-apps/plugin-os";
import { useRouter } from "vue-router";
import {
  checkAccessibilityPermission,
  checkScreenRecordingPermission,
} from "tauri-plugin-macos-permissions-api";
import { getCurrentWebview } from "@tauri-apps/api/webview";

const router = useRouter();
const stateStore = useStateStore();
const currentZoomFactor = ref(1);

const init = async () => {
  if (!(await websocketInit())) {
    return;
  }
  if ((await platform()) == "macos") {
    if (!(await getLocalValue("isPythonAttributed"))) {
      if (!(await xattrPython())) {
        return;
      }
    }

    if (
      !(await checkAccessibilityPermission()) ||
      !(await checkScreenRecordingPermission())
    ) {
      router.push({
        path: "/app/permission_manager",
      });
    }
  } else {
    // windows直接跳过,不需要申请
    stateStore.app.enable.hasAccessibilityPermission = true;
    stateStore.app.enable.hasScreenRecordingPermission = true;
  }
};

const xattrPython = async () => {
  try {
    await invoke("xattr_python");
  } catch (error) {
    msgError(error.message);
    return false;
  }
  await setLocalValue("isPythonAttributed", true);
  return true;
};

const websocketInit = async () => {
  try {
    const config = await invoke("get_app_config");
    await openWebsocket(config.app.websocket_port);
    stateStore.app.enable.isWebsocketAlive = await isWebsocketAlive(
      config.app.websocket_port
    );
  } catch (error) {
    msgError(error.message);
    return false;
  }
  return true;
};

const shortcutZoom = async (event) => {
  if (
    (event.key === "=" && event.ctrlKey) ||
    (event.key === "=" && event.metaKey)
  ) {
    event.preventDefault();
    stateStore.zoom.factor = Math.min(
      stateStore.zoom.factor + 0.1,
      stateStore.zoom.max
    );
  }
  if (
    (event.key === "-" && event.ctrlKey) ||
    (event.key === "-" && event.metaKey)
  ) {
    event.preventDefault();
    stateStore.zoom.factor = Math.max(
      stateStore.zoom.factor - 0.1,
      stateStore.zoom.min
    );
  }
  if (
    (event.key === "0" && event.ctrlKey) ||
    (event.key === "0" && event.metaKey)
  ) {
    event.preventDefault();
    stateStore.zoom.factor = 1;
  }
};

listen("backend:update:project", async (event) => {
  const project = event.payload;
  if (project == null) {
    stateStore.project.exists = false;
    return;
  }
  stateStore.project.exists = true;
  stateStore.project.name = project.name;
  stateStore.project.language = project.language;
  stateStore.project.path = project.path;
  stateStore.project.mainFile = project.main_file;
  stateStore.project.kiwiVersion = project.kiwi_version;
  stateStore.project.mainFileFullPath = await join(
    project.path,
    project.main_file
  );
});

listen("msg:error", (event) => {
  msgError(event.payload);
});

watchEffect(async () => {
  if (stateStore.zoom.factor != currentZoomFactor.value) {
    currentZoomFactor.value = stateStore.zoom.factor;
    await getCurrentWebview().setZoom(currentZoomFactor.value);
  }
});

onMounted(async () => {
  await init();
  //zoom
  window.addEventListener("keyup", shortcutZoom);
});

onUnmounted(async () => {
  //cancel zoom
  window.removeEventListener("keyup", shortcutZoom);
});
</script>
<template>
  <router-view></router-view>
</template>
<style scoped></style>
