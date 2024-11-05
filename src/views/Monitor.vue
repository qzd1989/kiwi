<script setup>
import { ref, onMounted, onUnmounted, reactive } from "vue";
import { invoke } from "@tauri-apps/api/core";
import {
  arrayImageDataToBase64ImageData,
  cropBase64Image,
  base64ToPixels,
} from "../utils/common";
import MatchImage from "../components/monitor/MatchImage.vue";
const bgLight = "/src/assets/canvas-bg-light.png";
const bgDark = "/src/assets/canvas-bg-dark.png";
const bgUrl = ref(bgLight);
const monitors = ref([]);
const monitorKey = ref("null");
const monitor = reactive({
  key: null,
  size: {
    width: 0,
    height: 0,
  },
  buffer: null,
});
//matches
const showImage = ref(false);
const showLocatingColor = ref(false);
const showColor = ref(false);
const showRecognizeText = ref(false);
const imageForm = reactive({
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
// matches end

//canvas
const canvasRef = ref(null);
const point = reactive({ x: 0, y: 0 });
const offsetX = 0; //坐标偏移量(为了让坐标在鼠标尖上,而不是鼠标中间)
const offsetY = 0; //坐标偏移量(为了让坐标在鼠标尖上,而不是鼠标中间)
const beginAt = reactive({ x: 0, y: 0 });
const endAt = reactive({ x: 0, y: 0 });
const endTheoreticalAt = reactive({ x: 0, y: 0 }); //理论值,有(offsetX,offsetY)偏差,只为显示使用.
const shouldDrawCapture = ref(false);
const isCaptured = ref(false);
const isCapturing = ref(false);
const capturedRect = reactive({
  size: {
    width: 0,
    height: 0,
  },
});
//canvas end

async function getMonitors() {
  return [
    {
      name: "primary display",
      key: "primary_display",
    },
  ];
}
async function capture() {
  cancelCapture();
  if (!monitorKey) return;
  const size = await invoke("display_size");
  monitor.size = JSON.parse(size);
  const buffer = await invoke("snapshot");
  monitor.buffer = buffer;
  draw();
}

function draw() {
  if (canvasRef.value != null && monitor.buffer != null) {
    const canvas = canvasRef.value;
    const u8 = new Uint8Array(monitor.buffer);
    const imageData = new ImageData(monitor.size.width, monitor.size.height);
    imageData.data.set(u8);
    canvas.width = monitor.size.width;
    canvas.height = monitor.size.height;
    const ctx = canvas.getContext("2d");
    ctx.putImageData(imageData, 0, 0);
  }
}

const toggleBg = () => {
  bgUrl.value = bgUrl.value == bgDark ? bgLight : bgDark;
};

function drawCapturedRect() {
  const ctx = canvasRef.value.getContext("2d");
  ctx.beginPath();
  const x = beginAt.x;
  const y = beginAt.y;
  let width = endTheoreticalAt.x - beginAt.x - offsetX;
  let height = endTheoreticalAt.y - beginAt.y - offsetY;
  if (isCapturing.value) {
    width = point.x - beginAt.x - offsetX;
    height = point.y - beginAt.y - offsetY;
  }
  ctx.rect(x, y, width, height);
  capturedRect.size.width = width;
  capturedRect.size.height = height;
  ctx.strokeStyle = "#489029";
  ctx.stroke();
}

function onCanvasMouseMove(event) {
  const rect = canvasRef.value.getBoundingClientRect();
  const left = Math.round(rect.left);
  const top = Math.round(rect.top);
  point.x = event.clientX - left;
  point.y = event.clientY - top;
  draw();
  if (shouldDrawCapture.value) {
    drawCapturedRect();
  }
}

function onCanvasMouseOut(event) {
  point.x = -1;
  point.y = -1;
}

function onCanvasMouseDown(event) {
  draw();
  endAt.x = -1;
  endAt.y = -1;
  beginAt.x = point.x - offsetX;
  beginAt.y = point.y - offsetY;
  isCaptured.value = false;
  isCapturing.value = true;
  shouldDrawCapture.value = true;
}

function onCanvasMouseUp(event) {
  endTheoreticalAt.x = point.x;
  endTheoreticalAt.y = point.y;
  endAt.x = endTheoreticalAt.x - offsetX;
  endAt.y = endTheoreticalAt.y - offsetY;
  isCaptured.value = true;
  isCapturing.value = false;
  shouldDrawCapture.value = true;
  if (
    Math.abs(beginAt.x - endAt.x) < 10 ||
    Math.abs(beginAt.y - endAt.y) < 10
  ) {
    shouldDrawCapture.value = false;
    cancelCapture();
  }
  draw();
  if (shouldDrawCapture.value) {
    drawCapturedRect();
  }
}

function cancelCapture() {
  shouldDrawCapture.value = false;
  isCaptured.value = false;
  isCapturing.value = false;
  beginAt.x = 0;
  beginAt.y = 0;
  endAt.x = 0;
  endAt.y = 0;
  endTheoreticalAt.x = 0;
  endTheoreticalAt.y = 0;
  capturedRect.size.width = 0;
  capturedRect.size.height = 0;
  draw();
  if (shouldDrawCapture.value) {
    drawCapturedRect();
  }
}

function actionPosition() {
  const x = Math.min(beginAt.x, endAt.x);
  const y = Math.max(beginAt.y, endAt.y);
  return { x, y };
}

async function matchImage() {
  showImage.value = true;
  //get data
  const x = Math.min(beginAt.x, endAt.x);
  const y = Math.min(beginAt.y, endAt.y);
  const width = Math.abs(beginAt.x - endAt.x);
  const height = Math.abs(beginAt.y - endAt.y);
  const monitorBase64Data = arrayImageDataToBase64ImageData(
    monitor.buffer,
    monitor.size.width,
    monitor.size.height
  );
  imageForm.monitor.size = monitor.size;
  imageForm.monitor.base64Data = monitorBase64Data;
  imageForm.captured.point = {
    x,
    y,
  };
  imageForm.captured.size = {
    width,
    height,
  };
  imageForm.captured.base64Data = await cropBase64Image(
    monitorBase64Data,
    x,
    y,
    width,
    height
  );
}

onMounted(async () => {
  monitors.value = await getMonitors();
  //init
  monitorKey.value = "primary_display";
  capture();
});
onUnmounted(() => {
  //todo
});
</script>
<template>
  <el-container class="container">
    <el-container>
      <el-header>
        <el-button type="primary" plain @click="getMonitors" class="refresh">
          <el-icon><Refresh /></el-icon>
        </el-button>
        <el-button type="primary" plain @click="toggleBg" class="toggle-bg">
          <el-text>{{ bgUrl == bgDark ? "Light" : "Dark" }}</el-text>
        </el-button>
        <el-select v-model="monitorKey" placeholder="Select" class="monitors">
          <el-option
            v-for="item in monitors"
            :key="item.key"
            :label="'monitor: ' + item.name"
            :value="item.key"
          />
        </el-select>
        <el-button
          type="primary"
          plain
          @click="capture"
          :disabled="monitorKey == null"
          class="capture"
        >
          <el-text>capture</el-text>
        </el-button>
      </el-header>
      <el-main
        class="main"
        :style="{
          'background-image': `url(${bgUrl})`,
        }"
      >
        <!-- work -->
        <div class="work">
          <!-- actions -->
          <div
            class="actions"
            v-if="shouldDrawCapture && isCaptured"
            :style="{
              top: actionPosition().y + 2 + 'px',
              left: actionPosition().x + 'px',
            }"
          >
            <!-- match image -->
            <el-icon title="match image" @click="matchImage"
              ><Picture
            /></el-icon>
            <!-- match locating colors -->
            <el-icon title="match locating colors" @click=""
              ><Orange
            /></el-icon>
            <!-- match color -->
            <el-icon title="match color" @click=""><Pointer /></el-icon>
            <!-- recognize text -->
            <el-icon title="recognize text" @click=""><View /></el-icon>
            <!-- close -->
            <el-icon
              title="close"
              @click="cancelCapture"
              v-show="
                !showImage &&
                !showLocatingColor &&
                !showColor &&
                !showRecognizeText
              "
            >
              <CircleClose />
            </el-icon>
          </div>
          <!-- canvas -->
          <canvas
            class="canvas"
            ref="canvasRef"
            :width="monitor.size.width"
            :height="monitor.size.height"
            @mousedown="onCanvasMouseDown"
            @mouseup="onCanvasMouseUp"
            @mousemove="onCanvasMouseMove"
            @mouseout="onCanvasMouseOut"
          ></canvas>
          <!-- canvas end -->
        </div>
        <!-- work end -->
      </el-main>
      <el-footer>
        <span v-if="monitor.buffer">
          monitor size: ({{ monitor.size.width }}, {{ monitor.size.height }})
        </span>
        <span>position: ({{ point.x }}, {{ point.y }})</span>
        <span>beginAt: ({{ beginAt.x }}, {{ beginAt.y }})</span>
        <span>endAt: ({{ endAt.x }}, {{ endAt.y }})</span>
        <span
          >captured Rect Size: ({{ capturedRect.size.width }},
          {{ capturedRect.size.height }})
        </span>
      </el-footer>
    </el-container>
    <!-- matches -->
    <MatchImage v-if="showImage" @close="showImage = false" :form="imageForm" />
  </el-container>
</template>

<style scoped>
.el-header {
  display: flex;
  height: 30px;
  overflow: hidden;
  align-items: center;
  .refresh {
    margin-left: 5px;
    display: none;
  }
  .monitors {
    margin: 0px 5px;
  }
  .capture {
    margin-right: 5px;
  }
  .toggle-bg {
    margin-left: 5px;
  }
}
.main {
  overflow: auto;
  padding: 25px;
  padding-bottom: 20px;
  .work {
    color: white;
    position: relative;
    display: inline-block;
    .actions {
      display: inline-flex;
      overflow: hidden;
      border-radius: 5px;
      background-color: var(--el-color-success-dark-2);
      color: var(--Basic-White);
      position: absolute;
    }
    .actions .el-icon {
      padding: 5px;
      cursor: pointer;
    }
    .actions .el-icon:hover {
      color: var(--el-color-warning-light-5);
    }
  }
}
.el-footer {
  height: 30px;
  overflow: hidden;
  display: flex;
  align-items: center;
  font-size: 12px;
  padding: 0px 10px;
  span {
    margin-right: 10px;
  }
}
</style>
