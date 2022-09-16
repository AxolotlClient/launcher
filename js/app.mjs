import { invoke } from "@tauri-apps/api";


const playButton = document.querySelector(".play");

playButton.onclick = async() => {
    invoke('launch')
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
