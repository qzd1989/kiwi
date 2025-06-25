import { type } from "@tauri-apps/plugin-os";
import { ElMessage } from "element-plus";
export const msgSuccess = (msg) => {
  ElMessage({
    showClose: true,
    message: `${msg}`,
    type: "success",
    grouping: true,
  });
};

export const msgError = (msg) => {
  let message;
  if (msg instanceof Error) {
    message = msg.message;
  } else if (msg && typeof msg === "object" && "message" in msg) {
    message = msg.message;
  } else if (msg && typeof msg === "object") {
    message = JSON.stringify(msg);
  } else {
    message = msg;
  }
  ElMessage({
    showClose: true,
    message: `${message}`,
    type: "error",
    grouping: true,
    duration: 0,
  });
};

export const msgInfo = (msg) => {
  ElMessage({
    showClose: true,
    grouping: true,
    type: "info",
    message: `${msg}`,
  });
};

export const msgWarn = (msg) => {
  ElMessage({
    showClose: true,
    grouping: true,
    type: "warning",
    message: `${msg}`,
  });
};
