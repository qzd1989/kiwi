<script setup>
import { ref, onMounted, reactive } from "vue";
import { useStore } from "vuex";
import { getDefaultScriptFileByProjctPath } from "../stores/app";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { Stack, formatLogTime } from "./../utils/common";
import { useElementSize } from "@vueuse/core";
import { VideoPlay, VideoPause } from "@element-plus/icons-vue";

const props = defineProps(["height"]);
const store = useStore();
const runFile = ref(null);
const logs = ref(new Stack(100));
const logsContainerRef = ref(null);
const logsRef = ref(null);
const logsSize = reactive({ width: 0, height: 0 });
logsSize.value = useElementSize(logsRef);
async function runCurrent() {
  runFile.value = store.getters.filePath;
  await invoke("run", { file: runFile.value });
}
async function runProject() {
  runFile.value = await getDefaultScriptFileByProjctPath(
    store.getters.projectPath
  );
  await invoke("run", { file: runFile.value });
}
async function stop() {
  await invoke("stop");
}
async function clear() {
  logs.value = new Stack(100);
}
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
onMounted(async () => {});
</script>
<template>
  <el-container
    :style="{
      height: props.height - 20 + 'px',
    }"
  >
    <el-header>
      <el-button type="primary" @click="runCurrent" :icon="VideoPlay">
        current file
      </el-button>
      <el-button type="primary" @click="runProject" :icon="VideoPlay"
        >project</el-button
      >
      <el-button type="primary" @click="stop" :icon="VideoPause"
        >stop</el-button
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
