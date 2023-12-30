use anyhow::{bail, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use step_3::*;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Commands,

    /// Output directory
    #[clap(short, long("output"))]
    output_dir: PathBuf,

    /// Max processing images
    #[clap(short, long("max"), default_value_t = 1)]
    max_images: usize,

    /// Compression quality
    #[clap(short, long, default_value_t = 100)]
    quality: u8,
}

#[derive(Subcommand)]
enum Commands {
    /// List of URL and local paths
    Files {
        /// URL of remote image file
        #[clap(short, long("url"))]
        urls: Vec<String>,

        /// Path to image file
        #[clap(short, long("file"))]
        files: Vec<String>,
    },

    /// Path to file with URL list
    Source {
        /// Path to file with URL list
        #[clap(short, long("source"), env = "SOURCE_FILE", default_value = "-")]
        source_file: PathBuf,
    },
}

fn main() -> Result<()> {
    let Args {
        command,
        output_dir,
        max_images,
        quality,
    } = Args::parse();

    if !output_dir.exists() {
        bail!("output dir {output_dir:?} is not exists");
    }

    env_logger::init();

    rayon::ThreadPoolBuilder::new()
        .num_threads(max_images)
        .build_global()?;

    match command {
        Commands::Files { urls, files } => {
            process(&output_dir, quality, files.into_iter().chain(urls))
        }
        Commands::Source { source_file } => {
            let links = read_source_file(&source_file)?;
            process(&output_dir, quality, links.into_iter())
        }
    }
}
