import { http, tauri } from "@tauri-apps/api";
import { Command } from "@tauri-apps/api/shell";

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
    let versionArr = version.split(".").map(Number);
    let version2Arr = version2.split(".").map(Number);
    versionArr[2] ??= 0;
    version2Arr[2] ??= 0;

    let result = true;

    for (let i = 0; i < versionArr.length; i++) {
        result &&= versionArr[i] >= version2Arr[i];
    }
    
    return result;
}

export async function downloadFile(url, file) {
    return tauri.invoke("download_file", { url: url, file: file });
}

export async function computeSha1(file) {
    return tauri.invoke("compute_sha1", { file: file });
}

// If something here needs to be changed feel free to
export async function spawn(cmd, args, options = "") {
    const command = new Command(cmd, args, options);
    command.on('close', data => {
        console.log(cmd + ` finished with code ${data.code} and signal ${data.signal}`);
    });
    command.on('error', error => console.error(`"${cmd}" error: "${error}"`));
    command.stdout.on('data', line => console.log(`"${cmd}": "${line}"`));
    command.stderr.on('data', line => console.log(`"${cmd}" stderr: "${line}"`));

    return await command.spawn();
}

export async function extractArchive(archive, target) {
    return tauri.invoke("extract_file", { archive: archive, target_dir: target });
}
