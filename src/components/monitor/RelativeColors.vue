<script setup>
import { ref, onMounted, reactive, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import {
  base64ToPixels,
  rgbToHex,
  drawArc,
  drawText,
  formatIntRange,
} from "@utils/common";
import { msgError, msgSuccess } from "@utils/msg";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
const props = defineProps(["params", "monitor"]);
const emits = defineEmits(["close", "drawItems", "clearAllItems"]);
const result = ref(null);
const code = ref(null);
const formRef = ref(null);
const form = reactive({
  points: [],
  offset: {
    r: 0,
    g: 0,
    b: 0,
  },
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
const pixels = ref([]);
const rules = reactive({
  "findArea.start.x": [{ required: true, message: "", trigger: "blur" }],
  "findArea.start.y": [{ required: true, message: "", trigger: "blur" }],
  "findArea.end.x": [{ required: true, message: "", trigger: "blur" }],
  "findArea.end.y": [{ required: true, message: "", trigger: "blur" }],
  "offset.r": [{ required: true, message: "", trigger: "blur" }],
  "offset.g": [{ required: true, message: "", trigger: "blur" }],
  "offset.b": [{ required: true, message: "", trigger: "blur" }],
});
const close = () => {
  emits("close");
};
const drawImage = () => {
  base64ToPixels(props.params.base64Data)
    .then((data) => {
      let arr = [];
      let index = 0;
      for (let y = 0; y < props.params.size.height; y++) {
        for (let x = 0; x < props.params.size.width; x++) {
          let pixel = {
            key: x + "," + y,
            hex: rgbToHex(data[index]),
            point: {
              x: x,
              y: y,
            },
          };
          arr.push(pixel);
          index++;
        }
      }
      pixels.value = arr;
    })
    .catch((error) => console.error(error));
};
const pushColor = async (locatingColor) => {
  if (
    form.points
      .map((item) => {
        return item.key;
      })
      .includes(locatingColor.key)
  ) {
    msgError("The point is already exist!");
    return;
  }
  result.value = code.value = null;
  locatingColor.relativePoint = { x: -1, y: -1 };
  form.points.push(locatingColor);
  caculateRelativePoints();
};
const unAdd = async () => {
  result.value = code.value = null;
  form.points.pop();
  caculateRelativePoints();
};
const removeColor = (locatingColor) => {
  result.value = code.value = null;
  form.points = form.points.filter((item) => {
    const compareItem = item.point.x + "," + item.point.y;
    const compare = locatingColor.point.x + "," + locatingColor.point.y;
    return compareItem !== compare;
  });
  caculateRelativePoints();
};
const caculateRelativePoints = () => {
  if (form.points.length == 0) {
    return;
  }
  if (form.points.length == 1) {
    form.points[0].relativePoint.x = 0;
    form.points[0].relativePoint.y = 0;
    return;
  }
  const points = [];
  form.points.forEach((item) => {
    const color = {};
    Object.assign(color, item);
    points.push(color);
  });
  points.sort((a, b) => {
    if (a.point.y == b.point.y) {
      return a.point.x - b.point.x;
    } else {
      return a.point.y - b.point.y;
    }
  });
  points.forEach((item, index, origin) => {
    //the first element is the peak point
    if (index == 0) {
      origin[index].relativePoint.x = 0;
      origin[index].relativePoint.y = 0;
    }
    if (index > 0) {
      origin[index].relativePoint.x = item.point.x - origin[0].point.x;
      origin[index].relativePoint.y = item.point.y - origin[0].point.y;
    }
  });
  //assign points' relativePoint into points
  form.points.forEach((item, index, arr) => {
    let point = points
      .filter((c) => {
        return c.key == item.key;
      })
      .pop();
    arr[index].relativePoint.x = point.relativePoint.x;
    arr[index].relativePoint.y = point.relativePoint.y;
  });
  return;
};
const getRelativePoint = (key) => {
  if (form.points == 0) {
    return { x: -1, y: -1 };
  }
  return form.points
    .filter((item) => {
      return item.key == key;
    })
    .pop().relativePoint;
};
const getPeakKey = () => {
  if (form.points.length == 0) {
    return "";
  }
  return form.points
    .filter((item) => {
      return item.relativePoint.x == 0 && item.relativePoint.y == 0;
    })
    .pop().key;
};
const getRelativePoints = () => {
  if (form.points.length <= 1) {
    return [];
  }
  return form.points
    .filter((item) => item.relativePoint.x != 0 || item.relativePoint.y != 0)
    .map((item) => {
      const row = {
        point: {
          x: item.relativePoint.x,
          y: item.relativePoint.y,
        },
        hex: item.hex,
      };
      return row;
    });
};
const generateCode = async () => {
  const vertexHex = getVertexHex();
  const relativePoints = getRelativePoints();
  const startPoint = form.findArea.start;
  const endPoint = form.findArea.end;
  const rgbOffset = form.offset;
  try {
    code.value = await invoke("generate_find_relative_colors_code", {
      vertexHex,
      relativePoints,
      startPoint,
      endPoint,
      rgbOffset,
    });
  } catch (error) {
    msgError(error.message);
  }
};
const getVertexHex = () => {
  if (form.points.length == 0) {
    return "";
  }
  return form.points
    .filter((item) => {
      return item.relativePoint.x == 0 && item.relativePoint.y == 0;
    })
    .pop().hex;
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
  const max = 50;
  form.offset.r = formatIntRange(form.offset.r, { min: 0, max });
  form.offset.g = formatIntRange(form.offset.g, { min: 0, max });
  form.offset.b = formatIntRange(form.offset.b, { min: 0, max });
};
const findRelativeColors = async () => {
  correctForm();
  result.value = code.value = null;
  await formRef.value.validate(async (valid, fields) => {
    if (!valid) {
      return;
    }
    if (form.points.length == 0) {
      result.value = code.value = null;
      clearAllItems();
      msgError("The colors must not be empty.");
      return;
    }
    const vertexHex = getVertexHex();
    const relativePoints = getRelativePoints();
    const origin = props.monitor.base64Data;
    const startPoint = form.findArea.start;
    const endPoint = form.findArea.end;
    const rgbOffset = form.offset;
    try {
      const peak = await invoke("find_relative_colors", {
        origin,
        vertexHex,
        relativePoints,
        startPoint,
        endPoint,
        rgbOffset,
      });
      if (peak == null) {
        clearAllItems();
      } else {
        result.value = JSON.stringify(peak);
        drawItems(peak);
      }
      await generateCode();
    } catch (error) {
      clearAllItems();
      result.value = code.value = null;
      msgError(error.message);
    }
  });
};
const drawItems = (peak) => {
  emits("drawItems", {
    callback: (ctx) => {
      const title = `peak point(${peak.point.x}, ${peak.point.y})`;
      const titlePoint = {
        x: peak.point.x - 5,
        y: peak.point.y - 10,
      };
      drawArc(ctx, peak.point, 5);
      drawText(ctx, title, titlePoint);
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
  form.points = props.params.points ?? form.points;
  result.value = code.value = null;
  setTimeout(drawImage, 100);
};
watch(props.params, () => {
  loadData();
});
onMounted(async () => {
  loadData();
});
</script>
<template>
  <el-container>
    <el-header>Find Locating Colors</el-header>
    <el-main>
      <el-form ref="formRef" :model="form" :rules="rules" status-icon>
        <div class="work-area">
          <div class="canvas-work">
            <div class="pixels-box">
              <div
                class="pixels"
                :style="{
                  width: props.params.size.width * 7 + 'px',
                  height: props.params.size.height * 7 + 'px',
                }"
              >
                <div
                  class="pixel"
                  v-for="item in pixels"
                  :style="{ 'background-color': item.hex }"
                  @click="pushColor(item)"
                  :class="{
                    selected: form.points
                      .map((row) => {
                        return row.key;
                      })
                      .includes(item.key),
                    peak: item.key == getPeakKey(),
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
              prop="points"
              style="margin-bottom: 0px"
              v-show="form.points.length > 0"
            >
              <el-input
                class="color"
                style="margin-bottom: 2px"
                v-for="item in form.points"
                :value="
                  item.hex +
                  ' ' +
                  '(' +
                  item.point.x +
                  ',' +
                  item.point.y +
                  ')' +
                  '(' +
                  getRelativePoint(item.key).x +
                  ',' +
                  getRelativePoint(item.key).y +
                  ')'
                "
                disabled
              >
                <template #prepend>
                  <div
                    style="height: 10px; width: 10px; border-radius: 5px"
                    :style="{ backgroundColor: item.hex }"
                  ></div>
                </template>
                <template #append>
                  <el-button @click="removeColor(item)">×</el-button>
                </template>
              </el-input>
            </el-form-item>
          </div>
          <div class="item">
            <div class="title">
              <span>Find Area</span>
              <el-button type="primary" @click="findRelativeColors">
                findOne
              </el-button>
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
              <el-row :gutter="0">
                <el-col :span="8">
                  <el-form-item style="margin-bottom: 0px" prop="offset.r">
                    <el-input
                      v-model="form.offset.r"
                      autocapitalize="off"
                      autocorrect="off"
                      spellcheck="false"
                    >
                      <template #prepend>r</template>
                    </el-input>
                  </el-form-item>
                </el-col>
                <el-col :span="8">
                  <el-form-item prop="offset.g" style="margin-bottom: 0px">
                    <el-input
                      v-model="form.offset.g"
                      autocapitalize="off"
                      autocorrect="off"
                      spellcheck="false"
                    >
                      <template #prepend>g</template>
                    </el-input>
                  </el-form-item>
                </el-col>
                <el-col :span="8">
                  <el-form-item prop="offset.b" style="margin-bottom: 0px">
                    <el-input
                      v-model="form.offset.b"
                      autocapitalize="off"
                      autocorrect="off"
                      spellcheck="false"
                    >
                      <template #prepend>b</template>
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
                :rows="9"
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

    .peak {
      border-color: red;
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
