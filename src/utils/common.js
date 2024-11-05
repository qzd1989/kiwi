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
