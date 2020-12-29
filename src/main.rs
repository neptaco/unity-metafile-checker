use anyhow::{bail, Result};
use clap::Clap;
use std::io::{stdout, BufWriter};
use std::path::PathBuf;
use unity_metafile_checker::app::MetaFileChecker;

#[derive(Clap, Debug)]
#[clap(
    name = "unity-metafile-checker",
    author = "neptaco <neptacox@gmail.com>",
    about = "Check missing meta files for Unity",
)]
struct Opts {
    #[clap(short, long)]
    verbose: bool,

    #[clap(short, long)]
    path: Option<PathBuf>,

    #[clap(long)]
    base_path: Option<PathBuf>,
}

fn main() -> Result<()> {
    pretty_env_logger::init();

    let opts = Opts::parse();
    let path = opts.path.unwrap_or_else(|| PathBuf::from("."));

    if !(path.exists() && path.is_dir()) {
        bail!("{} required exists directory.", path.display());
    }

    let mut checker = MetaFileChecker::default();
    checker.check(&path)?;

    let base_path = opts.base_path.unwrap_or(path);

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    checker.show_results(&mut writer, &base_path)?;

    Ok(())
}
