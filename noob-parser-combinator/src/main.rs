mod parser_combinator;
use parser_combinator::*;

fn is_backslash(c: &char) -> bool {
    *c == '\\'
}

fn unquote(text: &str) -> String {
    let parser = make_parser_sequence(
        vec![
            make_parser_span_of(&is_backslash, true),
            make_parser_substring("\"")
        ]
    );
    let parsed = parser(text);

    match parsed {
        Some((parsed, rest)) => {
            println!("parsed: {:?}; rest: {}", parsed, rest);
        },
        None => {
            println!("no result");
        }
    }

    String::new()
}

fn main() {
    let test_string = "\\\\\"wasya\\\\\"";
    let _test_string_2 = "{ \"a\": \"some string with \\\"quotes\\\"\" }";

    let result = unquote(test_string);
    println!("{}", result);
}
