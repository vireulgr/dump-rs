mod parser_combinator;
use parser_combinator::*;

fn is_backslash(c: &char) -> bool {
    *c == '\\'
}

fn is_not_backslash_or_quote(c: &char) -> bool {
    *c != '\\' && *c != '"'
}

fn unquote(text: &str) -> String {
    let escaped_quote_parser = make_parser_sequence(
        vec![
            make_parser_span_of(&is_backslash, true),
            make_parser_substring("\""),
            make_parser_span_of(&is_not_backslash_or_quote, false),
            make_parser_span_of(&is_backslash, true),
            make_parser_substring("\"")
        ]
    );
    let parsed = escaped_quote_parser(text);

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
    let test_string = "\\\n\\\"wasya\\\n\\\"";
    let test_string_3 = "\"this is string containing \"nested \\\"quotes\\\" and \\\n escape ch: \\\\\\\t\" and\n escape ch: \\\t\"";
    //let _test_string_2 = "{ \"a\": \"some string with \\\"quotes\\\"\" }";

    //let result = unquote(test_string);
    //println!("{}", result);
    let escaped_symbol_parser = make_parser_sequence(
        vec![
            make_parser_span_of(&is_backslash, true),
            make_parser_one_of(
                vec![
                    make_parser_substring("\\"),
                    make_parser_substring("\""),
                    make_parser_substring("\n"),
                    make_parser_substring("\r"),
                    make_parser_substring("\t"),
                ])
        ]);

    let result = escaped_symbol_parser(test_string);
    match result {
        Some((parsed, rest)) => {
            println!("parsed: {:?}; rest: {}", parsed, rest);
        },
        None => {
            println!("no result");
        }
    }
}
