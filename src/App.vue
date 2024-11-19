<script setup>
import { onMounted, onUnmounted } from "vue";
import { type, arch } from "@tauri-apps/plugin-os";
import { createDir, exists } from "./utils/fs";
import { projectsDir } from "./stores/app";
import { message } from "@tauri-apps/plugin-dialog";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";
import { msgSuccess, msgError } from "./utils/msg";

async function initProjectsDir() {
  if (await exists(projectsDir)) {
    return;
  }
  await createDir(projectsDir);
}
async function initPython() {
  const availables = ["macos_aarch64", "windows_x86_64", "windows_aarch64"];
  const platform = type();
  const architecture = arch();
  const current = platform + "_" + architecture;
  if (!availables.includes(current)) {
    await message(
      `Not support:\n${current}\n Availables:\n${availables.join("\n")}`,
      {
        title: "Kiwi",
        kind: "error",
      }
    );
    await getCurrentWindow().destroy();
    return;
  }
  invoke("install_python", {
    platform,
    architecture,
  })
    .then(async () => {
      //安装结束
      msgSuccess("Install Python Success");
    })
    .catch(async (error) => {
      //安装报错
      msgError(error);
    });
}
onMounted(async () => {
  await initPython();
  await initProjectsDir();
});
onUnmounted(() => {});
</script>
<template>
  <router-view></router-view>
</template>
