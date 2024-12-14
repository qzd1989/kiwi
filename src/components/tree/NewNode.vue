<script setup>
import { ref, reactive, computed } from "vue";
const props = defineProps(["path", "type"]);
const emits = defineEmits(["update", "close"]);
const form = reactive({ name: "" });
const visible = ref(true);
const title = computed(() => `Create ${props.type}`);
function update() {
  emits("update", { newName: form.name, type: props.type });
}
function close() {
  emits("close");
}
</script>
<template>
  <el-form label-width="auto">
    <el-dialog
      draggable
      overflow
      v-model="visible"
      :title="title"
      :close-on-click-modal="false"
      :close-on-press-escape="false"
      :show-close="false"
      :destroy-on-close="true"
    >
      <div>
        <el-form-item label="path" prop="path">
          <el-input
            v-model="props.path"
            autocapitalize="off"
            autocorrect="off"
            spellcheck="false"
            readonly
            disabled
          ></el-input>
        </el-form-item>
        <el-form-item label="name" prop="name">
          <el-input
            v-model="form.name"
            autocapitalize="off"
            autocorrect="off"
            spellcheck="false"
          ></el-input>
        </el-form-item>
      </div>
      <template #footer>
        <div class="dialog-footer">
          <el-button @click="close">Close</el-button>
          <el-button type="primary" @click="update">Submit</el-button>
        </div>
      </template>
    </el-dialog>
  </el-form>
</template>
<style scoped></style>
