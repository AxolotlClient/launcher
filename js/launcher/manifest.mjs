import * as util from "../util.mjs";
const DEFAULT_URL = "https://launchermeta.mojang.com/mc/game/version_manifest.json";

export class Manifest {

    data;

    constructor(data) {
        this.data = data;
    }

    static async fetch(url) {
        return new Manifest(await util.getData(url ?? DEFAULT_URL));
    }

    getLatestRelease() {
        return this.data.latest.release;
    }

    getLatestSnapshot() {
        return this.data.latest.snapshot;
    }

    findVersion(id) {
        return this.getVersions().find((version) => version.getId() == id);
    }

    getVersions() {
        return this.data.versions.map((data) => new VersionHandle(data));
    }

}

export class VersionHandle {

    data;
    cached;

    constructor(data) {
        this.data = data;
    }

    getId() {
        return this.data.id;
    }

    getType() {
        return this.data.type;
    }

    getUrl() {
        return this.data.url;
    }

    async fetch() {
        if (!this.cached) {
            return this.cached = new Version(await util.getData(this.getUrl()));
        }

        return this.cached;
    }

}

export class Version {

    data;

    constructor(data) {
        this.data = data;
    }

}