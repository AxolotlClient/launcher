use crate::{
    minecraft::{java::get_java, modpack::get_modpack},
    util::download_file,
};
use anyhow::Result;
use config::Config;
use std::path::{Path, PathBuf};
use tauri::api::path::data_dir;

use super::java::Version;

struct Modpack {
    mc_version: String,
    pack_version: String,
    mr_slug: String,
}

pub(crate) async fn launch(config: Config) -> Result<()> {
    // Get paths (temporary)
    let mut data_dir: PathBuf = data_dir().unwrap();
    data_dir.push("AxolotlClient/");

    let slug = config.get_string("pack.slug").unwrap();
    let version = config.get_string("pack.version").unwrap();

    let mut instance_dir: PathBuf = data_dir.clone();
    instance_dir.push(format!("instances/{slug}/{version}/"));

    // Get java from config/download it
    let java = get_java(Version::from_mc_version(&version)).await;

    // Download minecraft?
    // download_minecraft(&version, instance_dir.clone());

    // Get pack from config
    // Extract modpack file to [DATA_DIR]/packs/NAME/MCVER/VERSION
    get_modpack(&slug, &version, instance_dir).await?;

    // Download quilt/fabric/legacy fabric/whatever (probably do this in mrpack)

    // Make .minecraft at [DATA_DIR]/instances/MCVER/.minecraft/

    // Download mods from mrpack into .minecraft

    todo!("Launch minecraft");

    Ok(())
}

fn download_minecraft(mc_version: &str, instance: PathBuf) {
    // https://launchermeta.mojang.com/mc/game/version_manifest.json
    todo!("download mc");
}
