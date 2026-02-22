use assert_cmd::Command;
use assert_cmd::cargo::cargo_bin;
use predicates::prelude::*;
use std::io;
use std::env;

mod shared;
use shared::{TestSetup, compute_hash, create_simple_test_setup, remove_dirs, validate_setup};

#[test]
fn test_move_except_single_file() {
    let setup : io::Result<TestSetup> = create_simple_test_setup(vec!["file1"]);
    validate_setup(&setup);

    let TestSetup{source_dir, dest_dir, files} = setup.unwrap();
    assert!(env::current_dir().unwrap() == *source_dir, "{} , {}", env::current_dir().unwrap().display(), source_dir.display());
 
    let mut cmd  = Command::new(cargo_bin!());
    cmd.current_dir(&source_dir)
        .arg("file1")
        .arg(&dest_dir);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!("{} Already Present!", dest_dir.display())));

    //file1 was moved from source to dest
    let file_moved = dest_dir.join("file1");
    assert!(file_moved.exists());

    let orig_hash: &String = &files[0].1;
    let current_hash: &String = &compute_hash(file_moved).unwrap();
    assert_eq!(orig_hash, current_hash);

    remove_dirs(&[source_dir, dest_dir]);
}

#[test]
fn test_multiple_files() {
    let setup : io::Result<TestSetup> = create_simple_test_setup(vec!["file1", "file2", "file3"]);
    validate_setup(&setup);

    let TestSetup{source_dir, dest_dir, files} = setup.unwrap();
    assert!(env::current_dir().unwrap() == *source_dir, "{} , {}", env::current_dir().unwrap().display(), source_dir.display());

    let mut cmd = Command::new(cargo_bin!());

    cmd.current_dir(&source_dir)
        .arg("file1")
        .arg("file2")
        .arg("file3")
        .arg(&dest_dir);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!("{} Already Present!", dest_dir.display())));

    for (original_path, original_hash) in &files {
        let file_name = original_path.file_name().unwrap();
        let moved_path = dest_dir.join(file_name);
        
        assert!(moved_path.exists(), "File {} should exist in destination", file_name.to_string_lossy());
        assert!(!original_path.exists(), "File {} should not exist in source", file_name.to_string_lossy());

        let current_hash = compute_hash(&moved_path).expect("Failed to compute hash of moved file");
        assert_eq!(original_hash, &current_hash, "Hash mismatch for file {}", file_name.to_string_lossy());
    }

    remove_dirs(&[source_dir, dest_dir]);
}

#[test]
fn test_glob_patterns() {
    let setup : io::Result<TestSetup> = create_simple_test_setup(vec!["file1", "file2", "file3"]);
    validate_setup(&setup);

    let TestSetup{source_dir, dest_dir, files} = setup.unwrap();
    assert!(env::current_dir().unwrap() == *source_dir, "{} , {}", env::current_dir().unwrap().display(), source_dir.display());

    let mut cmd = Command::new(cargo_bin!());
    cmd.current_dir(&source_dir)
        .arg("file*")
        .arg(&dest_dir);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!("{} Already Present!", dest_dir.display())));

    for (original_path, original_hash) in &files {
        let file_name = original_path.file_name().unwrap();
        let moved_path = dest_dir.join(file_name);
        
        assert!(moved_path.exists(), "File {} should exist in destination", file_name.to_string_lossy());
        assert!(!original_path.exists(), "File {} should not exist in source", file_name.to_string_lossy());

        let current_hash = compute_hash(&moved_path).expect("Failed to compute hash of moved file");
        assert_eq!(original_hash, &current_hash, "Hash mismatch for file {}", file_name.to_string_lossy());
    }

    remove_dirs(&[source_dir, dest_dir]);
}

#[test]
fn test_exclusions() {
    let file_names = vec![
        "file1", 
        "file2", 
        "file3", 
        "file4.txt", 
        "file5.txt", 
        "file6.txt"
    ];
    let setup : io::Result<TestSetup> = create_simple_test_setup(file_names.clone());
    validate_setup(&setup);

    let TestSetup{source_dir, dest_dir, files} = setup.unwrap();
    assert!(env::current_dir().unwrap() == *source_dir, "{} , {}", env::current_dir().unwrap().display(), source_dir.display());

    let mut cmd = Command::new(cargo_bin!());
    cmd.current_dir(&source_dir)
        .arg("file*")
        .arg(&dest_dir)
        .arg("-e")
        .arg("*.txt");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!("{} Already Present!", dest_dir.display())));

    // Check that non-txt files were moved
    for i in 0..3 {
        let file_name = file_names[i];
        let moved_path = dest_dir.join(file_name);
        let source_path = source_dir.join(file_name);

        assert!(moved_path.exists(), "File {} should have been moved to destination", file_name);
        assert!(!source_path.exists(), "File {} should no longer exist in source", file_name);

        let orig_hash = &files[i].1;
        let current_hash = compute_hash(&moved_path).unwrap();
        assert_eq!(orig_hash, &current_hash, "Hash mismatch for moved file {}", file_name);
    }

    // Check that .txt files were NOT moved
    for i in 3..6 {
        let file_name = file_names[i];
        let moved_path = dest_dir.join(file_name);
        let source_path = source_dir.join(file_name);

        assert!(!moved_path.exists(), "File {} should NOT have been moved to destination", file_name);
        assert!(source_path.exists(), "File {} should still exist in source", file_name);

        let orig_hash = &files[i].1;
        let current_hash = compute_hash(&source_path).unwrap();
        assert_eq!(orig_hash, &current_hash, "Hash mismatch for excluded file {}", file_name);
    }

    remove_dirs(&[source_dir, dest_dir]);
}

#[test]
fn test_new_directory_creation() {
    todo!();
}

#[test]
fn test_copy_only() {
    todo!();
}

#[test]
fn test_verbose_mode() {
    todo!();
}
