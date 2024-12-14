import { load } from "@tauri-apps/plugin-store";
import { homeDir } from "@tauri-apps/api/path";

const defaults = new Map();

//default stores
defaults.set("basePath", await homeDir());
defaults.set("zoomFactor", 1);
//default stores end

export class LocalStore {
  static async get(key) {
    this.store = await load("store.json", { autoSave: true });
    return (await this.store.get(key)) ?? defaults.get(key) ?? null;
  }
  static async set(key, value) {
    this.store = await load("store.json", { autoSave: true });
    await this.store.set(key, value);
    await this.store.save();
  }
  static async clear() {
    this.store = await load("store.json", { autoSave: true });
    await this.store.clear();
  }
}
