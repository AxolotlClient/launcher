use std::{fs, path::PathBuf};

use anyhow::{bail, Result};
use reqwest::Client;
use semver::VersionReq;
use serde_json::Value;

use crate::util::{download_file, extract_file, is_dir_empty, DataDir};

pub(crate) async fn install_modpack(
    slug: &str,
    mc_version: &str,
    data_dir: &DataDir,
    client: &Client,
) -> Result<()> {
    let modpack_dir = data_dir.get_instance_mrpack_dir(slug, mc_version);
    let minecraft_dir = data_dir.get_instance_minecraft_dir(slug, mc_version);

    let mut index_json = modpack_dir.clone();
    index_json.push("modrinth.index.json");
    let f = fs::read_to_string(index_json)?;
    let index_json: Value = serde_json::from_str(&f)?;

    // Download mods
    for file in index_json["files"].as_array().unwrap() {
        download_file(
            &file["downloads"][0].as_str().unwrap(),
            Some(&file["hashes"]["sha1"].as_str().unwrap()),
            &data_dir.get_mod_dir(slug, mc_version, file["path"].as_str().unwrap()),
            Some(client),
        )
        .await?;
    }

    if !&index_json["dependencies"]["fabric-loader"].is_null() {
        let mc_version =
            semver::Version::parse(&index_json["dependencies"]["minecraft"].as_str().unwrap())?;

        let fabric_req = VersionReq::parse(">=1.14")?;
        let legacy_fabric_req = VersionReq::parse("<=1.13.2, >=1.3")?;
        let cts_req = VersionReq::parse("1.16_combat-6")?;

        // todo: support https://minecraft-cursed-legacy.github.io/
        let maven = match mc_version {
            x if fabric_req.matches(&x) => "https://maven.fabricmc.net/",
            x if legacy_fabric_req.matches(&x) => "https://maven.legacyfabric.net/",
            x if cts_req.matches(&x) => "https://maven.combatreforged.com/",
            _ => bail!("Unsupported!"),
        };
    }

    todo!("Install modpack");
}

pub(crate) async fn get_modpack(
    slug: &str,
    mc_version: &str,
    data_dir: &DataDir,
    client: &Client,
) -> Result<()> {
    if is_dir_empty(&data_dir.get_instance_mrpack_dir(slug, mc_version))? {
        download_mrpack(
            slug,
            mc_version,
            &data_dir.get_instance_mrpack_dir(slug, mc_version),
            client,
        )
        .await?;
    }

    // Install modpack to .minecraft

    Ok(())
}

async fn download_mrpack(
    slug: &str,
    mc_version: &str,
    instance: &PathBuf,
    client: &Client,
) -> Result<()> {
    // Get versions
    let modrinth = ferinth::Ferinth::default();
    let versions = modrinth
        .list_versions_filtered(slug, Some(&["quilt", "fabric"]), Some(&[mc_version]), None)
        .await
        .unwrap();

    // Get version with latest timestamp
    let version = versions
        .iter()
        .max_by(|x, y| {
            x.date_published
                .timestamp()
                .cmp(&y.date_published.timestamp())
        })
        .unwrap();

    let mut file = instance.clone();
    file.push("pack.zip");
    let file = file;

    dbg!(&version.files[0].url);
    println!("Downloading Pack {}", slug);
    // Download file
    download_file(
        &version.files[0].url.as_str(),
        Some(&version.files[0].hashes.sha1),
        &file,
        Some(client),
    )
    .await?;

    println!("Extracting Pack {}", slug);
    extract_file(&file, &instance).await?;

    fs::remove_file(file)?;
    Ok(())
}
