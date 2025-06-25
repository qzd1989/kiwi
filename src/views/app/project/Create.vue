<script setup>
import { ref, onMounted, onUnmounted, reactive } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { join, sep, homeDir } from "@tauri-apps/api/path";
import { exists } from "@tauri-apps/plugin-fs";
import { open } from "@tauri-apps/plugin-dialog";
import { getLocalValue, setLocalValue } from "@utils/local-store";
import { msgError, msgSuccess } from "@utils/msg";
import { useRouter } from "vue-router";
import { ElLoading } from "element-plus";
import { listen } from "@tauri-apps/api/event";
import { getProjectRootDirectory } from "@utils/common";
const router = useRouter();
const formRef = ref(null);
const progressLoading = ref(null);
const projectPath = ref(null);
const languages = [{ value: "python" }];
const errorMessage = ref(null);
const form = reactive({
  name: "",
  language: "python",
  path: "",
  version: "1.0.0",
  rootDirectory: "",
});
const rules = reactive({
  name: [{ required: true, message: "", trigger: "blur" }],
});

const openPathSelector = async () => {
  try {
    const selectedPath = await open({
      directory: true,
      multiple: false,
      defaultPath: await getProjectRootDirectory(),
    });
    if (selectedPath) {
      form.rootDirectory = selectedPath + (await sep());
      await setLocalValue("projectRootDirectory", selectedPath);
    }
  } catch (error) {
    msgError(error.message);
  }
};

const saveProject = async () => {
  errorMessage.value = progressLoading.value = null;
  await formRef.value.validate(async (valid, fields) => {
    if (valid) {
      const path = await join(form.rootDirectory, form.path);
      const name = form.name;
      const language = form.language;
      try {
        await invoke("save_project", { name, language, path });
        projectPath.value = path;
        await invoke("init_project", { path });
        progressLoading.value = ElLoading.service({
          lock: true,
          text: "Project is initializing, please wait.",
          background: "rgba(0, 0, 0, 0.7)",
        });
      } catch (error) {
        msgError(`save failed: ${error.message}`);
      }
    } else {
      msgError(`save failed: fields invalid.`);
    }
  });
};

const init = async () => {
  form.rootDirectory = (await getProjectRootDirectory()) + (await sep());
};

listen("msg:error", (event) => {
  errorMessage.value = event.payload;
  if (progressLoading.value != null) {
    progressLoading.value.close();
  }
});

listen("progress:init_project", async (event) => {
  if (event.payload.percentage == 100 && errorMessage.value == null) {
    if (progressLoading.value != null) {
      progressLoading.value.close();
    }
    msgSuccess("Project created successfully.");
    router.push({
      path: "/app/project/panel",
      query: { path: projectPath.value },
    });
  }
});

onMounted(async () => {
  await init();
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
        <el-col :span="8" class="title">Create Project</el-col>
        <el-col :span="8" class="right"></el-col>
      </el-row>
    </el-header>
    <el-main
      ><el-form
        ref="formRef"
        :model="form"
        :rules="rules"
        label-position="top"
        :show-message="false"
      >
        <el-form-item label="Project Name" prop="name" :required="true">
          <el-input
            placeholder=""
            v-model="form.name"
            autocapitalize="off"
            autocorrect="off"
            spellcheck="false"
          ></el-input>
        </el-form-item>
        <el-form-item label="Programming Language" prop="description">
          <el-select v-model="form.language">
            <el-option
              v-for="item in languages"
              :key="item.value"
              :label="item.value"
              :value="item.value"
            ></el-option>
          </el-select>
        </el-form-item>
        <el-form-item label="Project Path" prop="path" :required="true">
          <el-input
            v-model="form.path"
            placeholder="Project Folder Name"
            autocapitalize="off"
            autocorrect="off"
            spellcheck="false"
          >
            <template #prepend>{{ form.rootDirectory }}</template>
            <template #append>
              <el-button @click="openPathSelector">
                <el-icon><FolderOpened /></el-icon>
              </el-button>
            </template>
          </el-input>
        </el-form-item>
        <el-form-item label="" prop="">
          <el-button type="primary" class="save" @click="saveProject">
            Create
          </el-button>
        </el-form-item>
      </el-form>
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
      .save {
        width: 100%;
        margin-top: 5px;
      }
    }
  }
}
</style>
