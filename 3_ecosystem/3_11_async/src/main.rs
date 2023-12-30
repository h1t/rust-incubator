use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

use step_3_11::download_all;

#[derive(Parser)]
struct Args {
    /// Maximum amount of threads to spawn
    #[clap(short, long, value_parser, default_value_t = num_cpus::get())]
    max_threads: usize,

    /// File to read links from
    file: PathBuf,
}

fn main() -> Result<()> {
    let Args { max_threads, file } = Args::parse();
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(max_threads)
        .enable_all()
        .build()?;

    rt.block_on(download_all(file))?;

    Ok(())
}
