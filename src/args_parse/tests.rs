use super::*;

fn svec(arr: &[&str]) -> Vec<String> {
    return arr.iter().map(|s| s.to_string()).collect();
}
#[test]
fn test_simple_parsing() {
    let input_args : Vec<String> = svec(&["target/debug/move_except", "-c", "-f", "-e", "this_path", "that_path"]);
    let parsed_args: CLIArgs = CLIArgs::from(input_args);
    assert_eq!(parsed_args, CLIArgs::new());
}


