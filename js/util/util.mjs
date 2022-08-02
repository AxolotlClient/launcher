import { http, tauri } from "@tauri-apps/api";

export async function getData(url, options) {
    const result = await http.fetch(url, options);

    if (result.ok) {
        return result.data;
    }

    throw new Error("Got status code " + result.status);
}

export function gameVersionEquals(version, version2) {
    if (version.split(".", 1)[0] === version2.split(".", 1)[0] &&
        version.split(".", 1)[1] === version2.split(".", 1)[1]) {
        return true;
    }
    return false;
}

// from MDN
export function hex(buffer) {
    return Array.from(new Uint8Array(buffer))
            .map((byte) => byte.toString(16).padStart(2, "0"))
            .join("");
}

export async function downloadFile(url, file) {
    return tauri.invoke("download_file", { url: url, file: file });
}

export async function computeSha1(file) {
    return tauri.invoke("compute_sha1", { file: file });
}