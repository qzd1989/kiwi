<script setup>
import { onMounted, onUnmounted, ref, computed } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import { clearLocalStore } from "@utils/local-store";
import { getProjectRootDirectory } from "@utils/common";
import { useStateStore } from "@utils/state-store";
import { invoke } from "@tauri-apps/api/core";
import { useRouter } from "vue-router";
import { msgError } from "@utils/msg";
const isDev = import.meta.env.DEV;
const router = useRouter();
const stateStore = useStateStore();
const appVersion = ref(null);
const isAppEnabled = computed(() => {
  return Object.values(stateStore.app.enable).every((v) => v === true);
});

const selectProject = async () => {
  try {
    const selectedPath = await open({
      directory: true,
      multiple: false,
      defaultPath: await getProjectRootDirectory(),
    });
    if (selectedPath) {
      router.push({
        path: "/app/project/panel",
        query: { path: selectedPath },
      });
    }
  } catch (error) {
    msgError(error.message);
  }
};

const goToCreateProject = async () => {
  router.push({
    path: "/app/project/create",
  });
};

const goToSetting = () => {
  router.push({
    path: "/app/setting",
  });
};

const getAppVersion = async () => {
  try {
    appVersion.value = await invoke("get_app_version");
  } catch (error) {
    msgError(error.message);
  }
};

onMounted(async () => {
  await getAppVersion();
});

onUnmounted(async () => {});
</script>
<template>
  <el-container>
    <el-main>
      <el-row :gutter="0">
        <el-col :span="24">
          <div class="logo">
            <el-icon :size="100" color="#1230BA"><Star /></el-icon>
          </div>
          <div class="title">Kiwi</div>
          <div class="sologan">Hands-free, everything on autopilot.</div>
        </el-col>
      </el-row>
      <el-row :gutter="0">
        <el-col :span="24">
          <el-button
            type="primary"
            @click="goToCreateProject"
            :disabled="!isAppEnabled"
            >Create Project</el-button
          >
        </el-col>
      </el-row>
      <el-row :gutter="0">
        <el-col :span="24">
          <el-button
            type="primary"
            @click="selectProject"
            :disabled="!isAppEnabled"
            >Open Project</el-button
          >
        </el-col>
      </el-row>
      <el-row :gutter="0">
        <el-col :span="24">
          <el-button type="primary" @click="goToSetting">Setting</el-button>
        </el-col>
      </el-row>
      <el-row :gutter="0">
        <el-col :span="24">
          <el-button type="primary" @click="clearLocalStore" v-if="isDev"
            >ClearLocalStore</el-button
          >
        </el-col>
      </el-row>
      <el-row :gutter="0" style="display: none">
        <el-col :span="24">
          <el-button type="primary" @click="">Online Market</el-button>
        </el-col>
      </el-row>
      <el-row :gutter="0">
        <el-col :span="24" class="version"> version: {{ appVersion }}</el-col>
      </el-row>
    </el-main>
  </el-container>
</template>
<style scoped>
.el-container {
  height: 100vh;
  display: flex;
  justify-content: center;
  align-items: center;
  .el-main {
    max-width: 70vw;
    margin: auto;
    .el-row {
      .el-col {
        margin: 10px 0;
        text-align: center;
      }
      .logo {
        margin-bottom: 10px;
        margin-top: -20px;
      }
      .title {
        font-size: 35px;
        font-weight: bold;
        color: #1230ba;
        margin-bottom: 5px;
      }
      .sologan {
        font-size: 14px;
        color: #666;
      }
      .el-button {
        width: 100%;
        min-height: 40px;
      }
      .version {
        font-size: 14px;
        color: #999;
      }
    }
  }
}
</style>
