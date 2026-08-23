use std::error::Error;
use std::env;
use std::process;
use std::fs;
use std::fmt;
use noob_grep::{search, search_case_insensitive};


////////////////////////////////////////////////////////////////////////////////
pub struct Config {
    pub file_path: String,
    pub query: String,
    pub ignore_case: bool,
}


////////////////////////////////////////////////////////////////////////////////
impl Config {
    // 
    pub fn build2<T>(mut args: T) -> Result<Self, &'static str> 
        where T: Iterator<Item = String>
    {
        args.next(); // skip first element

        let query = match args.next() {
            Some(text) => text,
            None => return Err("Cannot get query from command line"),
        };

        let file_path = match args.next() {
            Some(text) => text,
            None => return Err("Cannot get file path from command line"),
        };
        
        let ignore_case: bool = match args.next() {
            Some(text) => text == "--ignore-case",
            None => env::var("IGNORE_CASE").is_ok(),
        };

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}


////////////////////////////////////////////////////////////////////////////////
impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "File:  {}\nQuery: {}\nIgnore case: {}", self.file_path, self.query, self.ignore_case)
    }
}


////////////////////////////////////////////////////////////////////////////////
fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    let result = fs::read_to_string(cfg.file_path)?;

    //let found = if cfg.ignore_case {
    //    search_case_insensitive(&cfg.query, &result)
    //}
    //else {
    //    search(&cfg.query, &result)
    //};

    //for item in found {
    //    println!("{item}");
    //}
    fn job<'a, T>(obj: T) 
    where T: Iterator<Item = &'a str>
    {
        for item in obj {
            println!("{item}");
        }
    }

    if cfg.ignore_case {
        job(search_case_insensitive(&cfg.query, &result))
    }
    else {
        job(search(&cfg.query, &result))
    }

    Ok(())
}


////////////////////////////////////////////////////////////////////////////////
fn main() {
    let prog_config = Config::build2(env::args()).unwrap_or_else(|res| {
        eprintln!("Config error {}", res);
        process::exit(1);
    });

    if let Err(err) = run(prog_config) {
        eprintln!("Run error {}", err);
        process::exit(1);
    };
}
