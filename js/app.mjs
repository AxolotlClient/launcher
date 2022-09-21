import { invoke } from "@tauri-apps/api";

const playButton = document.querySelector(".play");

playButton.onclick = async() => {
    invoke("launch");
};

document.getElementById("avatar").src = "https://crafatar.com/avatars/[uuid].png";

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
    if(currentInstanceButton.querySelector("p").innerText !== old){
        console.log("Set instance to: "+currentInstanceButton.innerText)
        // TODO put in a tauri event to get this to the backend.
        // I don't want to touch the rust code because I am afraid of breaking stuff
    }
});

const accountSettings = document.querySelector(".account_settings");
const currentAccountButton = document.querySelector(".current_account");
let accountMenuOpen = false;

accountSettings.querySelectorAll("button").forEach((button) => button.onclick = () => {
    accountSettings.classList.toggle("extended");
    accountMenuOpen = !accountMenuOpen;
    document.querySelector(".current_account").querySelectorAll("img").forEach((img) => {
        if(img.src.includes("arrow_up.svg")) {
            img.style.transform = accountMenuOpen ? "scaleY(-1)" : "scaleY(1)";
        }
    })
});

document.body.addEventListener("mouseup", (event) => {
    if(!accountSettings.contains(event.target) && accountSettings.classList.contains("extended")) {
        currentAccountButton.click();
    }

    if(!instance.contains(event.target) && instance.classList.contains("extended")) {
        currentInstanceButton.click();
    }
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
