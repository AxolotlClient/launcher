import * as util from "../util/util.mjs";
import * as fsExtra from "tauri-plugin-fs-extra-api";
import { fs, http, path } from "@tauri-apps/api";
import { ResponseType } from "@tauri-apps/api/http.js";

export async function downloadModFromSlug(slug, gameVersion, path) {
    const modData = util.getData("https://api.modrinth.com/api/v2/project/" + slug + "/version");

    for (v in modData) {
        const gameVersions = v.game_versions;
        for (version in gameVersions) {
            if (util.gameVersionEquals(version, gameVersion)) {
                return await downloadMod(v, path);
            }
        }
    }

    throw new Error(```Could not find compatible mod version (slug: ${slug}, gameVersion: ${gameVersion}, path: ${path})```);
}

export async function downloadModFromVersionId(versionId, filePath) {
    return downloadMod(util.getData("https://api.modrinth.com/api/v2/version" + versionId), filePath);
}


async function downloadMod(modrinthVersion, path) {
    const url = modrinthVersion.files.find((file) => file.primary)?.url;

    if (!url) {
        throw new Error(```No URL found (modrinthVersion: ${modrinthVersion}, path: ${path})```);
    }

    const file = path.join(path, modrinthVersion.file_name);
    
    if (fsExtra.exists(file)) {
        return;
    }

    fs.writeFile(file, await util.getData(url, { responseType: 3 }));
}