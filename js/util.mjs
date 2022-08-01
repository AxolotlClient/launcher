import { http } from "@tauri-apps/api";

export async function getData(url) {
    const result = await http.fetch(url);

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