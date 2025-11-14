use std::path::PathBuf;

use crate::logging::{Level, Logger, log_incorrect_usage};

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct CLIArgs {
    copy_instead : bool,
    print_help : bool,
    verbosity: Level, 
    destination: Option<PathBuf>,
    files_to_move: Vec<PathBuf>,
    files_to_exclude: Vec<PathBuf>,

}

impl CLIArgs {
    pub fn new() -> Self {
        return CLIArgs {
            copy_instead: false,
            print_help: false,
            verbosity: Level::Info,
            destination: None,
            files_to_move: Vec::<PathBuf>::new(),
            files_to_exclude : Vec::<PathBuf>::new()
        };
    }
    pub fn update_options( &mut self, options: Vec<String>) {
        for option in options {
            if option.starts_with('-') && !option.starts_with("--") {
                for a in option.trim_start_matches('-').chars() {
                    match a {
                        'h' => self.print_help = true,
                        'c' => self.copy_instead = true,
                        'v' => self.verbosity = Level::Debug,
                        _ => { Logger::with_stdout(Level::Debug).info(&format!("{a} is a wrong, ignoring {a}")).log();}
                    }; 
                }
            }
            else if option.starts_with("--") {
                todo!();
            }
            else {
                panic!("{}",format!("This shouldn't be possible -> {option}"));
            }
            
        }
    }
    pub fn from(args_vec:Vec<String>) -> Self {
        if args_vec.len() < 2 {
            log_incorrect_usage();
            std::process::exit(1);
        }
        let mut my_type : Self = Self::new();
        let mut args_iter = args_vec.iter().skip(1).peekable(); 
        let mut options:Vec<String> = Vec::new();
        if args_iter.peek().is_some_and(|s| s.starts_with('-')) {
            while let Some(arg) = args_iter.next() {
                println!("{} {:?}", args_iter.len(), arg);
                options.push(arg.clone());
            }
        }
        my_type.update_options(options);
        // Next should be the Paths to be moved/copied
        let mut input_path: Vec<PathBuf> = Vec::new();
        if args_iter.len() == 0 {
            log_incorrect_usage();
        }
        //println!("{:?}", options);
        return my_type;
    }
} 
