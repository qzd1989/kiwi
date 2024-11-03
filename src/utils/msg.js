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
  ElMessage({
    showClose: true,
    message: `${msg}`,
    type: "error",
    grouping: true,
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
