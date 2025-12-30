use std::path::PathBuf;
use std::fmt;

use crate::logging::{Level, get_help_str};

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub enum ArgsParseError {
    NotEnoughArgs,
    HelpRequested,
    NotEnoughPaths,
    IncorrectOption(String),
}

#[derive(Debug, PartialEq)]
pub struct CLIArgs {
    pub copy_instead : bool,
    pub print_help : bool,
    pub verbosity: Level, 
    pub destination: PathBuf,
    pub files_to_move: Vec<PathBuf>,
    pub files_to_exclude: Vec<PathBuf>,

}

impl fmt::Display for ArgsParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArgsParseError::NotEnoughArgs => write!(f, "Not enough arguments provided."),
            ArgsParseError::HelpRequested => write!(f, "{}", get_help_str()),
            ArgsParseError::NotEnoughPaths => write!(f, "Not enough paths provided."),
            ArgsParseError::IncorrectOption(option) => write!(f, "Incorrect option: {}", option),
        }
    }
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

    pub fn from(args_vec:Vec<String>) -> Result<Self, ArgsParseError> {
        if args_vec.len() < 2 {
            return Err(ArgsParseError::NotEnoughArgs);
        }
        let new_object : Self = Self::new();
        let mut args_iter = args_vec.into_iter().skip(1).peekable();
        let mut options: Vec<String> = Vec::new();
        // Collect the args that starts with '-' (or '--') 
        while args_iter.peek().is_some_and(|s| s.starts_with('-')) {
            options.push(args_iter.next().unwrap());
        }

        // The rest of the options are suppoed to be here
        let paths:Vec<String> = args_iter.collect();

        let option_updated_object = new_object.update_options(options);
        if let Ok(val) = &option_updated_object && val.print_help  {
            return Err(ArgsParseError::HelpRequested);
        }
        let path_updated_object = option_updated_object?.update_paths(paths);
        

        return path_updated_object;
    }
    
    //fn santise_paths(mut self, rest_of_args: Vec<String>) -> Result<(Vec<PathBuf>, Option<Vec<PathBuf>>), ArgsParseError> {
    //    todo!();
    //} 

    fn update_options(mut self, options: Vec<String>) -> Result<Self, ArgsParseError> {
        for option in options {
            if option.starts_with('-') && !option.starts_with("--") {
                for a in option.trim_start_matches('-').chars() {
                    match a {
                        'h' => self.print_help = true, // this should actually return the HelpRequested error, but keeping it for now
                        'c' => self.copy_instead = true,
                        'v' => self.verbosity = Level::Debug,
                        _ => return Err(ArgsParseError::IncorrectOption(a.to_string())), 
                    }; 
                }
            }
            else if option.starts_with("--") {
                let trimmed_option = option.trim_start_matches("--");
                match trimmed_option {
                    "help" => self.print_help = true,
                    "copy" => self.copy_instead = true,
                    "verbose" => self.verbosity = Level::Debug,
                    _ => return Err(ArgsParseError::IncorrectOption(trimmed_option.to_string())),
                }
            }
            else {
                panic!("{}",format!("This shouldn't be possible -> {option}"));
            }
            
        }
        return Ok(self);
    }

    fn update_paths(mut self, paths: Vec<String>) -> Result<Self, ArgsParseError> {
        if paths.len() < 2 {
            return Err(ArgsParseError::NotEnoughPaths);
        }
        let mut path_bufs: Vec<PathBuf> = paths.into_iter().map(|path| PathBuf::from(path)).collect();
        self.destination = path_bufs.pop().unwrap();
        self.files_to_move = path_bufs; 
        return Ok(self);
    }
} 
