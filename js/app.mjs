import { invoke } from "@tauri-apps/api";

const playButton = document.querySelector(".play");

playButton.onclick = async() => {
    invoke("launch");
};

const instance = document.querySelector(".instance");
const currentInstanceButton = document.querySelector(".current_instance");
let instanceMenuOpen = false;

instance.querySelectorAll("button").forEach((button) => button.onclick = () => {
    instance.classList.toggle("extended");
    instanceMenuOpen = !instanceMenuOpen;
    currentInstanceButton.querySelector("img").style.transform = instanceMenuOpen ? "scaleY(-1)" : "scaleY(1)";
    const old = currentInstanceButton.innerText;
    currentInstanceButton.querySelector("p").innerText = button.innerText;
    button.querySelector("p").innerText = old;
});

const settingsButton = document.querySelector(".open_settings");
const main = document.querySelector(".main");
const settings = document.querySelector(".settings");

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
