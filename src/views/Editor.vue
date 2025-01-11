<script setup>
import { ref, onMounted, watch, computed, reactive, onUnmounted } from "vue";
import { useStore } from "vuex";
import { useElementSize } from "@vueuse/core";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import Python from "./../components/editors/Python.vue";
import { getAllWindows } from "@tauri-apps/api/window";
const props = defineProps(["width", "height", "files", "lastOpenedFile"]);
const emits = defineEmits(["remove:file"]);
const store = useStore();
const headerRef = ref(null);
const footerRef = ref(null);
const headerSize = reactive({ width: 0, height: 0 });
const footerSize = reactive({ width: 0, height: 0 });
const modifiedMap = ref(new Map());
headerSize.value = useElementSize(headerRef);
footerSize.value = useElementSize(footerRef);
const editorHeight = computed(() => {
  return props.height - headerSize.height - footerSize.height - 60;
});
const currentFile = ref(null);
const listeners = ref([]);
function select(file) {
  currentFile.value = file;
}
function close(event, file) {
  event.stopPropagation();
  emits("remove:file", { file: file });
  if (props.files.has(currentFile.value.path)) {
    return;
  }
  if (props.files.size == 0) {
    currentFile.value = null;
    return;
  }
  const first = Array.from(props.files)[0][1];
  select(first);
}
async function change(data) {
  const key = data.path;
  modifiedMap.value.set(key, true);
}
function save(data) {
  const key = data.path;
  modifiedMap.value.delete(key);
}
function isChanged(path) {
  return modifiedMap.value.has(path);
}
function addListener(data) {
  let event = data.event;
  let listener = data.listener;
  window.addEventListener(event, listener);
  listeners.value.push({ event, listener });
  console.log(listeners.value);
}
function clearListeners() {
  listeners.value.forEach((item) => {
    //window.removeEventListener(item.event, item.listener);
  });
  listeners.length = 0;
}
function openMonitor() {
  const monitor = new WebviewWindow("monitor", {
    url: "/monitor",
    title: "monitor",
  });
  monitor.once("tauri://created", async () => {
    console.log("window successfully created");
  });
  monitor.once("tauri://error", async () => {
    const windows = await getAllWindows();
    let has = false;
    for (const window of windows) {
      if (window.label == "monitor") {
        has = true;
        window.unminimize().then(() => {
          return window.setFocus();
        });
      }
    }
    if (!has) {
      console.log("open monitor failed");
    }
  });
}
watch(
  () => props.lastOpenedFile,
  () => {
    if (props.lastOpenedFile) {
      select(props.lastOpenedFile);
    }
  }
);
watch(currentFile, async () => {
  if (currentFile.value == null) {
    store.commit("currentFilePath", null);
    store.commit("currentFileName", null);
    return;
  }
  store.commit("currentFilePath", currentFile.value.path);
  store.commit("currentFileName", currentFile.value.name);
});
onMounted(async () => {});
onUnmounted(() => {});
</script>
<template>
  <el-container
    :style="{
      'max-width': props.width + 'px',
      'max-height': props.height + 'px',
    }"
  >
    <el-header ref="headerRef">
      <div class="bar">
        <ul class="menus">
          <li
            v-for="(file, key, index) in props.files.entries()"
            class="never-select"
            :class="{ active: currentFile && currentFile.path == file[1].path }"
            :key="file[1].path"
            @click="select(file[1])"
          >
            <div class="name">{{ file[1].name }}</div>
            <div
              v-if="!isChanged(file[1].path)"
              class="close"
              @click="close($event, file[1])"
            >
              ×
            </div>
            <div v-if="isChanged(file[1].path)" class="modified">•</div>
          </li>
        </ul>
        <div class="layouts">
          <el-icon @click="openMonitor"><Monitor /></el-icon>
        </div>
      </div>
      <div class="path" v-if="currentFile">
        {{ currentFile ? currentFile.path : "" }}
      </div>
    </el-header>
    <el-main v-if="currentFile">
      <Python
        :file="file[1]"
        :listeners="listeners"
        :key="file[1].path"
        :height="editorHeight"
        :width="props.width"
        v-for="(file, key, index) in props.files.entries()"
        v-show="file[1].path == currentFile.path"
        @change="change"
        @save="save"
        @listener:add="addListener"
        @listener:clear="clearListeners"
      />
    </el-main>
  </el-container>
</template>
<style scoped>
.el-container {
  overflow: hidden;
}
.bar {
  display: flex;
  justify-content: space-between;
}
.layouts {
  display: flex;
  align-items: center;
  margin-right: 10px;
  .el-icon {
    color: var(--NormalColor);
    cursor: pointer;
  }
  .el-icon:hover {
    color: var(--HighlightColor);
  }
}
.menus {
  display: flex;
  list-style: none;
  padding: 0px;
  margin: 0px;
  height: 39px;
  color: var(--HighlightColor);
  li {
    cursor: pointer;
    height: 100%;
    align-items: center;
    padding: 0px 10px;
    display: flex;
    .name {
      margin-right: 5px;
      white-space: nowrap;
      overflow: hidden;
      text-overflow: ellipsis;
    }
    .close,
    .modified {
      line-height: 10px;
      padding: 0px;
      margin: 0px;
      padding: 3px;
      border-radius: 3px;
      color: var(--NormalColor);
    }
    .close:hover {
      background-color: var(--LightFill);
    }
    .modified {
      color: var(--Third-Color);
      padding-bottom: 0px;
    }
  }
  li:hover,
  li.active {
    background-color: var(--Fill);
  }
  li.active {
    border-bottom: 1px solid var(--NormalColor);
  }
}
.path {
  color: #a4a5a6;
  font-size: var(--SecondarySize);
  padding: 10px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
.terminal {
  position: relative;
}
.gap {
  height: 5px;
  position: absolute;
  top: 0px;
  right: 0px;
  height: 100%;
}
.gap:hover,
.selected {
  cursor: col-resize;
  background-color: var(--Fill-Dark-2);
}
</style>
