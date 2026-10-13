type ParserResult = (Vec<String>, String);

fn space(source: &str) -> ParserResult {
    let mut source_iter = source.chars();
    let maybe_first_char = source_iter.next();
    if maybe_first_char.is_none() {
        return (vec![String::from("")], String::from(source));
    }
    let first_char = maybe_first_char.unwrap();

    if !first_char.is_whitespace() {
        return (vec![String::from("")], String::from(source));
    }
    return (vec![String::from(first_char)], String::from(source_iter.as_str()));
}

fn sequence(source: &str, chars: &str) -> ParserResult {
    (vec![], String::from(""))
}

const TEST_STRING: &str = "{\"ver\": \"12321\", \"the\":[\"value\", \"value2\"]}";


fn main() {
    let test_set: Vec<String> = vec!(String::from(" "), String::from("    "),  String::from(" some"));
    for item in test_set {
        println!("item: {:?}", item);
        println!("parsed: {:?}", space(item.as_str()));
    }
}
