<script setup>
import {
  ref,
  onMounted,
  watch,
  computed,
  reactive,
  onUnmounted,
  watchEffect,
} from "vue";
import { useStore } from "vuex";
import { getDefaultScriptFileByProjctPath } from "../stores/app";
const store = useStore();
const runFile = ref(null);

async function runCurrent() {
  runFile.value = store.getters.filePath;
  console.log(runFile.value);
}
async function runProject() {
  runFile.value = await getDefaultScriptFileByProjctPath(
    store.getters.projectPath
  );
  console.log(runFile.value);
}

async function stop() {}
</script>
<template>
  <el-button type="primary" @click="runCurrent">run current file</el-button>
  <el-button type="primary" @click="runProject">run</el-button>
  <el-button type="primary" @click="stop">stop</el-button>
</template>
