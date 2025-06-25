<script setup>
import { ref, onMounted, onUnmounted, reactive, nextTick } from "vue";
import { Delete } from "@element-plus/icons-vue";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { getAllWindows } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";
import { join } from "@tauri-apps/api/path";
import { useRoute, useRouter } from "vue-router";
import { useStateStore } from "@utils/state-store";
import { platform } from "@tauri-apps/plugin-os";
import { ElLoading } from "element-plus";
import {
  Stack,
  formatLogTime,
  minimizeAll,
  unminimizeAll,
  delay,
} from "@utils/common";
import { listen } from "@tauri-apps/api/event";
import { register, unregister } from "@tauri-apps/plugin-global-shortcut";
import { msgError, msgSuccess } from "@utils/msg";
const errorMessage = ref(null);
const reinitProgressLoading = ref(null);
const openProjectProgressLoading = ref(null);
const stateStore = useStateStore();
const route = useRoute();
const router = useRouter();
const currentFile = ref(null);
const logs = ref(new Stack(100));
const hiders = reactive({
  recorder: false,
  runProject: false,
  runScript: false,
});
const logScrollbarRef = ref(null);
const hotKeys = reactive({
  recorder: "F9",
  runScript: "F10",
  runProject: "F11",
  stopAll: "F12",
});

const initHotKeys = async () => {
  const pf = await platform();
  switch (pf) {
    case "macos":
      hotKeys.recorder = "F9";
      hotKeys.runScript = "F10";
      hotKeys.runProject = "F11";
      hotKeys.stopAll = "F12";
      break;
    default:
      hotKeys.recorder = "Ctrl+F9";
      hotKeys.runScript = "Ctrl+F10";
      hotKeys.runProject = "Ctrl+F11";
      hotKeys.stopAll = "Ctrl+F12";
      break;
  }
};

const openMonitor = async () => {
  const monitor = new WebviewWindow("monitor", {
    url: "/monitor",
    title: "monitor",
  });
  monitor.once("tauri://created", async () => {
    console.log("monitor successfully created");
  });
  monitor.once("tauri://error", async () => {
    const windows = await getAllWindows();
    let monitorExists = false;
    for (const window of windows) {
      if (window.label == "monitor") {
        monitorExists = true;
        window.unminimize().then(() => {
          return window.setFocus();
        });
      }
    }
    if (!monitorExists) {
      console.log("open monitor failed");
    }
  });
};

const clearLog = () => {
  logs.value = new Stack(100);
};

const record = async () => {
  const action = async () => {
    try {
      await invoke("run_recorder");
    } catch (error) {
      msgError(error.message);
    }
  };
  if (hiders.recorder) {
    minimizeAll().then(async () => {
      await action();
    });
    return;
  }
  await action();
};

const runScript = async () => {
  const action = async () => {
    try {
      const scriptAbsolutePath = await join(
        route.query.path,
        currentFile.value
      );
      await invoke("run", { path: scriptAbsolutePath });
    } catch (error) {
      msgError(error.message);
    }
  };
  if (hiders.runScript) {
    minimizeAll().then(async () => {
      await action();
    });
    return;
  }
  await action();
};

const runProject = async () => {
  const action = async () => {
    try {
      await invoke("run", { path: stateStore.project.mainFileFullPath });
    } catch (error) {
      msgError(error.message);
    }
  };
  if (hiders.runProject) {
    minimizeAll().then(async () => {
      await action();
    });
    return;
  }
  await action();
};

const stopAll = async () => {
  try {
    await invoke("stop_all");
  } catch (error) {
    msgError(error.message);
  }
};

const safeUnregisterHotkey = async (key) => {
  try {
    await unregister(key);
  } catch (error) {
    // unregister失败会报错,并且用isRegistered也会误判,所以此处就不显示错误了
  }
};

const safeRegisterHotkey = async (key, handler) => {
  await safeUnregisterHotkey(key);
  try {
    await register(key, handler);
  } catch (error) {
    msgError(error.message);
  }
};

const unregisterHotkeys = async () => {
  await safeUnregisterHotkey(hotKeys.recorder);
  await safeUnregisterHotkey(hotKeys.runScript);
  await safeUnregisterHotkey(hotKeys.runProject);
  await safeUnregisterHotkey(hotKeys.stopAll);
};

const registerHotkeys = async () => {
  await safeRegisterHotkey(hotKeys.recorder, async (event) => {
    if (event.state === "Released") await record();
  });
  await safeRegisterHotkey(hotKeys.runScript, async (event) => {
    if (event.state === "Released") await runScript();
  });
  await safeRegisterHotkey(hotKeys.runProject, async (event) => {
    if (event.state === "Released") await runProject();
  });
  await safeRegisterHotkey(hotKeys.stopAll, async (event) => {
    if (event.state === "Released") await stopAll();
  });
};

