use crate::util::download_file;
use anyhow::Result;
use config::Config;
use ferinth::*;
use std::path::Path;

pub(crate) async fn launch(config: Config) -> Result<()> {
    download_mrpack("stellar", "1.19.2").await?;
    Ok(())
}

async fn download_mrpack(id: &str, mc_version: &str) -> Result<()> {
    // Get versions
    let modrinth = Ferinth::default();
    let versions = modrinth
        .list_versions_filtered(id, Some(&["quilt"]), Some(&[mc_version]), None)
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

    // Download file
    download_file(
        version.files[0].url.clone(),
        version.files[0].hashes.sha1.clone(),
        "foo.txt".into(),
    )
    .await?;

    let body = reqwest::get(version.files[0].url.clone())
        .await?
        .text()
        .await?;

    // Save file to cache (get sha)
    // parse_mrpack(body);
    Ok(())
}

fn parse_mrpack(file: Box<Path>) {}
