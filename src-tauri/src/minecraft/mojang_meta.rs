use anyhow::Result;
use reqwest::Client;
use serde_json::Value;

use crate::util::{download_file, request_file, DataDir};

pub(crate) type ClassPath = String;

pub(crate) async fn get_minecraft(
    version: &str,
    data_dir: &DataDir,
    client: &Client,
) -> Result<ClassPath> {
    let resp =
        request_file("https://launchermeta.mojang.com/mc/game/version_manifest.json").await?;
    let resp: Value = serde_json::from_str(&resp)?;

    let mut class_path: ClassPath = String::new();

    let mut url = "";
    for i in resp["versions"].as_array().unwrap() {
        if i["id"].as_str().unwrap() == version {
            url = i["url"].as_str().unwrap();
        }
    }

    let resp = request_file(url).await?;
    let resp: Value = serde_json::from_str(&resp)?;

    let url = resp["downloads"]["client"]["url"].as_str().unwrap();

    dbg!(url);
    // let mut file_path = data_dir.get_version_dir(version);
    // file_path.push(format!("libraries/com/mojang/minecraft/{}/", version));
    // fs::create_dir_all(&file_path)?;
    // file_path.push("client.jar");

    println!("Downloading Minecraft Client");

    let file_path =
        data_dir.get_library_dir(&format!("com/mojang/minecraft/{}/client.jar", version))?;
    if !file_path.try_exists()? {
        download_file(
            url,
            Some(resp["downloads"]["client"]["sha1"].as_str().unwrap()),
            &file_path,
            Some(&client),
        )
        .await?;
    }
    class_path.push_str(&file_path.canonicalize()?.display().to_string());
    println!("Downloading Minecraft libraries");

    // Get libraries
    for i in resp["libraries"].as_array().unwrap() {
        let url = i["downloads"]["artifact"]["url"].as_str().unwrap();
        let path =
            data_dir.get_library_dir(i["downloads"]["artifact"]["path"].as_str().unwrap())?;

        if !path.try_exists()? {
            download_file(
                url,
                Some(i["downloads"]["artifact"]["sha1"].as_str().unwrap()),
                &path,
                Some(&client),
            )
            .await?;
        }
        class_path.push_str(":");
        class_path.push_str(&path.canonicalize()?.display().to_string());
    }

    // Get assets
    println!("Downloading Minecraft assets");

    let url = resp["assetIndex"]["url"].as_str().unwrap();
    let resp = request_file(url).await?;
    let resp: Value = serde_json::from_str(&resp)?;

    let path = data_dir.get_asset_index_dir(version)?;
    if !path.try_exists()? {
        download_file(url, None, &path, Some(&client)).await?;
    }
    // todo try join simulatneously.

    for i in resp["objects"].as_object().iter() {
        for j in i.values() {
            let hash = &j["hash"].as_str().unwrap();
            let path = data_dir.get_asset_dir(hash)?;
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
    dbg!(&class_path);
    Ok(class_path)
}
