use std::fs::{create_dir, create_dir_all, metadata, remove_dir_all, remove_file};
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
    fn mkdir_after_remove(&self) -> io::Result<PathBuf>;
}

impl<T: AsRef<Path>> PathExt for T {
    #[inline]
    fn full_str(&self) -> &str {
        self.as_ref().to_str().unwrap_or_default()
    }

    #[inline]
    fn ext_str(&self) -> &str {
        self.as_ref()
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or_default()
    }

    #[inline]
    fn stem_str(&self) -> &str {
        self.as_ref()
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default()
    }

    #[inline]
    fn name_str(&self) -> &str {
        self.as_ref()
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or_default()
    }

    #[inline]
    fn create_parent_dir_all(&self) -> io::Result<()> {
        if let Some(root) = self.as_ref().parent() {
            if !root.exists() {
                create_dir_all(root)?;
            }
        }
        Ok(())
    }

    #[inline]
    fn mkdir_after_remove(&self) -> io::Result<PathBuf> {
        if self.as_ref().exists() {
            if self.as_ref().is_file() {
                remove_file(self)?;
            } else {
                remove_dir_all(self)?;
            }
        }
        create_dir(self)?;
        Ok(self.as_ref().into())
    }

    #[inline]
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

    #[inline]
    fn is_file(&self) -> bool {
        metadata(self)
            .map(|metadata| metadata.is_file())
            .unwrap_or(false)
    }

    #[inline]
    fn is_dir(&self) -> bool {
        metadata(self)
            .map(|metadata| metadata.is_dir())
            .unwrap_or(false)
    }

    #[inline]
    fn walk_dir<F: Fn(&Path) -> bool>(&self, filter: F) -> Vec<PathBuf> {
        WalkDir::new(self)
            .into_iter()
            .filter_entry(|e| filter(e.path()))
            .filter_map(|e| e.map(|item| item.into_path()).ok())
            .collect()
    }
}

#[test]
fn test_path() {
    let path1 = PathBuf::from("Z:\\Movies\\[VCB-Studio] Fate Zero [Ma10p_1080p]\\[VCB-Studio] Fate Zero [04][Ma10p_1080p][x265_flac].mkv");
    println!("full path: {}", path1.full_str());
    println!("file ext: {}", path1.ext_str());
    println!("file stem: {}", path1.stem_str());
    println!("file name: {}", path1.name_str());
    let path2 = PathBuf::from("Z:\\Movies");
    let path3 = PathBuf::from("[VCB-Studio] Fate Zero [Ma10p_1080p]\\[VCB-Studio] Fate Zero [04][Ma10p_1080p][x265_flac].mkv");
    let path4 = path2.merge(path3);
    println!("merged full path: {}", path4.full_str());
    println!("file: {}", path1.is_file());
    println!("dir: {}", path2.is_dir());
    if let Some(parent) = path4.parent() {
        for path in parent.walk_dir(|p| p.is_dir()) {
            println!("subdir: {}", path.full_str());
        }
    }
}
