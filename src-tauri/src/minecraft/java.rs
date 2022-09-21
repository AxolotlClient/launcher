use std::fs;

use anyhow::bail;
use anyhow::Result;
use reqwest::Client;

use crate::config::Java;
use crate::util::{download_file, extract_file, extract_tar_gz, is_dir_empty, DataDir};

pub(crate) async fn get_java(version: i64, client: &Client) -> Result<Java> {
    let java_dir = DataDir::get_java_dir(&version.to_string());

    if is_dir_empty(&java_dir)? {
        download_java(version, client).await?;
    }

    for entry in glob::glob(&(java_dir.display().to_string() + "**/bin/java"))? {
        if let Ok(path) = entry {
            return Ok(Java {
                executable: path.display().to_string(),
                min_alloc: 1024,
                max_alloc: 2048,
            });
        };
    }

    bail!("Could not find a Java installation!")
}

async fn download_java(version: i64, client: &Client) -> Result<()> {
    let java_dir = DataDir::get_java_dir(&version.to_string());
    let url: String;

    match std::env::consts::ARCH {
        "x86_64" | "x86" | "aarch64" | "arm" => {}
        _ => {
            bail!("Unsupported architecture!")
        }
    }

    // So close...
    if std::env::consts::ARCH == "x86_64" {
        url = format!(
            "https://api.adoptium.net/v3/binary/latest/{}/ga/{}/{}/jre/hotspot/normal/eclipse",
            version,
            std::env::consts::OS,
            "x64"
        );
    } else {
        url = format!(
            "https://api.adoptium.net/v3/binary/latest/{}/ga/{}/{}/jre/hotspot/normal/eclipse",
            version,
            std::env::consts::OS,
            std::env::consts::ARCH
        );
    }

    let mut file = java_dir.clone();

    if cfg!(target_os = "windows") {
        file.push("jre.zip");
    } else {
        file.push("jre.tar.gz");
    }

    println!("Downloading Java {}", version);

    download_file(&url, None, &file, Some(client)).await?;

    println!("Extracting Java {}", version);

    if cfg!(target_os = "windows") {
        extract_file(&file, &java_dir).await?;
    } else {
        extract_tar_gz(&file, &java_dir).await?;
    }

    fs::remove_file(file)?;
    println!("Java installed.");

    Ok(())
}
