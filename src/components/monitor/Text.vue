<script setup>
import { ref, onMounted, reactive, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { drawBase64ImageOnCanvas, drawRect } from "@utils/common";
import { msgError, msgSuccess } from "@utils/msg";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";

const props = defineProps(["params", "monitor"]);
const emits = defineEmits(["close", "drawItems", "clearAllItems"]);
const result = ref(null);
const code = ref(null);
const formRef = ref(null);
const canvasRef = ref(null);
const loading = ref(false);
const form = reactive({
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
  size: {
    width: 0,
    height: 0,
  },
  base64Data: null,
});
const rules = reactive({
  "findArea.start.x": [{ required: true, message: "", trigger: "blur" }],
  "findArea.start.y": [{ required: true, message: "", trigger: "blur" }],
  "findArea.end.x": [{ required: true, message: "", trigger: "blur" }],
  "findArea.end.y": [{ required: true, message: "", trigger: "blur" }],
});

const close = () => {
  emits("close");
};

const drawImage = () => {
  if (canvasRef.value == null) {
    return;
  }
  drawBase64ImageOnCanvas(
    canvasRef.value,
    form.base64Data,
    0,
    0,
    form.size.width,
    form.size.height
  );
};

const generateCode = async () => {
  const startPoint = form.findArea.start;
  const endPoint = form.findArea.end;
  try {
    code.value = await invoke("generate_recognize_text_code", {
      startPoint,
      endPoint,
    });
  } catch (error) {
    msgError(error.message);
  }
};

const recognize = async () => {
  if (loading.value) return;
  result.value = code.value = null;
  const valid = await formRef.value.validate();
  if (!valid) return;
  const origin = props.monitor.base64Data;
  const startPoint = form.findArea.start;
  const endPoint = form.findArea.end;
  try {
    loading.value = true;
    const text = await invoke("recognize_text", {
      origin,
      startPoint,
      endPoint,
    });
    drawItem();
    result.value = text;
    await generateCode();
  } catch (error) {
    clearAllItems();
    result.value = code.value = null;
    msgError(error.message);
  } finally {
    loading.value = false;
  }
};

const drawItem = () => {
  emits("drawItems", {
    callback: (ctx) => {
      const areaPoint = form.findArea.start;
      const areaSize = {
        width: form.findArea.end.x - form.findArea.start.x,
        height: form.findArea.end.y - form.findArea.start.y,
      };
      drawRect(ctx, areaPoint, areaSize);
    },
  });
};

const clearAllItems = () => {
  emits("clearAllItems");
};

const copy = async () => {
  try {
    await writeText(code.value);
    msgSuccess("copy successed");
  } catch (error) {
    msgError(`copy failed: ${error.message}`);
  }
};

const loadData = () => {
  form.findArea = props.params.findArea;
  form.base64Data = props.params.base64Data;
  const width = props.params.findArea.end.x - props.params.findArea.start.x;
  const height = props.params.findArea.end.y - props.params.findArea.start.y;
  form.size.width = width;
  form.size.height = height;
  result.value = null;
  code.value = null;
  setTimeout(drawImage, 100);
};

watch(props.params, async () => {
  loadData();
});

onMounted(async () => {
  loadData();
});
</script>
<template>
  <el-container>
    <el-header>Recognize Text</el-header>
    <el-main>
      <el-form ref="formRef" :model="form" :rules="rules" status-icon>
        <div class="work-area">
          <div class="canvas-work">
            <div class="canvas-area">
              <canvas
                ref="canvasRef"
                :width="form.size.width"
                :height="form.size.height"
              ></canvas>
            </div>
          </div>
          <div class="item">
            <div class="title">
              <span>Find Area</span>
              <el-button type="primary" @click="recognize" :disabled="loading">
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
                      disabled
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
                      disabled
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
                      disabled
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
                      disabled
                    >
                      <template #prepend>end y</template>
                    </el-input>
                  </el-form-item>
                </el-col>
              </el-row>
            </div>
            <div>
              <el-input
                v-model="result"
                style="width: 100%"
                :rows="2"
                type="textarea"
                placeholder="result"
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
                :rows="4"
                type="textarea"
                placeholder="code"
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
