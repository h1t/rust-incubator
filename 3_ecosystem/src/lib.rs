use anyhow::{anyhow, Result};
use image::{codecs::jpeg::JpegEncoder, io::Reader};
use log::info;
use rayon::prelude::*;
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Cursor, Read, Seek},
    path::{Path, PathBuf},
    time::Instant,
};
use url::Url;

pub fn process(output_dir: &Path, quality: u8, paths: impl Iterator<Item = String>) -> Result<()> {
    paths
        .collect::<Vec<_>>()
        .par_iter()
        .map(|path| process_file(path, output_dir, quality))
        .collect::<Result<Vec<_>>>()?;

    Ok(())
}

fn process_file(path: &str, output_dir: &Path, quality: u8) -> Result<()> {
    let instant = Instant::now();
    let res = if let Ok(url) = Url::parse(path) {
        let file_path = url
            .path_segments()
            .ok_or_else(|| anyhow!("can't create file path from url"))?
            .collect::<Vec<_>>()
            .join("_");
        let output_path = &output_dir.join(file_path);
        let mut content = read_remote_content(url)?;
        process_jpeg_content(&mut content, output_path, quality)
    } else {
        let input_path = Path::new(path);
        let mut content = read_file_content(input_path)?;
        let file_name = input_path
            .file_name()
            .ok_or_else(|| anyhow!("bad output dir format"))?;
        let output_path = &output_dir.join(file_name);
        process_jpeg_content(&mut content, output_path, quality)
    };

    let elapsed = instant.elapsed().as_millis();
    info!("processing of {path} take {elapsed}ms");

    res
}

fn process_jpeg_content<T>(source: T, output_path: &Path, quality: u8) -> Result<()>
where
    T: BufRead + Seek,
{
    let image = Reader::with_format(source, image::ImageFormat::Jpeg).decode()?;

    let file = File::create(output_path)?;
    let mut file = BufWriter::new(file);
    let encoder = JpegEncoder::new_with_quality(&mut file, quality);

    image.write_with_encoder(encoder).map_err(Into::into)
}

pub fn read_source_file(path: &Path) -> Result<Vec<String>> {
    if path == PathBuf::from("-") {
        let buf = std::io::stdin().lock();
        Ok(read_source(buf))
    } else {
        let buf = File::open(path)?;
        Ok(read_source(buf))
    }
}

fn read_source(buf: impl std::io::Read) -> Vec<String> {
    BufReader::new(buf).lines().filter_map(Result::ok).collect()
}

fn read_file_content(path: &Path) -> Result<impl BufRead + Seek> {
    let input_path = Path::new(path);
    let file = File::open(input_path)?;
    Ok(BufReader::new(file))
}

fn read_remote_content(url: Url) -> Result<impl BufRead + Seek> {
    let mut content = Vec::new();
    let _ = ureq::get(url.path())
        .call()?
        .into_reader()
        .take(1_000_000)
        .read_to_end(&mut content)?;
    Ok(Cursor::new(content))
}
