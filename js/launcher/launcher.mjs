import { fs, path, shell } from "@tauri-apps/api";
import * as fsExtra from "tauri-plugin-fs-extra-api";
import * as paths from "../util/paths.mjs";
import * as config from "../config/config.mjs";
import { Manifest, Version } from "./mojang_meta.mjs";
import * as util from "../util/util.mjs";

export default class Launcher {

    /**
     * Prepares for launching the game.
     * @param options The launch options.
     * @param callback The callback.
     * @returns generated arguments
     */
    async setup(options, callback) {
        if (!options.version) {
            throw new Error("No version specified");
        }

        const versionFolder = await path.join(paths.DOT_MINECRAFT, "versions", options.version);

        if (!await fsExtra.exists(versionFolder)) {
            await fs.createDir(versionFolder, { recursive: true });
        }

        const versionJson = await path.join(versionFolder, options.version + ".json");
        const versionJar = await path.join(versionFolder, options.version + ".jar");

        callback("Loading version data...");

        let version;
        if (!await fsExtra.exists(versionJson)) {
            const manifest = await Manifest.fetch();
            version = await manifest.findVersion(options.version).fetch();
            await fs.writeTextFile(versionJson, JSON.stringify(version.data));
        } else {
            version = new Version(JSON.parse(await fs.readTextFile(versionJson)));
        }

        const classpath = [ ];

        callback("Downloading client...");

        await version.getClient().download(versionJar);
        classpath.push(versionJar);

        callback("Downloading libraries...");

        for (const library of version.getLibraries()) {
            if (!await library.isApplicable()) {
                continue;
            }

            const artifact = library.getDownloads().artifact;

            const libraryPath = await path.join(paths.LIBRARIES, artifact.getPath());
            const libraryFolder = await path.dirname(libraryPath);

            if (!await fsExtra.exists(libraryFolder)) {
                await fs.createDir(libraryFolder, { recursive: true });
            }

            await artifact.download(libraryPath);

            classpath.push(libraryPath);
        };

        const jre = util.gameVersionAtLeast(options.version, "1.17") ? config.getJRE17() : config.getJRE8();

        return [
            jre,
            "-cp", classpath.join(path.delimiter),
            version.getMainClass(),
            "--accessToken", "0",
            "--username", "Test",
            "--assetsDir", paths.ASSETS,
            "--assetIndex", version.getAssetIndex().getId(),
            "--version", "AxolotlClient-" + options.version
        ];
    }

    /**
     * Launches the game.
     * @param options The launch options.
     * @param callback function(stage, min, max): accepts progress.
     * If min and max are ommited, the progress is indeterminate.
     * If no arguments are provided, the launch is complete.
     */
    async launch(options, callback) {
        callback ??= (stage, min, max) => {
            if(!stage && !min && !max) {
                console.log("Progress () - done");
                return;
            }

            console.log(`Progress (${stage}, ${min && max ? `${min}, ${max}` : "indeterminate"})`);
        };

        const args = await this.setup(options, callback);
        callback("Spawning...");
        await util.spawn(args[0], args.slice(1));
        callback();
    }

}
