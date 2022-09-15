// read value from config

// write value to config

use config::Config;
use std::path::PathBuf;
use tauri::api::path::*;

pub(crate) fn load() -> Config {
    let mut config: PathBuf = config_dir().unwrap();
    config.push("AxolotlClient/config.toml");
    let mut data_dir = data_dir().unwrap();
    data_dir.push("AxolotlClient/");

    // Check if the file exists. If not, create the file.
    // See `config` docs for implementation.

    if !config.exists() {
        todo!();
    }

    // This will break if the directory is not UTF-8 encoded
    // for some reason. This shouldn't be a problem unless
    // the user's home folder has a weird name.
    Config::builder()
        .add_source(config::File::with_name(
            &config.to_string_lossy().to_string(),
        ))
        .build()
        .unwrap()
}
