use assert_cmd::Command;
use assert_cmd::cargo::cargo_bin;
use predicates::prelude::*;
use std::io;
use std::env;

mod shared;
use shared::{TestSetup, compute_hash, create_simple_test_setup, remove_dirs, validate_setup};

#[test]
fn test_move_except_single_file() {
    println!("Testing test_move_except_single_file");
    let setup : io::Result<TestSetup> = create_simple_test_setup(vec!["file1"]);
    validate_setup(&setup);

    let TestSetup{source_dir, dest_dir, files} = setup.unwrap();
    println!("source_dir: {}, source: {}", env::current_dir().unwrap().display(), source_dir.display());
    //assert!(env::current_dir().unwrap() == *source_dir, "{} , {}", env::current_dir().unwrap().display(), source_dir.display());
 
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
    println!("Testing test_multiple_files");
    let setup : io::Result<TestSetup> = create_simple_test_setup(vec!["file1", "file2", "file3"]);
    validate_setup(&setup);

    let TestSetup{source_dir, dest_dir, files} = setup.unwrap();
    println!("source_dir: {}, source: {}", env::current_dir().unwrap().display(), source_dir.display());
    //assert!(env::current_dir().unwrap() == *source_dir, "{} , {}", env::current_dir().unwrap().display(), source_dir.display());

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
    println!("Testing test_glob_patterns");
    let setup : io::Result<TestSetup> = create_simple_test_setup(vec!["file1", "file2", "file3"]);
    validate_setup(&setup);

    let TestSetup{source_dir, dest_dir, files} = setup.unwrap();
    println!("source_dir: {}, source: {}", env::current_dir().unwrap().display(), source_dir.display());
    //assert!(env::current_dir().unwrap() == *source_dir, "{} , {}", env::current_dir().unwrap().display(), source_dir.display());

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
    println!("Testing test_exclusions");
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
    println!("source_dir: {}, source: {}", env::current_dir().unwrap().display(), source_dir.display());
    //assert!(env::current_dir().unwrap() == *source_dir, "{} , {}", env::current_dir().unwrap().display(), source_dir.display());

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
    println!("Testing test_new_directory_creation");
    let setup : io::Result<TestSetup> = create_simple_test_setup(vec!["file1"]);
    validate_setup(&setup);

    let TestSetup{source_dir, dest_dir, files} = setup.unwrap();
    println!("source_dir: {}, source: {}", env::current_dir().unwrap().display(), source_dir.display());
    //assert!(env::current_dir().unwrap() == *source_dir, "{} , {}", env::current_dir().unwrap().display(), source_dir.display());
    
    // Create a new destination path by suffixing the existing one, ensuring it doesn't exist
    let mut new_dest_dir = dest_dir.clone();
    new_dest_dir.set_file_name(format!("{}_new", dest_dir.file_name().unwrap().to_str().unwrap()));
    
    let mut post_num = 1;
    while new_dest_dir.exists() && post_num < 100 {
        new_dest_dir.set_file_name(format!("{}_new_{}", dest_dir.file_name().unwrap().to_str().unwrap(), post_num));
        post_num += 1;
    }

    let mut cmd = Command::new(cargo_bin!());
    cmd.current_dir(&source_dir)
        .arg("file1")
        .arg(&new_dest_dir);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!("{} Created!", new_dest_dir.display())));

    assert!(new_dest_dir.exists(), "New destination directory should have been created");

    // Verify file1 was moved from source to the new destination
    let file_moved = new_dest_dir.join("file1");
    assert!(file_moved.exists(), "File should exist in the new destination");
    assert!(!source_dir.join("file1").exists(), "File should no longer exist in source");

    let orig_hash: &String = &files[0].1;
    let current_hash: &String = &compute_hash(file_moved).unwrap();
    assert_eq!(orig_hash, current_hash);

    remove_dirs(&[source_dir, dest_dir, new_dest_dir]);
}

