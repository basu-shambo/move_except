use glob::glob;
use std::{ borrow::Cow, env, path::{Path, PathBuf}, collections::HashSet};
use std::fs;
use std::io;
use std::ffi::OsStr;

use crate::logging::{Logger, Level};

trait ToAbsolutePath {
    fn to_absolute_path(&self) -> Cow<Path>;
}

impl ToAbsolutePath for Path {
    fn to_absolute_path(&self) -> Cow<Path> {
        if self.is_absolute() {
            return Cow::Borrowed(self);
        } else {
            assert!(
                self.is_relative(),
                "This should be a relative path {}",
                self.display()
            );
            let pwd = env::current_dir().unwrap();
            return Cow::Owned(pwd.join(self));
        }
    }
}

impl ToAbsolutePath for &PathBuf {
    fn to_absolute_path(&self) -> Cow<Path> {
        return self.as_path().to_absolute_path();
    }
}

fn get_files_from_glob(glob_str: &str, excluded_path: Option<&Vec<PathBuf>>) -> Vec<PathBuf> {
    let vec = if let Some(excluded_path) = excluded_path {
        glob(glob_str)
            .expect("Failed to read glob pattern")
            .filter_map(|e| {
                e.ok().and_then(|pathbuf| {
                    if pathbuf.to_absolute_path() == excluded_path[0].to_absolute_path() {
                        None
                    } else {
                        Some(pathbuf)
                    }
                })
            })
            .collect()
    } else {
        glob(glob_str)
            .expect("Failed to read glob pattern")
            .filter_map(Result::ok)
            .collect()
    };
    return vec;
}

pub fn create_files_from_input_globs(input_globs: Vec<String>) -> Vec<PathBuf> {
    let paths_from_globs: Vec<PathBuf> = input_globs
        .into_iter()
        .flat_map(|s| get_files_from_glob(&s, None))
        .collect();

    return paths_from_globs;
}

fn do_actual_transfer(file: &PathBuf, destination: &Path, copy_instead: bool ) -> Result<(), io::Error> {
    if copy_instead {
        return fs::copy(file, destination).map(|_| ());
    }
    else {
        return fs::rename(file,destination);
    }
}

pub fn handle_file_movement_core(files_to_move: Vec<PathBuf>, destination: PathBuf, log_level: Level, copy_instead:bool) {
    for file in files_to_move {
        assert!(destination.is_dir(), "This has to be a directory to work correctly ");
          
        let file_name_opt : Option<&OsStr> = file.file_name();
        if file_name_opt.is_none() {
            Logger::with_stdout(log_level).warn(&format!("Ignoring {}, as this can't be moved",file.display()));
            continue;
        }
        let dest_file_path = destination.join(file_name_opt.unwrap());
        let abs_destination_path = dest_file_path.to_absolute_path();
        let logging_terms = if copy_instead {("Copied", "copying")} else {("Moved", "moving")} ;
        match do_actual_transfer(&file, &abs_destination_path, copy_instead) {
            Ok(_) => {
                Logger::with_stdout(log_level).debug(&format!("{} file {} -> {}",logging_terms.0, file.display(), abs_destination_path.display())).log();

            }
            Err(e) => {
                Logger::with_stdout(log_level).error(&format!("Error {} {}. {}", logging_terms.1, file.display(), e)).log();
            }
        }
    }
}

pub fn get_files_to_move_and_destination(files_to_move: Vec<PathBuf>, files_to_exclude: Vec<PathBuf>, destination: String) -> (Vec<PathBuf>, PathBuf) {
    let absolute_paths_to_move = files_to_move.into_iter().map(|s| s.to_absolute_path().into_owned());

    let exclusion_set: HashSet<PathBuf> = files_to_exclude.into_iter().map(|s| s.to_absolute_path().into_owned()).collect();

    let filtered_absolute_paths_to_move: Vec<PathBuf> = absolute_paths_to_move.into_iter().filter(|b| !exclusion_set.contains(b)).collect();

    let absolute_dest_path : PathBuf = PathBuf::from(destination.as_str());
    return (filtered_absolute_paths_to_move, absolute_dest_path);
}

pub fn handle_file_movement( files_to_move: Vec<PathBuf>, destination: PathBuf, log_level: Level, copy_instead: bool) {
    //Check once that the destination doesn't exists or it exists and is a directory
    let is_dest_dir : bool = destination.is_dir();
    if destination.exists() && !is_dest_dir {
        Logger::with_stdout(log_level).error("{} is not a directory, and can't be used to move into").log();
    }
    // We know at this point that either the 'destination doesn't exist or its definitily a directory


    //Check if the destination exists otherwise make the directory
    match is_dest_dir {
        false => match std::fs::create_dir(&destination) {
            Ok(_) => {
                Logger::with_stdout(log_level).info(&format!("{} Created!",destination.display())).log();
            }
            Err(e) => {
                Logger::with_stdout(log_level).error(&format!("{} Can't be created.\n {}", destination.display(), e)).log();
                return;
            } 
        },
        true => {
            Logger::with_stdout(log_level).info(&format!("{} Already Present!", destination.display())).log();
        }
    };
    
    handle_file_movement_core(files_to_move, destination, log_level, copy_instead);

}
