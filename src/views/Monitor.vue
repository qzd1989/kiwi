<script setup>
import { ref, onMounted, onUnmounted, reactive } from "vue";
const bgLight = "/src/assets/canvas-bg-light.png";
const bgDark = "/src/assets/canvas-bg-dark.png";
const bgUrl = ref(bgLight);
const monitors = ref([]);
const monitorId = ref(null);
const monitor = reactive({
  id: 0,
  point: {
    x: 0,
    y: 0,
  },
  size: {
    width: 0,
    height: 0,
  },
  data: null,
});

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
//canvas end

const getMonitors = async () => {};
const getSnapshot = () => {
  const ws = instance();
  return new Promise((resolve, reject) => {
    ws.call(
      () => {
        return {
          monitor_request: true,
          call: "snapshot",
          identifier: "monitor:" + monitorId.value,
        };
      },
      (arrayBuffer) => {
        resolve(arrayBuffer);
      }
    );
  });
};
const capture = async () => {
  cancelCapture();
  for (const item of monitors.value) {
    if (item.id == monitorId.value) {
      monitor.id = item.id;
      monitor.point = item.point;
      monitor.size = item.size;
    }
  }
  getSnapshot().then(async (arrayBuffer) => {
    monitor.data = arrayBuffer;
    draw();
  });
};

const draw = () => {
  if (canvasRef.value != null && monitor.data != null) {
    const canvas = canvasRef.value;
    const u8 = new Uint8Array(monitor.data);
    const imageData = new ImageData(monitor.size.width, monitor.size.height);
    imageData.data.set(u8);
    canvas.width = monitor.size.width;
    canvas.height = monitor.size.height;
    const ctx = canvas.getContext("2d");
    ctx.putImageData(imageData, 0, 0);
  }
};

const toggleBg = () => {
  bgUrl.value = bgUrl.value == bgDark ? bgLight : bgDark;
};

function drawCapturedRect() {
  const ctx = canvasRef.value.getContext("2d");
  ctx.beginPath();
  if (isCapturing.value) {
    ctx.rect(
      beginAt.x,
      beginAt.y,
      point.x - beginAt.x - offsetX,
      point.y - beginAt.y - offsetY
    );
  } else {
    ctx.rect(
      beginAt.x,
      beginAt.y,
      endTheoreticalAt.x - beginAt.x - offsetX,
      endTheoreticalAt.y - beginAt.y - offsetY
    );
  }
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

onMounted(async () => {
  await getMonitors();
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
        <el-select v-model="monitorId" placeholder="Select">
          <el-option
            v-for="item in monitors"
            :key="item.id"
            :label="'monitor:' + item.id"
            :value="item.id"
          />
        </el-select>
        <el-button
          type="primary"
          plain
          @click="capture"
          :disabled="!monitorId"
          class="capture"
        >
          <el-text>capture</el-text>
        </el-button>
        <el-button type="primary" plain @click="toggleBg" class="toggle-bg">
          <el-text>{{ bgUrl == bgDark ? "Light" : "Dark" }}</el-text>
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
            <el-icon title="match image" @click=""><Picture /></el-icon>
            <!-- match locating colors -->
            <el-icon title="match locating colors" @click=""
              ><Orange
            /></el-icon>
            <!-- match color -->
            <el-icon title="match color" @click=""><Pointer /></el-icon>
            <!-- recognize text -->
            <el-icon title="recognize text" @click=""><View /></el-icon>
            <!-- close -->
            <el-icon title="close" @click="cancelCapture"
              ><CircleClose
            /></el-icon>
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
      <el-footer
        >position: ({{ point.x }}, {{ point.y }}) beginAt: ({{ beginAt.x }},
        {{ beginAt.y }}) endAt: ({{ endAt.x }}, {{ endAt.y }})</el-footer
      >
    </el-container>
    <!-- <el-aside width="200px">Sider</el-aside> -->
  </el-container>
</template>

<style scoped>
.el-header {
  display: flex;
  height: 30px;
  overflow: hidden;
  align-items: center;
  .refresh {
    margin: 0px 5px;
  }
  .capture {
    margin: 0px 5px;
  }
  .toggle-bg {
    margin: 0px 5px 0px 0px;
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
}
</style>
