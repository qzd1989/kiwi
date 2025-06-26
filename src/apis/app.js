import { join } from "@tauri-apps/api/path";
import { arch, platform } from "@tauri-apps/plugin-os";
import { onMounted, onUnmounted, ref, computed } from "vue";

const apiAddr = computed(() => {
  return import.meta.env.DEV
    ? "http://localhost:5173"
    : "https://kiwi.biexi.com";
});
const channel = computed(() => {
  return import.meta.env.DEV ? "beta" : "stable";
});

const buildApiUrl = (path, params) => {
  const url = new URL(apiAddr.value + path);
  Object.entries(params).forEach(([key, val]) => {
    url.searchParams.set(key, val);
  });
  return url.toString();
};
export const getVersion = async () => {
  const url = buildApiUrl("/version.json", {
    platform: await platform(),
    arch: await arch(),
    lang: "zh-CN",
    channel: channel.value,
  });
  try {
    const result = await fetch(url);
    if (result.ok) {
      const data = await result.json();
      return data;
    }
  } catch (error) {
    // 没有必要显示出来,允许断网
    console.log("getVersion error:", error);
  }
  return null;
};
