import { load } from "@tauri-apps/plugin-store";
import { homeDir } from "@tauri-apps/api/path";

const STORE_FILE = "kiwi.json";

let storeInstance = null;
let cachedDefaultKeys = null;

async function getStore() {
  if (!storeInstance) {
    storeInstance = await load(STORE_FILE, { autoSave: true });
  }
  return storeInstance;
}

async function getDefaultKeys() {
  if (cachedDefaultKeys) return cachedDefaultKeys;

  const home = await homeDir();
  cachedDefaultKeys = {
    projectRootDirectory: {
      key: "projectRootDirectory",
      default: home,
    },
    isPythonAttributed: {
      key: "isPythonAttributed",
      default: false,
    },
  };
  return cachedDefaultKeys;
}

function findDefaultKey(keys, key) {
  return Object.values(keys).find((entry) => entry.key === key);
}

export const getDefaultKey = async (key) => {
  const keys = await getDefaultKeys();

  const entry = findDefaultKey(keys, key);

  if (entry) {
    return entry.default;
  }

  return null;
};

export async function getLocalValue(key) {
  const store = await getStore();
  const value = await store.get(key);

  if (value !== null && value !== undefined) {
    return value;
  }

  const keys = await getDefaultKeys();
  const entry = findDefaultKey(keys, key);
  if (entry) {
    await store.set(key, entry.default);
    return entry.default;
  }

  return null;
}

export async function setLocalValue(key, value) {
  const store = await getStore();
  await store.set(key, value);
}

export async function deleteLocalValue(key) {
  const store = await getStore();
  await store.delete(key);
}

export async function clearLocalStore() {
  const store = await getStore();
  await store.clear();
}
