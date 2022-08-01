const { os, path } = window.__TAURI__;
const platformName = await os.platform();
const home = await path.homeDir();
const data = await path.dataDir();

let dotMinecraft;

switch (platformName) {
    case "darwin":
        dotMinecraft = await path.join(data, "minecraft");
        break;
    case "win32":
        dotMinecraft = await path.join(data, ".minecraft");
        break;
    default:
        dotMinecraft = await path.join(home, ".minecraft");
        break;
}

export const DOT_MINECRAFT = dotMinecraft;
export const ASSETS = await path.join(DOT_MINECRAFT, "assets");
export const LIBRARIES = await path.join(DOT_MINECRAFT, "libraries");
export const VERSIONS = await path.join(DOT_MINECRAFT, "versions");