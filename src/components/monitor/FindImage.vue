<script setup>
import { ref, onMounted, reactive, watch } from "vue";
import {
  drawBase64ImageOnCanvas,
  generateRandomString,
} from "../../utils/common";
import { invoke } from "@tauri-apps/api/core";
import { msgError } from "../../utils/msg";
import { useStore } from "vuex";
const props = defineProps(["form", "projectPath"]);
const emits = defineEmits(["close", "form"]);

const store = useStore();
const dataExtSideLength = 100; //额外扩展的画布长度,让图像居中方便擦除
const bgLight = "/src/assets/canvas-bg-light.png";
const bgDark = "/src/assets/canvas-bg-dark.png";
const bgUrl = ref(bgLight);
const showmagnifyingGlass = ref(false);
const point = reactive({ x: 0, y: 0 });
const magnifyingGlassSideLength = 50; //放大镜的实际边长
const isErasing = ref(false);
const eraserSmall = 5;
const eraserMedium = 10;
const eraserLarge = 20;
const eraserSideLength = ref(eraserLarge);
const relativePosition = reactive({ x: 0, y: 0 }); //相对于截图的位置
const canvasRef = ref(null);
const hiddenCanvasRef = ref(null);
const magnifyingGlassCanvasRef = ref(null);

const form = reactive({
  name: null,
  threshold: 0.99,
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
    size: {
      width: 0,
      height: 0,
    },
    base64Data: null,
  },
});
const originForm = reactive({
  monitor: {
    base64Data: null,
  },
  captured: {
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
function onImageMouseMove(event) {
  showmagnifyingGlass.value = true;
  const canvas = canvasRef.value;
  const rect = canvas.getBoundingClientRect();
  const left = Math.round(rect.left);
  const top = Math.round(rect.top);
  point.x = event.clientX - left;
  point.y = event.clientY - top;
  relativePosition.x = event.clientX - left - dataExtSideLength / 2;
  relativePosition.y = event.clientY - top - dataExtSideLength / 2;
  drawMagnifyingGlass();
  if (isErasing.value == true) {
    erase();
  }
}
function onImageMouseOut() {
  showmagnifyingGlass.value = false;
}
function onImageMouseDown() {
  isErasing.value = true;
  erase();
}
function onDataMouseUp() {
  isErasing.value = false;
}
function drawMagnifyingGlass() {
  const magnifyingGlassCanvas = magnifyingGlassCanvasRef.value;
  if (magnifyingGlassCanvas == null) {
    return;
  }
  const canvas = canvasRef.value;
  const ctx = canvas.getContext("2d");
  const imageData = ctx.getImageData(
    point.x - magnifyingGlassSideLength / 2,
    point.y - magnifyingGlassSideLength / 2,
    magnifyingGlassSideLength,
    magnifyingGlassSideLength
  );
  magnifyingGlassCanvas.width = magnifyingGlassSideLength;
  magnifyingGlassCanvas.height = magnifyingGlassSideLength;
  const magnifyingGlassCtx = magnifyingGlassCanvas.getContext("2d");
  magnifyingGlassCtx.putImageData(imageData, 0, 0);
  //draw erase rect
  magnifyingGlassCtx.fillStyle = "rgba(255, 0, 0, 0.8)";
  magnifyingGlassCtx.fillRect(
    magnifyingGlassSideLength / 2 - eraserSideLength.value / 2,
    magnifyingGlassSideLength / 2 - eraserSideLength.value / 2,
    eraserSideLength.value,
    eraserSideLength.value
  );
}
function erase() {
  const ctx = canvasRef.value.getContext("2d");
  var imageData = ctx.getImageData(
    point.x - eraserSideLength.value / 2,
    point.y - eraserSideLength.value / 2,
    eraserSideLength.value,
    eraserSideLength.value
  );
  for (var i = 0; i < imageData.data.length; i += 4) {
    imageData.data[i + 3] = 0;
  }
  ctx.putImageData(
    imageData,
    point.x - eraserSideLength.value / 2,
    point.y - eraserSideLength.value / 2
  );
  drawMagnifyingGlass();
}
function eraseFromOuter() {
  if (showmagnifyingGlass.value == true) {
    erase();
  }
}
function setEraserSize(size) {
  eraserSideLength.value = size;
}
function drawImage() {
  if (canvasRef.value == null) {
    return;
  }
  drawBase64ImageOnCanvas(
    canvasRef.value,
    form.captured.base64Data,
    dataExtSideLength / 2,
    dataExtSideLength / 2,
    form.captured.size.width,
    form.captured.size.height
  );
}
function toggleDataSuppliedBg() {
  bgUrl.value = bgUrl.value == bgDark ? bgLight : bgDark;
}

function resetImage() {
  form.captured.base64Data = originForm.captured.base64Data;
  drawImage();
}
async function findImage() {
  const x = Number(form.findArea.start.x);
  const y = Number(form.findArea.start.y);
  const width = Number(form.findArea.end.x - form.findArea.start.x);
  const height = Number(form.findArea.end.y - form.findArea.start.y);
  const threshold = Number(form.threshold);
  invoke("find_image", {
    origin: form.monitor.base64Data,
    template: form.captured.base64Data,
    x,
    y,
    width,
    height,
    threshold,
  })
    .then((weightPoint) => {
      result.value = JSON.stringify(weightPoint);
    })
    .catch((error) => {
      msgError(error);
    });
}

async function findImages() {
  const x = Number(form.findArea.start.x);
  const y = Number(form.findArea.start.y);
  const width = Number(form.findArea.end.x - form.findArea.start.x);
  const height = Number(form.findArea.end.y - form.findArea.start.y);
  const threshold = Number(form.threshold);
  invoke("find_images", {
    origin: form.monitor.base64Data,
    template: form.captured.base64Data,
    x,
    y,
    width,
    height,
    threshold,
  })
    .then((weightPoints) => {
      result.value = JSON.stringify(weightPoints);
    })
    .catch((error) => {
      msgError(error);
    });
}

watch(props.form, () => {
  Object.assign(form, props.form);
  Object.assign(originForm, props.form);
  setTimeout(drawImage, 100);
  form.findArea.end.x = form.monitor.size.width;
  form.findArea.end.y = form.monitor.size.height;
});

onMounted(async () => {
  form.name = generateRandomString(3);
  console.log("a", props.projectPath);
});
</script>
<template>
  <el-container>
    <el-header>Find Image</el-header>
    <el-main>
      <el-form ref="formRef" :model="form" :rules="rules" status-icon>
        <div class="work-area">
          <div class="canvas-work" @click="eraseFromOuter">
            <div class="canvas-area">
              <div
                class="canvas-bg"
                :style="{
                  width: form.captured.size.width + dataExtSideLength + 'px',
                  height: form.captured.size.height + dataExtSideLength + 'px',
                  'background-image': `url(${bgUrl})`,
                }"
              >
                <canvas
                  ref="canvasRef"
                  :width="form.captured.size.width + dataExtSideLength"
                  :height="form.captured.size.height + dataExtSideLength"
                  @mousemove="onImageMouseMove"
                  @mouseout="onImageMouseOut"
                  @mousedown="onImageMouseDown"
                  @mouseup="onDataMouseUp"
                ></canvas>
                <canvas
                  style="display: none"
                  ref="hiddenCanvasRef"
                  :width="form.captured.size.width"
                  :height="form.captured.size.height"
                ></canvas>
              </div>
              <div
                v-if="showmagnifyingGlass"
                class="magnifying-glass"
                :style="{
                  left: point.x + magnifyingGlassSideLength + 'px',
                  top: point.y - magnifyingGlassSideLength + 'px',
                  'background-image': `url(${bgUrl})`,
                }"
              >
                <canvas
                  ref="magnifyingGlassCanvasRef"
                  :width="eraserSideLength"
                  :height="eraserSideLength"
                  style="width: 100px; height: 100px"
                ></canvas>
              </div>
            </div>
          </div>
          <div class="actions">
            <el-button type="info" disabled style="min-width: 100px">
              {{ relativePosition.x }} × {{ relativePosition.y }}
            </el-button>
            <el-button
              type="primary"
              :plain="eraserSideLength == eraserSmall ? false : true"
              @click="setEraserSize(eraserSmall)"
            >
              {{ eraserSmall }}
            </el-button>
            <el-button
              type="primary"
              :plain="eraserSideLength == eraserMedium ? false : true"
              @click="setEraserSize(eraserMedium)"
            >
              {{ eraserMedium }}
            </el-button>
            <el-button
              type="primary"
              :plain="eraserSideLength == eraserLarge ? false : true"
              @click="setEraserSize(eraserLarge)"
            >
              {{ eraserLarge }}
            </el-button>
            <el-button type="danger" plain @click="resetImage">
              <el-icon><RefreshLeft /></el-icon>
            </el-button>
            <el-button
              type="primary"
              :plain="bgUrl == bgLight ? true : false"
              @click="toggleDataSuppliedBg"
              style="min-width: 80px"
            >
              {{ bgUrl == bgDark ? "Light" : "Dark" }}
            </el-button>
          </div>
          <div class="item">
            <el-form-item prop="imageName" style="margin-bottom: 0px">
              <el-input
                v-model="form.name"
                autocapitalize="off"
                autocorrect="off"
                spellcheck="false"
              >
                <template #prepend>image name</template>
                <template #append>.png</template>
              </el-input>
            </el-form-item>
            <el-form-item prop="imagePath" style="margin-bottom: 0px">
              <el-input
                v-model="store.getters.currentProjectPath"
                autocapitalize="off"
                autocorrect="off"
                spellcheck="false"
                disabled
              >
                <template #prepend>image path</template>
              </el-input>
            </el-form-item>
          </div>
          <div class="item">
            <div class="title">
              <span>Find Area</span>
              <div>
                <el-button type="primary" @click="findImage">
                  findOne
                </el-button>
                <el-button type="primary" @click="findImages">
                  findMultiple
                </el-button>
              </div>
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
              <el-form-item prop="imageName" style="margin-bottom: 0px">
                <el-input
                  v-model="form.threshold"
                  autocapitalize="off"
                  autocorrect="off"
                  spellcheck="false"
                >
                  <template #prepend>threshold</template>
                </el-input>
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
              <el-button type="primary" @click=""> copy </el-button>
            </div>
            <div>
              <el-input
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
      <el-button @click="close">Cancel</el-button>
      <el-button type="primary" @click=""> Save </el-button>
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
    .item {
      background-color: var(--Light-Fill);
      margin: 10px 0px;
      border-radius: 5px;
      padding: 10px;
      display: flex;
      flex-direction: column;
      align-items: stretch;
      gap: 10px;
      .title {
        font-size: 14px;
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
.magnifying-glass {
  position: absolute;
  left: 0;
  top: 0;
  color: wheat;
  width: 100px;
  height: 100px;
  border: 1px solid #000;
  background-repeat: repeat;
}
</style>
