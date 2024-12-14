<script setup>
import { onMounted, reactive, watch } from "vue";
import { useStore } from "vuex";
import { minZoomFactor, maxZoomFactor } from "./../stores/app";
const props = defineProps({
  visible: Boolean,
});
const emits = defineEmits(["update"]);
const store = useStore();
const form = reactive({ zoomFactor: 1 });
function close() {
  emits("update", { visible: false });
}
watch(form, () => {
  store.commit("zoomFactor", form.zoomFactor);
});
onMounted(() => {
  form.zoomFactor = store.state.zoomFactor;
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
      <div>
        <el-form-item label="Zoom Factor" prop="zoomFactor">
          <el-slider
            v-model="form.zoomFactor"
            :min="minZoomFactor"
            :max="maxZoomFactor"
            :step="0.1"
            show-input
          />
        </el-form-item>
      </div>
      <template #footer>
        <div class="dialog-footer">
          <el-button type="primary" @click="close">Close</el-button>
        </div>
      </template>
    </el-dialog>
  </el-form>
</template>
<style scoped></style>
