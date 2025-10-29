use std::path::PathBuf;

#[derive(Debug)]
pub struct CLIArgs {
    copy_instead : bool,
    print_help : bool,
    verbosity: u32,
    destination: Option<PathBuf>,
    files_to_move: Vec<PathBuf>,
    files_to_exclude: Vec<PathBuf>,

}

impl CLIArgs {
    fn empty() -> Self {
        return CLIArgs {
            copy_instead: false,
            print_help: false,
            verbosity: 0,
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

fn print_help() {
    println!(
        "\
    move_except - Move or copy files with optional exclusions

USAGE:
    mve [options] <files_to_move> <destination>
    mve [options] <files_to_move> <destination> -e, --exclude <files_or_folders_to_exclude>

OPTIONS:
    -c, --copy        Copy files instead of moving them
    -h, --help        Show this help message and exit
    -v, --verbose     Print whats going on 

EXAMPLES:
    mve file1.txt file2.txt /backup/
    mve -c src/*.rs /tmp/
    mve dir1 dir2 --exclude node_modules .git
"
    );
}
