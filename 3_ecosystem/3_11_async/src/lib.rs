use std::{error::Error, path::PathBuf};

use reqwest::Client;
use reqwest::Url;
use tokio::io::AsyncWriteExt;
use tokio::{fs::File, io::AsyncBufReadExt, io::BufReader};

pub async fn download_all(file: PathBuf) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let file = File::open(file).await?;
    let file = BufReader::new(file);
    let mut lines = file.lines();
    let mut handlers = Vec::new();

    while let Ok(Some(line)) = lines.next_line().await {
        if !line.is_empty() {
            let h = tokio::spawn(download_url(client.clone(), line));
            handlers.push(h);
        }
    }

    for h in handlers {
        let _ = h.await?;
    }

    Ok(())
}

async fn download_url(client: Client, url: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    let url: Url = url.parse()?;
    let url_content = client.get(url.clone()).send().await?.text().await?;
    let file_path = url
        .path_segments()
        .ok_or("can't create file path from url")?
        .collect::<Vec<_>>()
        .join("_");
    let mut file = File::create(file_path).await?;

    file.write_all(url_content.as_bytes()).await?;

    Ok(())
}
