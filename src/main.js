import { createApp } from "vue";
import ElementPlus from "element-plus";
import * as ElementPlusIconsVue from "@element-plus/icons-vue";
import { store } from "./stores/state";
import { createRouter, createWebHistory } from "vue-router";

import "element-plus/dist/index.css";
import "./style.css";

import App from "./App.vue";
import Home from "./views/Home.vue";
import Monitor from "./views/Monitor.vue";

const routes = [
  { path: "/", component: Home },
  { path: "/monitor", component: Monitor },
];

const router = createRouter({
  history: createWebHistory(),
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
