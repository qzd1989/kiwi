<script setup>
import { ref, onMounted, reactive, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { base64ToPixels, rgbToHex } from "../../utils/common";
import { msgError, msgSuccess } from "../../utils/msg";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
const props = defineProps(["form"]);
const emits = defineEmits(["close", "form"]);
const code = ref("");

const form = reactive({
  offset: {
    r: 0,
    g: 0,
    b: 0,
  },
  locatingColors: [],
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
const pixels = ref([]);
const result = ref("");
const rules = reactive({
  name: [{ required: true, message: "", trigger: "blur" }],
});
function close() {
  emits("close");
}
function drawImage() {
  base64ToPixels(form.captured.base64Data)
    .then((data) => {
      let arr = [];
      let index = 0;
      for (let y = 0; y < form.captured.size.height; y++) {
        for (let x = 0; x < form.captured.size.width; x++) {
          let locatingColor = {
            hex: rgbToHex(data[index]),
            point: {
              x: x,
              y: y,
            },
          };
          arr.push(locatingColor);
          index++;
        }
      }
      pixels.value = arr;
    })
    .catch((error) => console.error(error));
}

async function unAdd() {
  if (form.locatingColors.length > 0) {
    form.locatingColors.pop();
  }
}

async function pushColor(locatingColor) {
  if (
    form.locatingColors
      .map((item) => {
        return item.hex;
      })
      .includes(locatingColor.hex)
  ) {
    msgError("The color is already exist!");
    return;
  }
  form.locatingColors.push(locatingColor);
}

function removeColor(hex) {
  form.locatingColors = form.locatingColors.filter((item) => item.hex !== hex);
}

async function findColor() {
  const colors = JSON.stringify(form.locatingColors);
  const x = Number(form.findArea.start.x);
  const y = Number(form.findArea.start.y);
  const width = Number(form.findArea.end.x - form.findArea.start.x);
  const height = Number(form.findArea.end.y - form.findArea.start.y);
  const r = Number(form.offset.r);
  const g = Number(form.offset.g);
  const b = Number(form.offset.b);
  invoke("find_color", {
    origin: form.monitor.base64Data,
    colors,
    x,
    y,
    width,
    height,
    offsetR: r,
    offsetG: g,
    offsetB: b,
  })
    .then((locatingColors) => {
      result.value = JSON.stringify(locatingColors);
      const colors = [];
      for (const color of form.locatingColors) {
        colors.push(`"${color.hex}"`);
      }
      code.value = `find_color([${colors.join(",")}], (${
        form.findArea.start.x
      },${form.findArea.start.y}), (${form.findArea.end.x},${
        form.findArea.end.y
      }), (${form.offset.r},${form.offset.g},${form.offset.b}))`;
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
  form.locatingColors = [];
  setTimeout(drawImage, 100);
  // form.findArea.end.x = form.monitor.size.width;
  // form.findArea.end.y = form.monitor.size.height;

  form.findArea.start = form.captured.point;
  form.findArea.end = {
    x: form.captured.point.x + form.captured.size.width,
    y: form.captured.point.y + form.captured.size.height,
  };
});

onMounted(async () => {});
</script>
<template>
  <el-container>
    <el-header>Find Colors</el-header>
    <el-main>
      <el-form ref="formRef" :model="form" :rules="rules" status-icon>
        <div class="work-area">
          <div class="canvas-work">
            <div class="pixels-box">
              <div
                class="pixels"
                :style="{
                  width: form.captured.size.width * 7 + 'px',
                  height: form.captured.size.height * 7 + 'px',
                }"
              >
                <div
                  class="pixel"
                  v-for="item in pixels"
                  :style="{ 'background-color': item.hex }"
                  @click="pushColor(item)"
                  :class="{
                    selected: form.locatingColors
                      .map((row) => {
                        return row.point.x + ',' + row.point.y;
                      })
                      .includes(item.point.x + ',' + item.point.y),
                  }"
                ></div>
              </div>
            </div>
          </div>
          <div class="actions">
            <el-button size="small" type="danger" @click="unAdd">
              <el-icon><Back /></el-icon>
            </el-button>
          </div>
          <div class="item">
            <div class="title">Colors</div>
            <el-form-item
              prop="colors"
              style="margin-bottom: 0px"
              v-show="form.locatingColors.length > 0"
            >
              <el-input
                class="color"
                style="margin-bottom: 2px"
                v-for="item in form.locatingColors"
                :value="item.hex"
              >
                <template #prepend>
                  <div
                    style="height: 10px; width: 10px; border-radius: 5px"
                    :style="{ backgroundColor: item.hex }"
                  ></div>
                </template>
                <template #append>
                  <el-button @click="removeColor(item.hex)">×</el-button>
                </template>
              </el-input>
            </el-form-item>
          </div>
          <div class="item">
            <div class="title">
              <span>Find Area</span>
              <el-button type="primary" @click="findColor"> find </el-button>
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
              <el-row :gutter="0">
                <el-col :span="8">
                  <el-form-item prop="offsetR" style="margin-bottom: 0px">
                    <el-input
                      v-model="form.offset.r"
                      autocapitalize="off"
                      autocorrect="off"
                      spellcheck="false"
                    >
                    </el-input>
                  </el-form-item>
                </el-col>
                <el-col :span="8">
                  <el-form-item prop="offsetG" style="margin-bottom: 0px">
                    <el-input
                      v-model="form.offset.g"
                      autocapitalize="off"
                      autocorrect="off"
                      spellcheck="false"
                    >
                    </el-input>
                  </el-form-item>
                </el-col>
                <el-col :span="8">
                  <el-form-item prop="offsetB" style="margin-bottom: 0px">
                    <el-input
                      v-model="form.offset.b"
                      autocapitalize="off"
                      autocorrect="off"
                      spellcheck="false"
                    >
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
.canvas-bg {
  background-repeat: repeat;
  overflow: hidden;
}
.actions {
  margin-top: 10px;
  display: flex;
  justify-content: space-around;
  margin-bottom: 10px;
}
.el-button.current {
  background-color: rgb(97, 97, 97);
}
</style>
