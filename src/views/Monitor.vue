<script setup>
import { ref, onMounted, onUnmounted, reactive, watchEffect } from "vue";
import { useStore } from "vuex";
import { useResizeObserver } from "@vueuse/core";
import { Stack } from "./../utils/common";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { emitTo } from "@tauri-apps/api/event";
import { basename } from "@tauri-apps/api/path";
import {
  arrayImageDataToBase64ImageData,
  cropBase64Image,
} from "../utils/common";
import FindImage from "../components/monitor/FindImage.vue";
import FindLocatingColors from "../components/monitor/FindLocatingColors.vue";
import FindColors from "../components/monitor/FindColors.vue";
import FindTexts from "../components/monitor/FindTexts.vue";
// gap caculation
const store = useStore();
const windowRef = ref(null);
const leftRef = ref(null);
const rightRef = ref(null);
const headerRef = ref(null);
const mainRef = ref(null);
const windowSize = reactive({ width: 0, height: 0 });
const leftWidth = ref(0);
const rightWidth = ref(0);
const gapLength = ref(10);
const pagePoint = reactive({ x: 0, y: 0 });
const mouseVerticalStack = new Stack(2);
const draggingRight = ref(false);
function moveListener(event) {
  const containerRect = windowRef.value.$el.getBoundingClientRect();
  pagePoint.x = event.clientX - containerRect.left;
  pagePoint.y = event.clientY - containerRect.top;
  mouseVerticalStack.push(pagePoint.y);
  //to left
  if (draggingRight.value) {
    const remain = windowSize.width;
    rightWidth.value = Math.max(windowSize.width - pagePoint.x, 0);
    rightWidth.value = Math.min(rightWidth.value, remain - gapLength.value);
    leftWidth.value = remain - rightWidth.value;
  }
}
function upListener() {
  draggingRight.value = false;
}
function drag(event, area) {
  event.preventDefault();
  switch (area) {
    case "right":
      draggingRight.value = true;
      break;
  }
}
useResizeObserver(windowRef, (entries) => {
  const entry = entries[0];
  const { width, height } = entry.contentRect;
  windowSize.width = width;
  windowSize.height = height;
  store.commit("windowSize", windowSize);
  leftWidth.value = windowSize.width - rightRef.value.offsetWidth;
});
// gap caculation end

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
//find
const showImage = ref(false);
const showLocatingColors = ref(false);
const showColors = ref(false);
const showTexts = ref(false);
const form = reactive({
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

const formTexts = reactive({
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
// find end

//canvas
const canvasRef = ref(null);
const point = reactive({ x: 0, y: 0 });
const hex = ref(null);
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
  monitor.size = await invoke("display_size");
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
    const ctx = canvas.getContext("2d", { willReadFrequently: true });
    ctx.putImageData(imageData, 0, 0);
    hex.value = getPixelHex(ctx, point.x, point.y);
  }
}

function getPixelHex(ctx, x, y) {
  var imageData = ctx.getImageData(x, y, 1, 1);
  var data = imageData.data;
  var r = data[0];
  var g = data[1];
  var b = data[2];
  var hex =
    "#" +
    ((1 << 24) + (r << 16) + (g << 8) + b).toString(16).slice(1).toUpperCase();
  return hex;
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
  capturedRect.size.width = Math.abs(width);
  capturedRect.size.height = Math.abs(height);
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

function closeFind() {
  showImage.value = false;
  showLocatingColors.value = false;
  showTexts.value = false;
  showColors.value = false;
  rightWidth.value = 0;
}

async function findImage() {
  closeFind();
  rightWidth.value = 420;
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
  form.monitor.base64Data = monitorBase64Data;
  form.monitor.size = monitor.size;
  form.captured.size = {
    width,
    height,
  };
  form.captured.base64Data = await cropBase64Image(
    monitorBase64Data,
    x,
    y,
    width,
    height
  );
  cancelCapture();
}

async function findLocatingColors() {
  closeFind();
  rightWidth.value = 420;
  showLocatingColors.value = true;
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
  form.monitor.base64Data = monitorBase64Data;
  form.monitor.size = monitor.size;
  form.captured.size = {
    width,
    height,
  };
  form.captured.base64Data = await cropBase64Image(
    monitorBase64Data,
    x,
    y,
    width,
    height
  );
  cancelCapture();
}

async function findColors() {
  closeFind();
  rightWidth.value = 420;
  showColors.value = true;
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
  form.monitor.base64Data = monitorBase64Data;
  form.monitor.size = monitor.size;
  form.captured.size = {
    width,
    height,
  };
  form.captured.base64Data = await cropBase64Image(
    monitorBase64Data,
    x,
    y,
    width,
    height
  );
  cancelCapture();
}
async function findTexts() {
  closeFind();
  rightWidth.value = 420;
  showTexts.value = true;
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
  formTexts.monitor.base64Data = monitorBase64Data;
  formTexts.captured.point = { x, y };
  formTexts.captured.size = {
    width,
    height,
  };
  formTexts.captured.base64Data = await cropBase64Image(
    monitorBase64Data,
    x,
    y,
    width,
    height
  );
  cancelCapture();
}

async function getProjectPath() {
  await emitTo("main", "get:project-path");
}

//listen event from main
listen("update:project-path", async (event) => {
  if (event.payload.path == null) {
    return;
  }
  store.commit("currentProjectPath", event.payload.path);
  store.commit("currentProjectName", await basename(event.payload.path));
});

onMounted(async () => {
  monitors.value = await getMonitors();
  //init
  monitorKey.value = "primary_display";
  if ((await getCurrentWindow().label) == "monitor") {
    capture();
  }
  // gap
  document.addEventListener("mousemove", moveListener);
  document.addEventListener("mouseup", upListener);
  // gap end

  //get project path
  await getProjectPath();
});
onUnmounted(() => {
  // gap
  document.removeEventListener("mousemove");
  document.removeEventListener("mouseup");
  // gap end
});
</script>
<template>
  <el-container
    class="container"
    ref="windowRef"
    :style="{
      '--gap-width': gapLength + 'px',
    }"
  >
    <el-container
      ref="leftRef"
      class="left"
      :style="{
        width: leftWidth + 'px',
      }"
    >
      <el-header ref="headerRef">
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
        ref="mainRef"
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
            <!-- find image -->
            <el-icon title="find image" @click="findImage"><Picture /></el-icon>
            <!-- find locating colors -->
            <el-icon title="find locating colors" @click="findLocatingColors"
              ><Orange
            /></el-icon>
            <!-- find colors-->
            <el-icon title="find color" @click="findColors"
              ><Pointer
            /></el-icon>
            <!-- recognize text -->
            <el-icon title="recognize text" @click="findTexts"
              ><View
            /></el-icon>
            <!-- close -->
            <el-icon title="close" @click="cancelCapture">
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
        <span>hex: {{ hex }}</span>
        <span>beginAt: ({{ beginAt.x }}, {{ beginAt.y }})</span>
        <span>endAt: ({{ endAt.x }}, {{ endAt.y }})</span>
        <span
          >captured Rect Size: ({{ capturedRect.size.width }},
          {{ capturedRect.size.height }})
        </span>
      </el-footer>
    </el-container>
    <el-aside
      ref="rightRef"
      class="right"
      :style="{
        width: rightWidth + 'px',
      }"
    >
      <div
        class="gap-vertical"
        @mousedown="drag($event, 'right')"
        :class="{ selected: draggingRight }"
      ></div>
      <div class="find-area">
        <FindImage v-if="showImage" @close="closeFind" :form="form" />
        <FindLocatingColors
          v-if="showLocatingColors"
          @close="closeFind"
          :form="form"
        />
        <FindColors v-if="showColors" @close="closeFind" :form="form" />
        <FindTexts v-if="showTexts" @close="closeFind" :form="formTexts" />
      </div>
    </el-aside>
  </el-container>
</template>

<style scoped>
.container {
  height: 100vh;
  .left {
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
        display: none;
      }
    }
    .el-main {
      overflow: auto;
      padding: 5px;
      padding-bottom: 0px;
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
  }
  .right {
    position: relative;
    .gap-vertical {
      position: absolute;
      left: 0px;
      top: 0px;
      width: var(--gap-width);
      height: 100%;
    }
    .gap-vertical:hover,
    .gap-vertical.selected {
      cursor: col-resize;
    }
    .find-area {
      position: relative;
      margin-left: var(--gap-width);
      margin-right: var(--gap-width);
    }
  }
}
</style>
