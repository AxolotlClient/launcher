// read value from config

// write value to config

use crate::minecraft::launcher::JarPath;
use crate::util::read_from_file;
use crate::util::write_to_file;
use crate::util::DataDir;
use anyhow::Result;
use config::Config;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::api::path::*;

pub(crate) fn load() -> Config {
    let mut config: PathBuf = config_dir().unwrap();
    config.push("AxolotlClient/config.toml");
    let mut data_dir = data_dir().unwrap();
    data_dir.push("AxolotlClient/");

    // Check if the file exists. If not, create the file.
    // See `config` docs for implementation.

    // if !config.exists() {
    //     todo!();
    // }

    // This will break if the directory is not UTF-8 encoded
    // for some reason. This shouldn't be a problem unless
    // the user's home folder has a weird name.
    Config::builder()
        .add_source(config::File::with_name("default-config.toml"))
        .add_source(config::File::with_name(&config.to_string_lossy().to_string()).required(false))
        .build()
        .unwrap()

    // todo: return as serde-serialized, for type safety reasons  (or whatever, i barely understand serde)
}

pub(crate) async fn set_instance_config(
    config: &InstanceConfig,
    instance_slug: &str,
) -> Result<()> {
    let mut pack_toml = DataDir::get_instance_dir(instance_slug).clone();
    pack_toml.push("pack.toml");
    write_to_file(&toml::to_string(&config)?, &pack_toml)?;
    Ok(())
}
pub(crate) async fn get_instance_config(instance_slug: &str) -> Result<InstanceConfig> {
    let mut pack_toml = DataDir::get_instance_dir(instance_slug).clone();
    pack_toml.push("pack.toml");

    Ok(toml::from_str(&read_from_file(&pack_toml)?)?)
}

#[derive(Deserialize, Serialize)]
pub(crate) struct InstanceConfig {
    pub(crate) name: String,
    pub(crate) remote: String,
    pub(crate) modrinth: Option<Modrinth>,
    pub(crate) local_file: Option<LocalFile>,
    pub(crate) launch: Launch,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Modrinth {
    pub(crate) project_id: String,
    pub(crate) version_id: String,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct LocalFile {
    pub(crate) path: String,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Launch {
    pub(crate) modloader: Modloader,
    pub(crate) java: Java,
    pub(crate) minecraft: Minecraft,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Modloader {
    pub(crate) loader_type: String,
    pub(crate) mod_path: JarPath,
    pub(crate) main_class: String,
    pub(crate) class_path: JarPath,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Minecraft {
    pub(crate) version: String,
    pub(crate) main_class: String,
    pub(crate) class_path: JarPath,
}
#[derive(Deserialize, Serialize)]
pub(crate) struct Java {
    pub(crate) executable: String,
    pub(crate) min_alloc: i64,
    pub(crate) max_alloc: i64,
}
