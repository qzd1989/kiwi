import { getAllWindows } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";
import { exists } from "@tauri-apps/plugin-fs";
import { getLocalValue, getDefaultKey } from "@utils/local-store";
import { msgError } from "./msg";
export class Stack {
  constructor(limit) {
    this.stack = [];
    this.limit = limit;
  }
  push(element) {
    if (this.stack.length >= this.limit) {
      this.stack.shift();
    }
    this.stack.push(element);
  }
  stack() {
    return this.stack;
  }
}

export function drawBase64ImageOnCanvas(canvas, data, x, y, width, height) {
  return new Promise((resolve, reject) => {
    const imageData = new Image();
    imageData.onload = function () {
      const ctx = canvas.getContext("2d");
      ctx.drawImage(imageData, x, y, width, height);
      resolve(ctx);
    };
    imageData.src = data;
  });
}

//data可以是ArrayBuffer类型,也可以是Array类型:[0,0,0,0]
export function arrayImageDataToBase64ImageData(data, width, height) {
  var canvas = document.createElement("canvas");
  canvas.width = width;
  canvas.height = height;
  const ctx = canvas.getContext("2d");
  const imageData = new ImageData(new Uint8ClampedArray(data), width, height);
  ctx.putImageData(imageData, 0, 0);
  return canvas.toDataURL("image/png");
}

export function cropBase64Image(base64Data, x, y, width, height) {
  return new Promise((resolve, reject) => {
    // 创建一个 image 对象
    const img = new Image();
    img.onload = function () {
      // 创建一个 canvas 元素
      const canvas = document.createElement("canvas");
      const ctx = canvas.getContext("2d");

      // 设置 canvas 的尺寸为裁剪区域的大小
      canvas.width = width;
      canvas.height = height;

      // 绘制图像到 canvas，并指定裁剪区域
      ctx.drawImage(img, x, y, width, height, 0, 0, width, height);

      // 转换 canvas 内容为新的 Base64 数据
      const croppedBase64 = canvas.toDataURL("image/png");
      resolve(croppedBase64);
    };
    img.onerror = function () {
      reject(new Error("Could not load image."));
    };
    // 设置 image 的 src 为 Base64 数据
    img.src = base64Data;
  });
}

export function base64ToPixels(base64String) {
  // 创建一个HTML图像对象
  let img = new Image();
  // 解码Base64字符串并设置为图像源
  img.src = base64String;

  return new Promise((resolve, reject) => {
    // 确保图像加载完成后再处理
    img.onload = function () {
      // 创建一个canvas元素
      let canvas = document.createElement("canvas");
      // 设置canvas大小与图像一致
      canvas.width = this.width;
      canvas.height = this.height;
      // 获取2D渲染上下文
      let ctx = canvas.getContext("2d");
      // 将图像绘制到canvas上
      ctx.drawImage(this, 0, 0, canvas.width, canvas.height);
      // 获取图像数据
      let imageData = ctx.getImageData(0, 0, canvas.width, canvas.height).data;

      // 创建一个二维数组来存储像素点颜色
      let pixels = [];
      for (let i = 0; i < imageData.length; i += 4) {
        // 每四个元素代表一个像素点的RGBA值
        pixels.push([imageData[i], imageData[i + 1], imageData[i + 2]]);
      }

      resolve(pixels); // 解析Promise，返回像素数组
    };

    img.onerror = function () {
      reject(
        new Error("Failed to load image from the provided Base64 string.")
      );
    };
  });
}

/**
 * 将RGB颜色值转换为十六进制颜色值,也可以传入RGBA,只取RGB
 * @param {Array} rgbArray - 一个包含三个整数的数组，表示RGB颜色值（例如[255, 99, 71]）
 * @returns {string} - 转换后的十六进制颜色值（例如"#ff6347"）
 */
export function rgbToHex(rgbArray) {
  /**
   * 将单个RGB颜色值转换为两位的十六进制字符串
   * @param {number} colorValue - 单个RGB颜色值（0-255之间的整数）
   * @returns {string} - 两位的十六进制颜色值
   */
  function toHex(colorValue) {
    var hex = colorValue.toString(16); // 将数字转换为十六进制字符串
    return hex.length === 1 ? "0" + hex : hex; // 确保两位字符，如果只有一位则前面加0
  }

  // 构建最终的十六进制颜色值
  var hexColor =
    "#" + toHex(rgbArray[0]) + toHex(rgbArray[1]) + toHex(rgbArray[2]);
  return hexColor; // 返回转换后的十六进制颜色值
}
export function generateRandomString(length) {
  const characters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
  return Array.from({ length }, () =>
    characters.charAt(Math.floor(Math.random() * characters.length))
  ).join("");
}

