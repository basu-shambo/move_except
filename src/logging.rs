use std::io::Write;

#[cfg(test)]
mod tests;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Error,
    Warn,
    Info,
    Debug
}


pub struct Logger< Writable: Write> {
    writer : Writable,
    log_level: Level,
    lower: bool, // This is meant to tell if the lower logs needs to printed or not.
    print_string: Option<String>,
}

impl Logger<std::io::Stdout> {
    pub fn with_stdout(level: Level) -> Self {
        return Logger {
            writer: std::io::stdout(),
            log_level: level,
            lower: true,
            print_string: None
        };
    }
}

impl<Writable: Write> Logger<Writable> {
    pub fn with(level:Level, writer: Writable) -> Self {
        return Logger {
            writer: writer,
            log_level:level,
            lower:true,
            print_string: None
        };
    }
    fn conditionally_print_log(&mut self, log: &str, curr_level:Level) {
        if (curr_level == self.log_level) || (self.lower &&  (curr_level < self.log_level)) {
            self.print_string.get_or_insert(String::new()).push_str(log);
        }
    }
    fn make_final_print(mut self) {
        if let Some(log) = &self.print_string{
            let write_out = writeln!(self.writer,"{}",log);
            if let Err(_o) = write_out {
                panic!("Absolute Idiocy, can't even write to memory");
            }
        }
    }
    pub fn log(self) {
        self.make_final_print();
    }
    pub fn only(mut self) -> Self {
        self.lower = false;
        return self;
    }
    pub fn error(mut self, log: &str) -> Self {
        self.conditionally_print_log(log, Level::Error);
        return self;
    } 
    pub fn warn(mut self, log: &str) -> Self {
        self.conditionally_print_log(log, Level::Warn);
        return self;
    }
    pub fn info(mut self, log: &str) -> Self {
        self.conditionally_print_log(log, Level::Info);
        return self;
    }
    pub fn debug(mut self, log: &str) -> Self {
        self.conditionally_print_log(log, Level::Debug);
        return self;
    }
} 

pub fn help_str() -> &'static str {
    return 
        "\
    move_except - Move or copy files with optional exclusions

    USAGE:
        mve [options] <files_to_move> <destination> [-e, --exclude <files_or_folders_to_exclude>]

    OPTIONS:
        -c, --copy        Copy files instead of moving them
        -h, --help        Show this help message and exit
        -v, --verbose     Print whats going on 

    EXAMPLES:
        mve file1.txt file2.txt /backup/
        mve -c src/*.rs /tmp/
        mve dir1 dir2 --exclude node_modules .git
    ";
}

pub fn log_incorrect_usage() {
    return Logger::with_stdout(Level::Info).error("This isn't the correct usage\n").info(help_str()).log();
}
