use glob::glob;
use std::{
    borrow::Cow,
    env,
    collections::HashSet,
    path::{Path, PathBuf},
};

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

pub fn get_files_to_move_and_destination(files_to_move: Vec<PathBuf>, files_to_exclude: Vec<PathBuf>, destination: &String) -> (Vec<PathBuf>, PathBuf) {
    let absolute_paths_to_move: Vec<PathBuf> = files_to_move
        .into_iter()
        .map(|s| s.to_absolute_path().into_owned())
        .collect();

    let absolute_path_to_exclude: Vec<PathBuf> = files_to_exclude
        .into_iter()
        .map(|s| s.to_absolute_path().into_owned())
        .collect();

    let destination: PathBuf = Path::new(&destination).to_absolute_path().into_owned();

    let exclude_set : HashSet<PathBuf> = absolute_path_to_exclude.into_iter().collect(); 
    let filtered_paths_to_move : Vec<PathBuf> = absolute_paths_to_move.into_iter().filter(|s| !exclude_set.contains(s)).collect();
    return (filtered_paths_to_move, destination);
}

pub fn get_files_from_glob(glob_str: &str, excluded_path: Option<&Vec<PathBuf>>) -> Vec<PathBuf> {
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

pub fn move_listed_files(files_to_move: Vec<PathBuf>, into_path: &Path) {
    for file in files_to_move {
        let new_dest = into_path.join(file.file_name().unwrap());
        match std::fs::rename(&file, &new_dest) {
            Ok(_) => {}
            Err(e) => println!("Error moving file from {} -> {}", file.display(), e),
        }
    }
}

pub fn handle_file_movement(
    files_to_move: &Vec<PathBuf>,
    destination: &PathBuf,
    files_to_exclude: &Vec<PathBuf>,
) {
    //Check if the destination exists otherwise make the directory
    match destination.is_dir() {
        false => match std::fs::create_dir(destination) {
            Ok(_) => {}
            Err(e) => panic!("Error Create the directory {}\n", e),
        },
        true => {}
    };
    println!("{:?}", files_to_move);

    //Get the input paths and then check if the glob matches
    //1. Expand the glob into path and check which files match it and collect into a vector

    //2. expand the files_to_exclude and check which files matches it and then collect it into a vector

    //3. remove the files in the exclude_list from the files_to_move
}
