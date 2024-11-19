import { currentDir } from "../utils/fs";
import { sep } from "@tauri-apps/api/path";

export const rootDir = await currentDir();
export const projectsDir = (await currentDir()) + (await sep()) + "projects";
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
