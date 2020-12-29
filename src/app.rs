use anyhow::{ensure, Result};
use pathdiff::diff_paths;
use std::fs;
use std::fs::read_dir;
use std::io::Write;
use std::path::PathBuf;

#[derive(Default)]
pub struct MetaFileChecker {
    /// asset exists, but not exists meta file
    missing_meta_files: Vec<PathBuf>,

    /// meta file exists, but not exists asset file
    missing_assets: Vec<PathBuf>,
}

impl MetaFileChecker {
    fn is_extension_equals(path: &PathBuf, ext: &str) -> bool {
        path.extension().map(|x| x == ext).unwrap_or(false)
    }

    fn is_ignore_file(path: &PathBuf) -> Result<bool> {
        let file_name = path.file_name().unwrap().to_string_lossy();

        if file_name.starts_with('.') || file_name.ends_with('~') {
            return Ok(true);
        }
        if file_name == "cvs" {
            return Ok(true);
        }
        if Self::is_extension_equals(path, "tmp") {
            return Ok(true);
        }

        if path.is_dir() && Self::is_empty_dir(&path)? {
            return Ok(true);
        }
        Ok(false)
    }

    fn get_meta_path(path: &PathBuf) -> PathBuf {
        let mut meta_path = path.as_os_str().to_os_string();
        meta_path.push(".meta");
        PathBuf::from(meta_path)
    }

    fn get_asset_path_from_meta(path: &PathBuf) -> PathBuf {
        let mut asset_path = path.clone();
        asset_path.set_extension("");
        asset_path
    }

    fn is_metafile(path: &PathBuf) -> bool {
        if !path.is_file() {
            return false;
        }
        Self::is_extension_equals(path, "meta")
    }

    fn is_empty_dir(path: &PathBuf) -> Result<bool> {
        ensure!(path.is_dir(), "file not supported args. {}", path.display());

        for entry in read_dir(path)? {
            let path = entry?.path();

            if path.is_file() {
                return Ok(false);
            }

            if !Self::is_empty_dir(&path)? {
                return Ok(false);
            }
        }

        Ok(true)
    }

    pub fn check(&mut self, parent: &PathBuf) -> Result<()> {
        let entries = fs::read_dir(parent)?;

        for entry in entries {
            let path = entry?.path();

            if Self::is_ignore_file(&path)? {
                continue;
            }

            if Self::is_metafile(&path) {
                let asset_path = Self::get_asset_path_from_meta(&path);
                if !asset_path.exists() {
                    self.missing_assets.push(path)
                }
            } else {
                let meta_path = Self::get_meta_path(&path);
                if !meta_path.exists() {
                    debug!("metafile missing -> [{}]", meta_path.display());
                    self.missing_meta_files.push(path.clone());
                }

                if path.is_dir() {
                    self.check(&path)?;
                }
            }
        }

        Ok(())
    }

    pub fn show_results<W: Write>(&self, writer: &mut W, base_path: &PathBuf) -> Result<()> {
        if !self.missing_meta_files.is_empty() {
            writer.write_all(b"Missing metafiles:\n")?;
            for metafile in &self.missing_meta_files {
                if let Some(relative_path) = diff_paths(metafile, base_path) {
                    writeln!(writer, "\t{}", relative_path.display())?;
                }
            }
        }

        if !self.missing_assets.is_empty() {
            writer.write_all(b"Missing assets:\n")?;
            for asset in &self.missing_assets {
                if let Some(relative_path) = diff_paths(asset, base_path) {
                    writeln!(writer, "\t{}", relative_path.display())?;
                }
            }
        }

        Ok(())
    }
}
