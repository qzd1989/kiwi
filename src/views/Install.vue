<script setup>
import { ref, onMounted } from "vue";
import { type, arch } from "@tauri-apps/plugin-os";
import { invoke } from "@tauri-apps/api/core";
import { msgError } from "../utils/msg";
const emits = defineEmits(["finished"]);
const progress = ref(0);
async function initialize() {
  let platform = type();
  let architecture = arch();
  progress.value = 1;
  invoke("initialize_projects", { architecture })
    .then(async (result) => {
      if (result) {
        return invoke("install_tesseract", { architecture });
      }
    })
    .then(async (result) => {
      if (result) {
        return invoke("install_tessdata", { architecture });
      }
    })
    .then(async (result) => {
      if (result) {
        return invoke("install_python", { architecture });
      }
    })
    .then(async (result) => {
      if (!result) {
        return invoke("repair_python", { architecture });
      } else {
        return result;
      }
    })
    .then(async (result) => {
      if (result) {
        return invoke("install_pip", { architecture });
      }
    })
    .then(async (result) => {
      if (result) {
        return invoke("install_whl", { architecture });
      }
    })
    .then(async () => {
      progress.value = 100;
    })
    .catch((error) => {
      console.log(error);
      msgError(error);
    });
}
async function begin() {
  invoke("init").then(async () => {
    emits("finished");
  });
}
onMounted(async () => {});
</script>
<template>
  <div v-if="progress < 100" class="install">
    <div class="install-box">
      <el-button
        type="primary"
        @click="initialize"
        size="large"
        :disabled="progress > 0"
        >Initialize</el-button
      >
      <div class="description" v-if="progress > 0">Please Wait...</div>
    </div>
  </div>
  <div v-if="progress == 100" class="begin">
    <el-button type="success" @click="begin" size="large">Begin</el-button>
  </div>
</template>
<style scoped>
.install {
  height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  .install-box {
    position: relative;
    text-align: center;
    .description {
      position: absolute;
      top: 50px;
      display: block;
      width: 300px;
      left: 50%;
      transform: translateX(-50%);
    }
  }
}
.begin {
  height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
}
</style>
