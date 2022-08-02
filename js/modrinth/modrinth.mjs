import * as util from "../util.mjs";
import * as fsExtra from "tauri-plugin-fs-extra-api";
import { fs, path } from "@tauri-apps/api";
import { http } from "@tauri-apps/api";

export async function downloadModFromSlug(slug, gameVersion, path) {
    let modData = util.getData("https://api.modrinth.com/api/v2/project/" + slug + "/version");

    for (v in modData) {
        let mc_versions = v.game_versions;
        for (version in mc_versions) {
            if (util.gameVersionEquals(version, gameVersion)) {
                console.log("Found compatible mod version!")
                return downloadMod(v, path);
            }
        }
    }

    console.error("Compatible version could not be found!");
}

export async function downloadModFromVersionId(versionId, path) {
    return downloadMod(util.getData("https://api.modrinth.com/api/v2/version" + versionId), path);
}


async function downloadMod(modrinthVersion, path) {
    let url;
    for (file in modrinthVersion.files) {
        if (file.primary == true) {
            url = file.url;
        }
    }
    if (url == undefined) {
        console.error("Somehow a version without an attached file existed?\n Mod authors should not do this!");
        return;
    }

    let file = path.join(path, modrinthVersion.file_name);
    if (!fsExtra.exists(file)) {
        let data = await http.fetch(url);
        fs.writeFile(file, data);
        console.log("Downloaded Mod " + modrinthVersion + " to file " + file);
    } else {
        console.log("Mod already exists on disk. Nothing has been downloaded.");
    }
}