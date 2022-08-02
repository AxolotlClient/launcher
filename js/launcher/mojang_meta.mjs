import { fs } from "@tauri-apps/api";
import * as fsExtra from "tauri-plugin-fs-extra-api";
import * as util from "../util/util.mjs";
const DEFAULT_URL = "https://launchermeta.mojang.com/mc/game/version_manifest.json";

export class Handle {

    cached;

    getUrl() {
        throw new Error("Cannot find URL of " + this);
    }

    constructReferenced(data) {
        throw new Error("Cannot construct an object refered to by " + this);
    }

    async fetch() {
        if (!this.cached) {
            return this.cached = this.constructReferenced(await util.getData(this.getUrl()));
        }

        return this.cached;
    }

}

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

export class VersionHandle extends Handle {

    data;

    constructor(data) {
        super();
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

    constructReferenced(data) {
        return new Version(data);
    }

}

export class Version {

    data;

    constructor(data) {
        this.data = data;
    }

    getMainClass() {
        return this.data.mainClass;
    }

    getAssetIndex() {
        return new AssetIndexHandle(this.data.assetIndex);
    }

    getClient() {
        return new Download(this.data.downloads.client);
    }

}

export class AssetIndexHandle extends Handle {
    
    data;

    constructor(data) {
        super();
        this.data = data;
    }

    getId() {
        return this.data.id;
    }

    getUrl() {
        return this.data.url;
    }

    constructReferenced(data) {
        return new AssetIndex(data);
    }

}

export class AssetIndex {

    data;

    constructor(data) {
        this.data = data;
    }

}

export class Download {

    data;

    constructor(data) {
        this.data = data;
    }

    getSha1() {
        return this.data.sha1;
    }

    getUrl() {
        return this.data.url;
    }

    async download(path) {
        if (await fsExtra.exists(path) && await util.computeSha1(path) === this.getSha1()) {
            return;
        }

        await util.downloadFile(this.getUrl(), path);
    }
    
}