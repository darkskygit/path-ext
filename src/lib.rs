use std::fs::{create_dir_all, metadata};
use std::io::{self, Result};
use std::path::is_separator;
use walkdir::WalkDir;

pub use std::path::{Path, PathBuf};

pub trait PathExt {
    fn full_str(&self) -> &str;
    fn ext_str(&self) -> &str;
    fn stem_str(&self) -> &str;
    fn name_str(&self) -> &str;
    fn create_parent_dir_all(&self) -> Result<()>;
    fn merge<P: AsRef<Path>>(&self, append: P) -> PathBuf;
    fn is_file(&self) -> bool;
    fn is_dir(&self) -> bool;
    fn walk_dir<F: Fn(&Path) -> bool>(&self, filter: F) -> Vec<PathBuf>;
}

impl<T: AsRef<Path>> PathExt for T {
    fn full_str(&self) -> &str {
        self.as_ref().to_str().unwrap_or_default()
    }

    fn ext_str(&self) -> &str {
        self.as_ref()
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or_default()
    }

    fn stem_str(&self) -> &str {
        self.as_ref()
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default()
    }

    fn name_str(&self) -> &str {
        self.as_ref()
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or_default()
    }

    fn create_parent_dir_all(&self) -> io::Result<()> {
        if let Some(root) = self.as_ref().parent() {
            if !root.exists() {
                create_dir_all(root)?;
            }
        }
        Ok(())
    }

    fn merge<P: AsRef<Path>>(&self, append: P) -> PathBuf {
        self.as_ref()
            .iter()
            .chain(append.as_ref().iter().filter(|&component| {
                if let Some(c) = component
                    .to_os_string()
                    .into_string()
                    .unwrap_or_default()
                    .chars()
                    .next()
                {
                    !is_separator(c)
                } else {
                    false
                }
            }))
            .collect()
    }

    fn is_file(&self) -> bool {
        metadata(self)
            .map(|metadata| metadata.is_file())
            .unwrap_or(false)
    }

    fn is_dir(&self) -> bool {
        metadata(self)
            .map(|metadata| metadata.is_dir())
            .unwrap_or(false)
    }

    fn walk_dir<F: Fn(&Path) -> bool>(&self, filter: F) -> Vec<PathBuf> {
        WalkDir::new(self)
            .into_iter()
            .filter_entry(|e| filter(e.path()))
            .filter_map(|e| e.map(|item| item.into_path()).ok())
            .collect()
    }
}
