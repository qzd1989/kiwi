<script setup>
import { ref, onMounted, reactive } from "vue";
import { LocalStore } from "../stores/local";
import { msgSuccess, msgError } from "./../utils/msg";
import { join, sep } from "@tauri-apps/api/path";
import {
  projectDir,
  resourceDir,
  scriptDir,
  defaultScriptFile,
} from "./../stores/app";
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
const nameRef = ref(null);
const rules = reactive({
  name: [{ required: true, message: "", trigger: "blur" }],
});

const form = reactive({ name: null });
const emits = defineEmits(["open:project", "close"]);
const mainScriptFileName = "main.py";
async function save() {
  //validate
  await formRef.value.validate(async (valid, fields) => {
    if (valid) {
      try {
        const project = await projectDir(form.name);
        if (await exists(project)) {
          msgError(`project already exists`);
          return;
        }
        const resouce = await resourceDir(form.name);
        const script = await scriptDir(form.name);
        const defaultFile = await defaultScriptFile(form.name);
        await createDir(project);
        await createDir(resouce);
        await createDir(script);
        await createFile(defaultFile);
        msgSuccess(`create project successfully`);
        emits("open:project", { project: project });
        emits("close");
      } catch (e) {
        msgError(`save failed: ${e}`);
      }
    } else {
      msgError(`save failed: fields invalid`);
    }
  });
}
onMounted(async () => {
  setTimeout(() => {
    nameRef.value.focus();
  }, 100);
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
        <el-form-item label="Name" prop="name">
          <div style="display: flex" class="path">
            <el-input
              v-model="form.name"
              placeholder=""
              autocapitalize="off"
              autocorrect="off"
              spellcheck="false"
              ref="nameRef"
            >
            </el-input>
          </div>
        </el-form-item>
      </span>
      <template #footer>
        <div class="dialog-footer">
          <el-button @click="emits('close')">Cancel</el-button>
          <el-button type="primary" @click="save"> Create </el-button>
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
