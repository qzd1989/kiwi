<script setup>
import { ref, onMounted, reactive, watch, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useStateStore } from "@utils/state-store";
import { sep } from "@tauri-apps/api/path";
import {
  drawBase64ImageOnCanvas,
  cropBase64Image,
  drawRect,
  drawText,
  formatFloatRange,
  formatIntRange,
} from "@utils/common";
import { msgError, msgSuccess, msgWarn } from "@utils/msg";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
const props = defineProps(["params", "monitor", "imageDataPath"]);
const emits = defineEmits(["close", "drawItems", "clearAllItems"]);
const stateStore = useStateStore();
const originBase64Data = ref("");
const formRef = ref(null);
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
const result = ref(null);
const code = ref(null);
const findType = ref("findImage");
const rules = reactive({
  name: [{ required: true, message: "", trigger: "blur" }],
  "findArea.start.x": [{ required: true, message: "", trigger: "blur" }],
  "findArea.start.y": [{ required: true, message: "", trigger: "blur" }],
  "findArea.end.x": [{ required: true, message: "", trigger: "blur" }],
  "findArea.end.y": [{ required: true, message: "", trigger: "blur" }],
  threshold: [{ required: true, trigger: "blur" }],
});
const filePath = computed(() => {
  if (form.name == null) {
    return "";
  }
  const relativeFilePath = formatImageName(form.name);
  return relativeFilePath + ".png";
});
const fullFilePath = computed(() => {
  if (!form.name?.trim()) {
    return "";
  }
  return (
    stateStore.project.path +
    sep() +
    props.imageDataPath +
    sep() +
    filePath.value
  );
});
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
  base64Data: null,
});

const formatImageName = (name) => {
  if (name == null) {
    return "";
  }
  return name.replace(/[\u3000 ]/g, "").replace(/[/\\]+/g, sep());
};

const close = () => {
  emits("close");
};

const onImageMouseMove = (event) => {
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
};

const onImageMouseOut = () => {
  showmagnifyingGlass.value = false;
};

const onImageMouseDown = () => {
  isErasing.value = true;
  erase();
};

const onDataMouseUp = () => {
  isErasing.value = false;
};

const drawMagnifyingGlass = () => {
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
};

const erase = () => {
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
};

const eraseFromOuter = () => {
  if (showmagnifyingGlass.value == true) {
    erase();
  }
};

const setEraserSize = (size) => {
  eraserSideLength.value = size;
};

const drawImage = () => {
  if (canvasRef.value == null) {
    return;
  }
  drawBase64ImageOnCanvas(
    canvasRef.value,
    form.base64Data,
    dataExtSideLength / 2,
    dataExtSideLength / 2,
    props.params.size.width,
    props.params.size.height
  );
};

const toggleDataSuppliedBg = () => {
  bgUrl.value = bgUrl.value == bgDark ? bgLight : bgDark;
};

const resetImage = () => {
  form.base64Data = originBase64Data.value;
  drawImage();
};

const formatItems = (items) => {
  const data = [];
  for (const item of items) {
    const row = {
      x: item.point.x,
      y: item.point.y,
      width: props.params.size.width,
      height: props.params.size.height,
      title: item.weight.toString(),
    };
    data.push(row);
  }
  return data;
};

const drawItems = (items) => {
  emits("drawItems", {
    callback: (ctx) => {
      for (let item of items) {
        const point = {
          x: item.x,
          y: item.y,
        };
        const size = {
          width: item.width,
          height: item.height,
        };
        const textPoint = {
          x: item.x,
          y: item.y - 5,
        };
        drawRect(ctx, point, size);
        drawText(ctx, item.title, textPoint);
      }
    },
  });
};

const clearAllItems = () => {
  emits("clearAllItems");
};

