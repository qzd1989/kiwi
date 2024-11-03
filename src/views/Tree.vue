<script setup>
import { ref, onMounted, reactive, watchEffect, watch } from "vue";
import {
  createDir,
  rename as fsRename,
  remove as fsRemove,
  exists,
  readDir,
  writeFile,
} from "./../utils/fs";
import { useStore } from "vuex";
import { ElMessageBox } from "element-plus";
import { msgError, msgSuccess, msgInfo } from "./../utils/msg";
import { join, basename, extname, sep } from "@tauri-apps/api/path";
import { useElementSize } from "@vueuse/core";
import { open as openDialog } from "@tauri-apps/plugin-dialog";
import { LocalStore } from "../stores/local";
import Setting from "../components/Setting.vue";
import NewProject from "../components/NewProject.vue";
const props = defineProps(["files"]);
const emits = defineEmits(["add:file", "clear:files"]);
const store = useStore();
const editableExtensions = ["py"];
//project info
const path = ref(null);
const name = ref(null);
const data = ref([]);
//project info end
const headerRef = ref(null);
const footerRef = ref(null);
const treeRef = ref(null);
const headerSize = reactive({ width: 0, height: 0 });
const footerSize = reactive({ width: 0, height: 0 });
headerSize.value = useElementSize(headerRef);
footerSize.value = useElementSize(footerRef);
const mainHeight = ref(0);
const currentNode = ref(null);
const editedNode = ref(null);
const editedData = ref(null);
const editedValue = ref("");
const isEditing = ref(false);
const settingVisible = ref(false);
const newProjectVisible = ref(false);
const treeProps = {
  children: "children",
  label: "name",
  disabled: "disabled",
  class: (data, node) => {
    if (data.is_dir && node.isLeaf) {
      return "node-leaf-directory";
    }
    if (data.is_dir && !node.isLeaf) {
      return "node-directory";
    }
    let arr = data.name.split(".");
    if (arr.length == 0) {
      return "node-leaf-file node-leaf-unknown";
    }
    return "node-leaf-file node-leaf-" + arr.pop();
  },
};
async function fetch(dir) {
  try {
    let results = [];
    let entries = await readDir(dir);
    for (let entry of entries) {
      if (entry.is_symlink) {
        continue;
      }
      if (entry.name.startsWith(".")) {
        continue;
      }
      if (entry.is_dir) {
        let subPath = await join(dir, entry.name);
        let subEntries = await readDir(subPath);
        for (let subEntry of subEntries) {
          subEntry.disabled = true;
          subEntry.path = await join(dir, entry.name, subEntry.name);
        }
        subEntries = sort(subEntries);
        entry.children = subEntries;
      }
      entry.disabled = true;
      entry.path = await join(dir, entry.name);
      results.push(entry);
    }
    results = sort(results);
    return results;
  } catch (e) {
    msgError(`fetch dir error: ${e}`);
  }
}
function sort(array) {
  array.sort((a, b) => {
    if (a.is_dir && b.is_dir) {
      if (a.children && a.children.length > 0) {
        return -1;
      }
      return 1;
    }
    if (a.is_dir) {
      return -1;
    }
    if (b.is_dir) {
      return 1;
    }
    return 0;
  });
  return array;
}
async function expand(data, node, event) {
  node.data.children = await fetch(node.data.path);
}
function render(h, { node, data, store }) {
  let code = btoa(node.data.path);
  if (isEditing.value && editedNode.value == node) {
    editedData.value = data;
    setTimeout(() => {
      let selector = document.querySelector(`input[data-code="${code}"]`);
      let arr = node.data.name.split(".");
      if (arr[0].length == 0 || arr.length == 1) {
        selector.selectionStart = 0;
        selector.selectionEnd = node.data.name.length;
      } else {
        selector.selectionStart = 0;
        selector.selectionEnd = arr[0].length;
      }
      selector.setSelectionRange(
        selector.selectionStart,
        selector.selectionEnd
      );
      selector.focus();
    }, 10);
    return h(
      "span",
      null,
      h("input", {
        value: node.data.name,
        type: "text",
        autocapitalize: "off",
        autocorrect: "off",
        spellcheck: "false",
        class: "node-input",
        "data-code": code,
        onkeyup: (event) => {
          editedValue.value = event.target.value;
        },
      })
    );
  }
  return h(
    "div",
    {
      "data-code": code,
      class: "node-label",
      onDblclick: async (event) => {
        if (node.isLeaf && !node.data.is_dir) {
          try {
            let extension = await extname(node.data.path);
            if (!editableExtensions.includes(extension)) {
              return;
            }
            emits("add:file", { file: node.data });
          } catch (e) {
            return;
          }
        }
      },
    },
    h(
      "div",
      {
        class: "node-label-text",
      },
      node.data.name
    ),
    h(
      "div",
      {
        class: "node-label-action",
        onClick: async (event) => {
          await remove(event, node, data);
        },
      },
      "-"
    )
  );
}
async function click(entry, node, tree, event) {
  store.commit("focus", "left");
  if (node != editedNode.value) {
    await rename();
  }
  currentNode.value = node;
  isEditing.value = false;
  window.removeEventListener("keyup", enterHandler);
  window.addEventListener("keyup", enterHandler);
}

