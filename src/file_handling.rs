use std::{env, path::{Path, PathBuf}, borrow::Cow};
use glob::glob;

trait ToAbsolutePath {
    fn to_absolute_path(&self) -> Cow<Path>;
}

impl ToAbsolutePath for Path {
    fn to_absolute_path(&self) -> Cow<Path> {
        if self.is_absolute() {
            return Cow::Borrowed(self);
        }
        else {
            assert!(self.is_relative(), "This should be a relative path {}", self.display());
            let pwd = env::current_dir().unwrap();
            return Cow::Owned(pwd.join(self));
        }
    }
}
pub fn get_files_to_move(glob_str:&str, excluded_path: Option<&Path>) -> Vec<PathBuf> {
    //let pwd: &Path = env::current_dir();
    //println!("{}", pwd.display());
    let vec = if let Some(excluded_path) = excluded_path {
        //println!("{:?}", excluded_path);
        glob(glob_str)
            .expect("Failed to read glob pattern")
            .filter_map(|e| {
                e.ok().and_then(|pathbuf| {
                    if pathbuf.to_absolute_path() == excluded_path.to_absolute_path() {
                        None
                    }
                    else {
                        Some(pathbuf)
                    }
                })
            })
            .collect()
            
    }
    else {
        glob(glob_str)
            .expect("Failed to read glob pattern")
            .filter_map(Result::ok)
            .collect()


    };
    //println!("{:#?}", vec);
    return vec;
}

pub fn move_listed_files(files_to_move:Vec<PathBuf>, into_path:&Path) {
    for file in files_to_move {
        let new_dest = into_path.join(file.file_name().unwrap());
        match std::fs::rename(&file, &new_dest) {
            Ok(_) => {},
            Err(e) => println!("Error moving file from {} -> {}", file.display(), e),
        }
    }    
}

