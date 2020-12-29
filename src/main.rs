use clap::Clap;
use std::path::PathBuf;
use anyhow::{Result};
use std::io::{BufWriter, stdout};
use unity_metafile_checker::app::MetaFileChecker;

#[derive(Clap, Debug)]
#[clap(
name = "unity_metafile_checker",
author = "neptaco",
about = "check unity-metafiles"
)]
struct Opts {
    #[clap(short, long)]
    verbose: bool,

    #[clap(short, long)]
    path: Option<PathBuf>,
}


fn main() -> Result<()> {
    pretty_env_logger::init();

    let opts = Opts::parse();
    let path = opts.path.unwrap_or(PathBuf::from("."));

    if !(path.exists() && path.is_dir()) {
        eprintln!("{} required exists directory.", path.display());
    }

    let mut checker = MetaFileChecker::new();
    checker.check(&path)?;

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    checker.show_results(&mut writer, &path)?;

    Ok(())
}

