#[repr(u8)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Error,
    Warn,
    Info,
    Debug
}


struct Logger {
    log_level: Level,
    lower: bool, // This is meant to tell if the lower logs needs to printed or not.
    print_string: Option<String>,
}

impl Drop for Logger {
    fn drop(&mut self) {
        self.make_final_print();
    }
}
impl Logger {
    fn new(level:Level) -> Self {
        return Logger {
            log_level:level,
            lower:true,
            print_string: None
        };
    }
    fn conditionally_print_log(&mut self, log: &str, curr_level:Level) {
        if (curr_level == self.log_level) || (self.lower &&  (curr_level < self.log_level)) {
            self.print_string.get_or_insert(String::new()).push_str(log.clone());
        }
    }
    fn make_final_print(&mut self) {
        if let Some(log) = &self.print_string{
            println!("{}",log);
        }
    }
    fn only(&mut self) -> &Self {
        self.lower = false;
        return self;
    }
    fn error(&mut self, log: &str) -> &Self {
        self.conditionally_print_log(log, Level::Error);
        return self;
    } 
    fn warn(&mut self, log: &str) -> &Self {
        self.conditionally_print_log(log, Level::Warn);
        return self;
    }
    fn info(&mut self, log: &str) -> &Self {
        self.conditionally_print_log(log, Level::Info);
        return self;
    }
    fn debug(&mut self, log: &str) -> &Self {
        self.conditionally_print_log(log, Level::Debug);
        return self;
    }
} 
