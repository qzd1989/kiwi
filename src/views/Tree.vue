<script setup>
import { ref, onMounted, watchEffect } from "vue";
import {
  createDir,
  rename,
  remove,
  exists,
  readDir,
  writeFile,
} from "./../utils/fs";
import { msgError, msgSuccess, msgInfo } from "./../utils/msg";
import { join, extname, basename } from "@tauri-apps/api/path";
import { useStore } from "vuex";
import { ElMessageBox } from "element-plus";
import { editableFileTypes } from "../stores/app";
import { emitTo, listen } from "@tauri-apps/api/event";
import RenameNode from "../components/tree/RenameNode.vue";
import NewNode from "../components/tree/NewNode.vue";
const props = defineProps(["files"]);
const emits = defineEmits(["add:file", "clear:files"]);
const store = useStore();
const treeRef = ref(null);
const data = ref([]);
const currentNode = ref(null);
const currentName = ref("");
const currentPath = ref("");
const renameVisible = ref(false);
const newVisible = ref(false);
const newType = ref("File");
const treeProps = {
  children: "subEntries",
  label: "name",
  disabled: "disabled",
  class: (data, node) => {
    if (data.is_dir && node.isLeaf) {
      return "node-leaf-directory";
    }
    if (data.is_dir && !node.isLeaf) {
      return "node-directory";
    }
    let splited = data.name.split(".");
    if (splited.length <= 1) {
      return "node-file node-file-unknown";
    }
    return `node-file node-file-${splited.pop()}`;
  },
};
async function updateNode(data) {
  //new receive
  if (data.newName != undefined) {
    const newPath = await join(currentPath.value, data.newName);
    if (await exists(newPath)) {
      msgError(`create failed: ${newPath} is existed`);
      newVisible.value = true;
      return;
    }
    try {
      switch (data.type) {
        case "Directory":
          await createDir(newPath);
          break;
        case "File":
          await writeFile(newPath, "", false);
          break;
      }
    } catch (e) {
      msgError(`create failed: ${e}`);
      return;
    }
    newVisible.value = false;
    msgSuccess(`create ${newPath} successfully`);
    await refresh(currentNode.value);
    currentNode.value.expand();
    return;
  }
  //rename receive
  if (data.updateName != currentName.value) {
    const newPath = currentNode.value.data.path.replace(
      currentName.value,
      data.updateName
    );
    if (await exists(newPath)) {
      msgError(`rename failed: ${newPath} is existed`);
      renameVisible.value = true;
      return;
    }
    try {
      await rename(currentNode.value.data.path, newPath);
    } catch (e) {
      msgError(`rename failed: ${e}`);
      return;
    }
    renameVisible.value = false;
    currentNode.value.data.name = currentName.value = data.updateName;
    currentNode.value.data.path = newPath;
    msgSuccess(`update successfully`);
    return;
  }
}
async function delNode(node, data) {
  const isParent = (path) => {
    const keys = props.files.keys();
    for (let key of keys) {
      if (key.startsWith(path)) {
        return true;
      }
      return false;
    }
  };
  if (props.files.has(data.path) || isParent(data.path)) {
    msgError(`save or close first`);
    return;
  }
  ElMessageBox.confirm(`are you sure to delete: ${data.name}`, "Warning", {
    confirmButtonText: "OK",
    cancelButtonText: "Cancel",
    showClose: false,
    type: "warning",
  })
    .then(async () => {
      await remove(data.path)
        .then(() => {
          node.remove();
          currentNode.value = null;
          currentName.value = "";
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
async function fetchNode(directory) {
  try {
    const sort = function (entries) {
      entries.sort((a, b) => {
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
      return entries;
    };
    const filter = function (entry) {
      if (entry.is_symlink) {
        return false;
      }
      if (entry.name.startsWith(".")) {
        return false;
      }
      return true;
    };
    const map = function (entry) {
      return entry;
    };
    let results = [];
    let entries = (await readDir(directory)).filter(filter).map(map);
    for (let entry of entries) {
      let subDirectory = await join(directory, entry.name);
      if (entry.is_dir) {
        entry.subEntries = sort(
          (await readDir(subDirectory)).filter(filter).map(map)
        );
      }
      results.push(entry);
    }
    return sort(results);
  } catch (e) {
    msgError(e);
  }
}
function nodeRender(h, { node, data, store }) {
  return h(
    "div",
    {
      class: "rendered",
      onDblclick: async () => {
        if (!node.data.is_file) {
          return false;
        }
        try {
          let fileType = await extname(node.data.path);
          if (!editableFileTypes.includes(fileType)) {
            return;
          }
          emits("add:file", { file: node.data });
        } catch (e) {
          msgInfo(`open file error: ${e}`);
          return;
        }
      },
    },
    h(
      "div",
      {
        class: "name",
      },
      data.name
    ),
    h(
      "div",
      {
        class: "operations",
      },
      h(
        "span",
        {
          class: "del",
          onClick: async () => {
            await delNode(node, data);
          },
        },
        "-"
      )
    )
  );
}
async function nodeClick(entry, node, treeNode, event) {
  store.commit("focus", "left");
  currentNode.value = node;
  currentName.value = node.data.name;
  currentPath.value = node.data.path;
  window.removeEventListener("keyup", shortcutEnter);
  window.addEventListener("keyup", shortcutEnter);
}
async function nodeExpand(data, node, event) {
  store.commit("focus", "left");
  node.data.subEntries = await fetchNode(node.data.path);
}
async function shortcutEnter(event) {
  if (event.key == "Enter") {
    renameVisible.value = true;
  }
}
async function newDirectory() {
  if (!currentNode.value || !currentNode.value.data.is_dir) {
    msgInfo("Please select a directory first");
    return;
  }
  newType.value = "Directory";
  newVisible.value = true;
}
async function newFile() {
  if (!currentNode.value || !currentNode.value.data.is_dir) {
    msgInfo("Please select a directory first");
    return;
  }
  newType.value = "File";
  newVisible.value = true;
}
async function refresh(node) {
  if (!node || !node.data.is_dir) {
    return;
  }
  node.data.subEntries = await fetchNode(node.data.path);
  node.expand();
  msgSuccess(`refresh successfully`);
}
//listen event from monitor.FindImage
listen("update:resources", async (event) => {
  const node = treeRef.value.getNode(
    data.value.filter((node) => node.name == "resources").pop().$treeNodeId
  );
  node.data.subEntries = await fetchNode(node.data.path);
  node.expand();
});
//listen event from monitor
listen("get:project-path", async (event) => {
  await emitTo("main", "update:project-path", {
    path: store.getters.currentProjectPath,
  });
});
async function test() {
  let defaultDirectory = "/Users/kiwi/rust/opencv-rust";
  defaultDirectory = "/Users/kiwi/Documents/KiwiProjects/hello";
  data.value = await fetchNode(defaultDirectory);
  store.commit("currentProjectName", await basename(defaultDirectory));
  store.commit("currentProjectPath", defaultDirectory);
  await emitTo("monitor", "update:project-path", {
    path: defaultDirectory,
  });
}
watchEffect(async () => {
  if (store.getters.focus != "left") {
    window.removeEventListener("keyup", shortcutEnter);
  } else {
    window.removeEventListener("keyup", shortcutEnter);
    window.addEventListener("keyup", shortcutEnter);
  }
});
onMounted(async () => {
  await test();
});
</script>
<template>
  <el-container class="container" ref="containerRef">
    <el-header ref="headerRef">
      <div class="name" v-if="store.getters.currentProjectName">
        {{ store.getters.currentProjectName }}
      </div>
      <div class="operations" v-if="store.getters.currentProjectName">
        <el-icon @click="refresh(currentNode)"><Refresh /></el-icon>
        <el-icon @click="newDirectory"><FolderAdd /></el-icon>
        <el-icon @click="newFile"><DocumentAdd /></el-icon>
      </div>
    </el-header>
    <el-main ref="mainRef" class="unselectable">
      <el-tree
        ref="treeRef"
        style="max-width: 600px"
        :data="data"
        :props="treeProps"
        :highlight-current="true"
        :render-content="nodeRender"
        @node-click="nodeClick"
        @node-expand="nodeExpand"
      />
    </el-main>
    <el-footer ref="footerRef"> </el-footer>
    <RenameNode
      v-if="renameVisible"
      :name="currentName"
      @close="renameVisible = false"
      @update="updateNode"
    />
    <NewNode
      v-if="newVisible"
      :path="currentPath"
      :type="newType"
      @close="newVisible = false"
      @update="updateNode"
    />
  </el-container>
</template>
<style scoped>
.el-container {
  .el-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    background-color: var(--SecondaryFill);
    height: 40px;
    .name {
      font-weight: bold;
      text-transform: uppercase;
      margin-left: 10px;
    }
    .operations {
      margin-right: 10px;
      .el-icon {
        margin-left: 8px;
        cursor: pointer;
      }
      .el-icon:hover {
        color: var(--HighlightColor);
      }
    }
  }
  :deep(.el-tree-node__content:hover),
  :deep(
      .el-tree--highlight-current
        .el-tree-node.is-current
        > .el-tree-node__content
    ) {
    background-color: var(--SecondaryFill);
  }
  .el-main {
    :deep(.el-tree-node__content) {
      height: 30px;
    }
    .el-tree {
      font-size: var(--NormalSize);
      background: none;
      color: var(--NormalColor);
    }
    :deep(.node-leaf-directory > .el-tree-node__content > .rendered) {
      background-image: url("./../assets/directory.svg");
      background-size: 15px;
      background-repeat: no-repeat;
      background-position: left center;
      padding-left: 20px;
      margin-left: -15px;
    }
    :deep(.node-file > .el-tree-node__content > .rendered) {
      background-image: url("./../assets/file.svg");
      background-size: 15px;
      background-repeat: no-repeat;
      background-position: left center;
      padding-left: 20px;
      margin-left: -15px;
    }
    :deep(.node-file-py > .el-tree-node__content > .rendered) {
      background-image: url("./../assets/py.svg");
    }
    :deep(.node-file-png > .el-tree-node__content > .rendered) {
      background-image: url("./../assets/png.svg");
    }

    :deep(.rendered) {
      width: 100%;
      display: flex;
      justify-content: space-between;
      .name {
      }
      .operations {
        display: flex;
        margin-right: 10px;
        text-align: center;
        .del {
          display: none;
          cursor: pointer;
          padding: 0px 10px;
        }
        .del:hover {
          color: var(--HighlightColor);
        }
      }
    }
    :deep(.rendered:hover .operations .del) {
      display: block;
    }
  }
  .el-footer {
  }
}
</style>
