use std::io::Write;
use std::{env, fs};
use std::collections::HashMap;
use std::process;

struct Config {
    file_name: String
}

fn parse_args<'a, T>(args: &mut T) -> Result<Config, &'static str>
where T: Iterator<Item = String>
{
    args.next();
    let file_name = match args.next() {
        Some(s) => s,
        None => {
            return Err("No file name in params!")
        }
    };
    Ok(Config { file_name })
}

fn run(some: Config) -> Result<(), &'static str> {
    let text = match fs::read_to_string(&some.file_name) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("{}", err);
            return Err("cannot read file")
        }
    };

    let result = my_replace(&text);

    let mut replaced_file = match fs::File::create(format!("{}_replaced", &some.file_name)) {
        Ok(file) => file,
        _ => {
            return Err("Cannot create file for writing");
        }
    };

    if let Err(_) = replaced_file.write_all(result.as_bytes()) {
        return Err("Should write to replaced file without errors")
    }

    Ok(())
}

fn my_replace(src: &str) -> String {
    let replaces = HashMap::from([
        ("var" , "\nvar"),
        ("let" , "\nlet"),
        ("const" , "\nconst"),
        ("return" , "\nreturn"),
        ("class" , "\nclass"),
        ("try" , "\ntry"),
        ("catch" , "\ncatch"),
        ("if" , "\nif"),
        ("else" , "\nelse"),
        ("while" , "\nwhile"),
    ]);


    let mut result_vec = Vec::new();
    for line in src.lines() {
        let mut container = Vec::new();
        for entry in &replaces {
            loop {
                // поиск с конца
                if let Some(new_pos) = line.rfind(entry.0) {
                    // если нашли - заменяем
                    let temp: String = line[new_pos..line.len()].into();
                    // и сохраняем изменённую строку для последующей склейки в результат
                    container.push(temp.replace(entry.0, entry.1));
                }
                else { // не нашли - к следующему варианту замены из replaces
                    break;
                }
            }
        }
        let replaced_line = container.iter().rev().fold(String::from(""), |mut acc, item| {
            acc.push_str(item);
            acc
        });
        result_vec.push(replaced_line);
    }


    result_vec.iter().fold(String::from(""), |mut acc, item| {
        acc.push_str(item);
        acc
    })
}

fn main() {
    let config = match parse_args(&mut env::args()) {
        Ok(cfg) => cfg,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(-1);
        }
    };

    if let Err(err) = run(config) {
        eprintln!("{}", err);
        process::exit(-2);
    }
}
