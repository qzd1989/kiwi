import { defineStore } from "pinia";

export const useStateStore = defineStore("store", {
  state: () => ({
    app: {
      enable: {
        isWebsocketAlive: false,
        // macos
        hasAccessibilityPermission: false,
        hasScreenRecordingPermission: false,
      },
    },
    project: {
      exists: false,
      name: null,
      language: null,
      path: null,
      mainFile: null,
      mainFileFullPath: null,
      kiwiVersion: null,
    },
    windowSize: {
      width: 0,
      height: 0,
    },
    zoom: {
      factor: 1,
      min: 0.5,
      max: 1.5,
    },
  }),
  persist: true,
});

// export const isAppEnabled = () => {
//   const stateStore = useStore();
//   return computed(() =>
//     Object.values(stateStore.app.enable).every((v) => v === true)
//   );
// };

// export const useStateStore = defineStore("state", () => {
//   const app = reactive({
//     enable: {
//       isWebsocketAlive: false,
//     },
//   });
//   const isAppEnabled = computed(() => {
//     return Object.values(app.enable).every((v) => v === true);
//   });
//   const project = reactive({
//     exists: false,
//     name: null,
//     language: null,
//     path: null,
//     mainFile: null,
//     mainFileFullPath: null,
//     kiwiVersion: null,
//   });
//   const windowSize = reactive({
//     width: 0,
//     height: 0,
//   });

//   const zoom = reactive({
//     factor: 1,
//     min: 0.5,
//     max: 1.5,
//   });

//   // const zoomFactor = ref(1);
//   // const zoomFactorMax = ref(1.5);
//   // const zoomFactorMin = ref(0.5);

//   return {
//     app,
//     isAppEnabled,
//     project,
//     windowSize,
//     zoomFactor,
//     zoomFactorMax,
//     zoomFactorMin,
//   };
// });
