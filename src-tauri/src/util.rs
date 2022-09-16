use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Cursor;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use tauri::api::path::data_dir;

macro_rules! str_err {
    ($res:expr) => {
        $res.map_err(|err| err.to_string())
    };
}

pub(crate) async fn download_file(url: &str, sha1: Option<String>, path: &PathBuf) -> Result<()> {
    let response = str_err!(reqwest::get(url).await).map_err(|e| anyhow!(e))?;
    let content = response.bytes().await?;

    if sha1.is_some() {
        if calculate_hash(&content, sha1.unwrap()) {
            let mut dest = File::create(path)?;
            // let content = str_err!(response.bytes().await).map_err(|e| anyhow!(e))?;
            dest.write_all(&content)?;
            return Ok(());
        } else {
            return Err(anyhow!("Downloaded file did not match hash."));
        }
    } else {
        let mut dest = File::create(path)?;
        dest.write_all(&content)?;
        return Ok(());
    }
}

fn calculate_hash(file: &bytes::Bytes, sha1: String) -> bool {
    // todo: actually calculate hash. crypto crates suck.
    // sha1 isnt collision resistant and sha512 may take too long.
    true
}

pub(crate) async fn extract_file(archive: &PathBuf, target_dir: &PathBuf) -> Result<()> {
    let f = File::open(archive)?;

    let buf_reader = BufReader::new(f);
    // let buffer = buf_reader.buffer();

    zip_extract::extract(buf_reader, &target_dir, false).context(
        "Could not extract Zip file: ".to_owned()
            + &archive.display().to_string()
            + " to directory "
            + &target_dir.display().to_string(),
    )?;

    Ok(())
}

pub(crate) async fn extract_tar_gz(archive: &PathBuf, target_dir: &PathBuf) -> Result<()> {
    let f = File::open(&archive)?;
    let tarfile = flate2::read::GzDecoder::new(f);
    let mut archive = tar::Archive::new(tarfile);

    archive.unpack(target_dir)?;

    Ok(())
}

pub(crate) fn is_dir_empty(dir: &PathBuf) -> Result<bool> {
    Ok(dir.read_dir()?.next().is_none())
}

pub(crate) struct DataDir {
    path: PathBuf,
}

impl DataDir {
    pub(crate) fn new() -> Self {
        let mut dir = data_dir().unwrap();
        dir.push("AxolotlClient/");
        fs::create_dir_all(&dir).unwrap();

        Self { path: dir }
    }
    pub(crate) fn get_java_dir(&self, version: &str) -> &PathBuf {
        let mut dir = self.path.clone();
        dir.push(format!("java/{version}/"));
        fs::create_dir_all(&dir).unwrap();
        &dir
    }

    pub(crate) fn get_instance_dir(&self, slug: &str, version: &str) -> &PathBuf {
        let mut dir = self.path.clone();
        dir.push(format!("instances/{slug}/{version}/"));
        fs::create_dir_all(&dir).unwrap();
        &dir
    }

    pub(crate) fn get_instance_mrpack_dir(&self, slug: &str, version: &str) -> &PathBuf {
        let mut dir = self.path.clone();
        dir.push(format!("instances/{slug}/{version}/mrpack/"));
        fs::create_dir_all(&dir).unwrap();
        &dir
    }

    pub(crate) fn get_instance_minecraft_dir(&self, slug: &str, version: &str) -> &PathBuf {
        let mut dir = self.path.clone();
        dir.push(format!("instances/{slug}/{version}/.minecraft/"));
        fs::create_dir_all(&dir).unwrap();
        &dir
    }
}
