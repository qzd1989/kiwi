<script setup>
import { onMounted, reactive } from "vue";
import { LocalStore } from "../stores/local";
import { open as openDialog } from "@tauri-apps/plugin-dialog";
import { msgSuccess, msgError } from "./../utils/msg";
const props = defineProps({
  visible: Boolean,
});
const form = reactive({ basePath: "" });
const emits = defineEmits(["update"]);
function close() {
  emits("update", { visible: false });
}
async function changeBasePath() {
  try {
    const directory = await openDialog({
      multiple: false,
      directory: true,
      defaultPath: form.basePath,
    });
    if (!directory) return;
    form.basePath = directory;
  } catch (e) {
    msgError(`open directory failed: ${e}`);
  }
}
async function save() {
  try {
    await LocalStore.set("basePath", form.basePath);
    msgSuccess(`save settings successfully`);
    close();
  } catch (e) {
    msgError(`save failed: ${e}`);
  }
}
onMounted(async () => {
  form.basePath = await LocalStore.get("basePath");
});
</script>
<template>
  <el-form label-width="auto">
    <el-dialog
      draggable
      overflow
      :close-on-click-modal="false"
      :close-on-press-escape="false"
      v-model="props.visible"
      title="Settings"
      :show-close="false"
      :destroy-on-close="true"
    >
      <span>
        <el-form-item label="BasePath" prop="basePath">
          <div style="display: flex" class="basepath">
            <el-input
              placeholder=""
              v-model="form.basePath"
              disabled
            ></el-input>
            <el-button type="primary" @click="changeBasePath">Change</el-button>
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
.basepath {
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
