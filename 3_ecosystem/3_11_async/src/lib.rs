use std::path::PathBuf;

use anyhow::{anyhow, Result};
use futures_util::future;
use reqwest::Client;
use reqwest::Url;
use tokio::io::AsyncWriteExt;
use tokio::{fs::File, io::AsyncBufReadExt, io::BufReader};
use tokio_stream::wrappers::LinesStream;
use tokio_stream::StreamExt;

pub async fn download_all(file: PathBuf) -> Result<()> {
    let client = Client::new();
    let file = File::open(file).await?;
    let file = BufReader::new(file);
    let stream = LinesStream::new(file.lines());
    let handlers: Vec<_> = stream
        .filter_map(Result::ok)
        .map(|url| download_url(client.clone(), url))
        .collect()
        .await;
    let _ = future::join_all(handlers).await;

    Ok(())
}

async fn download_url(client: Client, url: String) -> Result<()> {
    let url: Url = url.parse()?;
    let url_content = client.get(url.clone()).send().await?.text().await?;
    let file_path = url
        .path_segments()
        .ok_or_else(|| anyhow!("can't create file path from url"))?
        .collect::<Vec<_>>()
        .join("_");
    let mut file = File::create(file_path).await?;

    file.write_all(url_content.as_bytes()).await?;

    Ok(())
}
