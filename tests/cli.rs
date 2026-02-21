use assert_cmd::Command;
use assert_cmd::cargo::cargo_bin;
use predicates::prelude::*;
use std::io;
use std::env;

mod shared;
use shared::{TestSetup, compute_hash, create_simple_test_setup, remove_dirs, validate_setup};

#[test]
fn test_move_except_single_file() {
    let setup : io::Result<TestSetup> = create_simple_test_setup(vec!["file1", "file2", "file3"]);
    validate_setup(&setup);

    let TestSetup{source_dir, dest_dir, files} = setup.unwrap();
    assert!(env::current_dir().unwrap() == *source_dir);
 
    let mut cmd  = Command::new(cargo_bin!());
    cmd.current_dir(&source_dir)
        .arg("file1")
        .arg("file2")
        .arg("file3")
        .arg(&dest_dir);

     

//    let mut cmd = Command::cargo_bin("move_except").unwrap();
//    cmd.arg(source_dir_str)
//        .arg(dest_dir_str)
//        .arg("--except")
//        .arg("*.log");
//
//    cmd.assert()
//        .success()
//        .stdout(predicate::str::contains("Moved 2 files"));
//
//    // file1.txt and file3.tmp should be moved
//    let moved_file1_path = setup.dest_dir.join("file1.txt");
//    let moved_file3_path = setup.dest_dir.join("file3.tmp");
//    assert!(moved_file1_path.exists());
//    assert!(moved_file3_path.exists());
//
//    // The excluded file file2.log should not be moved
//    let excluded_file_path = setup.source_dir.join("file2.log");
//    assert!(excluded_file_path.exists());
//
//    // The moved files should not be in the source directory anymore
//    let original_file1_path = setup.source_dir.join("file1.txt");
//    let original_file3_path = setup.source_dir.join("file3.tmp");
//    assert!(!original_file1_path.exists());
//    assert!(!original_file3_path.exists());
//
//    // Verify hashes of moved files
//    let original_hash_file1 = &setup.files.iter().find(|(p, _)| p.ends_with("file1.txt")).unwrap().1;
//    let moved_hash_file1 = compute_hash(&moved_file1_path).unwrap();
//    assert_eq!(original_hash_file1, &moved_hash_file1);
//
//    let original_hash_file3 = &setup.files.iter().find(|(p, _)| p.ends_with("file3.tmp")).unwrap().1;
//    let moved_hash_file3 = compute_hash(&moved_file3_path).unwrap();
//    assert_eq!(original_hash_file3, &moved_hash_file3);

    remove_dirs(&[source_dir, dest_dir]);
}
