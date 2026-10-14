//pub mod parser_combinator {
    pub type ParserResult<T> = Option<(T, String)>;
    pub type Parser<'a, T> = Box<dyn Fn(&str) -> ParserResult<T> + 'a>;

    #[allow(unused)]
    pub fn make_parser_substring<'a>(pattern: &'a str) -> Parser<'a, String> {
        Box::new(move |text| {
            if text.starts_with(pattern) {
                return Some(
                    (
                        String::from(pattern),
                        String::from_iter(
                            text
                               .chars()
                               .skip(pattern.len())
                        )
                    )
                );
            }

            None
        })
    }

    #[allow(unused)]
    pub fn make_parser_span_of<'a>(predicate: Box<dyn Fn(&char) -> bool>, non_empty: bool) -> Parser<'a, String> {
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
            if parsed_string.is_empty() && non_empty {
                return None;
            }

            let mut rest_string = match maybe_current_char {
                Some(c) => String::from(c) + &String::from_iter(text_iter),
                None => String::from_iter(text_iter)
            };

            Some((parsed_string, rest_string))
        })
    }

    #[allow(unused)]
    pub fn make_parser_sequence<'a>(parsers: Vec<Parser<'a, String>>) -> Parser<'a, Vec<String>> {
        Box::new(move |text| {
            let mut results: Vec<String> = Vec::new();
            let mut temp = String::from(text);
            for parser in &parsers {
                let result = parser(&temp);
                if result.is_none() {
                    return None;
                }
                let (t, rest) = result.unwrap();
                temp = rest;
                results.push(t);
            }
            Some((results, temp))
        })
    }

    #[allow(unused)]
    pub fn make_parser_one_of<'a>(parsers: Vec<Parser<'a, String>>) -> Parser<'a, String> {
        Box::new(move |text| {
            let mut temp = String::from(text);
            for parser in &parsers {
                let result = parser(&temp);
                if result.is_some() {
                    return result;
                }
            }

            None
        })
    }
//}

#[cfg(test)]
mod test {

    use super::*;
    use std::fmt::Debug;
    //use std::fmt::Display;

    fn test_and_print<'a, T, V>(parser: Parser<'a, T>, test_data: V) 
        where T: Debug, V: Iterator<Item = &'a str>
    {
        for item in test_data {
            let result = parser(item);
            println!("item: {:_>10}; parsed: {:?}", item, result);
        }
    }

    #[test]
    fn space_parser_works() {
        let test_set_space: [&str; 4] = [" ", "    ", " some", ""];
        let spaces = make_parser_span_of(Box::new(|c| c.is_whitespace()), false);
        test_and_print(spaces, test_set_space.into_iter());
    }

    #[test]
    fn natural_number_parser_works() {
        let test_set_natural: [&str; 4] = ["8", "2324", "8some", ""];
        let natural = make_parser_span_of(Box::new(|c| ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'].binary_search(c).is_ok()), false);
        test_and_print(natural, test_set_natural.into_iter());
    }

    #[test]
    fn substring_parser_works() {
        let test_set_substr: [&str; 4] = ["foo", "foobarbaz", "foooolr[", ""];
        let subs = make_parser_substring("foo");
        test_and_print(subs, test_set_substr.into_iter());
    }

    #[test]
    fn sequence_parser_works() {
        let test_set_substr: [&str; 4] = ["foo 2131", "foo23", "foo 2   ", ""];
        let sequence = make_parser_sequence(
            vec![
                make_parser_substring("foo"),
                make_parser_span_of(Box::new(|c| c.is_whitespace()), true),
                make_parser_span_of(Box::new(|c| ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'].binary_search(c).is_ok()), true),
                make_parser_span_of(Box::new(|c| c.is_whitespace()), false),
            ]);
        test_and_print(sequence, test_set_substr.into_iter());
    }
}
