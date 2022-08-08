import { fs, os } from "@tauri-apps/api";
import * as fsExtra from "tauri-plugin-fs-extra-api";
import * as util from "../util/util.mjs";
const DEFAULT_URL = "https://launchermeta.mojang.com/mc/game/version_manifest.json";

async function getOsName() {
    switch (await os.type()) {
        case "Linux":
            return "linux";
        case "Darwin":
            return "osx";
        case "Windows_NT":
            return "windows";
    }
}

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

    getLibraries() {
        return this.data.libraries.map((data) => new Library(data));
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

export class Library {

    data;

    constructor(data) {
        this.data = data;
    }

    getName() {
        return this.data.name;
    }

    getDownloads() {
        const res = {};

        for(const key in this.data.downloads) {
            const value = this.data.downloads[key];
            res[key] = new Download(value);
        }

        return res;
    }

    // from fabric-loom
    async isApplicable() {
        if (!this.data.rules) {
            return true;
        }
        
        let result = false;

        for (const rule of this.data.rules) {
            if (rule.os) {
                if (rule.os.name === await getOsName()) {
                    return rule.action === "allow";
                }
            } else {
                result = rule.action === "allow";
            }
        }

        return result;
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

    getPath() {
        return this.data.path;
    }

    async download(path) {
        if (await fsExtra.exists(path) && await util.computeSha1(path) === this.getSha1()) {
            return;
        }

        await util.downloadFile(this.getUrl(), path);
    }
    
}