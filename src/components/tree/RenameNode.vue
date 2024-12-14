<script setup>
import { ref, reactive, watch, computed, onMounted } from "vue";
const props = defineProps(["name"]);
const emits = defineEmits(["update", "close"]);
const form = reactive({ name: "" });
const visible = ref(true);
const name = computed(() => props.name);
function update() {
  emits("update", { updateName: form.name });
}
function close() {
  emits("close");
}
onMounted(() => {
  form.name = props.name;
});
</script>
<template>
  <el-form label-width="auto">
    <el-dialog
      draggable
      overflow
      :close-on-click-modal="false"
      :close-on-press-escape="false"
      v-model="visible"
      title="Rename"
      :show-close="false"
      :destroy-on-close="true"
    >
      <div>
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
