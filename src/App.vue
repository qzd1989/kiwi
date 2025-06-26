<script setup>
import "element-plus/dist/index.css";
import { onMounted, onUnmounted, watchEffect, ref, reactive } from "vue";
import { join } from "@tauri-apps/api/path";
import { useStateStore } from "@utils/state-store";
import { getLocalValue, setLocalValue } from "@utils/local-store";
import { listen } from "@tauri-apps/api/event";
import { msgError, msgSuccess } from "@utils/msg";
import { openWebsocket, isWebsocketAlive } from "@utils/common";
import { invoke } from "@tauri-apps/api/core";
import { platform } from "@tauri-apps/plugin-os";
import { useRouter } from "vue-router";
import {
  checkAccessibilityPermission,
  checkScreenRecordingPermission,
} from "tauri-plugin-macos-permissions-api";
import { getCurrentWebview } from "@tauri-apps/api/webview";
import { getVersion } from "./apis/app";
import { open } from "@tauri-apps/plugin-shell";

const router = useRouter();
const stateStore = useStateStore();
const currentZoomFactor = ref(1);
const version = reactive({
  shouldUpdate: false,
  title: null,
  changelogs: [],
  download_url: null,
});

const init = async () => {
  await focus();
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

const focus = async () => {
  await getCurrentWebview().setFocus();
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

const checkVersion = async () => {
  // reset
  version.shouldUpdate = false;
  version.title = null;
  version.changelogs = [];
  version.download_url = null;

  let data = await getVersion();
  if (data == null) return;

  version.title = `New Verion: ${data.latest}`;
  version.download_url = data.download_url;
  version.changelogs = data.changelog.split(";");

  try {
    const appVersion = await invoke("get_app_version");
    if (data.minimum_supported != appVersion || data.force_update) {
      version.shouldUpdate = true;
    }
  } catch (error) {
    msgError(error.message);
    version.shouldUpdate = false;
  }
};

const updateVersion = async () => {
  try {
    if (version.download_url != null) {
      await open(version.download_url);
    }
  } catch (error) {
    msgError(error.message);
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
  await checkVersion();
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
  <!-- version update dialog -->
  <el-dialog
    v-model="version.shouldUpdate"
    :title="version.title"
    width="80vw"
    :align-center="true"
    :show-close="false"
    :close-on-press-escape="false"
    :close-on-click-modal="false"
  >
    <el-scrollbar max-height="50vh">
      <ul class="new-version-changelogs">
        <li v-for="item in version.changelogs">{{ item }}</li>
      </ul>
    </el-scrollbar>

    <template #footer>
      <div class="dialog-footer">
        <el-button type="primary" @click="updateVersion"> Download </el-button>
      </div>
    </template>
  </el-dialog>
</template>
<style scoped>
.new-version-changelogs {
  list-style: none;
  padding: 0;
  margin: 0px;
  padding-left: 20px;
  padding-right: 20px;
  li {
    line-height: 25px;
    opacity: 0.7;
  }
}
</style>
