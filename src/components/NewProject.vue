<script setup>
import { ref, onMounted, reactive } from "vue";
import { LocalStore } from "../stores/local";
import { msgSuccess, msgError } from "./../utils/msg";
import { join, sep } from "@tauri-apps/api/path";
import {
  createDir,
  createFile,
  rename as fsRename,
  remove as fsRemove,
  exists,
  readDir,
  writeFile,
} from "./../utils/fs";
import { invoke } from "@tauri-apps/api/core";
const props = defineProps({
  visible: Boolean,
});
const formRef = ref(null);
const rules = reactive({
  name: [{ required: true, message: "", trigger: "blur" }],
});

const form = reactive({ name: "", basePath: "" });
const emits = defineEmits(["update"]);
const mainScriptFileName = "main.py";
function close() {
  emits("update", { visible: false });
}
async function save() {
  //validate
  await formRef.value.validate(async (valid, fields) => {
    if (valid) {
      try {
        let projectFolder = await join(form.basePath, form.name);
        let resourceFolder = await join(form.basePath, form.name, "resources");
        let scriptFolder = await join(form.basePath, form.name, "scripts");
        let mainScriptFile = await join(
          form.basePath,
          form.name,
          "scripts",
          mainScriptFileName
        );
        if (await exists(projectFolder)) {
          return;
        }
        await createDir(projectFolder);
        await createDir(scriptFolder);
        await createDir(resourceFolder);
        await createFile(mainScriptFile);
        msgSuccess(`create project successfully`);
        emits("update", { path: projectFolder });
        await LocalStore.set("projectPath", projectFolder);
        close();
      } catch (e) {
        msgError(`save failed: ${e}`);
      }
    } else {
      msgError(`save failed: fields invalid`);
    }
  });
}
onMounted(async () => {
  form.basePath = await LocalStore.get("basePath");
  form.sep = await sep();
});
</script>
<template>
  <el-form label-width="auto" ref="formRef" :model="form" :rules="rules">
    <el-dialog
      draggable
      overflow
      :close-on-click-modal="false"
      :close-on-press-escape="false"
      v-model="props.visible"
      title="New Project"
      :show-close="false"
      :destroy-on-close="true"
    >
      <span>
        <el-form-item label="Path" prop="name">
          <div style="display: flex" class="path">
            <el-input
              v-model="form.name"
              placeholder="name"
              autocapitalize="off"
              autocorrect="off"
              spellcheck="false"
            >
              <template #prepend>{{ form.basePath }}{{ form.sep }}</template>
            </el-input>
          </div>
        </el-form-item>
      </span>
      <template #footer>
        <div class="dialog-footer">
          <el-button @click="close">Cancel</el-button>
          <el-button type="primary" @click="save"> Confirm </el-button>
        </div>
      </template>
    </el-dialog>
  </el-form>
</template>
<style scoped>
.path {
  width: 100%;
  display: flex;
  .el-input {
    width: 100%;
  }
  .el-button {
    margin-left: 10px;
  }
}
</style>
