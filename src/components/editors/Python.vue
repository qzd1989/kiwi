<script setup>
import { ref, onMounted, watch, watchEffect, onUnmounted } from "vue";
import { readFile, pythonCheck, writeFile } from "./../../utils/api";
import { EditorView } from "codemirror";
import CodeMirror from "vue-codemirror6";
import { lintGutter, linter } from "@codemirror/lint";
import { python, pythonLanguage } from "@codemirror/lang-python";
import { oneDark } from "@codemirror/theme-one-dark";
import { autocompletion } from "@codemirror/autocomplete";
import { msgError, msgSuccess } from "./../../utils/msg";
import { useStore } from "vuex";
import { listen } from "@tauri-apps/api/event";
const codeMirrorRef = ref(null);
const store = useStore();
const props = defineProps(["file", "listeners", "width", "height"]);
const emits = defineEmits(["change", "save", "listener:add", "listener:clear"]);
const content = ref("");
const diyDiagnostics = ref([]);
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
  linter(
    (view) => {
      return diyLinter(view);
    },
    { delay: 500 }
  ),
  lintGutter(),
  oneDark,
  EditorView.lineWrapping,
];
function diyLinter(view) {
  const doc = view.state.doc;
  if (doc.length <= 0) {
    return [];
  }
  if (diyDiagnostics.value.length <= 0) {
    return [];
  }
  let diagnostics = [];
  for (const diyDiagnostic of diyDiagnostics.value) {
    let diagnostic = {
      from:
        doc.line(diyDiagnostic.location.row).from +
        diyDiagnostic.location.column -
        1,
      to:
        doc.line(diyDiagnostic.end_location.row).from +
        diyDiagnostic.end_location.column -
        1,
      severity: "error",
      message: diyDiagnostic.message + " : " + diyDiagnostic.filename,
    };
    diagnostics.push(diagnostic);
  }
  return diagnostics;
}
async function check() {
  diyDiagnostics.value = [];
  await pythonCheck(content.value);
}
async function shortcuts(event) {
  //todo
  console.log("hh");
  if (
    (event.key === "s" && event.ctrlKey) ||
    (event.key === "s" && event.metaKey)
  ) {
    //on save
    event.preventDefault();
    try {
      await check();
      await writeFile(props.file.path, content.value, false).then((result) => {
        if (!result) {
          return;
        }
        if (props.file.path == store.getters.currentFilePath) {
          emits("save", { path: props.file.path });
        }
        msgSuccess(`save file success`);
      });
    } catch (e) {
      msgError(`save file ${props.file.path} failed: ${e}`);
    }
    return;
  } else {
    emits("change", { path: props.file.path });
    await check();
  }
}
listen("python_check:error", async (event) => {
  let errors = JSON.parse(event.payload.data);
  if (errors.length == 0) {
    return;
  }
  for (const error of errors) {
    if (error.fix != null) {
      return;
    }
    let diyDiagnostic = error;
    diyDiagnostic.key =
      diyDiagnostic.from + "-" + diyDiagnostic.to + "-" + diyDiagnostic.message;
    for (const item of diyDiagnostics.value) {
      if (item.key == diyDiagnostic.key) {
        return;
      }
    }
    diyDiagnostics.value.push(diyDiagnostic);
  }
});

watchEffect(async () => {
  if (store.getters.focus == "editor") {
    emits("listener:clear");
    emits("listener:add", { event: "keyup", listener: shortcuts });
  }
  //on menu changing
  if (store.getters.currentFilePath == props.file.path) {
    await check();
  }
});
async function loadContent() {
  try {
    await readFile(props.file.path).then((data) => {
      content.value = data;
    });
  } catch (e) {
    content.value = "";
    msgError(`get file content error: ${e}`);
  }
}
onMounted(async () => {
  //on open
  await loadContent();
  await check();
  emits("listener:clear");
  emits("listener:add", { event: "keyup", listener: shortcuts });
});
onUnmounted(async () => {
  emits("listener:clear");
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
