mod parser_combinator;
use parser_combinator::*;
//use std::fmt::Debug;

// fn run_and_print<'a, T, V>(parser: Parser<'a, T>, test_data: V) 
//     where T: Debug, V: Iterator<Item = &'a str>
// {
//     for item in test_data {
//         let result = parser(item);
//         println!("item: {:_>10}; parsed: {:?}", item, result);
//     }
// }

fn main() {
    let parser = make_parser_json_object();

    let result = parser("");
    match result {
        None => println!("none"),
        Some((parsed, rest)) => println!("parsed {:?}, rest {}", parsed, rest),
    }
    let result = parser("{}");
    match result {
        None => println!("none"),
        Some((parsed, rest)) => println!("parsed {:?}, rest {}", parsed, rest),
    }
    let result = parser("{\"wasya\": true, \"petya\" : null, \"kostya\": 123142, \"foo\": \"bar\"}");
    match result {
        None => println!("none"),
        Some((parsed, rest)) => println!("parsed {:?}, rest {}", parsed, rest),
    }
}

