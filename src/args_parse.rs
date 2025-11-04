use std::path::PathBuf;

use crate::logging::Level;

#[derive(Debug)]
pub struct CLIArgs {
    copy_instead : bool,
    print_help : bool,
    verbosity: Level, 
    destination: Option<PathBuf>,
    files_to_move: Vec<PathBuf>,
    files_to_exclude: Vec<PathBuf>,

}

impl CLIArgs {
    fn empty() -> Self {
        return CLIArgs {
            copy_instead: false,
            print_help: false,
            verbosity: Level::Info,
            destination: None,
            files_to_move: Vec::<PathBuf>::new(),
            files_to_exclude : Vec::<PathBuf>::new()
        };
    }
    pub fn new(args_vec:Vec<String>) -> Self {
        let mut args_iter = args_vec.iter().skip(1).peekable(); 
        let mut options:Vec<String> = Vec::new();
        if args_iter.peek().is_some_and(|s| s.starts_with('-')) {
            while let Some(arg) = args_iter.next() {
                options.push(arg.clone());
            }
        }
        // Next should be the Paths to be moved/copied

        println!("{:?}", options);
        return Self::empty();
    }
} 
