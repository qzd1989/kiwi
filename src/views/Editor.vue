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
import { useElementSize } from "@vueuse/core";
import { writeFile } from "./../utils/fs";
import { msgError, msgSuccess } from "./../utils/msg";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import Python from "./../components/editors/Python.vue";
const props = defineProps(["width", "height", "files", "lastOpenedFile"]);
const emits = defineEmits(["remove:file"]);
const store = useStore();
const headerRef = ref(null);
const footerRef = ref(null);
const headerSize = reactive({ width: 0, height: 0 });
const footerSize = reactive({ width: 0, height: 0 });
const modifiedMap = ref(new Map());
const funny = ref(false); //这个值用于重新渲染menus,因为modified本应为ref,现在改成了function,所以没法即时更新,搭这个变量的车重新渲染
headerSize.value = useElementSize(headerRef);
footerSize.value = useElementSize(footerRef);
const editorHeight = computed(() => {
  return props.height - headerSize.height - footerSize.height - 60;
});
const currentFile = ref(null);
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
async function shortcutSave(event) {
  if (
    (event.key === "s" && event.ctrlKey) ||
    (event.key === "s" && event.metaKey)
  ) {
    event.preventDefault();
    if (!currentFile.value) {
      return;
    }
    const key = currentFile.value.path;
    if (!modifiedMap.value.has(key)) {
      return;
    }
    const content = modifiedMap.value.get(key);
    //save file
    try {
      console.log(key, content);
      await writeFile(key, content, false).then((result) => {
        if (!result) {
          return;
        }
        modifiedMap.value.delete(key);
        funny.value = !funny.value;
        msgSuccess(`save file success`);
      });
    } catch (e) {
      msgError(`save file ${key} failed: ${e}`);
    }
  }
}
async function modify(data) {
  const key = data.path;
  const content = data.content;
  modifiedMap.value.set(key, content);
}
function modified(file) {
  const key = file.path;
  if (!modifiedMap.value.has(key)) {
    return false;
  }
  const content = modifiedMap.value.get(key);
  if (content == file.content) {
    return false;
  }
  return true;
}

function openMonitor() {
  const monitor = new WebviewWindow("monitor", {
    url: "/monitor",
    title: "monitor",
  });
  monitor.once("tauri://created", async () => {
    console.log("window successfully created");
  });
  monitor.once("tauri://error", function (e) {
    msgError(`open monitor failed: ${e}`);
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
watchEffect(async () => {
  if (store.getters.focus != "editor") {
    window.removeEventListener("keydown", shortcutSave);
  } else {
    window.addEventListener("keydown", shortcutSave);
  }
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
            :funny="funny"
            @click="select(file[1])"
          >
            <div class="name">{{ file[1].name }}</div>
            <div
              v-if="!modified(file[1])"
              class="close"
              @click="close($event, file[1])"
            >
              ×
            </div>
            <div v-if="modified(file[1])" class="modified">•</div>
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
        :key="file[1].path"
        :height="editorHeight"
        :width="props.width"
        v-for="(file, key, index) in props.files.entries()"
        v-show="file[1].path == currentFile.path"
        @modify="modify"
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
    color: var(--Primary-Color);
    cursor: pointer;
  }
  .el-icon:hover {
    color: var(--Highlight-Color);
  }
}
.menus {
  display: flex;
  list-style: none;
  padding: 0px;
  margin: 0px;
  height: 29px;
  color: var(--Highlight-Color);
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
      color: var(--Secondary-Color);
    }
    .close:hover {
      background-color: var(--Primary-Color);
    }
    .modified {
      color: var(--Third-Color);
      padding-bottom: 0px;
    }
  }
  li:hover {
    background-color: var(--Fill-Dark-1);
  }
  li.active {
    background-color: var(--Fill-Dark);
    border-bottom: 1px solid var(--Primary-Color);
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
