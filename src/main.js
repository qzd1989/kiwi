import { createApp } from "vue";
import ElementPlus from "element-plus";
import * as ElementPlusIconsVue from "@element-plus/icons-vue";
import { store } from "./stores/state";
import { createRouter, createWebHashHistory } from "vue-router";

import "element-plus/dist/index.css";
import "./style.css";

import App from "./App.vue";
import Home from "./views/Home.vue";
import Monitor from "./views/Monitor.vue";
import Hello from "./views/Hello.vue";

const routes = [
  { path: "/", component: Home },
  { path: "/monitor", component: Monitor },
  { path: "/hello", component: Hello },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

const app = createApp(App);
app.use(store);
app.use(ElementPlus, { size: "small" });
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
  app.component(key, component);
}
app.use(router);
app.mount("#app");