#[test]
fn test_copy_only() {
    println!("Testing test_copy_only with specific globs and exclusions");
    let file_names = vec![
        "a.txt", 
        "b.txt", 
        "c.rs", 
        "d.rs", 
        "e.md"
    ];
    let setup : io::Result<TestSetup> = create_simple_test_setup(file_names.clone());
    validate_setup(&setup);

    let TestSetup{source_dir, dest_dir, files} = setup.unwrap();
    println!("source_dir: {}, source: {}", env::current_dir().unwrap().display(), source_dir.display());
    //assert!(env::current_dir().unwrap() == *source_dir, "{} , {}", env::current_dir().unwrap().display(), source_dir.display());
    
    let mut cmd = Command::new(cargo_bin!());
    cmd.current_dir(&source_dir)
        .arg("-c")       // Copy mode
        .arg("*.txt")    // Match txt files
        .arg("*.rs")     // Match rs files
        .arg(&dest_dir)  // Destination
        .arg("-e")       // Exclude flag
        .arg("b.txt");   // Exclude specific file

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!("{} Already Present!", dest_dir.display())));

    // Helper to check if file exists in dir
    let check_exists = |dir: &std::path::PathBuf, file: &str| dir.join(file).exists();

    // 1. Files that should be COPIED
    let copied_files = vec!["a.txt", "c.rs", "d.rs"];
    for name in &copied_files {
        assert!(check_exists(&dest_dir, name), "File {} should be in destination", name);
        assert!(check_exists(&source_dir, name), "File {} should still be in source", name);
        
        // Check hash
        let src_hash = &files.iter().find(|(p, _)| p.file_name().unwrap() == *name).unwrap().1;
        let dest_path = dest_dir.join(name);
        let dest_hash = compute_hash(&dest_path).unwrap();
        assert_eq!(src_hash, &dest_hash, "Hash mismatch for {}", name);
    }

    // 2. Files that should NOT be copied (Excluded or didn't match)
    let not_copied_files = vec!["b.txt", "e.md"];
    for name in &not_copied_files {
        assert!(!check_exists(&dest_dir, name), "File {} should NOT be in destination", name);
        assert!(check_exists(&source_dir, name), "File {} should still be in source", name);
    }

    remove_dirs(&[source_dir, dest_dir]);
}

#[test]
fn test_verbose_mode() {
    println!("Testing test_verbose_mode");
    let file_names = vec![
        "a.txt", 
        "b.txt", 
        "c.rs", 
        "d.rs", 
        "e.md"
    ];
    let setup : io::Result<TestSetup> = create_simple_test_setup(file_names.clone());
    validate_setup(&setup);

    let TestSetup{source_dir, dest_dir, ..} = setup.unwrap();
    println!("source_dir: {}, source: {}", env::current_dir().unwrap().display(), source_dir.display());
    //assert!(env::current_dir().unwrap() == *source_dir, "{} , {}", env::current_dir().unwrap().display(), source_dir.display());
    
    let mut cmd = Command::new(cargo_bin!());
    cmd.current_dir(&source_dir)
        .arg("-c")       // Copy mode
        .arg("-v")       // Verbose mode
        .arg("*.txt")    // Match txt files
        .arg("*.rs")     // Match rs files
        .arg(&dest_dir)  // Destination
        .arg("-e")       // Exclude flag
        .arg("b.txt");   // Exclude specific file

    let output = cmd.output().expect("Failed to execute command");
    assert!(output.status.success());

    let stdout_str = String::from_utf8_lossy(&output.stdout);
    //println!("Stdout: {}", stdout_str);

    assert!(stdout_str.contains(&format!("{} Already Present!", dest_dir.display())));

    let copied_files = vec!["a.txt", "c.rs", "d.rs"];
    let not_copied_files = vec!["b.txt", "e.md"];

    // 1. Files that should be COPIED and LOGGED
    for name in &copied_files {
        let source_path = source_dir.join(name);
        let dest_path = dest_dir.join(name);
        
        // "Copied file <source> -> <dest>"
        // Note: The format string is constructed to match the `logging.rs` output
        let expected_log = format!("Copied file {} -> {}", source_path.display(), dest_path.display());
        
        assert!(stdout_str.contains(&expected_log), "Stdout should contain log: {}", expected_log);
        assert!(dest_path.exists(), "File {} should exist in destination", name);
    }

    // 2. Files that should NOT be copied (Excluded or didn't match)
    for name in &not_copied_files {
        let source_path = source_dir.join(name);
        let dest_path = dest_dir.join(name);
        
        let log = format!("Copied file {} -> {}", source_path.display(), dest_path.display());
        assert!(!stdout_str.contains(&log), "Stdout should NOT contain log: {}", log);
        assert!(!dest_path.exists(), "File {} should NOT exist in destination", name);
    }

    remove_dirs(&[source_dir, dest_dir]);
}
