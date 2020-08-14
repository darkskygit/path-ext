# path-ext

Provide convenient methods for path operations

# Using

```rust
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
```

output:

```
running 1 test
full path: Z:\Movies\[VCB-Studio] Fate Zero [Ma10p_1080p]\[VCB-Studio] Fate Zero [04][Ma10p_1080p][x265_flac].mkv
file ext: mkv
file stem: [VCB-Studio] Fate Zero [04][Ma10p_1080p][x265_flac]
file name: [VCB-Studio] Fate Zero [04][Ma10p_1080p][x265_flac].mkv
merged full path: Z:\Movies\[VCB-Studio] Fate Zero [Ma10p_1080p]\[VCB-Studio] Fate Zero [04][Ma10p_1080p][x265_flac].mkv
file: true
dir: true
subdir: Z:\Movies\[VCB-Studio] Fate Zero [Ma10p_1080p]
subdir: Z:\Movies\[VCB-Studio] Fate Zero [Ma10p_1080p]\CDs
subdir: Z:\Movies\[VCB-Studio] Fate Zero [Ma10p_1080p]\Scans
subdir: Z:\Movies\[VCB-Studio] Fate Zero [Ma10p_1080p]\SPs
test test_path ... ok
```
