<script setup>
import { ref, onMounted, watch, computed, onUnmounted } from "vue";
import {
  createDir,
  createFile,
  rename as fsRename,
  remove as fsRemove,
  exists,
  readDir,
  writeFile,
  readFile,
} from "./../../utils/fs";
//code mirror
import { EditorView } from "codemirror";
import CodeMirror from "vue-codemirror6";
import { linter, lintGutter } from "@codemirror/lint";
import { python, pythonLanguage } from "@codemirror/lang-python";
import { oneDark } from "@codemirror/theme-one-dark";
import * as eslint from "eslint-linter-browserify";
import { autocompletion } from "@codemirror/autocomplete";
import { msgError } from "./../../utils/msg";
const codeMirrorRef = ref(null);
//code mirror end
const props = defineProps(["file", "width", "height"]);
const emits = defineEmits(["modify"]);
const content = ref("");
const original = ref("");
const extensions = [
  autocompletion({
    tooltipClass: (state) => {
      return "custom-tooltip";
    },
  }),
  python(),
  pythonLanguage.data.of({
    autocomplete: [],
  }),
  lintGutter(),
  oneDark,
  EditorView.lineWrapping,
];
async function loadContent() {
  try {
    await readFile(props.file.path).then((data) => {
      content.value = data;
      original.value = data;
    });
  } catch (e) {
    content.value = "";
    msgError(`get file content error: ${e}`);
  }
}

watch(content, (newValue) => {
  if (content.value != original.value) {
    emits("modify", { path: props.file.path, content: content.value });
  }
});

onMounted(async () => {
  await loadContent();
});
onUnmounted(() => {});
</script>
<template>
  <el-container
    :style="{
      '--editor-height': props.height + 'px',
      '--editor-width': props.width + 'px',
    }"
  >
    <code-mirror
      ref="codeMirrorRef"
      basic
      v-model="content"
      :model-value="content"
      :tab="true"
      :extensions="extensions"
      :style="{
        height: props.height + 'px',
        width: '100%',
      }"
      placeholder="Support Python"
      :lineWrapping="true"
    />
  </el-container>
</template>
<style scoped>
:deep(.cm-editor) {
  height: var(--editor-height);
  width: var(--editor-width);
}
</style>
