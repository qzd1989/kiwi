<script setup>
import { ref, onMounted, reactive } from "vue";
import { drawBase64ImageOnCanvas } from "../../utils/common";
const showDialog = ref(true);
const props = defineProps(["form"]);
const emits = defineEmits(["close"]);

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
const activeNames = ref([0]);
const canvasRef = ref(null);
const hiddenCanvasRef = ref(null);
const previewCanvasRef = ref(null);
const magnifyingGlassCanvasRef = ref(null);

const form = reactive({
  image: {
    name: null,
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
const originForm = reactive({
  captured: {
    base64Data: null,
  },
});
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
function drawPreview() {
  if (previewCanvasRef.value == null) {
    return;
  }
  const canvas = previewCanvasRef.value;
  drawBase64ImageOnCanvas(
    canvas,
    form.monitor.base64Data,
    0,
    0,
    form.monitor.size.width,
    form.monitor.size.height
  ).then((ctx) => {
    ctx.strokeStyle = "red";
    ctx.rect(
      form.captured.point.x,
      form.captured.point.y,
      form.captured.size.width,
      form.captured.size.height
    );
    ctx.stroke();
  });
}
function toggleDataSuppliedBg() {
  bgUrl.value = bgUrl.value == bgDark ? bgLight : bgDark;
}

function resetImage() {
  form.captured.base64Data = originForm.captured.base64Data;
  drawImage();
}
async function save() {}
onMounted(async () => {
  Object.assign(form, props.form);
  Object.assign(originForm, props.form);
  setTimeout(drawImage, 100);
  setTimeout(drawPreview, 100);
});
</script>
<template>
  <el-dialog
    v-model="showDialog"
    title="Image"
    :overflow="true"
    :show-close="false"
    width="98%"
    :fullscreen="true"
    @close="close"
  >
    <el-form ref="formRef" :model="form" :rules="rules" status-icon>
      <div class="work-area">
        <el-row :gutter="10">
          <el-col :span="8">
            <el-input
              disabled
              :value="form.monitor.size.width + '×' + form.monitor.size.height"
            >
              <template #prepend>Monitor Size</template>
            </el-input>
          </el-col>

          <el-col :span="8">
            <el-input
              disabled
              :value="
                form.captured.size.width + '×' + form.captured.size.height
              "
            >
              <template #prepend>Image Size</template>
            </el-input>
          </el-col>
          <el-col :span="8">
            <el-form-item prop="name">
              <el-input
                v-model="form.image.name"
                placeholder="image name"
                autocapitalize="off"
                autocorrect="off"
                spellcheck="false"
              >
                <template #prepend>Name</template>
              </el-input>
            </el-form-item></el-col
          >
        </el-row>
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
            :plain="bgUrl == bgLight ? false : true"
            @click="toggleDataSuppliedBg"
            style="min-width: 80px"
          >
            {{ bgUrl == bgDark ? "Light" : "Dark" }}
          </el-button>
        </div>
        <div class="fields">
          <el-collapse v-model="activeNames" style="margin-top: 10px">
            <el-collapse-item title="Snapshoot" name="1">
              <div class="snapshot">
                <canvas
                  ref="previewCanvasRef"
                  :width="form.monitor.size.width"
                  :height="form.monitor.size.height"
                ></canvas>
              </div>
            </el-collapse-item>
          </el-collapse>
        </div>
      </div>
    </el-form>
    <template #footer>
      <div class="dialog-footer">
        <el-button @click="close">Cancel</el-button>
        <el-button type="primary" @click="save"> Save </el-button>
      </div>
    </template>
  </el-dialog>
</template>
<style scoped>
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
  width: 100%;
  display: flex;
  justify-content: center;
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
.snapshot {
  overflow-x: scroll;
}
</style>
