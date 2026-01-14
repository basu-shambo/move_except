use glob::glob;
use std::{ borrow::Cow, env, path::{Path, PathBuf}, collections::HashSet};

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

pub fn move_listed_files(files_to_move: Vec<PathBuf>, destination: PathBuf, log_level: Level) {
    for file in files_to_move {
        match std::fs::rename(&file, &destination) {
            Ok(_) => {
                Logger::with_stdout(log_level).debug(&format!("Moving file {} -> {}",file.display(), destination.display())).log();

            }
            Err(e) => {
                Logger::with_stdout(log_level).error(&format!("Error moving {}. {}", file.display(), e)).log();
            }
        }
    }
}

pub fn get_files_to_move_and_destination(files_to_move: Vec<PathBuf>, files_to_exclude: Vec<PathBuf>, destination: String) -> (Vec<PathBuf>, PathBuf) {
    let absolute_paths_to_move = files_to_move.into_iter().map(|s| s.to_absolute_path().into_owned());

    let exclusion_set: HashSet<PathBuf> =files_to_exclude.into_iter().map(|s| s.to_absolute_path().into_owned()).collect();

    let filtered_absolute_paths_to_move: Vec<PathBuf> = absolute_paths_to_move.into_iter().filter(|b| !exclusion_set.contains(b)).collect();

    let absolute_dest_path : PathBuf = PathBuf::from(destination.as_str());
    return (filtered_absolute_paths_to_move, absolute_dest_path);
}

pub fn handle_file_movement( files_to_move: Vec<PathBuf>, destination: PathBuf, log_level: Level) {
    //Check if the destination exists otherwise make the directory
    match destination.is_dir() {
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
    
    move_listed_files(files_to_move, destination, log_level);

}
