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
import { codeCheck } from "../utils/api";
import { msgInfo } from "../utils/msg";
const props = defineProps(["height", "files"]);
const store = useStore();
const runFile = ref(null);
const hideWhileRunning = ref(false);
const logs = ref(new Stack(100));
const logsContainerRef = ref(null);
const logsRef = ref(null);
const logsSize = reactive({ width: 0, height: 0 });
logsSize.value = useElementSize(logsRef);
async function runCurrent() {
  if (props.files.size == 0) {
    return;
  }
  runFile.value = store.getters.currentFilePath;
  await invoke("run", { file: runFile.value });
}
async function runProject() {
  runFile.value = await getDefaultScriptFileByProjctPath(
    store.getters.currentProjectPath
  );
  await invoke("run", { file: runFile.value });
}
async function stop() {
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
    window.unminimize().then(() => {
      return window.setFocus();
    });
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
async function check() {
  if (props.files.size == 0) {
    return;
  }
  runFile.value = store.getters.currentFilePath;
  await codeCheck(runFile.value);
}
listen("code_check:error", async (event) => {
  let errors = JSON.parse(event.payload.data);
  if (errors.length == 0) {
    return;
  }
  console.log(errors);
});
listen("run:status", async (event) => {
  if (event.payload.data == "running") {
    if (hideWhileRunning.value) {
      await minimizeAll();
    }
  }
  if (event.payload.data == "stopped") {
    if (hideWhileRunning.value) {
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
});
</script>
<template>
  <el-container
    :style="{
      height: props.height - 20 + 'px',
    }"
  >
    <el-header>
      <el-checkbox
        class="hide-while-running"
        v-model="hideWhileRunning"
        title="hide while running"
        tabindex="-1"
      ></el-checkbox>
      <el-button
        type="primary"
        @click="runCurrent"
        :icon="VideoPlay"
        v-show="files.size > 0"
        style="margin-left: 12px"
        tabindex="-1"
      >
        current file (F9)
      </el-button>
      <el-button type="primary" @click="check">code check</el-button>
      <el-button
        type="primary"
        @click="runProject"
        :icon="VideoPlay"
        tabindex="-1"
        >project (F10)</el-button
      >
      <el-button type="primary" @click="stop" :icon="VideoPause" tabindex="-1"
        >stop (F11)</el-button
      >
      <el-button type="primary" @click="clear">clear</el-button>
    </el-header>
    <el-main ref="logsContainerRef">
      <div class="logs" ref="logsRef">
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
    margin-bottom: 10px;
    display: flex;
    justify-content: right;
  }
  .el-main {
    height: 100%;
    overflow-y: scroll;
    .logs {
      .log {
        font-size: var(--SecondarySize);
        padding: 1px 0px;
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