async function enterHandler(event) {
  if (event.key == "Enter") {
    isEditing.value = !isEditing.value;
    if (!isEditing.value) {
      await rename();
    }
    if (isEditing.value) {
      editedNode.value = currentNode.value;
    }
  }
}
async function rename() {
  let name = editedValue.value.slice();
  let data = editedData.value;
  let separator = await sep();
  if (name.length == 0) {
    return;
  }
  if (data == null) {
    return;
  }
  let arr = editedData.value.path.split(separator);
  arr.pop();
  let originPath = data.path;
  let originName = data.name;
  let newPath = await join(arr.join(separator), name);
  if (originPath == newPath) {
    return;
  }
  if (await exists(newPath)) {
    msgError(`rename failed: ${name} is existed`);
    return;
  }
  data.name = name;
  data.path = await join(arr.join(separator), name);
  editedNode.value.setData(data);
  try {
    await fsRename(originPath, data.path);
  } catch (e) {
    msgError(`rename failed: ${e}`);
    editedData.value.name = originName;
    editedData.value.path = originPath;
    editedNode.value.setData(editedData.value);
    return;
  }
  editedData.value = null;
  editedNode.value = null;
  editedValue.value = "";
}
async function newFolder() {
  let name = "newFolder";
  let entry = {
    name,
    is_dir: true,
    disabled: true,
    children: [],
  };
  let nodeType = "file";
  if (currentNode.value && currentNode.value.data.is_dir) {
    nodeType = "directory";
  } else if (
    currentNode.value == null ||
    Array.isArray(currentNode.value.parent.data)
  ) {
    nodeType = "rootDirectory";
  }
  switch (nodeType) {
    case "rootDirectory":
      entry.path = await join(path.value, name);
      break;
    case "directory":
      entry.path = await join(currentNode.value.data.path, name);
      break;
    case "file":
      entry.path = await join(currentNode.value.parent.data.path, name);
      break;
  }
  if (await exists(entry.path)) {
    return;
  }
  if (!(await createDir(entry.path))) {
    return;
  }
  switch (nodeType) {
    case "rootDirectory":
      treeRef.value.data.push(entry);
      break;
    case "directory":
      entry.path = await join(currentNode.value.data.path, name);
      currentNode.value.doCreateChildren([entry]);
      currentNode.value.expand();
      break;
    case "file":
      currentNode.value.parent.doCreateChildren([entry]);
      break;
  }
}
async function newFile() {
  let name = "newFile";
  let entry = {
    name,
    is_dir: false,
    disabled: true,
    children: [],
  };
  let nodeType = "file";
  if (currentNode.value && currentNode.value.data.is_dir) {
    nodeType = "directory";
  } else if (
    currentNode.value == null ||
    Array.isArray(currentNode.value.parent.data)
  ) {
    nodeType = "rootDirectory";
  }
  switch (nodeType) {
    case "rootDirectory":
      entry.path = await join(path.value, name);
      break;
    case "directory":
      entry.path = await join(currentNode.value.data.path, name);
      break;
    case "file":
      entry.path = await join(currentNode.value.parent.data.path, name);
      break;
  }
  if (await exists(entry.path)) {
    return;
  }
  if (!(await writeFile(entry.path, ""))) {
    return;
  }
  switch (nodeType) {
    case "rootDirectory":
      treeRef.value.data.push(entry);
      break;
    case "directory":
      entry.path = await join(currentNode.value.data.path, name);
      currentNode.value.doCreateChildren([entry]);
      currentNode.value.expand();
      break;
    case "file":
      currentNode.value.parent.doCreateChildren([entry]);
      break;
  }
}
async function newProject(projectPath) {
  newProjectVisible.value = true;
  path.value = projectPath;
  emits("clear:files");
}
async function openProject() {
  try {
    const basePath = await LocalStore.get("basePath");
    const directory = await openDialog({
      multiple: false,
      directory: true,
      defaultPath: basePath,
    });
    if (!directory) return;
    path.value = directory;
    await LocalStore.set("projectPath", path.value);
    emits("clear:files");
  } catch (e) {
    msgError(`open project failed: ${e}`);
  }
}
async function openSetting() {
  settingVisible.value = true;
}
async function clearStore() {
  await LocalStore.clear();
}
async function updateSetting(result) {
  settingVisible.value = result.visible;
}
async function updateNewProject(result) {
  if (result.visible != undefined) {
    newProjectVisible.value = result.visible;
    return;
  }
  // new Project
  if (result.path != undefined) {
    newProjectVisible.value = false;
    newProject(result.path);
  }
}
async function refresh() {
  if (path.value) {
    data.value = await fetch(path.value);
  }
}
async function remove(event, node, data) {
  if (props.files.has(data.path)) {
    msgError(`save or close first`);
    return;
  }
  ElMessageBox.confirm(`are you sure to delete: ${data.path}`, "Warning", {
    confirmButtonText: "OK",
    cancelButtonText: "Cancel",
    type: "warning",
  })
    .then(async () => {
      //delete
      await fsRemove(data.path)
        .then(() => {
          node.remove();
          //reset
          currentNode.value = null;
          msgSuccess(`Delete completed`);
        })
        .catch((e) => {
          msgError(`delete ${data.path} failed: ${e}`);
        });
    })
    .catch(() => {
      msgInfo(`Delete canceled`);
    });
}
watch(path, async (newPath) => {
  data.value = await fetch(newPath);
  name.value = await basename(newPath);
});
watchEffect(async () => {
  if (store.getters.focus != "left") {
    window.removeEventListener("keyup", enterHandler);
  } else {
    window.addEventListener("keyup", enterHandler);
  }
  mainHeight.value =
    store.getters.windowSize.height -
    headerSize.height -
    footerSize.height -
    61;
});
onMounted(async () => {
  let projectPath = await LocalStore.get("projectPath");
  if (await exists(projectPath)) {
    path.value = projectPath;
  }
});
</script>
<template>
  <el-container
    class="container"
    :class="{ unselect: !isEditing }"
    ref="containerRef"
  >
    <el-header ref="headerRef" :class="{ 'no-data': path == null }">
      <div class="project-name" v-if="path">
        {{ name }}
      </div>
      <div v-if="path">
        <el-icon @click="refresh"><Refresh /></el-icon>
        <el-icon @click="newFolder"><FolderAdd /></el-icon>
        <el-icon @click="newFile"><DocumentAdd /></el-icon>
      </div>
    </el-header>
    <el-main :style="{ height: mainHeight + 'px' }">
      <el-tree
        ref="treeRef"
        node-key="path"
        :data="data"
        :props="treeProps"
        :render-content="render"
        :highlight-current="false"
        @node-expand="expand"
        @node-click="click"
      />
    </el-main>
    <el-footer ref="footerRef">
      <el-row :gutter="0">
        <el-col :span="6"
          ><div class="action" @click="newProjectVisible = true">New</div>
        </el-col>
        <el-col :span="6">
          <div class="action" @click="openProject">Open</div>
        </el-col>
        <el-col :span="6">
          <div class="action" @click="openSetting">Setting</div>
        </el-col>
        <el-col :span="6">
          <div class="action" @click="clearStore">Clear</div>
        </el-col>
      </el-row>
    </el-footer>
  </el-container>
  <Setting
    v-if="settingVisible"
    :visible="settingVisible"
    @update="updateSetting"
  />
  <NewProject
    v-if="newProjectVisible"
    :visible="newProjectVisible"
    @update="updateNewProject"
  />