export function generateRandomHexColor() {
  return (
    "#" +
    Math.floor(Math.random() * 0xffffff)
      .toString(16)
      .padStart(6, "0")
  );
}
export function getCurrentTimeWithMilliseconds() {
  var now = new Date();
  var year = now.getFullYear();
  var month = ("0" + (now.getMonth() + 1)).slice(-2); // 月份从0开始，所以需要+1
  var day = ("0" + now.getDate()).slice(-2);
  var hour = ("0" + now.getHours()).slice(-2);
  var minute = ("0" + now.getMinutes()).slice(-2);
  var second = ("0" + now.getSeconds()).slice(-2);
  var millisecond = ("00" + now.getMilliseconds()).slice(-3); // 毫秒可能是一位或两位数，需要确保是三位数
  console.log(
    year +
      "-" +
      month +
      "-" +
      day +
      " " +
      hour +
      ":" +
      minute +
      ":" +
      second +
      "." +
      millisecond
  );
  return (
    year +
    "-" +
    month +
    "-" +
    day +
    " " +
    hour +
    ":" +
    minute +
    ":" +
    second +
    "." +
    millisecond
  );
}

export function formatLogTime(timestamp) {
  const date = new Date(timestamp * 1000);
  const year = date.getFullYear().toString().slice(-2);
  const month = String(date.getMonth() + 1).padStart(2, "0");
  const day = String(date.getDate()).padStart(2, "0");
  const hours = String(date.getHours()).padStart(2, "0");
  const minutes = String(date.getMinutes()).padStart(2, "0");
  const seconds = (date.getSeconds() + (timestamp % 1))
    .toFixed(3)
    .padStart(6, "0");
  const formattedTime = `${year}-${month}-${day} ${hours}:${minutes}:${seconds}`;
  return formattedTime;
}

export function drawText(ctx, text, point) {
  ctx.lineWidth = 3;
  ctx.strokeStyle = "#ffffff";
  ctx.font = "12px system-ui";
  ctx.strokeText(text, point.x, point.y);
  ctx.fillStyle = "#ff0000";
  ctx.fillText(text, point.x, point.y);
}

export function drawArc(ctx, point, radius) {
  ctx.lineWidth = 3;
  ctx.strokeStyle = "#ffffff";
  ctx.beginPath();
  ctx.arc(point.x, point.y, radius, 0, Math.PI * 2);
  ctx.stroke();
  ctx.lineWidth = 1;
  ctx.strokeStyle = "#ff0000";
  ctx.beginPath();
  ctx.arc(point.x, point.y, radius, 0, Math.PI * 2);
  ctx.stroke();
  ctx.lineWidth = 3;
  ctx.strokeStyle = "#ffffff";
  ctx.beginPath();
  ctx.arc(point.x, point.y, 1, 0, Math.PI * 2);
  ctx.stroke();
  ctx.lineWidth = 1;
  ctx.strokeStyle = "#ff0000";
  ctx.beginPath();
  ctx.arc(point.x, point.y, 1, 0, Math.PI * 2);
  ctx.stroke();
}

export function drawRect(ctx, point, size) {
  ctx.lineWidth = 3;
  ctx.strokeStyle = "#ffffff";
  ctx.strokeRect(point.x, point.y, size.width, size.height);
  ctx.lineWidth = 1;
  ctx.strokeStyle = "#ff0000";
  ctx.strokeRect(point.x, point.y, size.width, size.height);
}
export const minimizeAll = async () => {
  const windows = await getAllWindows();
  for (const window of windows) {
    await window.minimize();
  }
};
export const unminimizeAll = async () => {
  const windows = await getAllWindows();
  for (const window of windows) {
    window.unminimize().then(() => {
      return window.setFocus();
    });
  }
};
export const delay = (ms) => {
  return new Promise((resolve) => setTimeout(resolve, ms));
};

export const openWebsocket = async (port) => {
  const websocketPort = String(port);
  return await invoke("open_websocket", { port: websocketPort });
};

export const formatPoint = (point) => {
  const x = parseInt(point.x);
  const y = parseInt(point.y);
  return { x, y };
};

export const formatIntRange = (data, range, d) => {
  const result = parseInt(data);
  if (Number.isNaN(result)) {
    return d;
  }
  if (result > range.max) {
    return range.max;
  }
  if (result < range.min) {
    return range.min;
  }
  return result;
};

export const formatFloatRange = (data, range, d) => {
  const result = parseFloat(data);
  if (Number.isNaN(result)) {
    return d;
  }
  if (result > range.max) {
    return range.max;
  }
  if (result < range.min) {
    return range.min;
  }
  return result;
};

export const shutdownWebsocket = async () => {
  return await invoke("shutdown_websocket");
};

export const isWebsocketAlive = async (port) => {
  const websocketPort = String(port);
  return await invoke("is_websocket_alive", { port: websocketPort });
};

export const getProjectRootDirectory = async () => {
  const path = await getLocalValue("projectRootDirectory");
  const exists = await invoke("path_exists", { path });
  if (exists) {
    return path;
  }
  return await getDefaultKey("projectRootDirectory");
};