const logScrollToBottom = () => {
  nextTick(() => {
    if (logScrollbarRef.value) {
      setTimeout(() => {
        const wrap = logScrollbarRef.value?.wrapRef;
        if (wrap) {
          wrap.scrollTop = wrap.scrollHeight;
        }
      }, 10);
    }
  });
  1;
};

const openProject = async (path) => {
  errorMessage.value = reinitProgressLoading.value = null;
  try {
    let status = await invoke("verify_project", { path });
    let openAction = async () => {
      await invoke("open_project", { path });
      currentFile.value = stateStore.project.mainFile;
    };
    switch (status) {
      case "valid":
        break;
      case "moved":
        loadingStopped();
        reinitProgressLoading.value = ElLoading.service({
          lock: true,
          text: "The project has been moved, and is now being reinitialized. Please wait.",
          background: "rgba(0, 0, 0, 0.7)",
        });
        await invoke("reinit_project", { path });
        break;
      default:
        throw new Error("This is not a valild kiwi project.");
    }
    await openAction();
  } catch (error) {
    msgError(error.message);
    router.push({
      path: "/app/home",
    });
  }
};

const revealProject = async () => {
  try {
    await invoke("reveal_project_folder");
  } catch (error) {
    msgError(error.message);
  }
};

const openProjectInEditor = async () => {
  try {
    await invoke("open_project_in_editor");
  } catch (error) {
    msgError(error.message);
  }
};

const loadingBegin = () => {
  openProjectProgressLoading.value = ElLoading.service({
    lock: true,
    text: "The project is opening, Please wait.",
    background: "rgba(0, 0, 0, 0.7)",
  });
};

const loadingStopped = () => {
  if (openProjectProgressLoading.value != null) {
    openProjectProgressLoading.value.close();
    openProjectProgressLoading.value = null;
  }
};

const loadingFinished = () => {
  if (openProjectProgressLoading.value != null) {
    openProjectProgressLoading.value.close();
  }
};

listen("msg:error", (event) => {
  errorMessage.value = event.payload;
  if (reinitProgressLoading.value != null) {
    reinitProgressLoading.value.close();
  }
});

listen("progress:reinit_project", async (event) => {
  if (event.payload.percentage == 100 && errorMessage.value == null) {
    if (reinitProgressLoading.value != null) {
      reinitProgressLoading.value.close();
    }
    msgSuccess("The project has been reinitialized successfully.");
  }
});

listen("update:record_file", async (event) => {
  if (event.payload.data.length > 0) {
    currentFile.value = event.payload.data;
  }
});

listen("run:status", async (event) => {
  if (event.payload.data == "running") {
    if (hiders.runProject || hiders.runScript) {
      await minimizeAll();
    }
  }
  if (event.payload.data == "stopped") {
    if (hiders.recorder || hiders.runProject || hiders.runScript) {
      await unminimizeAll();
    }
  }
});

listen("log:info", (event) => {
  const log = {
    type: "info",
    data: event.payload.data,
    time: event.payload.time,
  };
  logs.value.push(log);
  logScrollToBottom();
});

listen("log:success", (event) => {
  const log = {
    type: "success",
    data: event.payload.data,
    time: event.payload.time,
  };
  logs.value.push(log);
  logScrollToBottom();
});

listen("log:error", (event) => {
  const log = {
    type: "error",
    data: event.payload.data,
    time: event.payload.time,
  };
  logs.value.push(log);
  logScrollToBottom();
});

onMounted(async () => {
  loadingBegin();
  await openProject(route.query.path);
  await initHotKeys();
  await delay(100);
  await registerHotkeys();
  loadingFinished();
});

