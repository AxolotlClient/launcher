#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use data_encoding::HEXLOWER;
use ring::digest::{Context, SHA1_FOR_LEGACY_USE_ONLY};
use std::{io::{BufReader, Read, Write, Cursor}, fs::File, path::PathBuf};
use tauri_plugin_fs_extra::FsExtra;

macro_rules! str_err {
    ($res:expr) => {
        $res.map_err(|err| err.to_string())
    };
}

// next two functions: adapted from "Rust Cookbook" - I don't know rust

#[tauri::command]
async fn download_file(url: String, file: String) -> Result<(), String> {
    let target = url;
    let response = str_err!(reqwest::get(target).await)?;
    let mut dest = str_err!(File::create(file))?;
    let content = str_err!(response.bytes().await)?;
    str_err!(dest.write_all(&content))?;
    Ok(())
}

#[tauri::command]
async fn compute_sha1(file: String) -> Result<String, String> {
    let input = str_err!(File::open(file))?;
    let mut reader = BufReader::new(input);
    let mut context = Context::new(&SHA1_FOR_LEGACY_USE_ONLY);
    let mut buffer = [0; 1024];

    loop {
        let count = str_err!(reader.read(&mut buffer))?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(HEXLOWER.encode(context.finish().as_ref()))
}

#[tauri::command]
async fn extract_file(archive: String, target_dir: String) -> Result<(), String> {
    match zip_extract::extract(Cursor::new(&archive), &PathBuf::from(&target_dir), false) {
        Ok(()) => return Ok(()),
        Err(error) => return Err("Could not extract Zip file: ".to_owned()
        +&archive + " to directory "
        +&target_dir + " with Error "
        +&error.to_string()),
    };
}

fn main() {
    tauri::Builder::default()
        .plugin(FsExtra::default())
        .invoke_handler(tauri::generate_handler![download_file, compute_sha1, extract_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
