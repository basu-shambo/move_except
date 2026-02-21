use std::fs::{File, create_dir, create_dir_all, remove_dir_all};
use std::path::{Path, PathBuf};
use std::io;
use std::env;

use sha2::{Digest, Sha256};
pub struct TestSetup {
    pub source_dir: PathBuf,
    pub dest_dir: PathBuf,
    pub files: Vec<(PathBuf, String)>,
}

const BASE_PATH : &'static str = "/tmp/";

fn create_source_dest(source_dir_name: &str, dest_dir_name: &str) -> io::Result<(PathBuf, PathBuf)> {
    let source_dir : PathBuf = PathBuf::from(BASE_PATH).join(source_dir_name);
    let dest_dir : PathBuf = PathBuf::from(BASE_PATH).join(dest_dir_name);

    let _ = create_dir(&source_dir)?;
    let _ = create_dir(&dest_dir)?;
    
    return Ok((source_dir,dest_dir));
}

pub fn remove_dirs(dirs: &[impl AsRef<Path>]) {
    let _ = dirs.iter().map(|e| remove_dir_all(e));
}

pub fn validate_setup(test_setup: &io::Result<TestSetup>) {
    if let Ok(TestSetup { source_dir, dest_dir, files }) = test_setup {
        assert!(source_dir.exists());
        assert!(dest_dir.exists());
        for file in files {
            assert!(file.0.exists());
        }
        //Also assert that the current directory is the source directory from the 
        assert!(env::current_dir().unwrap() == *source_dir);
    } else {
        panic!("Test setup failed");
    }
}

pub fn create_simple_test_setup<StrType: AsRef<str>>(input_files:Vec<StrType>) -> io::Result<TestSetup> {
    let source_path_name: String = "source".to_string();
    let dest_path_name: String = "dest".to_string();
    
    return create_test_setup(source_path_name, dest_path_name, input_files);

}

fn change_dir_to(to_path:&PathBuf)  {
        let changed_dir = env::set_current_dir(to_path);
        assert!(changed_dir.is_ok());
}
pub fn create_test_setup<StrType: AsRef<str>>(mut source_path_name: String, mut dest_path_name: String, input_files:Vec<StrType>) -> io::Result<TestSetup> {
    let mut temp_paths = create_source_dest(&source_path_name, &dest_path_name);
    if temp_paths.is_err() {
        let mut post_num = 1;
        while temp_paths.is_err() && post_num < 100 {
            source_path_name = format!("{}_{}", source_path_name, post_num);
            dest_path_name = format!("{}_{}", dest_path_name, post_num);
            post_num += 1;
            temp_paths = create_source_dest(&source_path_name, &dest_path_name);
        }
    }

    assert!(temp_paths.is_ok(), "The paths, can't be created, failing");
    let mut files_with_hashes : Vec<(PathBuf, String)> = Vec::new();  
    
    let (source_dir, dest_dir ) = temp_paths.unwrap();
    for path_name in input_files.iter() {
        let current_path : PathBuf = source_dir.clone().join(path_name.as_ref());
        if !current_path.exists() {
            if let Some(parent) = current_path.parent() {
                let parent_created = create_dir_all(parent);
                if parent_created.is_err() {
                    println!("Some error, ignoring {}", current_path.display());
                    continue;
                }
            }
            let file = File::create(&current_path);
            if file.is_err() {
                println!("Some error, ignoring {}", current_path.display());
                continue;
            }
            let hashed_result = compute_hash(&current_path);
            if let Ok(hashed_value) = hashed_result {
                files_with_hashes.push((current_path, hashed_value));
            }
        } 
    }
    change_dir_to(&source_dir); 
    return Ok(TestSetup {
        source_dir : source_dir,
        dest_dir : dest_dir,
        files: files_with_hashes, 
    });
}

pub fn compute_hash<PathType: AsRef<Path>>(path: PathType) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let _ = io::copy(&mut file, &mut hasher);
    return Ok(format!("{:x}", hasher.finalize()));
} 
