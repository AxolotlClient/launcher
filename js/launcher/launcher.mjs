import { fs, path } from "@tauri-apps/api";
import { DOT_MINECRAFT } from "../paths.mjs";
import Manifest from "./manifest.mjs";

export default class Launcher {

    /**
     * Prepares for launching the game.
     * @returns generated arguments
     */
    async setup(options) {
        if(!options.version) throw new Error("No version specified");

        const versionFolder = await path.join(DOT_MINECRAFT, "versions", options.version);

        await fs.createDir(versionFolder, { recursive: true });

        return [  ];
    }

    async launch(options) {
        const args = await this.setup(options);
    }

}