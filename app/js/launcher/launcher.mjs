import "../paths.mjs";
import Manifest from "./manifest.mjs";

export default class Launcher {

    /**
     * Prepares for launching the game.
     * @returns generated arguments
     */
    async setup(version) {
        let launchManifest = await Manifest.fetch();

        console.log(launchManifest);

        return [  ];
    }

    async launch(version) {
        const args = await this.setup(version);
    }

}