onUnmounted(async () => {
  await unregisterHotkeys();
});
</script>
<template>
  <el-header class="page-header">
    <el-row :gutter="0">
      <el-col :span="8" class="left">
        <router-link to="/">
          <el-icon :size="20" color="#fff"><ArrowLeft /></el-icon>
        </router-link>
      </el-col>
      <el-col :span="8" class="title">{{ stateStore.project.name }}</el-col>
      <el-col :span="8" class="right">
        <el-icon class="monitor" :size="20" color="#fff" @click="openMonitor"
          ><Monitor
        /></el-icon>
      </el-col>
    </el-row>
  </el-header>
  <el-container class="main">
    <el-aside width="20vw" class="actions">
      <el-row :gutter="0">
        <el-col :span="24">
          <div class="run">
            <el-tooltip
              class="box-item"
              effect="dark"
              content="Hide while running"
              placement="right-start"
            >
              <el-checkbox v-model="hiders.runProject" size="large"
            /></el-tooltip>
            <el-button type="success" size="large" @click="runProject">
              Run Project ({{ hotKeys.runProject }})
            </el-button>
          </div>
        </el-col>
      </el-row>
      <el-row :gutter="0">
        <el-col :span="24">
          <el-button type="danger" size="large" @click="stopAll">
            Stop All ({{ hotKeys.stopAll }})
          </el-button>
        </el-col>
      </el-row>
      <el-divider direction="horizontal" class="divider"></el-divider>
      <el-row :gutter="0">
        <el-col :span="24">
          <div class="run">
            <el-tooltip
              class="box-item"
              effect="dark"
              content="Hide while running"
              placement="right-start"
            >
              <el-checkbox v-model="hiders.recorder" size="large"
            /></el-tooltip>
            <el-button
              type="primary"
              size="large"
              :plain="true"
              @click="record"
            >
              Record ({{ hotKeys.recorder }})
            </el-button>
          </div>
        </el-col>
      </el-row>
      <el-divider direction="horizontal" class="divider"></el-divider>
      <el-row :gutter="0">
        <el-col :span="24">
          <el-button
            type="info"
            size="large"
            @click="revealProject"
            :plain="true"
          >
            Open
          </el-button>
        </el-col>
      </el-row>
      <el-row :gutter="0">
        <el-col :span="24">
          <el-tooltip
            effect="dark"
            content="If “Edit” doesn’t work, install VS Code or set “edit_command“ manually in config.toml."
            placement="bottom"
          >
            <el-button
              type="info"
              size="large"
              @click="openProjectInEditor"
              :plain="true"
            >
              Edit
            </el-button>
          </el-tooltip>
        </el-col>
      </el-row>
    </el-aside>
    <el-container>
      <el-main>
        <div class="actions">
          <el-tooltip
            effect="dark"
            content="Relative Path Of Current File"
            placement="bottom"
          >
            <el-input
              size="default"
              placeholder="main.py"
              v-model="currentFile"
            >
            </el-input>
          </el-tooltip>
          <div class="run">
            <el-tooltip
              class="box-item"
              effect="dark"
              content="Hide while running"
              placement="right-start"
            >
              <el-checkbox v-model="hiders.runScript" size="large"
            /></el-tooltip>
            <el-button
              type="success"
              size="default"
              @click="runScript"
              :plain="true"
              >Run File ({{ hotKeys.runScript }})</el-button
            >
          </div>
          <div class="run" style="display: none">
            <el-tooltip
              class="box-item"
              effect="dark"
              content="Hide while Recording"
              placement="right-start"
            >
              <el-checkbox v-model="hiders.recorder" size="large"
            /></el-tooltip>
            <el-button type="primary" size="default" @click="record">
              Record (F9)
            </el-button>
          </div>
        </div>
        <div class="log-content">
          <div class="clear">
            <el-button
              type="warning"
              size="small"
              @click="clearLog"
              :icon="Delete"
              :plain="true"
              >Clear Log</el-button
            >
          </div>
          <el-tabs type="border-card">
            <el-tab-pane label="Log">
              <el-scrollbar class="logs" ref="logScrollbarRef">
                <ul>
                  <li class="log" :class="log.type" v-for="log in logs.stack">
                    <span class="time">{{ formatLogTime(log.time) }}</span>
                    <span class="data">{{ log.data }}</span>
                  </li>
                </ul>
              </el-scrollbar>
            </el-tab-pane>
          </el-tabs>
        </div>
      </el-main>
    </el-container>
  </el-container>
</template>
<style scoped>
.page-header {
  .monitor {
    cursor: pointer;
    :hover {
      color: gold;
    }
  }
}
.main {
  margin: 10px 10px 0px 10px;
  .run {
    position: relative;
    .el-button {
      position: relative;
      z-index: 1;
    }
    .el-checkbox {
      position: absolute;
      right: 0;
      bottom: 0;
      z-index: 2;
      height: 14px;
    }
  }
  .actions {
    .el-row {
      .el-button {
        width: 100%;
      }
      margin-bottom: 10px;
    }
    .divider {
      margin-top: 10px;
      margin-bottom: 10px;
    }
  }
  .el-main {
    padding: 0;
    margin-left: 10px;
    .actions {
      display: flex;
      .run {
        .el-checkbox {
          right: 2px;
        }
        margin-left: 10px;
      }
      margin-bottom: 10px;
    }
    .log-content {
      position: relative;
      .el-tabs {
        position: relative;
        z-index: 1;
      }
      .clear {
        right: 8px;
        top: 8px;
        position: absolute;
        z-index: 2;
        cursor: pointer;
        :hover {
          color: black;
        }
      }
      :deep(.el-tabs__content) {
        padding: 0px;
      }
      .logs {
        overflow: hidden;
        line-height: 20px;
        color: #999;
        font-size: 13px;
        overflow-y: auto;
        height: calc(100vh - 140px);
        ul {
          list-style: none;
          padding: 0px;
          margin: 10px;
          .log {
            .time {
              display: inline-block;
              min-width: 150px;
            }
          }
          .log.error {
            color: red;
          }
          .log.success {
            color: green;
          }
        }
      }
    }
  }
}
</style>
