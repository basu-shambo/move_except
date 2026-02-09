use std::{
    env, fs,
    path::PathBuf,
};

mod file_handling;
use file_handling::{create_files_from_input_globs, get_files_to_move_and_destination, handle_file_movement,};

mod args_parse;
use args_parse::{ArgsParseError, CLIArgs};

mod logging;
use logging::{Level, Logger};

fn main() {
    let mut program_print_level = Level::Info;
    let args_vec: Vec<String> = env::args().collect();
    let args_rep: Result<CLIArgs, ArgsParseError> = CLIArgs::from(args_vec.clone());
    if let Err(err) = args_rep {
        match err {
            // The help thing is not an error of providing correct args, but its an arg in the sense that the program doesn't really run, instead there is a quick exit
            ArgsParseError::HelpRequested => Logger::with_stdout(Level::Info)
                .info(&err.to_string())
                .log(),
            _ => Logger::with_stdout(Level::Error)
                .error(&err.to_string())
                .log(),
        }
        return;
    }


    let parsed_args: CLIArgs = args_rep.unwrap();
    program_print_level = parsed_args.verbosity;

    let destination: String = parsed_args.destination;
    let globbed_files_to_move: Vec<String> = parsed_args.files_to_move;
    let optional_globbed_files_to_exclude: Option<Vec<String>> = parsed_args.files_to_exclude;

    let files_to_move: Vec<PathBuf> = create_files_from_input_globs(globbed_files_to_move.clone());
    let files_to_exclude: Vec<PathBuf> = match optional_globbed_files_to_exclude {
        Some(globbed_files_to_exclude) if !globbed_files_to_exclude.is_empty() => {
            create_files_from_input_globs(globbed_files_to_exclude.clone())
        }
        _ => Vec::<PathBuf>::new(),
    };

    let (files_to_move, destination_path): (Vec<PathBuf>, PathBuf) = get_files_to_move_and_destination(files_to_move, files_to_exclude, destination);
    
    handle_file_movement(files_to_move, destination_path, program_print_level);
}
