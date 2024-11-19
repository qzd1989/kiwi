import { createStore } from "vuex";

export const store = createStore({
  state() {
    return {
      focus: "left", //区域焦点: left, editor, terminal, right
      windowSize: {
        width: 0,
        height: 0,
      },
    };
  },
  getters: {
    focus(state) {
      return state.focus;
    },
    windowSize(state) {
      return state.windowSize;
    },
  },
  mutations: {
    focus(state, val) {
      state.focus = val;
    },
    windowSize(state, val) {
      state.windowSize = val;
    },
  },
});
