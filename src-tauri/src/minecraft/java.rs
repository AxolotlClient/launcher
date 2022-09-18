use std::{fs, path::PathBuf};

use anyhow::bail;
use anyhow::Result;
use reqwest::Client;
use semver::VersionReq;

use crate::util::{download_file, extract_file, extract_tar_gz, is_dir_empty, DataDir};

#[derive(Clone, Copy, Debug)]
pub(crate) enum Version {
    Java17,
    Java8,
}

impl Version {
    pub(crate) fn from_mc_version(mc_version: &str) -> Result<Version> {
        let mc_version = semver::Version::parse(&mc_version.replace("_", "+"))?;

        let eight_req = VersionReq::parse("<1.17")?;

        if eight_req.matches(&mc_version) {
            return Ok(Version::Java8);
        } else {
            return Ok(Version::Java17);
        }
    }
    fn version(&self) -> &str {
        match &self {
            Version::Java17 => "17",
            Version::Java8 => "8",
        }
    }
}

pub(crate) async fn get_java(
    version: Version,
    data_dir: &DataDir,
    client: &Client,
) -> Result<PathBuf> {
    let java_dir = data_dir.get_java_dir(version.version());

    if is_dir_empty(&java_dir)? {
        download_java(version, &java_dir, client).await?;
    }

    for entry in glob::glob(&(java_dir.display().to_string() + "**/bin/java"))? {
        if let Ok(path) = entry {
            return Ok(path);
        };
    }

    bail!("Could not find a Java installation!")
}

async fn download_java(version: Version, java_dir: &PathBuf, client: &Client) -> Result<()> {
    let mut url = String::new();

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
            version.version(),
            std::env::consts::OS,
            "x64"
        );
    } else {
        url = format!(
            "https://api.adoptium.net/v3/binary/latest/{}/ga/{}/{}/jre/hotspot/normal/eclipse",
            version.version(),
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

    println!("Downloading Java {}", version.version());

    download_file(&url, None, &file, Some(client)).await?;

    println!("Extracting Java {}", version.version());

    if cfg!(target_os = "windows") {
        extract_file(&file, &java_dir).await?;
    } else {
        extract_tar_gz(&file, &java_dir).await?;
    }

    fs::remove_file(file)?;
    println!("Java installed.");

    Ok(())
}
