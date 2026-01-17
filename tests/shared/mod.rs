use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use sha2::{Digest, Sha256};
use tempfile::{tempdir, TempDir};

pub struct TestSetup {
    pub temp_dir: TempDir,
    pub source_dir: PathBuf,
    pub dest_dir: PathBuf,
    pub files: Vec<(PathBuf, String)>,
}

pub fn create_test_setup() -> io::Result<TestSetup> {
    let temp_dir = tempdir()?;
    let source_dir = temp_dir.path().join("source");
    let dest_dir = temp_dir.path().join("dest");
    fs::create_dir_all(&source_dir)?;
    fs::create_dir_all(&dest_dir)?;

    let mut files = Vec::new();
    let file_names = ["file1.txt", "file2.log", "file3.tmp"];

    for name in &file_names {
        let path = source_dir.join(name);
        let mut file = File::create(&path)?;
        writeln!(file, "content of {}", name)?;
        let hash = compute_hash(&path)?;
        files.push((path, hash));
    }

    Ok(TestSetup {
        temp_dir,
        source_dir,
        dest_dir,
        files,
    })
}

pub fn compute_hash<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut sha256 = Sha256::new();
    io::copy(&mut file, &mut sha256)?;
    Ok(format!("{:x}", sha256.finalize()))
}
