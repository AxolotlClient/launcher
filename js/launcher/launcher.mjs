import { fs, path } from "@tauri-apps/api";
import * as fsExtra from "tauri-plugin-fs-extra-api";
import * as paths from "../paths.mjs";
import { Manifest, Version } from "./manifest.mjs";

export default class Launcher {

    /**
     * Prepares for launching the game.
     * @returns generated arguments
     */
    async setup(options) {
        if (!options.version) {
            throw new Error("No version specified");
        }

        // lazy manifest variable
        let manifest;

        const versionFolder = await path.join(paths.DOT_MINECRAFT, "versions", options.version);

        if (!fsExtra.exists(versionFolder)) {
            await fs.createDir(versionFolder, { recursive: true });
        }

        const versionJson = await path.join(versionFolder, options.version + ".json");

        let version;
        if (!await fsExtra.exists(versionJson)) {
            manifest ??= await Manifest.fetch();
            version = await manifest.findVersion(options.version).fetch();
            console.log(JSON.stringify(version.data));
            await fs.writeTextFile(versionJson, JSON.stringify(version.data));
        } else {
            version = new Version(JSON.parse(await fs.readTextFile(versionJson)));
        }

        return [  ];
    }

    async launch(options) {
        const args = await this.setup(options);
    }

}