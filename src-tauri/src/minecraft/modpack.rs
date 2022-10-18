use std::{
    env,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use anyhow::{anyhow, bail, Context, Result};
use reqwest::{Client, StatusCode};
use semver::VersionReq;
use serde_json::Value;

use crate::{
    config::{Launch, Modloader, Modrinth},
    util::{download_file, extract_file, request_file, verify_hash, DataDir},
};

use super::{launcher::JarPath, mojang_meta::get_minecraft};

pub(crate) async fn install_mrpack(
    instance_slug: &str,
    mrpack_path: &PathBuf,
    client: &Client,
) -> Result<Launch> {
    // extract mrpack into temp dir
    let temp_dir = env::temp_dir();
    fs::create_dir_all(&temp_dir)?;
    let mut mrpack_contents = temp_dir.clone();
    mrpack_contents.push("mrpack");

    extract_file(&mrpack_path, &mrpack_contents).await?;

    let mut index_json = mrpack_contents.clone();
    index_json.push("modrinth.index.json");
    let f = fs::read_to_string(index_json)?;
    let index_json: Value = serde_json::from_str(&f)?;

    dbg!("Installing modloader");
    let mut launch = install_modloader(client, &index_json)
        .await
        .context("Failed installing modloader")?;
    dbg!("Installing mods");
    install_mods(
        &instance_slug,
        client,
        &index_json,
        &mrpack_contents,
        &mut launch.modloader.mod_path,
    )
    .await
    .context("Failed installing mods")?;

    Ok(launch)
}

async fn install_modloader(client: &Client, index_json: &Value) -> Result<Launch> {
    let (java, minecraft) = get_minecraft(
        &index_json["dependencies"]["minecraft"].as_str().unwrap(),
        &client,
    )
    .await?;

    let mut class_path = JarPath::new();
    let mut main_class = String::new();
    let mut loader_type = String::from("vanilla");

    // Install fabric
    if !&index_json["dependencies"]["fabric-loader"].is_null() {
        loader_type = String::from("fabric");
        // install net.fabricmc.fabric-loader. construct classpath
        let fabric_version = &index_json["dependencies"]["fabric-loader"]
            .as_str()
            .unwrap();
        let lib_path = maven_download(
            "https://maven.fabricmc.net/",
            None,
            "net.fabricmc",
            "fabric-loader",
            &fabric_version,
            client,
            None,
        )
        .await?;
        class_path.add_class(&lib_path);

        // get json. get libraries from json & install them. construct classpath
        let url = format!(
            "https://maven.fabricmc.net/net/fabricmc/fabric-loader/{}/fabric-loader-{}.json",
            fabric_version, fabric_version
        );
        let resp = request_file(&url, &client).await?; // woo i love json parsing. i could do it all day because im so filled with joy
        let resp: Value = serde_json::from_str(&resp)?;

        main_class = resp["mainClass"]["client"].as_str().unwrap().to_owned();
        for i in resp["libraries"]["common"].as_array().unwrap() {
            dbg!(&i);
            let mut name = i["name"].as_str().unwrap().split(":");
            let lib_path = maven_download(
                i["url"].as_str().unwrap(),
                None,
                name.next().unwrap(),
                name.next().unwrap(),
                name.next().unwrap(),
                client,
                None,
            )
            .await?;
            class_path.add_class(&lib_path);
        }

        // find intermediary maven & download correct intermediary version
        let version_string = &index_json["dependencies"]["minecraft"].as_str().unwrap();
        let mc_version = semver::Version::parse(&version_string.replace("_", "+"))?;

        let fabric_req = VersionReq::parse(">=1.14")?;
        let legacy_fabric_req = VersionReq::parse("<=1.13.2, >=1.3")?;
        let cts_req = version_string == &"1.16_combat-6";

        // todo: support https://minecraft-cursed-legacy.github.io/
        let intermediary_maven = match &mc_version {
            v if fabric_req.matches(&v) => "https://maven.fabricmc.net/",
            v if legacy_fabric_req.matches(&v) => "https://maven.legacyfabric.net/",
            _ if cts_req => "https://maven.combatreforged.com/",
            _ => bail!("Unsupported!"),
        };
        let fallback_maven = "https://maven.fabricmc.net";

        let identifier = match &mc_version {
            v if legacy_fabric_req.matches(&v) => "net.legacyfabric",
            _ => "net.fabricmc",
        };
        maven_download(
            intermediary_maven,
            Some(fallback_maven),
            identifier,
            "intermediary",
            version_string,
            client,
            Some(&"v2"),
        )
        .await?;
        class_path.add_class(&lib_path);
    } else if !&index_json["dependencies"]["quilt-loader"].is_null() {
        loader_type = String::from("quilt");
        // install org.quiltmc.quilt-loader. construct classpath
        let quilt_version = &index_json["dependencies"]["quilt-loader"].as_str().unwrap();
        let lib_path = maven_download(
            "https://maven.quiltmc.org/repository/release/",
            None,
            "org.quiltmc",
            "quilt-loader",
            &quilt_version,
            client,
            None,
        )
        .await?;
        class_path.add_class(&lib_path);

        // get json. get libraries from json & install them. construct classpath
        let url = format!("https://maven.quiltmc.org/repository/release/org/quiltmc/quilt-loader/{}/quilt-loader-{}.json",
            quilt_version, quilt_version
        );
        let resp = request_file(&url, &client).await?;
        let resp: Value = serde_json::from_str(&resp)?;

        main_class = resp["mainClass"]["client"].as_str().unwrap().to_owned();
        for i in resp["libraries"]["common"].as_array().unwrap() {
            dbg!(&i);
            let mut name = i["name"].as_str().unwrap().split(":");
            let lib_path = maven_download(
                i["url"].as_str().unwrap(),
                None,
                name.next().unwrap(),
                name.next().unwrap(),
                name.next().unwrap(),
                client,
                None,
            )
            .await?;
            class_path.add_class(&lib_path);
        }

        // find intermediary maven & download correct intermediary version
        let version_string = &index_json["dependencies"]["minecraft"].as_str().unwrap();

        let intermediary_maven = "https://maven.fabricmc.net/";
        let identifier = "net.fabricmc";
        let path = maven_download(
            intermediary_maven,
            None,
            identifier,
            "intermediary",
            version_string,
            client,
            Some(&"v2"),
        )
        .await?;
        class_path.add_class(&path);
    }
    Ok(Launch {
        modloader: Modloader {
            loader_type,
            mod_path: JarPath::new(),
            main_class,
            class_path,
        },
        java,
        minecraft,
    })
}

async fn install_mods(
    instance_slug: &str,
    client: &Client,
    index_json: &Value,
    temp_dir: &PathBuf,
    mod_path: &mut JarPath,
) -> Result<()> {
    // Download mods
    for file in index_json["files"].as_array().unwrap() {
        let path = DataDir::get_mod_dir(instance_slug, file["path"].as_str().unwrap());
        if !path.try_exists()? {
            download_file(
                &file["downloads"][0].as_str().unwrap(),
                Some(&file["hashes"]["sha1"].as_str().unwrap()),
                &path,
                Some(client),
            )
            .await?;
        }
        mod_path.add_class(&path);
    }

    let mut override_dir = temp_dir.clone();
    override_dir.push("overrides/");

    // Apply overrides
    dircpy::copy_dir(
        &override_dir,
        DataDir::get_instance_minecraft_dir(instance_slug),
    )?;
    Ok(())
}

pub(crate) async fn fetch_mrpack(version_id: &str, client: &Client) -> Result<(PathBuf, Modrinth)> {
    // Get version
    let modrinth = ferinth::Ferinth::default();
    let version = modrinth.get_version(version_id).await?;

    // Download file
    let mut dir = env::temp_dir();
    fs::create_dir_all(&dir)?;
    dir.push("pack.mrpack");
    download_file(version.files[0].url.as_str(), None, &dir, Some(&client)).await?;

    // Return config segment
    Ok((
        dir,
        Modrinth {
            project_id: version.project_id,
            version_id: version.id,
        },
    ))
}

async fn maven_download(
    maven_url: &str,
    fallback_maven_url: Option<&str>,
    identifier: &str,
    name: &str,
    version: &str,
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
    let path = DataDir::get_library_dir(&path_suffix)?;
    if !path.try_exists()? {
        // download file.
        let response = client
            .get(url)
            .header("User-Agent", "github_org/AxolotlClient (me@j0.lol)")
            .send()
            .await;
        let response: reqwest::Response = match response {
            Ok(r) => r,
            Err(e)
                if e.status().ok_or(anyhow!("issue accessing maven!"))?
                    == StatusCode::NOT_FOUND =>
            {
                if fallback_maven_url.is_some() {
                    let url = fallback_maven_url.unwrap().to_owned() + &path_suffix;
                    client
                        .get(url)
                        .header("User-Agent", "github_org/AxolotlClient (me@j0.lol)")
                        .send()
                        .await?
                } else {
                    bail!("issue accessing maven!");
                }
            }
            Err(_) => {
                bail!("issue accessing maven!");
            }
        };
        let content = response.bytes().await?;

        let mut dest = File::create(&path)?;
        dest.write_all(&content)?;
    }
    Ok(path)
}
