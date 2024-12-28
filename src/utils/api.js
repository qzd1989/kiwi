import { invoke } from "@tauri-apps/api/core";
import { msgError } from "./msg";

export async function createDir(path) {
  return await invoke("create_dir", { path }).catch((error) => {
    msgError(error);
    throw error;
  });
}

export async function createFile(path) {
  return await invoke("create_file", { path }).catch((error) => {
    msgError(error);
    throw error;
  });
}

export async function rename(from, to) {
  return await invoke("rename", { from, to }).catch((error) => {
    msgError(error);
    throw error;
  });
}

export async function remove(path) {
  return await invoke("remove", { path }).catch((error) => {
    msgError(error);
    throw error;
  });
}

export async function exists(path) {
  return await invoke("exists", { path }).catch((error) => {
    msgError(error);
    throw error;
  });
}

export async function readDir(path) {
  return await invoke("read_dir", { path }).catch((error) => {
    msgError(error);
    throw error;
  });
}

export async function writeFile(path, contents, append) {
  return await invoke("write_file", { path, contents, append }).catch(
    (error) => {
      msgError(error);
      throw error;
    }
  );
}

export async function readFile(path) {
  return await invoke("read_file", { path }).catch((error) => {
    msgError(error);
    throw error;
  });
}

export async function codeCheck(path) {
  return await invoke("code_check", { path }).catch((error) => {
    msgError(error);
    throw error;
  });
}
