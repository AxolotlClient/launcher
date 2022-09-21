#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub(crate) mod config;
pub(crate) mod minecraft;
pub(crate) mod util;

use crate::config::get_instance_config;
use crate::config::{set_instance_config, InstanceConfig};
use anyhow::Result;
use minecraft::{
    launcher::launch_minecraft,
    modpack::{fetch_mrpack, install_mrpack},
};
use reqwest::Client;
use tauri_plugin_fs_extra::FsExtra;

#[tauri::command]
async fn launch(instance_slug: &str) -> Result<(), String> {
    // Read config

    match get_instance_config(instance_slug).await {
        Ok(_) => launch_minecraft(instance_slug).await.unwrap(),
        Err(_) => {
            return Err("Instance has not been installed properly, or does not exist".to_string())
        }
    };

    Ok(())
}

#[tauri::command]
async fn install_modrinth_pack(name: &str, version_id: &str) -> Result<(), String> {
    // instance slug must be validated, with no special characters
    let instance_slug: String = name
        .trim()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect();

    let c = Client::builder().build().unwrap();

    let (mrpack_path, modrinth) = fetch_mrpack(&version_id, &c).await.unwrap();

    let launch = install_mrpack(&instance_slug, &mrpack_path, &c)
        .await
        .unwrap();

    set_instance_config(
        &InstanceConfig {
            name: name.to_string(),
            remote: "modrinth".to_owned(),
            modrinth: Some(modrinth),
            local_file: None,
            launch,
        },
        &instance_slug,
    )
    .await
    .unwrap();
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

fn main() {
    // check if default packs are installed
    // if not install them

    tauri::Builder::default()
        .plugin(FsExtra::default())
        .invoke_handler(tauri::generate_handler![launch, install_modrinth_pack])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
