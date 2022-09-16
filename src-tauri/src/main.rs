#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub(crate) mod config;
pub(crate) mod minecraft;
pub(crate) mod util;

use anyhow::Result;
use tauri_plugin_fs_extra::FsExtra;

macro_rules! str_err {
    ($res:expr) => {
        $res.map_err(|err| err.to_string())
    };
}

#[tauri::command]
async fn launch() -> Result<(), String> {
    // Read config
    let config = config::load();

    // Launch the game
    minecraft::launcher::launch(config).await.unwrap();

    Ok(())
}

// next two functions: adapted from "Rust Cookbook" - I don't know rust

// #[tauri::command]
// async fn compute_sha1(file: String) -> Result<String, String> {
//     let input = str_err!(File::open(file))?;
//     let mut reader = BufReader::new(input);
//     let mut context = Context::new(&SHA1_FOR_LEGACY_USE_ONLY);
//     let mut buffer = [0; 1024];
//
//     loop {
//         let count = str_err!(reader.read(&mut buffer))?;
//         if count == 0 {
//             break;
//         }
//         context.update(&buffer[..count]);
//     }
//
//     Ok(HEXLOWER.encode(context.finish().as_ref()))
// }

#[tauri::command]

fn main() {
    tauri::Builder::default()
        .plugin(FsExtra::default())
        .invoke_handler(tauri::generate_handler![launch])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