const correctForm = async () => {
  form.findArea.start.x = formatIntRange(
    form.findArea.start.x,
    {
      min: 0,
      max: props.monitor.size.width - 1,
    },
    0
  );
  form.findArea.start.y = formatIntRange(
    form.findArea.start.y,
    {
      min: 0,
      max: props.monitor.size.height - 1,
    },
    0
  );
  form.findArea.end.x = formatIntRange(
    form.findArea.end.x,
    {
      min: form.findArea.start.x + 1,
      max: props.monitor.size.width,
    },
    props.monitor.size.width
  );
  form.findArea.end.y = formatIntRange(
    form.findArea.end.y,
    {
      min: form.findArea.start.y + 1,
      max: props.monitor.size.height,
    },
    props.monitor.size.height
  );
  form.threshold = formatFloatRange(
    form.threshold,
    { min: 0.5, max: 1.0 },
    0.99
  );
};

const generateFindImageCode = async () => {
  const subpath = formatImageName(form.name);
  const startPoint = form.findArea.start;
  const endPoint = form.findArea.end;
  const threshold = form.threshold;
  try {
    code.value = await invoke("generate_find_image_code", {
      subpath,
      startPoint,
      endPoint,
      threshold,
    });
  } catch (error) {
    msgError(error.message);
  }
};

const generateFindImagesCode = async () => {
  const subpath = formatImageName(form.name);
  const startPoint = form.findArea.start;
  const endPoint = form.findArea.end;
  const threshold = form.threshold;
  try {
    code.value = await invoke("generate_find_images_code", {
      subpath,
      startPoint,
      endPoint,
      threshold,
    });
  } catch (error) {
    msgError(error.message);
  }
};

const findImage = async () => {
  correctForm();
  findType.value = "findImage";
  result.value = code.value = null;
  const origin = props.monitor.base64Data;
  const template = form.base64Data;
  const startPoint = form.findArea.start;
  const endPoint = form.findArea.end;
  const threshold = form.threshold;
  try {
    const weightPoint = await invoke("find_image", {
      origin,
      template,
      startPoint,
      endPoint,
      threshold,
    });
    if (weightPoint == null) {
      clearAllItems();
    } else {
      result.value = JSON.stringify(weightPoint);
      drawItems(formatItems([weightPoint]));
    }
    await generateFindImageCode();
  } catch (error) {
    clearAllItems();
    result.value = code.value = null;
    msgError(error.message);
  }
};

const findImages = async () => {
  correctForm();
  findType.value = "findImages";
  result.value = code.value = null;
  const origin = props.monitor.base64Data;
  const template = form.base64Data;
  const startPoint = form.findArea.start;
  const endPoint = form.findArea.end;
  const threshold = formatFloatRange(form.threshold, 0.5, 1);
  try {
    const weightPoints = await invoke("find_images", {
      origin,
      template,
      startPoint,
      endPoint,
      threshold,
    });
    if (weightPoints.length == 0) {
      clearAllItems();
    } else {
      result.value = JSON.stringify(weightPoints);
      drawItems(formatItems(weightPoints));
    }
    await generateFindImagesCode();
  } catch (error) {
    clearAllItems();
    result.value = code.value = null;
    msgError(error.message);
  }
};

const saveAndCopy = async () => {
  correctForm();
  if (!form.name?.trim()) {
    msgWarn("You need to enter the image name first.");
    return;
  }
  if (!code.value?.trim()) {
    msgWarn("Result is null.");
    return;
  }
  rules.name[0].required = true;
  await formRef.value.validate(async (valid, fields) => {
    if (!valid) {
      return;
    }
    const data = await cropBase64Image(
      canvasRef.value.toDataURL("image/png"),
      dataExtSideLength / 2,
      dataExtSideLength / 2,
      props.params.size.width,
      props.params.size.height
    );
    try {
      await invoke("save_image", {
        name: form.name,
        data,
      });
      await writeText(code.value);
      msgSuccess("copy successed");
    } catch (error) {
      msgError(error.message);
    }
  });
};

const copyFullFilePath = async () => {
  try {
    await writeText(fullFilePath.value);
    msgSuccess("copy successed");
  } catch (error) {
    msgError(`copy failed: ${error.message}`);
  }
};

