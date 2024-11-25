import { invoke } from "@tauri-apps/api/core";
import { sep } from "@tauri-apps/api/path";

export const projectsDir = await invoke("projects_dir");
export const scriptDirName = "scripts";
export const resourceDirName = "resources";
export const defaultScriptName = "main.py";

export const projectDir = async (project) => {
  return projectsDir + (await sep()) + project;
};
export const resourceDir = async (project) => {
  return (
    projectsDir + (await sep()) + project + (await sep()) + resourceDirName
  );
};
export const scriptDir = async (project) => {
  return projectsDir + (await sep()) + project + (await sep()) + scriptDirName;
};
export const defaultScriptFile = async (project) => {
  return (
    projectsDir +
    (await sep()) +
    project +
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

export const getProjectByProjectPath = async (projectAbsPath) => {
  return projectAbsPath.replace(scriptDir + (await sep()), "");
};
