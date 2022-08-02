import Launcher from "./launcher/launcher.mjs";
import * as config from "./config/config.mjs";

document.addEventListener('load', () => {
    config.load();
    config.save();
})

const playButton = document.querySelector(".play");
const launcher = new Launcher();

playButton.onclick = async() => {
    try {
        await launcher.launch({ "version": "1.8.9" });
    } catch (error) {
        // TODO better error handling
        console.error(error);
    }
};

const settingsButton = document.querySelector(".open_settings");
const main = document.querySelector(".main");
const settings = document.querySelector(".settings");

settingsButton.onclick = () => {
    try {
        main.style.visibility = "hidden";
        settings.style.visibility = "visible";
    } catch (error) {
        console.error(error);
    }
}

const exitButton = document.querySelector(".close_settings");

exitButton.onclick = () => {
    try {
        main.style.visibility = "visible";
        settings.style.visibility = "hidden";
    } catch (error) {
        console.error(error);
    }
}