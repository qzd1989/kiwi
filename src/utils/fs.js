import { invoke } from "@tauri-apps/api/core";
import { msgError } from "./msg";

export async function currentDir() {
  return await invoke("current_dir").catch((e) => {
    msgError(e);
    return null;
  });
}
export async function createDir(path) {
  return await invoke("create_dir", { path }).catch((e) => {
    msgError(e);
    return false;
  });
}

export async function createFile(path) {
  return await invoke("create_file", { path }).catch((e) => {
    msgError(e);
    return false;
  });
}

export async function rename(from, to) {
  return await invoke("rename", { from, to }).catch((e) => {
    msgError(e);
    return false;
  });
}

export async function remove(path) {
  return await invoke("remove", { path }).catch((e) => {
    msgError(e);
    return false;
  });
}

export async function exists(path) {
  return await invoke("exists", { path }).catch((e) => {
    msgError(e);
    return false;
  });
}

export async function readDir(path) {
  return await invoke("read_dir", { path }).catch((e) => {
    msgError(e);
    return false;
  });
}

export async function writeFile(path, contents) {
  return await invoke("write_file", { path, contents }).catch((e) => {
    msgError(e);
    return false;
  });
}

export async function readFile(path) {
  return await invoke("read_file", { path }).catch((e) => {
    msgError(e);
    return null;
  });
}
