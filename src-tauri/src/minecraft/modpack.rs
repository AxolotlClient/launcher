use std::{fs, path::PathBuf};

use anyhow::{bail, Result};
use reqwest::Client;
use semver::VersionReq;
use serde_json::Value;

use crate::util::{download_file, extract_file, is_dir_empty, request_file, DataDir};

use super::{launcher::MinecraftLaunch, mojang_meta::ClassPath};

pub(crate) type ModPath = String;
pub(crate) async fn install_modpack(
    slug: &str,
    mc_version: &str,
    data_dir: &DataDir,
    client: &Client,
    mcl: &mut MinecraftLaunch,
) -> Result<()> {
    // todo: improve modularity. take mrpack path as argument.
    // extract mrpack in temp & ditch it afterwards.
    // leave mrpack file alone
    let mut mod_path: ModPath = String::new();

    let modpack_dir = data_dir.get_instance_mrpack_dir(slug, mc_version);
    let minecraft_dir = data_dir.get_instance_minecraft_dir(slug, mc_version);

    let mut index_json = modpack_dir.clone();
    index_json.push("modrinth.index.json");
    let f = fs::read_to_string(index_json)?;
    let index_json: Value = serde_json::from_str(&f)?;

    // Download mods
    for file in index_json["files"].as_array().unwrap() {
        let path = data_dir.get_mod_dir(slug, mc_version, file["path"].as_str().unwrap());
        if !path.try_exists()? {
            download_file(
                &file["downloads"][0].as_str().unwrap(),
                Some(&file["hashes"]["sha1"].as_str().unwrap()),
                &path,
                Some(client),
            )
            .await?;
        }
        mcl.add_mod(&path);
    }

    let mut override_dir = modpack_dir;
    override_dir.push("overrides/");

    // Apply overrides
    dircpy::copy_dir(&override_dir, &minecraft_dir)?;

    // Install fabric
    if !&index_json["dependencies"]["fabric-loader"].is_null() {
        // install net.fabricmc.fabric-loader. construct classpath
        let fabric_version = &index_json["dependencies"]["fabric-loader"]
            .as_str()
            .unwrap();
        let lib_path = maven_download(
            "https://maven.fabricmc.net/",
            "net.fabricmc",
            "fabric-loader",
            &fabric_version,
            data_dir,
            client,
            None,
        )
        .await?;
        mcl.add_class(&lib_path);

        // get json. get libraries from json & install them. construct classpath
        let url = format!(
            "https://maven.fabricmc.net/net/fabricmc/fabric-loader/{}/fabric-loader-{}.json",
            fabric_version, fabric_version
        );
        let resp = request_file(&url).await?; // woo i love json parsing. i could do it all day because im so filled with joy
        let resp: Value = serde_json::from_str(&resp)?;

        mcl.main_class = resp["mainClass"]["client"].as_str().unwrap().to_owned();
        for i in resp["libraries"]["common"].as_array().unwrap() {
            dbg!(&i);
            let mut name = i["name"].as_str().unwrap().split(":");
            let lib_path = maven_download(
                i["url"].as_str().unwrap(),
                name.next().unwrap(),
                name.next().unwrap(),
                name.next().unwrap(),
                data_dir,
                client,
                None,
            )
            .await?;
            mcl.add_class(&lib_path);
        }

        // find intermediary maven & download correct intermediary version
        let version_string = &index_json["dependencies"]["minecraft"].as_str().unwrap();
        let mc_version = semver::Version::parse(&version_string.replace("_", "+"))?;

        let fabric_req = VersionReq::parse(">=1.14")?;
        let legacy_fabric_req = VersionReq::parse("<=1.13.2, >=1.3")?;
        let cts_req = version_string == &"1.16_combat-6";

        // todo: support https://minecraft-cursed-legacy.github.io/
        let intermediary_maven = match &mc_version {
            x if fabric_req.matches(&x) => "https://maven.fabricmc.net/",
            x if legacy_fabric_req.matches(&x) => "https://maven.legacyfabric.net/",
            x if cts_req => "https://maven.combatreforged.com/",
            _ => bail!("Unsupported!"),
        };
        let identifier = match &mc_version {
            x if legacy_fabric_req.matches(&x) => "net.legacyfabric",
            x => "net.fabricmc",
        };
        let path = maven_download(
            intermediary_maven,
            identifier,
            "intermediary",
            version_string,
            data_dir,
            client,
            Some(&"v2"),
        )
        .await?;
        mcl.add_class(&lib_path);
    } else if !&index_json["dependencies"]["quilt-loader"].is_null() {
        // install org.quiltmc.quilt-loader. construct classpath
        let quilt_version = &index_json["dependencies"]["quilt-loader"].as_str().unwrap();
        let lib_path = maven_download(
            "https://maven.quiltmc.org/repository/release/",
            "org.quiltmc",
            "quilt-loader",
            &quilt_version,
            data_dir,
            client,
            None,
        )
        .await?;
        mcl.add_class(&lib_path);

        // get json. get libraries from json & install them. construct classpath
        let url = format!("https://maven.quiltmc.org/repository/release/org/quiltmc/quilt-loader/{}/quilt-loader-{}.json",
            quilt_version, quilt_version
        );
        let resp = request_file(&url).await?;
        let resp: Value = serde_json::from_str(&resp)?;

        mcl.main_class = resp["mainClass"]["client"].as_str().unwrap().to_owned();
        for i in resp["libraries"]["common"].as_array().unwrap() {
            dbg!(&i);
            let mut name = i["name"].as_str().unwrap().split(":");
            let lib_path = maven_download(
                i["url"].as_str().unwrap(),
                name.next().unwrap(),
                name.next().unwrap(),
                name.next().unwrap(),
                data_dir,
                client,
                None,
            )
            .await?;
            mcl.add_class(&lib_path);
        }

        // find intermediary maven & download correct intermediary version
        let version_string = &index_json["dependencies"]["minecraft"].as_str().unwrap();
        let mc_version = semver::Version::parse(&version_string.replace("_", "+"))?;

        let intermediary_maven = "https://maven.fabricmc.net/";
        let identifier = "net.fabricmc";
        let path = maven_download(
            intermediary_maven,
            identifier,
            "intermediary",
            version_string,
            data_dir,
            client,
            Some(&"v2"),
        )
        .await?;
        mcl.add_class(&path);
    }

    Ok(())
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
async fn maven_download(
    maven_url: &str,
    identifier: &str,
    name: &str,
    version: &str,
    data_dir: &DataDir,
    client: &Client,
    suffix: Option<&str>,
) -> Result<PathBuf> {
    dbg!(identifier);

    let path_suffix = match suffix {
        Some(s) => {
            format!(
                "{}/{name}/{version}/{name}-{version}-{s}.jar",
                &identifier.replace(".", "/"),
            )
        }
        None => {
            format!(
                "{}/{name}/{version}/{name}-{version}.jar",
                &identifier.replace(".", "/"),
            )
        }
    };

    let url = maven_url.to_owned() + &path_suffix;
    let path = data_dir.get_library_dir(&path_suffix)?;
    if !path.try_exists()? {
        download_file(&url, None, &path, Some(&client)).await?;
    }
    Ok(path)
}
