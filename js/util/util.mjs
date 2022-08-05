import { http, tauri } from "@tauri-apps/api";

export async function getData(url, options) {
    const result = await http.fetch(url, options);

    if (result.ok) {
        return result.data;
    }

    throw new Error("Got status code " + result.status);
}

export function gameVersionEquals(version, version2) {
    if (version.length === version2.length) {
        let versionArr = version.split(".");
        let version2Arr = version2.split(".");
        for (i = 0; i < versionArr.length; i++) {
            if (versionArr[i] != version2Arr[i]) {
                return false;
            }
        }
        return true;
    }
    return false;
}

export function gameVersionAtLeast(version, version2) {
    let versionArr = version.split(".");
    let version2Arr = version2.split(".");
    for (let i = 0; i < versionArr.length; i++) {
        if (versionArr[i] < version2Arr[i]) {
            return true;
        }
    }
    return false;
}

export async function downloadFile(url, file) {
    return tauri.invoke("download_file", { url: url, file: file });
}

export async function computeSha1(file) {
    return tauri.invoke("compute_sha1", { file: file });
}

export async function spawn(cmd, args) {
    return tauri.invoke("spawn_program", { program: cmd, args: args });
}