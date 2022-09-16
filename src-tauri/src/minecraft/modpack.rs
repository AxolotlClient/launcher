use std::{fs, path::PathBuf};

use anyhow::Result;

use crate::util::{download_file, extract_file, is_dir_empty};

pub(crate) async fn get_modpack(slug: &str, mc_version: &str, instance: PathBuf) -> Result<()> {
    let mut mrpack_directory = instance;
    mrpack_directory.push("mrpack/");

    fs::create_dir_all(&mrpack_directory)?;

    if is_dir_empty(&mrpack_directory)? {
        download_mrpack(slug, mc_version, mrpack_directory).await?;
    }

    // Install modpack to .minecraft

    Ok(())
}

async fn download_mrpack(slug: &str, mc_version: &str, instance: PathBuf) -> Result<()> {
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
        version.files[0].url.clone().as_str(),
        Some(version.files[0].hashes.sha1.clone()),
        &file,
    )
    .await?;

    println!("Extracting Pack {}", slug);
    extract_file(&file, &instance).await?;

    fs::remove_file(file)?;
    Ok(())
}
