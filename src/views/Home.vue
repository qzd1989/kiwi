<script setup>
import { ref, onMounted, onUnmounted, reactive } from "vue";
import { useResizeObserver } from "@vueuse/core";
import { useStore } from "vuex";
import { Stack } from "./../utils/common";
import Tree from "./Tree.vue";
import Editor from "./Editor.vue";
import Monitor from "./Monitor.vue";
import Log from "./Log.vue";
const store = useStore();
const windowRef = ref(null);
const leftRef = ref(null);
const middleRef = ref(null);
const rightRef = ref(null);
const bottomRef = ref(null);
const windowSize = reactive({ width: 0, height: 0 });
const middleWidth = ref(0);
const topHeight = ref(0);
const leftWidth = ref(200);
const bottomHeight = ref(200);
const rightWidth = ref(0);
const gapLength = ref(10);
const point = reactive({ x: 0, y: 0 });
const mouseHorizontalStack = new Stack(2);
const mouseVerticalStack = new Stack(2);
const draggingLeft = ref(false);
const draggingRight = ref(false);
const draggingBottom = ref(false);
const files = ref(new Map());
const lastOpenedFile = ref(null);
function moveListener(event) {
  const containerRect = windowRef.value.getBoundingClientRect();
  point.x = event.clientX - containerRect.left;
  point.y = event.clientY - containerRect.top;
  mouseHorizontalStack.push(point.x);
  mouseVerticalStack.push(point.y);
  //to right
  if (draggingLeft.value) {
    const remain = windowSize.width - rightRef.value.offsetWidth;
    leftWidth.value = Math.max(point.x, gapLength.value);
    leftWidth.value = Math.min(leftWidth.value, remain);
    middleWidth.value = remain - leftWidth.value;
  }
  //to left
  if (draggingRight.value) {
    const remain = windowSize.width - leftRef.value.offsetWidth;
    rightWidth.value = Math.max(windowSize.width - point.x, 0);
    rightWidth.value = Math.min(rightWidth.value, remain - gapLength.value);
    middleWidth.value = remain - rightWidth.value;
  }
  //to up and down
  if (draggingBottom.value) {
    topHeight.value = Math.max(point.y, 5);
    topHeight.value = Math.min(topHeight.value, windowSize.height);
    bottomHeight.value = windowSize.height - topHeight.value;
  }
}
function upListener() {
  draggingLeft.value = false;
  draggingRight.value = false;
  draggingBottom.value = false;
}
function drag(event, area) {
  event.preventDefault();
  switch (area) {
    case "left":
      draggingLeft.value = true;
      break;
    case "right":
      draggingRight.value = true;
      break;
    case "bottom":
      draggingBottom.value = true;
      break;
  }
}
function addFile(data) {
  const key = data.file.path;
  const file = data.file;
  lastOpenedFile.value = file;
  if (files.value.has(key)) {
    return;
  }
  files.value.set(key, file);
}
function removeFile(data) {
  const key = data.file.path;
  if (!files.value.has(key)) {
    return;
  }
  files.value.delete(key);
  lastOpenedFile.value = null;
}
function clearFiles() {
  files.value.clear();
  lastOpenedFile.value = null;
}
useResizeObserver(windowRef, (entries) => {
  const entry = entries[0];
  const { width, height } = entry.contentRect;
  windowSize.width = width;
  windowSize.height = height;
  store.commit("windowSize", windowSize);
  middleWidth.value =
    windowSize.width - leftRef.value.offsetWidth - rightRef.value.offsetWidth;
  topHeight.value = windowSize.height - bottomHeight.value;
});
onMounted(async () => {
  document.addEventListener("mousemove", moveListener);
  document.addEventListener("mouseup", upListener);
});
onUnmounted(() => {
  document.removeEventListener("mousemove");
  document.removeEventListener("mouseup");
});
</script>
<template>
  <div
    class="container"
    ref="windowRef"
    :style="{
      '--gap-width': gapLength + 'px',
    }"
  >
    <div
      ref="leftRef"
      class="left"
      :style="{
        width: leftWidth + 'px',
        minWidth: leftWidth + 'px',
      }"
      @click="store.commit('focus', 'left')"
    >
      <div class="tree" v-show="leftWidth != gapLength">
        <Tree :files="files" @add:file="addFile" @clear:files="clearFiles" />
      </div>
      <div
        class="gap-vertical"
        @mousedown="drag($event, 'left')"
        :class="{ selected: draggingLeft }"
      ></div>
    </div>
    <div
      ref="middleRef"
      class="middle"
      :style="{
        width: middleWidth + 'px',
      }"
    >
      <div
        class="workspace"
        :style="{
          height: windowSize.height + 'px',
        }"
      >
        <div
          v-show="store.getters.projectPath != null"
          class="editor"
          :style="{
            height: topHeight + 'px',
          }"
          @click="store.commit('focus', 'editor')"
        >
          <div
            class="editor-view"
            :style="{
              height: topHeight - gapLength + 'px',
              overflow: 'hidden',
            }"
          >
            <Editor
              v-show="files.size > 0"
              :width="middleWidth"
              :height="topHeight - gapLength"
              :files="files"
              :lastOpenedFile="lastOpenedFile"
              @remove:file="removeFile"
            />
          </div>
          <div
            class="gap-horizontal"
            @mousedown="drag($event, 'bottom')"
            :class="{ selected: draggingBottom }"
          ></div>
        </div>
        <div
          v-show="store.getters.projectPath != null"
          ref="bottomRef"
          class="terminal"
          :style="{
            height: bottomHeight + 'px',
          }"
          @click="store.commit('focus', 'terminal')"
        >
          <div class="log">
            <Log :height="bottomHeight" :files="files" />
          </div>
        </div>
      </div>
      <div
        class="gap-vertical"
        @mousedown="drag($event, 'right')"
        :class="{ selected: draggingRight }"
      ></div>
    </div>
    <div
      ref="rightRef"
      class="right"
      :style="{
        width: rightWidth + 'px',
      }"
      @click="store.commit('focus', 'right')"
    >
      <Monitor />
    </div>
  </div>
</template>

<style scoped>
.container {
  display: flex;
  width: 100%;
  height: 100vh;
  overflow: hidden;
}
.left,
.middle,
.right {
  display: flex;
  overflow: hidden;
  position: relative;
}
.left {
  background-color: var(--Fill);
  color: var(--Primary-Color);
  .tree {
    width: 100%;
  }
  .gap-vertical {
    position: absolute;
    right: 0px;
    top: 0px;
  }
}
.middle {
  background-color: var(--Secondary-Fill);
  .workspace {
    position: relative;
    width: 100%;
    .editor {
      position: relative;
      height: 100%;
    }
    .terminal {
      background-color: var(--Fill);
      .log {
        padding: 10px;
        color: var(--Primary-Color);
        font-size: 12px;
      }
    }
  }
  .gap-vertical {
    position: absolute;
    right: 0px;
    top: 0px;
  }
}
.gap-horizontal {
  height: var(--gap-width);
  width: 100%;
}
.gap-vertical {
  width: var(--gap-width);
  height: 100%;
}
.gap-vertical:hover,
.gap-vertical.selected,
.gap-horizontal:hover,
.gap-horizontal.selected {
  background-color: var(--Third-Fill);
}
.gap-vertical:hover,
.gap-vertical.selected {
  cursor: col-resize;
}
.gap-horizontal:hover,
.gap-horizontal.selected {
  cursor: row-resize;
}
</style>
