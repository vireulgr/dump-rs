pub trait Decor {
    fn display(&self);
}

pub enum LogLevel {
    Error,
    Warn,
    Info,
}

impl LogLevel {
    fn to_string(&self) -> String {
        match self {
            LogLevel::Error =>  String::from("E"),
            LogLevel::Warn =>  String::from("W"),
            LogLevel::Info =>  String::from("I"),
        }
    }
}

pub struct LogMessage {
    pub text: String,
    pub severity: LogLevel,
}

pub struct HelpMessage {
    pub command: String,
    pub short: String,
    pub required: bool,
    pub description: String,
}


impl Decor for LogMessage {
    fn display(&self) {
        println!(">>>[{}] {}<<<", self.severity.to_string(), self.text)
    }
}

impl Decor for HelpMessage {
    fn display(&self) {
        let req_str = if self.required { String::from("required") } else { String::from("") };
        println!("{} ({}) {}\n    {}", self.command, self.short, req_str, self.description);
    }
}

pub fn print_decorated(what: &impl Decor) {
    what.display();
}
