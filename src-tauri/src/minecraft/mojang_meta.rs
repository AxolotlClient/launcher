use anyhow::Result;

use crate::util::DataDir;

pub(crate) async fn download_minecraft(version: &str, data_dir: &DataDir) -> Result<()> {
    // https://launchermeta.mojang.com/mc/game/version_manifest.json
    todo!("download mc");
}
