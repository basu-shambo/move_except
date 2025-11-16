use std::path::PathBuf;

use crate::logging::{Level, Logger, log_incorrect_usage, get_help_str};

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct CLIArgs {
    copy_instead : bool,
    print_help : bool,
    verbosity: Level, 
    destination: PathBuf,
    files_to_move: Vec<PathBuf>,
    files_to_exclude: Vec<PathBuf>,

}

impl CLIArgs {
    pub fn new() -> Self {
        return CLIArgs {
            copy_instead: false,
            print_help: false,
            verbosity: Level::Info,
            destination: PathBuf::new(), 
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
    pub fn update_paths(&mut self, paths: Vec<String>) {
        if paths.len() < 2 {
            log_incorrect_usage(Some(Logger::with_stdout(Level::Error).error("Provide the correct Paths")));
        }
        let mut path_bufs: Vec<PathBuf> = paths.into_iter().map(|path| PathBuf::from(path)).collect();
        self.destination = path_bufs.pop().unwrap();
        self.files_to_move = path_bufs; 
    }

    pub fn from(args_vec:Vec<String>) -> Self {
        if args_vec.len() < 2 {
            log_incorrect_usage(None);
            std::process::exit(1);
        }
        let mut new_object : Self = Self::new();
        let mut args_iter = args_vec.into_iter().skip(1).peekable();
        let mut options: Vec<String> = Vec::new();
        while args_iter.peek().is_some_and(|s| s.starts_with('-')) {
            options.push(args_iter.next().unwrap());
        }
        let paths:Vec<String> = args_iter.collect();

        println!("{:?}", options);
        println!("{:?}", paths);
        new_object.update_options(options);
        if new_object.print_help {
            Logger::with_stdout(Level::Info).info(get_help_str()).log();
            std::process::exit(1);
        }
        new_object.update_paths(paths);
        return new_object;
    }
} 
