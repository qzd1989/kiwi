import { invoke } from "@tauri-apps/api/core";
import { sep } from "@tauri-apps/api/path";

export const projectsDir = await invoke("projects_dir");
export const defaultScript = "main.py";

export const projectDir = async (project) => {
  return projectsDir + (await sep()) + project;
};
export const resourceDir = async (project) => {
  return projectsDir + (await sep()) + project + (await sep()) + "resources";
};
export const scriptDir = async (project) => {
  return projectsDir + (await sep()) + project + (await sep()) + "scripts";
};
export const defaultScriptFile = async (project) => {
  return (
    projectsDir +
    (await sep()) +
    project +
    (await sep()) +
    "scripts" +
    (await sep()) +
    defaultScript
  );
};
