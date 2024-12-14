<script setup>
import { ref, onMounted, reactive, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { drawBase64ImageOnCanvas } from "../../utils/common";
import { msgError, msgSuccess } from "../../utils/msg";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
const props = defineProps(["form"]);
const emits = defineEmits(["close", "form"]);
const code = ref("");
const canvasRef = ref(null);
const supportedLanguages = [
  {
    label: "Chinese",
    value: "chi_sim",
  },
  {
    label: "English",
    value: "eng",
  },
  {
    label: "Number",
    value: "snum",
  },
];

const form = reactive({
  languages: ["chi_sim", "eng"],
  findArea: {
    start: {
      x: 0,
      y: 0,
    },
    end: {
      x: 0,
      y: 0,
    },
  },
  monitor: {
    size: {
      width: 0,
      height: 0,
    },
    base64Data: null,
  },
  captured: {
    point: {
      x: 0,
      y: 0,
    },
    size: {
      width: 0,
      height: 0,
    },
    base64Data: null,
  },
});
const result = ref("");
const rules = reactive({
  name: [{ required: true, message: "", trigger: "blur" }],
});
function close() {
  emits("close");
}
function drawImage() {
  if (canvasRef.value == null) {
    return;
  }
  drawBase64ImageOnCanvas(
    canvasRef.value,
    form.captured.base64Data,
    0,
    0,
    form.captured.size.width,
    form.captured.size.height
  );
}
async function findText() {
  const langs = JSON.stringify(form.languages);
  const x = Number(form.findArea.start.x);
  const y = Number(form.findArea.start.y);
  const width = Number(form.findArea.end.x - form.findArea.start.x);
  const height = Number(form.findArea.end.y - form.findArea.start.y);
  invoke("find_text", {
    origin: form.monitor.base64Data,
    langs,
    x,
    y,
    width,
    height,
  })
    .then((text) => {
      result.value = text;
      const langs = [];
      for (const language of form.languages) {
        langs.push(`"${language}"`);
      }
      code.value = `find_text((${form.findArea.start.x},${
        form.findArea.start.y
      }), (${form.findArea.end.x},${form.findArea.end.y}), [${langs.join(
        ","
      )}])`;
    })
    .catch((error) => {
      msgError(error);
    });
}
async function copy() {
  try {
    await writeText(code.value);
    msgSuccess("copy successed");
  } catch (e) {
    msgError(`copy failed: ${e}`);
  }
}
watch(props.form, () => {
  Object.assign(form, props.form);
  form.findArea.start = form.captured.point;
  form.findArea.end = {
    x: form.captured.point.x + form.captured.size.width,
    y: form.captured.point.y + form.captured.size.height,
  };
  setTimeout(drawImage, 100);
});

onMounted(async () => {});
</script>
<template>
  <el-container>
    <el-header>Recognize Texts</el-header>
    <el-main>
      <el-form ref="formRef" :model="form" :rules="rules" status-icon>
        <div class="work-area">
          <div class="canvas-work">
            <div class="canvas-area">
              <canvas
                ref="canvasRef"
                :width="form.captured.size.width"
                :height="form.captured.size.height"
              ></canvas>
            </div>
          </div>
          <div class="item">
            <div class="title">
              <span>Find Area</span>
              <el-button type="primary" @click="findText">
                Recognize
              </el-button>
            </div>
            <div style="margin-bottom: -10px">
              <el-row :gutter="10">
                <el-col :span="12">
                  <el-form-item style="margin-bottom: 10px">
                    <el-input
                      v-model="form.findArea.start.x"
                      autocapitalize="off"
                      autocorrect="off"
                      spellcheck="false"
                    >
                      <template #prepend>start x</template>
                    </el-input>
                  </el-form-item>
                </el-col>
                <el-col :span="12">
                  <el-form-item style="margin-bottom: 10px">
                    <el-input
                      v-model="form.findArea.start.y"
                      autocapitalize="off"
                      autocorrect="off"
                      spellcheck="false"
                    >
                      <template #prepend>start y</template>
                    </el-input>
                  </el-form-item>
                </el-col>
              </el-row>
              <el-row :gutter="10">
                <el-col :span="12">
                  <el-form-item style="margin-bottom: 10px">
                    <el-input
                      v-model="form.findArea.end.x"
                      autocapitalize="off"
                      autocorrect="off"
                      spellcheck="false"
                    >
                      <template #prepend>end x</template>
                    </el-input>
                  </el-form-item>
                </el-col>
                <el-col :span="12">
                  <el-form-item style="margin-bottom: 10px">
                    <el-input
                      v-model="form.findArea.end.y"
                      autocapitalize="off"
                      autocorrect="off"
                      spellcheck="false"
                    >
                      <template #prepend>end y</template>
                    </el-input>
                  </el-form-item>
                </el-col>
              </el-row>
            </div>
            <div>
              <el-form-item prop="languages" style="margin-bottom: 0px">
                <el-select
                  v-model="form.languages"
                  multiple
                  filterable
                  default-first-option
                  :reserve-keyword="false"
                  placeholder="supported language"
                >
                  <el-option
                    v-for="item in supportedLanguages"
                    :key="item.value"
                    :label="item.label"
                    :value="item.value"
                  />
                </el-select>
              </el-form-item>
            </div>
            <div>
              <el-input
                v-model="result"
                style="width: 100%"
                :rows="2"
                type="textarea"
                placeholder="results"
                readonly
                :autosize="true"
              />
            </div>
          </div>
          <div class="item">
            <div class="title">
              <span>Code</span>
              <el-button type="primary" @click="copy"> copy </el-button>
            </div>
            <div>
              <el-input
                v-model="code"
                style="width: 100%"
                :rows="2"
                type="textarea"
                placeholder="code example"
                readonly
              />
            </div>
          </div>
        </div>
      </el-form>
    </el-main>
    <el-footer>
      <el-button @click="close">Close</el-button>
    </el-footer>
  </el-container>
</template>
<style scoped>
.el-container {
  height: 100vh;
  .el-header {
    padding: 10px 0px;
  }
  .el-main {
    overflow-x: hidden;
    overflow-y: auto;
    .pixels-box {
      max-width: 300px;
      max-height: 300px;
      overflow: scroll;
    }
    .pixels {
      border: 1px solid #000;
      display: flex;
      flex-wrap: wrap;
    }
    .pixel {
      width: 5px;
      height: 5px;
      border: 1px solid #000;
    }
    .pixel:hover,
    .selected {
      border-color: white;
    }
    .item {
      background-color: var(--LightFill);
      margin: 10px 0px;
      border-radius: 5px;
      padding: 10px;
      display: flex;
      flex-direction: column;
      align-items: stretch;
      gap: 10px;
      .title {
        display: flex;
        justify-content: space-between;
        align-items: center;
      }
    }
  }
  .el-footer {
    text-align: right;
    padding: 10px 0px;
  }
}
.canvas-work {
  display: flex;
  justify-content: center;
}
.canvas-area {
  position: relative;
}
.actions {
  margin-top: 10px;
  display: flex;
  justify-content: space-around;
  margin-bottom: 10px;
}
</style>