const loadData = () => {
  form.name = props.params.name ?? form.name;
  form.findArea = props.params.findArea;
  form.base64Data = originBase64Data.value = props.params.base64Data;
  result.value = code.value = null;
  setTimeout(drawImage, 100);
};

watch(
  () => form.name,
  async () => {
    if (findType.value == "findImage") {
      await generateFindImageCode();
    }
    if (findType.value == "findImages") {
      await generateFindImagesCode();
    }
  }
);

watch(props.params, async () => {
  loadData();
});

onMounted(async () => {
  loadData();
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
                  width: props.params.size.width + dataExtSideLength + 'px',
                  height: props.params.size.height + dataExtSideLength + 'px',
                  'background-image': `url(${bgUrl})`,
                }"
              >
                <canvas
                  ref="canvasRef"
                  :width="props.params.size.width + dataExtSideLength"
                  :height="props.params.size.height + dataExtSideLength"
                  @mousemove="onImageMouseMove"
                  @mouseout="onImageMouseOut"
                  @mousedown="onImageMouseDown"
                  @mouseup="onDataMouseUp"
                ></canvas>
                <canvas
                  style="display: none"
                  ref="hiddenCanvasRef"
                  :width="props.params.size.width"
                  :height="props.params.size.height"
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
          <div style="display: flex; justify-content: center; margin-top: 10px">
            <el-button type="info" disabled>
              {{ relativePosition.x }} × {{ relativePosition.y }}
            </el-button>
          </div>
          <div class="actions">
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
            <el-form-item prop="name" style="margin-bottom: 0px">
              <el-input
                v-model="form.name"
                type="text"
                autocapitalize="off"
                autocorrect="off"
                spellcheck="false"
              >
                <template #prepend>image name</template>
                <template #append>.png</template>
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
                  <el-form-item
                    style="margin-bottom: 10px"
                    prop="findArea.start.x"
                  >
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
                  <el-form-item
                    style="margin-bottom: 10px"
                    prop="findArea.start.y"
                  >
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
                  <el-form-item
                    style="margin-bottom: 10px"
                    prop="findArea.end.x"
                  >
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
                  <el-form-item
                    style="margin-bottom: 10px"
                    prop="findArea.end.y"
                  >
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
              <el-form-item style="margin-bottom: 0px" prop="threshold">
                <el-tooltip
                  effect="dark"
                  content="min: 0.5, max:1 "
                  placement="left-start"
                >
                  <el-input
                    v-model="form.threshold"
                    autocapitalize="off"
                    autocorrect="off"
                    spellcheck="false"
                  >
                    <template #prepend>threshold</template>
                  </el-input>
                </el-tooltip>
              </el-form-item>
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
              <el-button type="primary" @click="saveAndCopy">
                save and copy
              </el-button>
            </div>
            <div>
              <el-input
                v-model="code"
                style="width: 100%"
                :rows="6"
                type="textarea"
                readonly
                :disabled="!form.name?.trim()"
              />
            </div>
          </div>
          <div class="item">
            <div class="title">
              <span>Full path of Image</span>
            </div>
            <div>
              <el-form-item style="margin-bottom: 0px">
                <el-input
                  v-model="fullFilePath"
                  autocapitalize="off"
                  autocorrect="off"
                  spellcheck="false"
                  readonly
                  :disabled="!form.name?.trim()"
                >
                  <template #append>
                    <el-button
                      type="primary"
                      @click="copyFullFilePath"
                      :disabled="!form.name?.trim()"
                    >
                      copy
                    </el-button>
                  </template>
                </el-input>
              </el-form-item>
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
  margin: 0px;
  display: flex;
  justify-content: space-around;
  flex-wrap: wrap;
  margin-top: 15px;
  margin-bottom: -20px;
  padding: 0px 10px;
  .el-button {
    margin-bottom: 10px;
  }
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
