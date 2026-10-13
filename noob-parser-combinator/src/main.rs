type ParserResult<T> = Option<(T, String)>;
type Parser<T> = Box<dyn Fn(&str) -> ParserResult<T>>;

fn span_of(predicate: Box<dyn Fn(&char) -> bool>) -> Parser<String> {
    Box::new(move |text| {
        let mut parsed_string = String::new();
        let mut text_iter = text.chars();
        let mut maybe_current_char = text_iter.next();
        while maybe_current_char.is_some() {
            let current_char = maybe_current_char.unwrap();
            if predicate(&current_char) {
                parsed_string.push(current_char);
                maybe_current_char = text_iter.next();
            }
            else {
                break;
            }
        }
        Some((parsed_string, String::from_iter(text_iter)))
    })
}

fn non_empty_span_of(predicate: Box<dyn Fn(&char) -> bool>) -> Parser<String> {
    Box::new(move |text| {
        let mut parsed_string = String::new();
        let mut text_iter = text.chars();
        let mut maybe_current_char = text_iter.next();
        while maybe_current_char.is_some() {
            let current_char = maybe_current_char.unwrap();
            if predicate(&current_char) {
                parsed_string.push(current_char);
                maybe_current_char = text_iter.next();
            }
            else {
                break;
            }
        }
        if parsed_string.is_empty() {
            return None;
        }
        Some((parsed_string, String::from_iter(text_iter)))
    })
}

fn space(source: &str) -> ParserResult<Vec<String>> {
    let mut source_iter = source.chars();
    let maybe_first_char = source_iter.next();
    if maybe_first_char.is_none() {
        return Some((vec![String::from("")], String::from(source)));
    }
    let first_char = maybe_first_char.unwrap();

    if !first_char.is_whitespace() {
        return Some((vec![String::from("")], String::from(source)));
    }
    return Some((vec![String::from(first_char)], String::from(source_iter.as_str())));
}

fn sequence(source: &str, chars: &str) -> ParserResult<Vec<String>> {
    Some((vec![], String::from("")))
}

//const TEST_STRING: &str = "{\"ver\": \"12321\", \"the\":[\"value\", \"value2\"]}";
const TEST_SET_SPACE: [&str; 4] = [" ", "    ", " some", ""];
const TEST_SET_ZEROS: [&str; 4] = ["0", "0000", "0some", ""];
const TEST_SET_NATURAL: [&str; 4] = ["8", "2324", "8some", ""];

fn main() {

    let natural = span_of(Box::new(|c| ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'].binary_search(c).is_ok()));
    let span_of_0s = span_of(Box::new(|c| *c == '0'));
    let space2 = span_of(Box::new(|c| c.is_whitespace()));
    for item in TEST_SET_NATURAL {
        println!("item: {:_>10}; parsed: {:?}", item, natural(item).unwrap().0);
    }
    for item in TEST_SET_ZEROS {
        println!("item: {:_>10}; parsed: {:?}", item, span_of_0s(item).unwrap().0);
    }
    for item in TEST_SET_SPACE {
        println!("item: {:_>10}; parsed: {:?}", item, space2(item).unwrap().0);
    }
}