</template>
<style scoped>
.el-header {
  display: flex;
  justify-content: space-between;
  background-color: var(--Secondary-Fill);
  height: 30px;
  padding: 8px 0px 0px 10px;
  overflow: hidden;
  .project-name {
    font-size: 12px;
    font-weight: bold;
  }
}
.el-header.no-data {
  background-color: var(--Fill);
}
.el-header .el-icon {
  margin-right: 10px;
}
.el-header .el-icon:hover {
  color: var(--Secondary-Color);
  cursor: pointer;
}
.el-main {
  overflow: auto;
}
:deep(.node-label) {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
}
:deep(.node-label-action) {
  cursor: pointer;
  display: none;
  margin-right: 10px;
}
:deep(.node-label-action:hover) {
  color: var(--Secondary-Color);
}
:deep(.node-label:hover .node-label-action) {
  display: block;
}
:deep(.node-input) {
  background-color: var(--Secondary-Fill);
  color: var(--Secondary-Color);
  border: none;
  width: 100px;
}

.el-tree {
  background-color: transparent;
  color: var(--Primary-Color);
}
.search {
  background-color: transparent;
  margin: 10px;
  :deep(.el-input__wrapper) {
    background-color: var(--Secondary-Fill);
  }
}
.action {
  height: 30px;
  line-height: 30px;
  text-align: center;
  background-color: var(--Secondary-Fill);
  font-size: 12px;
}
.action:hover {
  color: var(--Secondary-Color);
  cursor: pointer;
  background-color: var(--Secondary-Fill);
}
:deep(.el-tree-node__content:hover),
:deep(.el-tree-node.is-current > .el-tree-node__content) {
  background-color: var(--Secondary-Fill);
}
:deep(.node-leaf-directory .node-label) {
  background-image: url("./../assets/directory.svg");
  background-size: 15px;
  background-repeat: no-repeat;
  background-position: left center;
  padding-left: 20px;
  margin-left: -15px;
}
:deep(.node-leaf-file .node-label) {
  background-image: url("./../assets/file.svg");
  background-size: 15px;
  background-repeat: no-repeat;
  background-position: left center;
  padding-left: 20px;
  margin-left: -15px;
}
:deep(.node-leaf-js .node-label) {
  background-image: url("./../assets/js.svg");
}
:deep(.node-leaf-md .node-label) {
  background-image: url("./../assets/md.svg");
}
:deep(.node-leaf-json .node-label) {
  background-image: url("./../assets/json.svg");
}
:deep(.node-leaf-html .node-label) {
  background-image: url("./../assets/html.svg");
}
:deep(.node-leaf-svg .node-label) {
  background-image: url("./../assets/svg.svg");
}
:deep(.node-leaf-vue .node-label) {
  background-image: url("./../assets/vue.svg");
}
:deep(.node-leaf-toml .node-label) {
  background-image: url("./../assets/toml.svg");
}
:deep(.node-leaf-rs .node-label) {
  background-image: url("./../assets/rs.svg");
}
:deep(.node-leaf-ico .node-label) {
  background-image: url("./../assets/ico.svg");
}
:deep(.node-leaf-png .node-label) {
  background-image: url("./../assets/png.svg");
}
:deep(.node-leaf-css .node-label) {
  background-image: url("./../assets/css.svg");
}
:deep(.node-leaf-py .node-label) {
  background-image: url("./../assets/py.svg");
}
:deep(.el-collapse-transition-enter-active),
:deep(.el-collapse-transition-leave-active) {
  transition: none;
}
</style>
