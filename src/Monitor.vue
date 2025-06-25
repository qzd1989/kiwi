<script setup>
import { ref, onMounted, onUnmounted, reactive, watchEffect } from "vue";
import { useStateStore } from "@utils/state-store";
import { useResizeObserver } from "@vueuse/core";
import { Stack } from "@utils/common";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { join } from "@tauri-apps/api/path";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { getCurrentWebview } from "@tauri-apps/api/webview";
import { ElLoading } from "element-plus";
import {
  arrayImageDataToBase64ImageData,
  cropBase64Image,
} from "@utils/common";
import { msgError } from "@utils/msg";
import Image from "@components/monitor/Image.vue";
import RelativeColors from "@components/monitor/RelativeColors.vue";
import Colors from "@components/monitor/Colors.vue";
import Text from "@components/monitor/Text.vue";

const currentZoomFactor = ref(1);
const imageDataPath = ref("");
const dialogOpenCodeVisible = ref(false);
// gap caculation
const stateStore = useStateStore();
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
const moveListener = (event) => {
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
};
const upListener = () => {
  draggingRight.value = false;
};
const drag = (event, area) => {
  event.preventDefault();
  switch (area) {
    case "right":
      draggingRight.value = true;
      break;
  }
};
useResizeObserver(windowRef, (entries) => {
  const entry = entries[0];
  const { width, height } = entry.contentRect;
  windowSize.width = width;
  windowSize.height = height;
  stateStore.windowSize = windowSize;
  leftWidth.value = windowSize.width - rightRef.value.offsetWidth;
});
// gap caculation end
const bgLight = "/src/assets/canvas-bg-light.png";
const bgUrl = ref(bgLight);
const monitors = ref([]);
const monitorKey = ref("null");
const monitor = reactive({
  key: null,
  size: {
    width: 0,
    height: 0,
  },
  base64Data: null,
  originBase64Data: null, //for reset
});
//find
const drawItemsCallback = ref(null);
const showImage = ref(false);
const showLocatingColors = ref(false);
const showColors = ref(false);
const showTexts = ref(false);
const params = reactive({
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
const items = ref([]);
const codeForm = reactive({
  codeString: null,
});
// find end
//canvas
const progressLoading = ref(null);
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
const getMonitors = async () => {
  return [
    {
      name: "primary monitor",
      key: "primary_monitor",
    },
  ];
};
const capture = async () => {
  const action = async () => {
    try {
      monitor.size = await invoke("get_monitor_size");
      progressLoading.value = ElLoading.service({
        lock: true,
        text: "Generating screenshot, please wait.",
        background: "rgba(0, 0, 0, 0.7)",
      });
      await invoke("hide_windows_from_capture", {
        windows: ["main", "monitor"],
      });
      await invoke("capture_screenshot");
    } catch (error) {
      msgError(error.message);
    }
  };
  cancelCapture();
  if (!monitorKey) return;
  await action();
};
const draw = () => {
  if (canvasRef.value != null && monitor.base64Data != null) {
    const canvas = canvasRef.value;
    const ctx = canvas.getContext("2d", { willReadFrequently: true });
    const img = new window.Image();

    img.src = monitor.base64Data;
    img.onload = () => {
      canvas.width = monitor.size.width;
      canvas.height = monitor.size.height;
      ctx.drawImage(img, 0, 0);

      if (drawItemsCallback.value != null) {
        drawItemsCallback.value(ctx);
      }

      hex.value = getPixelHex(ctx, point.x, point.y);

      if (shouldDrawCapture.value) {
        drawCapturedRect();
      }
    };
  }
};
const drawItems = (data) => {
  drawItemsCallback.value = data.callback;
  draw();
};
const clearAllItems = () => {
  drawItemsCallback.value = null;
  draw();
};
const getPixelHex = (ctx, x, y) => {
  var imageData = ctx.getImageData(x, y, 1, 1);
  var data = imageData.data;
  var r = data[0];
  var g = data[1];
  var b = data[2];
  var hex =
    "#" +
    ((1 << 24) + (r << 16) + (g << 8) + b).toString(16).slice(1).toUpperCase();
  return hex;
};
const drawCapturedRect = () => {
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
};
const onCanvasMouseMove = (event) => {
  const rect = canvasRef.value.getBoundingClientRect();
  const left = Math.round(rect.left);
  const top = Math.round(rect.top);
  point.x = event.clientX - left;
  point.y = event.clientY - top;
  draw();
};
const onCanvasMouseOut = (event) => {
  point.x = -1;
  point.y = -1;
};
const onCanvasMouseDown = (event) => {
  if (event.button != 0) {
    return;
  }
  draw();
  endAt.x = -1;
  endAt.y = -1;
  beginAt.x = point.x - offsetX;
  beginAt.y = point.y - offsetY;
  isCaptured.value = false;
  isCapturing.value = true;
  shouldDrawCapture.value = true;
};
const onCanvasMouseUp = (event) => {
  if (event.button != 0) {
    return;
  }
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
};
const cancelCapture = () => {
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
};
const actionPosition = () => {
  const x = Math.min(beginAt.x, endAt.x);
  const y = Math.max(beginAt.y, endAt.y);
  return { x, y };
};
const closeFind = () => {
  showImage.value = false;
  showLocatingColors.value = false;
  showTexts.value = false;
  showColors.value = false;
  rightWidth.value = 0;
};
const findImageByCode = () => {
  closeFind();
  rightWidth.value = 420;
  showImage.value = true;
  cancelCapture();
};
const findImage = async () => {
  closeFind();
  rightWidth.value = 420;
  showImage.value = true;
  drawItemsCallback.value = null;
  //get data
  const x = Math.min(beginAt.x, endAt.x);
  const y = Math.min(beginAt.y, endAt.y);
  const width = Math.abs(beginAt.x - endAt.x);
  const height = Math.abs(beginAt.y - endAt.y);
  params.findArea.start.x = 0;
  params.findArea.start.y = 0;
  params.findArea.end.x = monitor.size.width;
  params.findArea.end.y = monitor.size.height;
  params.size = {
    width,
    height,
  };
  params.base64Data = await cropBase64Image(
    monitor.base64Data,
    x,
    y,
    width,
    height
  );
  cancelCapture();
};
const findLocatingColors = async () => {
  closeFind();
  rightWidth.value = 420;
  showLocatingColors.value = true;
  drawItemsCallback.value = null;
  //get data
  const x = Math.min(beginAt.x, endAt.x);
  const y = Math.min(beginAt.y, endAt.y);
  const width = Math.abs(beginAt.x - endAt.x);
  const height = Math.abs(beginAt.y - endAt.y);
  params.findArea.start.x = 0;
  params.findArea.start.y = 0;
  params.findArea.end.x = monitor.size.width;
  params.findArea.end.y = monitor.size.height;
  params.size = {
    width,
    height,
  };
  params.base64Data = await cropBase64Image(
    monitor.base64Data,
    x,
    y,
    width,
    height
  );
  cancelCapture();
};
const findColorsByCode = () => {
  closeFind();
  rightWidth.value = 420;
  showColors.value = true;
  cancelCapture();
};
const findColors = async () => {
  closeFind();
  rightWidth.value = 420;
  showColors.value = true;
  drawItemsCallback.value = null;
  //get data
  const x = Math.min(beginAt.x, endAt.x);
  const y = Math.min(beginAt.y, endAt.y);
  const width = Math.abs(beginAt.x - endAt.x);
  const height = Math.abs(beginAt.y - endAt.y);
  params.findArea.start.x = x;
  params.findArea.start.y = y;
  params.findArea.end.x = x + width;
  params.findArea.end.y = y + height;
  params.size = {
    width,
    height,
  };
  params.base64Data = await cropBase64Image(
    monitor.base64Data,
    x,
    y,
    width,
    height
  );
  cancelCapture();
};
const recognizeTextByCode = () => {
  closeFind();
  rightWidth.value = 420;
  showTexts.value = true;
  cancelCapture();
};
const recognizeText = async () => {
  closeFind();
  rightWidth.value = 420;
  showTexts.value = true;
  drawItemsCallback.value = null;
  //get data
  const x = Math.min(beginAt.x, endAt.x);
  const y = Math.min(beginAt.y, endAt.y);
  const width = Math.abs(beginAt.x - endAt.x);
  const height = Math.abs(beginAt.y - endAt.y);
  params.findArea.start.x = x;
  params.findArea.start.y = y;
  params.findArea.end.x = x + width;
  params.findArea.end.y = y + height;
  params.base64Data = await cropBase64Image(
    monitor.base64Data,
    x,
    y,
    width,
    height
  );
  cancelCapture();
};
const getProjectPath = async () => {
  try {
    const project = await invoke("get_project");
    stateStore.project.path = project.path;
  } catch (error) {
    msgError(error.message);
  }
};
const getImageDataPath = async () => {
  try {
    imageDataPath.value = await invoke("get_relative_image_data_path");
  } catch (error) {
    msgError(error.message);
  }
};
const shortcutZoom = async (event) => {
  if (
    (event.key === "=" && event.ctrlKey) ||
    (event.key === "=" && event.metaKey)
  ) {
    event.preventDefault();
    stateStore.zoom.factor = Math.min(
      stateStore.zoom.factor + 0.1,
      stateStore.zoom.max
    );
  }
  if (
    (event.key === "-" && event.ctrlKey) ||
    (event.key === "-" && event.metaKey)
  ) {
    event.preventDefault();
    stateStore.zoom.factor = Math.max(
      stateStore.zoom.factor - 0.1,
      stateStore.zoom.min
    );
  }
  if (
    (event.key === "0" && event.ctrlKey) ||
    (event.key === "0" && event.metaKey)
  ) {
    event.preventDefault();
    stateStore.zoom.factor = 1;
  }
};
const openByCode = async () => {
  const codeString = codeForm.codeString.replace(/\s/g, "");
  if (codeString.includes("find_image")) {
    //find_image, find_images
    const regex =
      /(.*?)\(["'](.*?)["'],\((.*?),(.*?)\),\((.*?),(.*?)\),(.*?)\)/;
    const result = codeString.match(regex);
    if (result.length != 8) {
      msgError("parse code error");
      return;
    }
    try {
      const name = result[2];
      const size = await invoke("get_image_size", {
        name,
        dataPath: imageDataPath.value,
      });
      const buffer = await invoke("get_image", {
        name,
        dataPath: imageDataPath.value,
      });
      const base64Data = arrayImageDataToBase64ImageData(
        buffer,
        size.width,
        size.height
      );
      params.findArea.start.x = result[3];
      params.findArea.start.y = result[4];
      params.findArea.end.x = result[5];
      params.findArea.end.y = result[6];
      params.size = size;
      params.base64Data = base64Data;
      params.name = name;
      findImageByCode();
      dialogOpenCodeVisible.value = false;
    } catch (error) {
      dialogOpenCodeVisible.value = true;
      msgError(error.message);
    }
  }
  if (codeString.includes("find_locating_color")) {
    //find_locating_color todo
    // const regex =
    //   /(.*?)\(["'](.*?)["'],["'](.*?)["'],\[(.*?)\],\(([\d]+),([\d]+)\),\(([\d]+),([\d]+)\),\((\-*[\d]+),(\-*[\d]+),(\-*[\d]+)\)\)/;
    // const result = codeString.match(regex);
  }
  if (codeString.includes("recognize_text")) {
    const regex = /(.*?)\(\(([\d]+),([\d]+)\),\(([\d]+),([\d]+)\)\)/;
    const result = codeString.match(regex);
    if (result.length != 6) {
      msgError("parse code error");
      return;
    }
    try {
      const x = Math.min(result[2], result[4]);
      const y = Math.min(result[3], result[5]);
      const width = Math.abs(result[2] - result[4]);
      const height = Math.abs(result[3] - result[5]);
      params.findArea.start.x = x;
      params.findArea.start.y = y;
      params.findArea.end.x = x + width;
      params.findArea.end.y = y + height;
      params.base64Data = await cropBase64Image(
        monitor.base64Data,
        x,
        y,
        width,
        height
      );
      recognizeTextByCode();
      dialogOpenCodeVisible.value = false;
    } catch (error) {
      dialogOpenCodeVisible.value = true;
      msgError(error.message);
    }
  }
};
const reset = () => {
  monitor.buffer = monitor.originBuffer;
  drawItemsCallback.value = null;
  items.value = [];
  draw();
};

listen("backend:update:frame", async (event) => {
  const base64Data = event.payload;
  monitor.base64Data = monitor.originBase64Data = base64Data;
  drawItemsCallback.value = null;
  draw();
  if (progressLoading.value != null) {
    progressLoading.value.close();
  }
  try {
    await invoke("show_windows_from_capture", { windows: ["main", "monitor"] });
  } catch (error) {
    msgError(error.message);
  }
});

//如果main窗口重新打开了project,project就跟随变化
listen("backend:update:project", async (event) => {
  const project = event.payload;
  if (project == null) {
    stateStore.project.exists = false;
    return;
  }
  stateStore.project.exists = true;
  stateStore.project.name = project.name;
  stateStore.project.language = project.language;
  stateStore.project.path = project.path;
  stateStore.project.mainFile = project.main_file;
  stateStore.project.kiwiVersion = project.kiwi_version;
  stateStore.project.mainFileFullPath = await join(
    project.path,
    project.main_file
  );
});

listen("msg:error", (event) => {
  msgError(event.payload);
});

watchEffect(async () => {
  if (stateStore.zoom.factor != currentZoomFactor.value) {
    currentZoomFactor.value = stateStore.zoom.factor;
    await getCurrentWebview().setZoom(currentZoomFactor.value);
  }
});

onMounted(async () => {
  monitors.value = await getMonitors();
  monitorKey.value = "primary_monitor";
  // gap
  document.addEventListener("mousemove", moveListener);
  document.addEventListener("mouseup", upListener);
  // gap end
  //zoom
  window.addEventListener("keyup", shortcutZoom);
  //get project path
  await getProjectPath();
  //get imageData path
  await getImageDataPath();
  //init
  if ((await getCurrentWindow().label) == "monitor") {
    await capture();
  }
});

onUnmounted(() => {
  //cancel zoom
  window.removeEventListener("keyup", shortcutZoom);
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
        <el-select
          v-model="monitorKey"
          placeholder="Select"
          class="monitors"
          disabled
        >
          <el-option
            v-for="item in monitors"
            :key="item.key"
            :label="'monitor: ' + item.name"
            :value="item.key"
          />
        </el-select>
        <div class="capture">
          <!-- <el-tooltip
            class="box-item"
            effect="dark"
            content="Hide while capturing"
            placement="bottom-end"
          >
            <el-checkbox v-model="hideWhileCapturing" size="large"
          /></el-tooltip> -->
          <el-button
            type="primary"
            plain
            @click="capture"
            :disabled="monitorKey == null"
          >
            <el-text>capture</el-text>
          </el-button>
        </div>
        <el-button
          type="primary"
          plain
          @click="reset"
          :disabled="monitorKey == null"
          style="margin-right: 5px"
        >
          <el-text>reset</el-text>
        </el-button>
        <el-button
          type="primary"
          plain
          @click="dialogOpenCodeVisible = true"
          :disabled="monitorKey == null"
          style="margin-left: 0px; margin-right: 5px; display: none"
        >
          <el-text>open</el-text>
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
            <el-icon title="find image" @click="findImage()"
              ><Picture
            /></el-icon>
            <!-- find locating colors -->
            <el-icon title="find locating colors" @click="findLocatingColors()"
              ><Orange
            /></el-icon>
            <!-- find colors-->
            <el-icon title="find color" @click="findColors()"
              ><Pointer
            /></el-icon>
            <!-- recognize text -->
            <el-icon title="recognize text" @click="recognizeText()"
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
        <span v-if="monitor.base64Data">
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
        <Image
          v-if="showImage"
          @close="closeFind"
          @drawItems="drawItems"
          @clearAllItems="clearAllItems"
          :params="params"
          :monitor="monitor"
          :imageDataPath="imageDataPath"
        />
        <RelativeColors
          v-if="showLocatingColors"
          @close="closeFind"
          @drawItems="drawItems"
          @clearAllItems="clearAllItems"
          :params="params"
          :monitor="monitor"
        />
        <Colors
          v-if="showColors"
          @close="closeFind"
          @drawItems="drawItems"
          @clearAllItems="clearAllItems"
          :params="params"
          :monitor="monitor"
        />
        <Text
          v-if="showTexts"
          @close="closeFind"
          @drawItems="drawItems"
          @clearAllItems="clearAllItems"
          :params="params"
          :monitor="monitor"
        />
      </div>
    </el-aside>
  </el-container>
  <el-dialog v-model="dialogOpenCodeVisible" title="Open By Code">
    <el-form ref="form" :model="codeForm">
      <el-form-item label="" prop="name" label-position="top">
        <el-input
          v-model="codeForm.codeString"
          :rows="3"
          type="textarea"
          placeholder='Paste your code here, support functions: (find_image, find_images, recognize_text), such as: find_image("test_pic", (0, 0), (1728, 1117), 0.99)'
        />
      </el-form-item>
      <div style="text-align: center">
        <el-button type="primary" size="default" @click="openByCode">
          Open
        </el-button>
      </div>
    </el-form>
  </el-dialog>
</template>

<style scoped>
.container {
  height: 100vh;
  .left {
    .el-header {
      padding: 0px;
      height: 40px;
      display: flex;
      align-items: center;
      justify-content: space-between;
      .monitors {
        margin: 0px 5px;
      }
      .capture {
        margin-right: 5px;
        position: relative;
        .el-checkbox {
          height: 14px;
          position: absolute;
          right: 0;
          bottom: 0;
          z-index: 2;
        }
        .el-button {
          position: relative;
          z-index: 1;
        }
      }
      .others {
        display: flex;
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
      font-size: 12px;
      overflow: hidden;
      display: flex;
      align-items: center;
      height: 20px;
      padding: 0px 10px;
      color: #666;
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
