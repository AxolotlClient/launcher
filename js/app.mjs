import { invoke } from "@tauri-apps/api";

const playButton = document.querySelector(".play");

playButton.onclick = async() => {
    invoke('launch')
};

const settingsButton = document.querySelector(".open_settings");
const main = document.querySelector(".main");
const settings = document.querySelector(".settings");
const bg = document.querySelector(".bg_image")

settingsButton.onclick = () => {
    main.style.display = "none";
    settings.style.display = "block";
    //bg.style.filter = "blur(12px)"; can't really decide on whether to use this or not
};

const exitButton = document.querySelector(".close_settings");

exitButton.onclick = () => {
    main.style.display = "block";
    settings.style.display = "none";
    //bg.style.filter = "blur(0px)"; can't really decide on whether to use this or not
};
