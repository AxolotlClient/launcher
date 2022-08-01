import { http } from "@tauri-apps/api";
const DEFAULT_URL = "https://launchermeta.mojang.com/mc/game/version_manifest.json";

export default class Manifest {

    data;

    constructor(data) {
        this.data = data;
    }

    static async fetch(url) {
        url ??= DEFAULT_URL;

        const result = await http.fetch(url);

        if (result.ok) {
            return result.data;
        }

        throw new Error("Got status code " + result.status);
    }

}