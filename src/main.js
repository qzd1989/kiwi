import ElementPlus from "element-plus";

import { createApp } from "vue";
import { createRouter, createWebHistory } from "vue-router";
import { createPinia } from "pinia";
import piniaPluginPersistedstate from "pinia-plugin-persistedstate";
import * as ElementPlusIconsVue from "@element-plus/icons-vue";
import "./css/default.css";

import App from "./App.vue";
import Monitor from "./Monitor.vue";

import Home from "./views/app/Home.vue";
import PermissionManager from "./views/app/PermissionManager.vue";
import Setting from "./views/app/Setting.vue";
import ProjectCreate from "./views/app/project/Create.vue";
import ProjectPanel from "./views/app/project/Panel.vue";

const routes = [
  { path: "/", redirect: "/app/home" },
  { path: "/app/home", component: Home },
  { path: "/app/setting", component: Setting },
  { path: "/app/permission_manager", component: PermissionManager },
  { path: "/app/project/create", component: ProjectCreate },
  { path: "/app/project/panel", component: ProjectPanel },
  { path: "/monitor", component: Monitor },
];
const router = createRouter({
  history: createWebHistory(),
  routes,
});
const pinia = createPinia();
pinia.use(piniaPluginPersistedstate);

const app = createApp(App);
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
  app.component(key, component);
}
app.use(ElementPlus, { size: "small" });
app.use(router);
app.use(pinia);
app.mount("#app");
