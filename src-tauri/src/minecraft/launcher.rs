use crate::{
    minecraft::{java::get_java, modpack::get_modpack},
    util::{download_file, DataDir},
};
use anyhow::Result;
use config::Config;
use std::path::{Path, PathBuf};
use tauri::api::path::data_dir;

use super::{java::Version, modpack::install_modpack, mojang_meta::download_minecraft};

struct Modpack {
    mc_version: String,
    pack_version: String,
    mr_slug: String,
}

pub(crate) async fn launch(config: Config) -> Result<()> {
    let data_dir = DataDir::new();

    // Get paths (temporary)
    let slug = config.get_string("pack.slug").unwrap();
    let version = config.get_string("pack.version").unwrap();

    // let mut instance_dir: PathBuf = data_dir.clone();
    // instance_dir.push(format!("instances/{slug}/{version}/"));

    // Check auth from startup.rs or whatever todo
    // let auth_token = authenticate(something) (auth.rs)

    // Get java from config/download it
    let java = get_java(Version::from_mc_version(&version), &data_dir).await?;

    // Download minecraft version. Install into .minecraft
    download_minecraft(&version, &data_dir).await?;

    // Get pack from config
    // Extract modpack file to [DATA_DIR]/packs/NAME/MCVER/VERSION
    get_modpack(&slug, &version, &data_dir).await?;

    // Download quilt/fabric/legacy fabric/whatever
    // Make .minecraft at [DATA_DIR]/instances/MCVER/.minecraft/
    // Download mods from mrpack into .minecraft
    install_modpack(&slug, &version, &data_dir).await?;

    launch_minecraft(&java, &data_dir).await?;

    Ok(())
}

async fn launch_minecraft(java: &PathBuf, instance: &DataDir) -> Result<()> {
    todo!("launch minecraft");
}
