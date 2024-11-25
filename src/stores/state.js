import { createStore } from "vuex";

export const store = createStore({
  state() {
    return {
      focus: "left", //区域焦点: left, editor, terminal, right
      windowSize: {
        width: 0,
        height: 0,
      },
      projectPath: null, //current project (abs path)
      filePath: null, //current file (abs path)
    };
  },
  getters: {
    focus(state) {
      return state.focus;
    },
    windowSize(state) {
      return state.windowSize;
    },
    projectPath(state) {
      return state.projectPath;
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
    projectPath(state, val) {
      state.projectPath = val;
    },
    filePath(state, val) {
      state.filePath = val;
    },
  },
});
