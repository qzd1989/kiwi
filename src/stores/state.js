import { fa } from "element-plus/es/locales.mjs";
import { createStore } from "vuex";

export const store = createStore({
  state() {
    return {
      //区域焦点: left, editor, terminal, right
      focus: "left",
      //为了让各views的宽高自适应
      windowSize: {
        width: 0,
        height: 0,
      },
      //缩放倍数
      zoomFactor: 1,
      //代码检查状态
      pythonListener: false,
      //各views使用
      currentProjectPath: null,
      currentProjectName: null,
      currentFilePath: null,
      currentFileName: null,
    };
  },
  getters: {
    focus(state) {
      return state.focus;
    },
    windowSize(state) {
      return state.windowSize;
    },
    zoomFactor(state) {
      return state.zoomFactor;
    },
    pythonListener(state) {
      return state.pythonListener;
    },
    currentProjectPath(state) {
      return state.currentProjectPath;
    },
    currentProjectName(state) {
      return state.currentProjectName;
    },
    currentFilePath(state) {
      return state.currentFilePath;
    },
    currentFileName(state) {
      return state.currentFileName;
    },
    filePath(state) {
      return state.filePath;
    },
  },
  mutations: {
    focus(state, val) {
      state.focus = val;
    },
    windowSize(state, val) {
      state.windowSize = val;
    },
    zoomFactor(state, val) {
      state.zoomFactor = val;
    },
    pythonListener(state, val) {
      state.pythonListener = val;
    },
    currentProjectPath(state, val) {
      state.currentProjectPath = val;
    },
    currentProjectName(state, val) {
      state.currentProjectName = val;
    },
    currentFilePath(state, val) {
      state.currentFilePath = val;
    },
    currentFileName(state, val) {
      state.currentFileName = val;
    },
    filePath(state, val) {
      state.filePath = val;
    },
  },
});
