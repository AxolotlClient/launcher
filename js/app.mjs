import Launcher from "./launcher/launcher.mjs";
/*import * as config from "./config/config.mjs"; // This is somehow broken, I have no idea how to fix it.

document.onload = async() => {
    try {
        await config.load();
        await config.save();
    } catch (error) {
        console.error(error);
    }
}*/

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
        main.style.display = "none";
        settings.style.display = "block";
        console.log("Opened Settings");
    } catch (error) {
        console.error(error);
    }
}

const exitButton = document.querySelector(".close_settings");

exitButton.onclick = () => {
    try {
        main.style.display = "block";
        settings.style.display = "none";
        console.log("Closed Settings");
    } catch (error) {
        console.error(error);
    }
}