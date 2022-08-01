import { http } from "@tauri-apps/api";

export async function getData(url) {
    const result = await http.fetch(url);

    if (result.ok) {
        return result.data;
    }

    throw new Error("Got status code " + result.status);
}