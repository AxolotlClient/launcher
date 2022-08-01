import Launcher from "./launcher/launcher.mjs";

const playButton = document.querySelector(".play");

playButton.onclick = async() => {
    try {
        let l = new Launcher();
        await l.launch("1.8.9");
    } catch (error) {
        // TODO better error handling
        console.error(error);
    }
};