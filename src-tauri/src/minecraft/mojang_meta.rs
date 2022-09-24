use anyhow::Result;
use reqwest::Client;
use serde_json::Value;

use crate::{
    config::{Java, Minecraft},
    minecraft::{java::get_java, launcher::JarPath},
    util::{download_file, request_file, DataDir},
};

pub(crate) async fn get_minecraft(version: &str, client: &Client) -> Result<(Java, Minecraft)> {
    let mut class_path = JarPath::new();
    let main_class: String;
    let resp = request_file(
        "https://launchermeta.mojang.com/mc/game/version_manifest.json",
        &client,
    )
    .await?;
    let resp: Value = serde_json::from_str(&resp)?;

    let mut url = "";
    for i in resp["versions"].as_array().unwrap() {
        if i["id"].as_str().unwrap() == version {
            url = i["url"].as_str().unwrap();
        }
    }

    let resp = request_file(url, &client).await?;
    let resp: Value = serde_json::from_str(&resp)?;

    let java_version = resp["javaVersion"]["majorVersion"].as_i64().unwrap();
    let java = get_java(java_version, &client).await?;
    // todo: download java from HERE with the version they give you instead of trying to figure out out
    // like an idiot.
    main_class = resp["mainClass"].as_str().unwrap().to_owned();
    // todo: this too. maybe make a Meta struct and return java version, main class and class path. make class path mutable

    let url = resp["downloads"]["client"]["url"].as_str().unwrap();

    println!("Downloading Minecraft Client");

    let file_path =
        DataDir::get_library_dir(&format!("com/mojang/minecraft/{}/client.jar", version))?;
    if !file_path.try_exists()? {
        download_file(
            url,
            Some(resp["downloads"]["client"]["sha1"].as_str().unwrap()),
            &file_path,
            Some(&client),
        )
        .await?;
    }
    class_path.add_class(&file_path);
    println!("Downloading Minecraft libraries");

    // Get libraries
    for i in resp["libraries"].as_array().unwrap() {
        let url = i["downloads"]["artifact"]["url"].as_str().unwrap();
        let path = DataDir::get_library_dir(i["downloads"]["artifact"]["path"].as_str().unwrap())?;

        if !path.try_exists()? {
            download_file(
                url,
                Some(i["downloads"]["artifact"]["sha1"].as_str().unwrap()),
                &path,
                Some(&client),
            )
            .await?;
        }
        class_path.add_class(&path);
    }

    // Get assets
    println!("Downloading Minecraft assets");

    let url = resp["assetIndex"]["url"].as_str().unwrap();
    let resp = request_file(url, &client).await?;
    let resp: Value = serde_json::from_str(&resp)?;

    let path = DataDir::get_asset_index_dir(version)?;
    if !path.try_exists()? {
        download_file(url, None, &path, Some(&client)).await?;
    }
    // todo try join simulatneously.

    for i in resp["objects"].as_object().iter() {
        for j in i.values() {
            let hash = &j["hash"].as_str().unwrap();
            let path = DataDir::get_asset_dir(hash)?;
            let url = format!(
                "https://resources.download.minecraft.net/{}/{}",
                hash.split_at(2 * hash.chars().nth(0).unwrap().len_utf8()).0,
                hash
            );
            if !path.try_exists()? {
                download_file(&url, None, &path, Some(&client)).await?;
            }
        }
    }
    Ok((
        java,
        Minecraft {
            version: version.to_owned(),
            main_class,
            class_path,
        },
    ))
}
