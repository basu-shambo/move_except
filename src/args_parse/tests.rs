use super::*;

fn svec(arr: &[&str]) -> Vec<String> {
    return arr.iter().map(|s| s.to_string()).collect();
}

#[test]
fn test_simple_parsing() {
    let input_args : Vec<String> = svec(&["target/debug/move_except", "-c", "-v",  "this_path", "that_path"]);
    let parsed_args: Result<CLIArgs, ArgsParseError>  = CLIArgs::from(input_args);
    let parsed_args = parsed_args.unwrap();
    let resultant = CLIArgs {
        copy_instead : true,
        print_help : false,
        verbosity : Level::Debug,
        destination: PathBuf::from("that_path"),
        files_to_move: vec![PathBuf::from("this_path")],
        files_to_exclude : vec![],
    };
    assert_eq!(parsed_args, resultant);
}

#[test]
fn test_help() {
    let input_args1 : Vec<String> = svec(&["target/debug/move_except", "-h"]);
    let parsed_args1:  Result<CLIArgs, ArgsParseError>  = CLIArgs::from(input_args1);

    let input_args2 : Vec<String> = svec(&["target/debug/move_except", "--help"]);
    let parsed_args2: Result<CLIArgs, ArgsParseError>  = CLIArgs::from(input_args2);

    assert_eq!(parsed_args1, Err(ArgsParseError::HelpRequested));
    assert_eq!(parsed_args2, Err(ArgsParseError::HelpRequested));

}

#[test]
fn test_help_with_others() {
    let input_args1 : Vec<String> = svec(&["target/debug/move_except", "-hcv"]);
    let parsed_args1:  Result<CLIArgs, ArgsParseError>  = CLIArgs::from(input_args1);

    let input_args2 : Vec<String> = svec(&["target/debug/move_expect", "-h", "-c", "-v"]);
    let parsed_args2: Result<CLIArgs, ArgsParseError> = CLIArgs::from(input_args2);

    let input_args3 : Vec<String> = svec(&["target/debug/move_except", "--help", "--copy", "--verbose"]);
    let parsed_args3:  Result<CLIArgs, ArgsParseError>  = CLIArgs::from(input_args3);

    let input_args4 : Vec<String> = svec(&["target/debug/move_except", "--help", "--verbose"]);
    let parsed_args4:  Result<CLIArgs, ArgsParseError>  = CLIArgs::from(input_args4);

    assert_eq!(parsed_args1, Err(ArgsParseError::HelpRequested));
    assert_eq!(parsed_args2, Err(ArgsParseError::HelpRequested));
    assert_eq!(parsed_args3, Err(ArgsParseError::HelpRequested));
    assert_eq!(parsed_args4, Err(ArgsParseError::HelpRequested));
}

#[test]
fn test_full_length_options() {
    let input_args1 : Vec<String> = svec(&["target/debug/move_except", "--help"]);
    let parsed_args1: Result<CLIArgs, ArgsParseError> = CLIArgs::from(input_args1);

    let input_args2: Vec<String> = svec(&["target/debug/move_except", "--copy", "this_path", "that_path"]);
    let parsed_args2: Result<CLIArgs, ArgsParseError> = CLIArgs::from(input_args2);

    let input_args3: Vec<String> = svec(&["target/debug/move_except", "--verbose", "this_path", "that_path"]);
    let parsed_args3: Result<CLIArgs, ArgsParseError> = CLIArgs::from(input_args3);

    assert_eq!(parsed_args1, Err(ArgsParseError::HelpRequested));
    assert_eq!(parsed_args2.unwrap().copy_instead, true);
    assert_eq!(parsed_args3.unwrap().verbosity, Level::Debug);
}

#[test]
fn test_no_options() {
    let input_args1: Vec<String> = svec(&["target/debug/move_except", "this_path", "that_path"]);
    let parsed_args1: Result<CLIArgs, ArgsParseError> = CLIArgs::from(input_args1);


    let input_args2: Vec<String> = svec(&["target/debug/move_except", "this_path", "that_path", "-e", "those_path*"]);
    let parsed_args2: Result<CLIArgs, ArgsParseError> = CLIArgs::from(input_args2);


    assert!(parsed_args1.is_ok());
    assert!(parsed_args2.is_ok());
}
