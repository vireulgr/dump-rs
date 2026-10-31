
//pub mod parser_combinator {
    pub type ParserResult<T> = Option<(T, String)>;
    pub type Parser<'a, T> = Box<dyn Fn(&str) -> ParserResult<T> + 'a>;

    #[allow(unused)]
    pub fn natural_number_predicate(c: &char) -> bool {
        ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'].binary_search(c).is_ok()
    }

    #[allow(unused)]
    pub fn is_alphanumeric_predicate(c: &char) -> bool {
        c.is_ascii_alphanumeric()
    }

    #[allow(unused)]
    pub fn whitespace_predicate(c: &char) -> bool {
        c.is_whitespace()
    }

    // captures foo in foobar string
    #[allow(unused)]
    pub fn make_parser_substring<'a>(pattern: &'a str) -> Parser<'a, String> {
        Box::new(move |text: &str| {
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
    pub fn make_parser_span_of<'a>(predicate: &'a (dyn Fn(&char) -> bool + 'a)) -> Parser<'a, String> {
        make_parser_span_of_range(predicate, 0, 1 << 31 - 1)
    }
    #[allow(unused)]
    pub fn make_parser_span_of_exact<'a>(predicate: &'a (dyn Fn(&char) -> bool + 'a), quantity: usize) -> Parser<'a, String> {
        make_parser_span_of_range(predicate, quantity, quantity)
    }
    #[allow(unused)]
    pub fn make_parser_span_of_at_least<'a>(predicate: &'a (dyn Fn(&char) -> bool + 'a), at_least: usize) -> Parser<'a, String> {
        make_parser_span_of_range(predicate, at_least, 1 << 31 - 1)
    }

    // captures cccc (where c is char) in strings like ccccamel
    #[allow(unused)]
    pub fn make_parser_span_of_range<'a>(predicate: &'a (dyn Fn(&char) -> bool + 'a), at_least: usize, at_most: usize) -> Parser<'a, String> {
        Box::new(move |text: &str| {
            let mut parsed_string = String::new();
            let mut text_iter = text.chars();
            let mut maybe_current_char = text_iter.next();
            let mut counter = 0usize;
            while maybe_current_char.is_some() && counter < at_most {
                let current_char = maybe_current_char.unwrap();
                if !predicate(&current_char) {
                    break;
                }
                parsed_string.push(current_char);
                counter += 1;
                maybe_current_char = text_iter.next();
            }

            if parsed_string.len() < at_least {
                return None;
            }

            let mut rest_string = match maybe_current_char {
                Some(c) => String::from(c) + &String::from_iter(text_iter),
                None => String::from_iter(text_iter)
            };

            Some((parsed_string, rest_string))
        })
    }

    // as /abcd/ in regex
    #[allow(unused)]
    pub fn make_parser_sequence<'a, T: 'a>(parsers: Vec<Parser<'a, T>>) -> Parser<'a, Vec<T>> {
        Box::new(move |text: &str| {
            let mut results: Vec<T> = Vec::new();
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

    // as (a|b|c|d) in regex
    #[allow(unused)]
    pub fn make_parser_one_of<'a, T: 'a>(parsers: Vec<Parser<'a, T>>) -> Parser<'a, T> {
        Box::new(move |text: &str| {
            for parser in &parsers {
                let result = parser(&text);
                if result.is_some() {
                    return result;
                }
            }

            None
        })
    }

    // as (a)* or (a)+ depending on non_empty parameter
    #[allow(unused)]
    pub fn make_parser_zero_or_many<'a, T: 'a>(parser: Parser<'a, T>, non_empty: bool) -> Parser<'a, Vec<T>> {
        Box::new(move |text: &str| {
            let mut result_vec = Vec::new();
            let mut temp = String::from(text);

            while let Some(result) = parser(&temp) {
                if result.1 == temp { break; }
                result_vec.push(result.0);
                temp = result.1;
            }
            if result_vec.len() == 0 && non_empty {
                return None;
            }
            Some((result_vec, temp))
        })
    }

    // as (a)? in regular expressions
    #[allow(unused)]
    fn make_parser_optional<'a, T>(parser: Parser<'a, T>, default_val: T) -> Parser<'a, T> 
        where T: Clone + 'a
    {
        Box::new(move |text: &str| {
            let local_value = default_val.clone();
            match parser(text) {
                Some(result) => Some(result),
                None => Some((local_value, text.to_string()))
            }
        })
    }

    #[allow(unused)]
    fn make_parser_map<'a, T, U>(transform: &'a (dyn Fn(T) -> U + 'a), parser: Parser<'a, T>) -> Parser<'a, U> {
        Box::new(move |text: &str| {
            parser(text)
                .map(move |item| (transform(item.0), item.1))
        })
    }
//}

#[cfg(test)]
mod test {

    use super::*;
    use std::fmt::Debug;
    use std::collections::HashMap;

    fn test_and_assert<'a, T, U, V>(parser: Parser<'a, T>, test_data: U, expected_data: V)
        where T: Debug + std::cmp::PartialEq,
              U: Iterator<Item = &'a &'a str>,
              V: Iterator<Item = ParserResult<T>>,
    {
        for item in std::iter::zip(test_data, expected_data) {
            let result = parser(item.0);
            assert_eq!(result, item.1);
        }
    }

    #[test]
    fn space_parser_works() {
        let test_data = [" ", "     ", " some", "foo  ", "    bar", ""];
        let expected_data: Vec<ParserResult<String>> = vec![
            Some((" ".to_string(), "".to_string())),
            Some(("     ".to_string(), "".to_string())),
            Some((" ".to_string(), "some".to_string())),
            Some(("".to_string(), "foo  ".to_string())),
            Some(("    ".to_string(), "bar".to_string())),
            Some(("".to_string(), "".to_string())),
        ];
        let zero_or_more_spaces = make_parser_span_of(&whitespace_predicate);
        test_and_assert(zero_or_more_spaces, test_data.iter(), expected_data.into_iter());

        let expected_data: Vec<ParserResult<String>> = vec![
            None,
            Some(("   ".to_string(), "  ".to_string())),
            None,
            None,
            Some(("   ".to_string(), " bar".to_string())),
            None
        ];
        let three_spaces = make_parser_span_of_exact(&whitespace_predicate, 3);
        test_and_assert(three_spaces, test_data.iter(), expected_data.into_iter());
    }

    #[test]
    fn natural_number_parser_works() {
        let test_data = ["8", "2324", "8some", "foo8789", ""];
        let expected_data : Vec<ParserResult<String>> = vec![
            Some(("8".to_string(), "".to_string())),
            Some(("2324".to_string(), "".to_string())),
            Some(("8".to_string(), "some".to_string())),
            Some(("".to_string(), "foo8789".to_string())),
            Some(("".to_string(), "".to_string())),
        ];

        let natural = make_parser_span_of(&natural_number_predicate);
        test_and_assert(natural, test_data.iter(), expected_data.into_iter());
    }

    #[test]
    fn substring_parser_works() {
        let test_data = ["foo", "foobarbaz", "foooolr[", "barfoobarbaz", ""];
        let expected_data: Vec<ParserResult<String>> = vec![
            Some(("foo".to_string(), "".to_string())),
            Some(("foo".to_string(), "barbaz".to_string())),
            Some(("foo".to_string(), "oolr[".to_string())),
            None,
            None,
        ];
        let subs = make_parser_substring("foo");
        test_and_assert(subs, test_data.iter(), expected_data.into_iter());
    }

    #[test]
    fn sequence_parser_works() {
        let test_data = ["foo 2131", "foo23", "foo 2   ", "foo 42 bar",  "foo 512 baz", "crab foo 2 baz butter", ""];

        let expected_data = vec![
            Some((vec!["foo".to_string(), " ".to_string(), "2131".to_string(), "".to_string(), "bar".to_string()], "".to_string())),
            None,
            Some((vec!["foo".to_string(), " ".to_string(), "2".to_string(), "   ".to_string(), "bar".to_string()], "".to_string())),
            Some((vec!["foo".to_string(), " ".to_string(), "42".to_string(), " ".to_string(), "bar".to_string()], "".to_string())),
            Some((vec!["foo".to_string(), " ".to_string(), "512".to_string(), " ".to_string(), "baz".to_string()], "".to_string())),
            None,
        ];


        let sequence = make_parser_sequence(
            vec![
            make_parser_substring("foo"),
            make_parser_span_of_at_least(&whitespace_predicate, 1),
            make_parser_span_of_at_least(&natural_number_predicate, 1),
            make_parser_span_of(&whitespace_predicate),
            make_parser_optional(
                make_parser_one_of(
                    vec![
                        make_parser_substring("bar"),
                        make_parser_substring("baz")
                    ]
                ),
                "bar".to_string()
                )
            ]);
        test_and_assert(sequence, test_data.iter(), expected_data.into_iter());
    }


    fn make_one_of_parser_for_test<'a>() -> Parser<'a, Vec<String>> {
        make_parser_one_of(
            vec![
                make_parser_sequence(
                    vec![
                        make_parser_substring("foo"),
                        make_parser_span_of_at_least(&whitespace_predicate, 1),
                        make_parser_span_of_at_least(&natural_number_predicate, 1),
                        make_parser_span_of(&whitespace_predicate),
                    ]),
                make_parser_sequence(
                    vec![
                        make_parser_substring("bar"),
                        make_parser_span_of_at_least(&whitespace_predicate, 1),
                        make_parser_span_of_at_least(&natural_number_predicate, 1),
                        make_parser_span_of(&whitespace_predicate),
                    ]),
                make_parser_sequence(
                    vec![
                        make_parser_substring("baz"),
                        make_parser_span_of_at_least(&whitespace_predicate, 1),
                        make_parser_span_of_at_least(&natural_number_predicate, 1),
                        make_parser_span_of(&whitespace_predicate),
                    ]
                )
            ]
        )
    }

    #[test]
    fn one_of_parser_works() {
        let test_data = [
            "foo 2131", "foo23", "foo 2   ",
            "bar 2131", "bar", "bar 2   ",
            "baz 2131", "baz", "baz 2   ",
            ""
        ];
        let test_results: Vec<ParserResult<Vec<String>>> = vec![
            Some((vec!["foo".to_string(), " ".to_string(), "2131".to_string(), "".to_string()], "".to_string())),
            None,
            Some((vec!["foo".to_string(), " ".to_string(), "2".to_string(), "   ".to_string()], "".to_string())),
            Some((vec!["bar".to_string(), " ".to_string(), "2131".to_string(), "".to_string()], "".to_string())),
            None,
            Some((vec!["bar".to_string(), " ".to_string(), "2".to_string(), "   ".to_string()], "".to_string())),
            Some((vec!["baz".to_string(), " ".to_string(), "2131".to_string(), "".to_string()], "".to_string())),
            None,
            Some((vec!["baz".to_string(), " ".to_string(), "2".to_string(), "   ".to_string()], "".to_string())),
            None
        ];

        let parser = make_one_of_parser_for_test();
        test_and_assert(parser, test_data.iter(), test_results.into_iter());
    }

    fn make_zero_or_many_parser_for_test<'a>() -> Parser<'a, Vec<Vec<String>>> {
        make_parser_zero_or_many(
            make_parser_one_of( // command
                vec![ // commands
                    make_parser_sequence(
                        vec![
                            make_parser_substring("forward"),
                            make_parser_span_of_at_least(&whitespace_predicate, 1),
                            make_parser_span_of_at_least(&natural_number_predicate, 1),
                            make_parser_span_of(&whitespace_predicate),
                        ]
                    ),
                    make_parser_sequence(
                        vec![
                            make_parser_substring("backward"),
                            make_parser_span_of_at_least(&whitespace_predicate, 1),
                            make_parser_span_of_at_least(&natural_number_predicate, 1),
                            make_parser_span_of(&whitespace_predicate),
                        ]
                    ),
                    make_parser_sequence(
                        vec![
                            make_parser_substring("rotate"),
                            make_parser_span_of_at_least(&whitespace_predicate, 1),
                            make_parser_span_of_at_least(&natural_number_predicate, 1),
                            make_parser_span_of_at_least(&whitespace_predicate, 1),
                            make_parser_optional(
                                make_parser_one_of(
                                    vec![
                                        make_parser_substring("left"),
                                        make_parser_substring("right"),
                                    ]
                                ),
                                "left".to_string()
                            ),
                            make_parser_span_of(&whitespace_predicate)
                        ]
                    )
                ]
            ),
            true
        )
    }

    #[test]
    fn zero_or_many_parser_works() {
        let test_data = [
            "",
            //"forward2131",
            //"forward 2131",
            //"forward 2131lwkdjf  ",
            //"forward 2131 forward 2131",
            //"forward 2131 backward 2131",
            //"forward 2131 backward 2131 rotate 2323 left",
            //"forward 2131 backward 2131 rotate 2323 right",
            "rotate 1 right rotate 1 left rotate 1 forward 1 backward 2",
        ];
        let test_results: Vec<ParserResult<Vec<Vec<String>>>> = vec![
            //Some((vec![], "".to_string())),
            None,
            Some((
                vec![
                    vec!["rotate".to_string(), " ".to_string(), "1".to_string(), " ".to_string(), "right".to_string(), " ".to_string()],
                    vec!["rotate".to_string(), " ".to_string(), "1".to_string(), " ".to_string(), "left".to_string(), " ".to_string()],
                    vec!["rotate".to_string(), " ".to_string(), "1".to_string(), " ".to_string(), "left".to_string(), "".to_string()],
                    vec!["forward".to_string(), " ".to_string(), "1".to_string(), " ".to_string()],
                    vec!["backward".to_string(), " ".to_string(), "2".to_string(), "".to_string()]
                ], 
                "".to_string()
            )),
        ];

        let parser = make_zero_or_many_parser_for_test();
        test_and_assert(parser, test_data.iter(), test_results.into_iter());
    }

    #[test]
    fn mapped_parser_works() {
        let parser1 = make_parser_sequence(
            vec![
            make_parser_substring("rotate"),
            make_parser_span_of_at_least(&whitespace_predicate, 1),
            make_parser_span_of_at_least(&natural_number_predicate, 1),
            make_parser_span_of(&whitespace_predicate),
            make_parser_optional(
                make_parser_one_of(
                    vec![
                        make_parser_substring("left"),
                        make_parser_substring("right")
                    ]
                ),
                "left".to_string()
            ),
            make_parser_span_of(&whitespace_predicate),
            ]);
        let transform = move |mut results: Vec<String>| {
            let mut result = HashMap::new();
            if results.len() < 5 {
                return result;
            }
            //let mut results_iter = results.into_iter();
            results.remove(5);
            let fourth = String::from(results.remove(4));
            results.remove(3);
            let second = String::from(results.remove(2));
            results.remove(1);
            results.remove(0);
            result.insert("angle".to_string(), second);
            result.insert("direction".to_string(), fourth);

            result

        };
        let parser2 = make_parser_map(&transform, parser1);

        let test_data = [
            "rotate 235 right",
            "rotate 180 left    ",
            "rotate 45"
        ];


        let parser_result1 = parser2(test_data[0]).unwrap();
        assert!(parser_result1.0.contains_key("direction"));
        assert!(parser_result1.0.contains_key("angle"));
        assert_eq!(parser_result1.0.get(&String::from("angle")), Some(&String::from("235")));
        assert_eq!(parser_result1.0.get(&String::from("direction")), Some(&String::from("right")));

        let parser_result2 = parser2(test_data[1]).unwrap();
        assert!(parser_result2.0.contains_key("direction"));
        assert!(parser_result2.0.contains_key("angle"));
        assert_eq!(parser_result2.0.get(&String::from("angle")), Some(&String::from("180")));
        assert_eq!(parser_result2.0.get(&String::from("direction")), Some(&String::from("left")));

        let parser_result3 = parser2(test_data[2]).unwrap();
        assert!(parser_result3.0.contains_key("direction"));
        assert!(parser_result3.0.contains_key("angle"));
        assert_eq!(parser_result3.0.get(&String::from("angle")), Some(&String::from("45")));
        assert_eq!(parser_result3.0.get(&String::from("direction")), Some(&String::from("left")));
    }
}
