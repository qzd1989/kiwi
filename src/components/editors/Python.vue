<script setup>
import { ref, onMounted, watch, watchEffect } from "vue";
import { readFile, codeCheck } from "./../../utils/api";
//code mirror
import { EditorView } from "codemirror";
import CodeMirror from "vue-codemirror6";
import { lintGutter, linter } from "@codemirror/lint";
import { python, pythonLanguage } from "@codemirror/lang-python";
import { oneDark } from "@codemirror/theme-one-dark";
import { autocompletion } from "@codemirror/autocomplete";
import { msgError } from "./../../utils/msg";
import { useStore } from "vuex";
import { syntaxTree } from "@codemirror/language";
import { invoke } from "@vueuse/core";
const codeMirrorRef = ref(null);
const store = useStore();
//code mirror end
const props = defineProps(["file", "width", "height"]);
const emits = defineEmits(["modify"]);
const content = ref("");
const original = ref("");
const extensions = [
  autocompletion({
    tooltipClass: () => {
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

async function shortcutCheck() {
  if (store.getters.codeChecking) {
    return;
  }
  await check();
}

async function check() {
  codeCheck(props.file.path)
    .then((data) => {
      console.log(data);
    })
    .catch((error) => {
      console.log(error);
    });
}

watch(content, () => {
  if (content.value != original.value) {
    emits("modify", { path: props.file.path, content: content.value });
  }
});

watchEffect(async () => {
  if (store.getters.focus != "editor") {
    window.removeEventListener("keyup", shortcutCheck);
  } else {
    window.addEventListener("keyup", shortcutCheck);
  }
});

onMounted(async () => {
  await loadContent();
  await check();
});
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
