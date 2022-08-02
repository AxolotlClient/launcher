import { fs, path } from "@tauri-apps/api";
import * as fsExtra from "tauri-plugin-fs-extra-api";

const configPath = "./AxolotlClient.json"; // What should the default config path be?

// Expand if needed, remove unused fields if the specific part is finished (e.g. auth)
config = {
    auth: {
        /*uuid1: { //Only an example. Must be populated in code.
            username: null,
            uuid: null,
            auth_token: null,
            refresh_token: null
        }*/
    },

    settings: {
        java: {
            jre8: "", // Need to look up MC's runtime path, or search for a system install!
            jre17: "",
            minMem: "2G",
            maxMem: "2G",
            arguments: [
                //...
            ]
        },

        game: {
            // Options that will be converted to launch arguments.
            fullscreen: false,
            height: 480,
            width: 640,
            detached: true
        },

        launcher: {
            minimizeOnGameStart: true,
            configPath: configPath,
        }
    },

    currentMCVersion: null,

    versionConfigs: {}
}

export async function save() {
    if (!fsExtra.exists(config.settings.launcher.configPath)) {
        fs.createDir(config.settings.launcher.configPath);
    }

    fs.writeTextFile(config.settings.launcher.configPath, JSON.stringify(config));
}

export async function load() {
    if (fsExtra.exists(config.settings.launcher.configPath)) {
        config = JSON.parse(fs.readTextFile(config.settings.launcher.configPath));
    }
}

export function getJRE8() {
    return config.settings.java.jre8;
}

export function getJRE17() {
    return config.settings.java.jre17;
}

/*
 * etc. Needs to be expanded in the future for all fields.
 */