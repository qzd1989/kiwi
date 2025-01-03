import { invoke } from "@tauri-apps/api/core";
import { sep } from "@tauri-apps/api/path";
export const minZoomFactor = 0.5;
export const maxZoomFactor = 1.5;
export const homeDir = await invoke("home_dir");
export const scriptDirName = "scripts";
export const resourceDirName = "resources";
export const moduleDirName = "modules";
export const defaultScriptName = "main.py";
export const editableFileTypes = ["py"];
export const moduleDir = async (projectName) => {
  return homeDir + (await sep()) + projectName + (await sep()) + moduleDirName;
};
export const resourceDir = async (projectName) => {
  return (
    homeDir + (await sep()) + projectName + (await sep()) + resourceDirName
  );
};
export const scriptDir = async (projectName) => {
  return homeDir + (await sep()) + projectName + (await sep()) + scriptDirName;
};
export const defaultScriptFile = async (projectName) => {
  return (
    homeDir +
    (await sep()) +
    projectName +
    (await sep()) +
    scriptDirName +
    (await sep()) +
    defaultScriptName
  );
};

export const getDefaultScriptFileByProjctPath = async (projectAbsPath) => {
  return (
    projectAbsPath +
    (await sep()) +
    scriptDirName +
    (await sep()) +
    defaultScriptName
  );
};
