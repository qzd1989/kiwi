<script setup>
import { onMounted, onUnmounted, reactive, ref } from "vue";
import { useStateStore } from "@utils/state-store";
import { getLocalValue, setLocalValue } from "@utils/local-store";
import {
  openWebsocket,
  shutdownWebsocket,
  isWebsocketAlive,
  delay,
} from "@utils/common";
import { msgError, msgSuccess, msgInfo } from "@utils/msg";
import { invoke } from "@tauri-apps/api/core";
const stateStore = useStateStore();
const shouldShowSaveSuccess = ref(true);
const form = reactive({
  originalWebsocketPort: 0,
  websocketPort: 0,
});
const rules = reactive({
  websocketPort: [{ required: true, message: "", trigger: "blur" }],
});

const saveSetting = async () => {
  shouldShowSaveSuccess.value = true;

  try {
    await saveWebsocketPort();
  } catch (error) {
    return;
  }

  try {
    await invoke("save_app_config", {
      websocketPort: String(form.websocketPort),
    });
    if (shouldShowSaveSuccess.value) {
      msgSuccess("Settings saved successfully.");
    }
  } catch (error) {
    msgError(error.message);
  }
};

const saveWebsocketPort = async () => {
  const isOriginalPortAlive = await isWebsocketAlive(
    form.originalWebsocketPort
  );
  const portChanged = form.originalWebsocketPort !== form.websocketPort;
  if (!portChanged) return;
  stateStore.app.enable.isWebsocketAlive = false;
  try {
    await shutdownWebsocket();
    await delay(200);
    await openWebsocket(form.websocketPort);
    msgSuccess(`WebSocket started successfully on port ${form.websocketPort}.`);
    form.originalWebsocketPort = form.websocketPort;
    stateStore.app.enable.isWebsocketAlive = true;
  } catch (error) {
    if (isOriginalPortAlive) {
      try {
        await delay(200);
        await shutdownWebsocket();
        await delay(200);
        await openWebsocket(form.originalWebsocketPort);
        stateStore.app.enable.isWebsocketAlive = true;
        const infoMsg = `WebSocket failed to start on port ${form.websocketPort}. Reverted to the original port ${form.originalWebsocketPort}, which started successfully.`;
        msgInfo(infoMsg);
        form.websocketPort = form.originalWebsocketPort;
        stateStore.app.enable.isWebsocketAlive = true;
        shouldShowSaveSuccess.value = false;
      } catch (rollbackError) {
        stateStore.app.enable.isWebsocketAlive = false;
        const errMsg = `WebSocket failed to start. Both the original port (${form.originalWebsocketPort}) and the new port (${form.websocketPort}) are unavailable.`;
        msgError(errMsg);
        throw new Error(errMsg);
      }
    } else {
      stateStore.app.enable.isWebsocketAlive = false;
      const errMsg = `WebSocket failed to start on port ${form.websocketPort}, and original port ${form.originalWebsocketPort} is also unavailable.`;
      msgError(errMsg);
      throw new Error(errMsg);
    }
  }
};

const loadConfig = async () => {
  try {
    const config = await invoke("get_app_config");
    form.websocketPort = form.originalWebsocketPort = config.app.websocket_port;
  } catch (error) {
    msgError(error.message);
  }
};

onMounted(async () => {
  await loadConfig();
});

onUnmounted(async () => {});
</script>
<template>
  <el-container>
    <el-header class="page-header">
      <el-row :gutter="0">
        <el-col :span="8" class="left">
          <router-link to="/">
            <el-icon :size="20" color="#fff"><ArrowLeft /></el-icon>
          </router-link>
        </el-col>
        <el-col :span="8" class="title">Setting</el-col>
        <el-col :span="8" class="right"></el-col>
      </el-row>
    </el-header>
    <el-main>
      <el-form
        ref="formRef"
        :model="form"
        :rules="rules"
        label-position="top"
        :show-message="false"
      >
        <el-form-item
          label="Websocket Port"
          prop="websocketPort"
          :required="true"
        >
          <el-input
            v-model="form.websocketPort"
            autocapitalize="off"
            autocorrect="off"
            spellcheck="false"
          >
          </el-input>
        </el-form-item>
        <el-form-item label="" prop="">
          <el-button type="primary" class="save" @click="saveSetting">
            Save
          </el-button>
        </el-form-item></el-form
      >
    </el-main>
  </el-container>
</template>
<style scoped>
.el-container {
  .el-main {
    display: flex;
    justify-content: center;
    .el-form {
      margin-top: 20px;
      width: 80vw;
      .tip {
        list-style: none;
        margin: 0px;
        padding: 0px;
        color: gray;
        line-height: 15px;
        li {
          margin-top: 5px;
          .success {
            color: green;
          }
        }
      }
      .save {
        width: 100%;
        margin-top: 5px;
      }
    }
  }
}
</style>
