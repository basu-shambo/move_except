use crate::logging::{Level, get_help_str};
use glob::glob;
use std::fmt;
use std::path::PathBuf;

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
    pub copy_instead: bool,
    pub print_help: bool,
    pub verbosity: Level,
    pub destination: String,
    pub files_to_move: Vec<String>,
    pub files_to_exclude: Option<Vec<String>>, // None means no exclusions, empty vector means options provided but not path
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
            destination: String::new(),
            files_to_move: Vec::<String>::new(),
            files_to_exclude: None,
        };
    }

    pub fn from(args_vec: Vec<String>) -> Result<Self, ArgsParseError> {
        if args_vec.len() < 2 {
            return Err(ArgsParseError::NotEnoughArgs);
        }
        let new_object: Self = Self::new();
        let mut args_iter = args_vec.into_iter().skip(1).peekable();
        let mut options: Vec<String> = Vec::new();
        // Collect the args that starts with '-' (or '--')
        while args_iter.peek().is_some_and(|s| s.starts_with('-')) {
            options.push(args_iter.next().unwrap());
        }

        // The rest of the options are suppoed to be here
        let paths: Vec<String> = args_iter.collect();
        let (movement_paths, exclusion_paths): (Vec<String>, Option<Vec<String>>) =
            Self::sanitise_paths(paths)?;

        let option_updated_object = new_object.update_options(options);
        // This shouldn't be handled here, but it works
        if let Ok(val) = &option_updated_object
            && val.print_help
        {
            return Err(ArgsParseError::HelpRequested);
        }
        let mut movement_path_updated_object =
            option_updated_object?.update_paths(movement_paths)?;

        movement_path_updated_object.files_to_exclude = exclusion_paths;
        return Ok(movement_path_updated_object);
    }

    fn sanitise_paths(
        rest_of_args: Vec<String>,
    ) -> Result<(Vec<String>, Option<Vec<String>>), ArgsParseError> {
        //
        let mut exclusions_started: bool = false;
        // movement_paths and exclude_paths are misnomers, they are just the globs (maybe paths), will process them later
        let mut movement_paths: Vec<String> = Vec::new();
        let mut exclude_paths: Vec<String> = Vec::new();

        for (i, s) in rest_of_args.into_iter().enumerate() {
            if s.starts_with('-') || s.starts_with("--") {
                if !(s != "-e" || s != "--exclude") {
                    return Err(ArgsParseError::IncorrectOption(s.to_string()));
                } else {
                    exclusions_started = true;

                    // The exclusions are at the end, if they start and the movement paths are less then that means there is an issue
                    if movement_paths.len() < 2 {
                        return Err(ArgsParseError::NotEnoughPaths);
                    }
                }
            } else if exclusions_started {
                exclude_paths.push(s);
            } else {
                assert!(!exclusions_started);
                movement_paths.push(s);
            }
        }

        // Empty exclude_paths means no path was specified even though the option is present, but None means no option provided
        // But I am not worried about that for now
        return Ok((movement_paths, exclusions_started.then_some(exclude_paths)));
    }

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
            } else if option.starts_with("--") {
                let trimmed_option = option.trim_start_matches("--");
                match trimmed_option {
                    "help" => self.print_help = true,
                    "copy" => self.copy_instead = true,
                    "verbose" => self.verbosity = Level::Debug,
                    _ => return Err(ArgsParseError::IncorrectOption(trimmed_option.to_string())),
                }
            } else {
                panic!("{}", format!("This shouldn't be possible -> {option}"));
            }
        }
        return Ok(self);
    }

    fn update_paths(mut self, paths: Vec<String>) -> Result<Self, ArgsParseError> {
        if paths.len() < 2 {
            return Err(ArgsParseError::NotEnoughPaths);
        }
        let mut glob_paths: Vec<String> =
            paths.into_iter().map(|path| String::from(path)).collect();
        self.destination = glob_paths.pop().unwrap();
        self.files_to_move = glob_paths;
        return Ok(self);
    }
}
