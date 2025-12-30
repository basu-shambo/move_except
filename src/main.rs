use std::{env, fs, path::{Path, PathBuf}};

mod file_handling;
use file_handling::{handle_file_movement};

mod args_parse;
use args_parse::{CLIArgs, ArgsParseError};

mod logging;
use logging::{Logger, Level};

fn main() {
    let PROGRAM_PRINT_LEVEL = Level::Info;
    let args_vec: Vec<String> = env::args().collect();
    let args_rep : Result<CLIArgs, ArgsParseError> = CLIArgs::from(args_vec.clone());
    if let Err(err) = args_rep {
        match err {
            // The help thing is not an error of providing correct args, but its an arg in the sense that the program doesn't really run, instead there is a quick exit
            ArgsParseError::HelpRequested => Logger::with_stdout(Level::Info).info(&err.to_string()).log(),
            _ => Logger::with_stdout(Level::Error).error(&err.to_string()).log(),
        }
        return;
    }

    let parsed_args: CLIArgs = args_rep.unwrap();
    let destination: &PathBuf = &parsed_args.destination;
    let files_to_move: &Vec<PathBuf> = &parsed_args.files_to_move;
    let files_to_exclude: &Vec<PathBuf> = &parsed_args.files_to_exclude;

    handle_file_movement(files_to_move, destination, files_to_exclude);


    
    //let move_into_str : &str = &args_vec[0];
    //let move_into_path  = Path::new(move_into_str);
    ////println!("{}", move_into_path == new_path);
    //
    //let current_path = "*";
    //
    ////let mut files_to_move: Vec<PathBuf> = vec![];
    //let files_to_move: Vec<PathBuf> = match move_into_path.is_dir() {
    //    true => get_files_to_move(current_path, Some(move_into_path)),
    //    false => {
    //        let files_to_move = get_files_to_move(current_path, None);
    //        match fs::create_dir(move_into_path) {
    //            Ok(_) => {},
    //            Err(e) => panic!("Error Creating the directory {}", e),
    //        }

    //        files_to_move
    //    },
    //};
    ////move_listed_files(files_to_move, move_into_path);
}
