import { fs, path } from "@tauri-apps/api";
import * as fsExtra from "tauri-plugin-fs-extra-api";

const configPath = await path.join(await path.appDir(), "AxolotlClient.json");

// Expand if needed, remove unused fields if the specific part is finished (e.g. auth)
let config = {
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
            minimizeOnGameStart: true
        }
    },

    currentMCVersion: null,

    versionConfigs: {}
};

export async function save() {
    if(!await fsExtra.exists(await path.appDir())) {
        await fs.createDir(await path.appDir());
    }

    await fs.writeTextFile(configPath, JSON.stringify(config));
}

export async function load() {
    if (await fsExtra.exists(configPath)) {
        config = { ...config, ...JSON.parse(await fs.readTextFile(configPath)) };
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