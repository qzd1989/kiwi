import { invoke } from "@tauri-apps/api/core";
import { sep } from "@tauri-apps/api/path";

export const projectsDir = await invoke("projects_dir");
export const scriptDirName = "scripts";
export const resourceDirName = "resources";
export const defaultScriptName = "main.py";

export const resourceDir = async (projectName) => {
  return (
    projectsDir + (await sep()) + projectName + (await sep()) + resourceDirName
  );
};
export const scriptDir = async (projectName) => {
  return (
    projectsDir + (await sep()) + projectName + (await sep()) + scriptDirName
  );
};
export const defaultScriptFile = async (projectName) => {
  return (
    projectsDir +
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
