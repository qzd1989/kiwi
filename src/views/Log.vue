<script setup>
import { ref, onMounted, reactive } from "vue";
import { useStore } from "vuex";
import { getDefaultScriptFileByProjctPath } from "../stores/app";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { getAllWindows } from "@tauri-apps/api/window";
import { Stack, formatLogTime } from "./../utils/common";
import { useElementSize } from "@vueuse/core";
import { VideoPlay, VideoPause } from "@element-plus/icons-vue";
import {
  register,
  unregister,
  isRegistered,
} from "@tauri-apps/plugin-global-shortcut";

const props = defineProps(["height", "files"]);
const store = useStore();
const runFile = ref(null);
const runAtFrontend = ref(false);
const env = ref(null);
const logs = ref(new Stack(100));
const logsContainerRef = ref(null);
const logsRef = ref(null);
const logsSize = reactive({ width: 0, height: 0 });
logsSize.value = useElementSize(logsRef);
async function runCurrent() {
  console.log("runCurrent");
  runAtFrontend.value = false;
  if (props.files.size == 0) {
    return;
  }
  runFile.value = store.getters.filePath;
  await invoke("run", { file: runFile.value });
}
async function runProject() {
  console.log("runProject");
  runAtFrontend.value = false;
  runFile.value = await getDefaultScriptFileByProjctPath(
    store.getters.projectPath
  );
  await invoke("run", { file: runFile.value });
}
async function stop() {
  console.log("stop");
  runAtFrontend.value = false;
  await invoke("stop");
}

async function runCurrentAtFrontEnd() {
  console.log("runCurrentAtFrontEnd");
  runAtFrontend.value = true;
  if (props.files.size == 0) {
    return;
  }
  runFile.value = store.getters.filePath;
  await invoke("run", { file: runFile.value });
}
async function runProjectAtFrontEnd() {
  console.log("runProjectAtFrontEnd");
  runAtFrontend.value = true;
  runFile.value = await getDefaultScriptFileByProjctPath(
    store.getters.projectPath
  );
  await invoke("run", { file: runFile.value });
}
async function stopAtFrontEnd() {
  console.log("stopAtFrontEnd");
  runAtFrontend.value = false;
  await invoke("stop");
}

async function clear() {
  logs.value = new Stack(100);
}
async function minimizeAll() {
  const windows = await getAllWindows();
  for (const window of windows) {
    await window.minimize();
  }
}
async function unminimizeAll() {
  const windows = await getAllWindows();
  for (const window of windows) {
    await window.unminimize();
  }
}
async function shortcutExecute() {
  if (await isRegistered("F9")) {
    await unregister("F9");
  }
  if (await isRegistered("F10")) {
    await unregister("F10");
  }
  if (await isRegistered("F11")) {
    await unregister("F11");
  }
  await register("F9", async (event) => {
    if (event.state == "Released") {
      await runCurrent();
    }
  });

  await register("F10", async (event) => {
    if (event.state == "Released") {
      await runProject();
    }
  });

  await register("F11", async (event) => {
    if (event.state == "Released") {
      await stop();
    }
  });
}
async function getEnv() {
  env.value = await invoke("env_string");
}
listen("run:status", async (event) => {
  if (event.payload.data == "running") {
    console.log("running", runAtFrontend.value);
    if (!runAtFrontend.value) {
      await minimizeAll();
    }
  }
  if (event.payload.data == "stopped") {
    console.log("running", runAtFrontend.value);
    if (!runAtFrontend.value) {
      await unminimizeAll();
    }
  }
});
listen("log:info", (event) => {
  logs.value.push(event.payload);
  if (logsContainerRef.value !== null) {
    setTimeout(() => {
      logsContainerRef.value.$el.scrollTop = logsSize.value.height;
    }, 50);
  }
});
listen("log:error", (event) => {
  logs.value.push(event.payload);
  if (logsContainerRef.value !== null) {
    setTimeout(() => {
      logsContainerRef.value.$el.scrollTop = logsSize.value.height;
    }, 50);
  }
});
onMounted(async () => {
  await shortcutExecute();
  await getEnv();
});
</script>
<template>
  <el-container
    :style="{
      height: props.height - 20 + 'px',
    }"
  >
    <el-header>
      <el-button
        type="primary"
        @click="runCurrentAtFrontEnd"
        :icon="VideoPlay"
        v-show="files.size > 0"
      >
        current file (F9)
      </el-button>
      <el-button type="primary" @click="runProjectAtFrontEnd" :icon="VideoPlay"
        >project (F10)</el-button
      >
      <el-button type="primary" @click="stopAtFrontEnd" :icon="VideoPause"
        >stop (F11)</el-button
      >
      <el-button type="primary" @click="clear">clear</el-button>
    </el-header>
    <el-main ref="logsContainerRef">
      <div class="logs" ref="logsRef">
        <div class="log">{{ env }}</div>
        <div class="log" v-for="log in logs.stack">
          <span class="time">{{ formatLogTime(log.time) }}</span>
          <span class="data">{{ log.data }}</span>
        </div>
      </div>
    </el-main>
  </el-container>
</template>

<style scoped>
.el-container {
  .el-header {
    text-align: right;
    margin-bottom: 10px;
  }
  .el-main {
    height: 100%;
    overflow-y: scroll;
    .logs {
      .log {
        font-size: 10px;
        line-height: 13px;
        .time {
          opacity: 0.5;
          margin-right: 5px;
        }
        .data {
        }
      }
    }
  }
}
</style>
