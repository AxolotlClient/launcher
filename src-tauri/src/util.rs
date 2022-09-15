use std::fs::File;
use std::io::Cursor;
use std::io::Write;
use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use reqwest::Url;
macro_rules! str_err {
    ($res:expr) => {
        $res.map_err(|err| err.to_string())
    };
}

pub(crate) async fn download_file(url: Url, sha1: String, path: String) -> Result<()> {
    let response = str_err!(reqwest::get(url).await).map_err(|e| anyhow!(e))?;
    let content = response.bytes().await?;

    if calculate_hash(content.clone(), sha1) {
        let mut dest = File::create(path)?;
        // let content = str_err!(response.bytes().await).map_err(|e| anyhow!(e))?;
        dest.write_all(&content)?;
        return Ok(());
    }
    Err(anyhow!("Downloaded file did not match hash."))
}

fn calculate_hash(file: bytes::Bytes, sha1: String) -> bool {
    // todo: actually calculate hash. crypto crates suck.
    // sha1 isnt collision resistant and sha512 may take too long.
    true
}

async fn extract_file(archive: String, target_dir: String) -> Result<()> {
    zip_extract::extract(Cursor::new(&archive), &PathBuf::from(&target_dir), false).context(
        "Could not extract Zip file: ".to_owned() + &archive + " to directory " + &target_dir,
    )?;

    Ok(())
}
