use assert_cmd::Command;
use predicates::prelude::*;

mod shared;
use shared::{compute_hash, create_simple_test_setup, remove_dirs};

#[test]
fn test_move_except_single_file() {
    let setup = create_simple_test_setup(vec!["file1", "file2", "file3"]).expect("Failed to create test setup");
    let source_dir_str = setup.source_dir.to_str().unwrap();
    let dest_dir_str = setup.dest_dir.to_str().unwrap();

    let mut cmd = Command::cargo_bin("move_except").unwrap();
    cmd.arg(source_dir_str)
        .arg(dest_dir_str)
        .arg("--except")
        .arg("*.log");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Moved 2 files"));

    // file1.txt and file3.tmp should be moved
    let moved_file1_path = setup.dest_dir.join("file1.txt");
    let moved_file3_path = setup.dest_dir.join("file3.tmp");
    assert!(moved_file1_path.exists());
    assert!(moved_file3_path.exists());

    // The excluded file file2.log should not be moved
    let excluded_file_path = setup.source_dir.join("file2.log");
    assert!(excluded_file_path.exists());

    // The moved files should not be in the source directory anymore
    let original_file1_path = setup.source_dir.join("file1.txt");
    let original_file3_path = setup.source_dir.join("file3.tmp");
    assert!(!original_file1_path.exists());
    assert!(!original_file3_path.exists());

    // Verify hashes of moved files
    let original_hash_file1 = &setup.files.iter().find(|(p, _)| p.ends_with("file1.txt")).unwrap().1;
    let moved_hash_file1 = compute_hash(&moved_file1_path).unwrap();
    assert_eq!(original_hash_file1, &moved_hash_file1);

    let original_hash_file3 = &setup.files.iter().find(|(p, _)| p.ends_with("file3.tmp")).unwrap().1;
    let moved_hash_file3 = compute_hash(&moved_file3_path).unwrap();
    assert_eq!(original_hash_file3, &moved_hash_file3);

    remove_dirs(&[setup.source_dir, setup.dest_dir]);
}
