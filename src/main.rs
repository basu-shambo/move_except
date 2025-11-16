use std::{env, fs, path::{Path, PathBuf}};

mod file_handling;
use file_handling::{get_files_to_move, move_listed_files};

mod args_parse;
use args_parse::{CLIArgs};

mod logging;
//use logging::{Logger, Level, log_incorrect_usage, get_help_str};

fn main() {
    let args_vec: Vec<String> = env::args().collect();
    let args_rep : CLIArgs = CLIArgs::from(args_vec.clone());
    println!("{:?}", args_rep);

    return;
    let move_into_str : &str = &args_vec[1];
    //let new_path = Path::new(&args_vec[2]);
    let move_into_path  = Path::new(move_into_str);
    //println!("{}", move_into_path == new_path);
    
    let current_path = "*";
    
    //let mut files_to_move: Vec<PathBuf> = vec![];
    let files_to_move: Vec<PathBuf> = match move_into_path.is_dir() {
        true => get_files_to_move(current_path, Some(move_into_path)),
        false => {
            let files_to_move = get_files_to_move(current_path, None);
            match fs::create_dir(move_into_path) {
                Ok(_) => {},
                Err(e) => panic!("Error Creating the directory {}", e),
            }

            files_to_move
        },
    };
    //move_listed_files(files_to_move, move_into_path);
}
