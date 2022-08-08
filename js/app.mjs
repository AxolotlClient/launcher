import Launcher from "./launcher/launcher.mjs";
import * as config from "./config/config.mjs";

await config.load();
await config.save();

const playButton = document.querySelector(".play");
const launcher = new Launcher();

playButton.onclick = async() => {
    try {
        await launcher.launch({ version: "1.19.2" });
    } catch (error) {
        // TODO better error handling
        console.error(error);
    }
};

const settingsButton = document.querySelector(".open_settings");
const main = document.querySelector(".main");
const settings = document.querySelector(".settings");

settingsButton.onclick = () => {
    main.style.display = "none";
    settings.style.display = "block";
};

const exitButton = document.querySelector(".close_settings");

exitButton.onclick = () => {
    main.style.display = "block";
    settings.style.display = "none";
};
