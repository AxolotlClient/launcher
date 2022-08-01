import * as util from "../util.mjs";
import * as fsExtra from "tauri-plugin-fs-extra-api";
import { fs, path } from "@tauri-apps/api";
import { http } from "@tauri-apps/api";
import { version } from "vite";

export async function downloadMod(slug, gameVersion, path) {
    let modData = util.getData("https://api.modrinth.com/api/v2/project/" + slug + "/version");

    let mod_version;
    for (v in modData) {
        mc_versions = v.game_versions;
        for (version in mc_versions) {
            if (util.gameVersionEquals(version, gameVersion)) {
                console.log("Found compatible mod version!")
                mod_version = v;
                break;
            }
        }
    }
    if (mod_version == undefined) {
        console.error("Compatible version could not be found!");
        return;
    }

    let url;
    for (file in mod_version.files) {
        if (file.primary == true) {
            url = file.url;
        }
    }
    if (url == undefined) {
        console.error("Somehow a version without an url existed!?");
        return;
    }

    file = path.join(path, mod_version.file_name);
    if (!fsExtra.exists(file)) {
        data = await http.fetch(url);
        fs.writeFile(file, data);
        console.log("Downloaded Mod " + mod_version + " to file " + file);
    } else {
        console.log("Mod already exists on disk. Nothing has been downloaded.");
    }
}