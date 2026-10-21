
//pub mod parser_combinator {
    pub type ParserResult<T> = Option<(T, String)>;
    pub type Parser<'a, T> = Box<dyn Fn(&str) -> ParserResult<T> + 'a>;

    #[allow(unused)]
    fn whitespace_predicate(c: &char) -> bool {
        c.is_whitespace()
    }

    #[allow(unused)]
    fn natural_number_predicate(c: &char) -> bool {
        ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'].binary_search(c).is_ok()
    }

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
    pub fn make_parser_span_of<'a>(predicate: &'a (dyn Fn(&char) -> bool + 'a), non_empty: bool) -> Parser<'a, String> {
        Box::new(move |text: &str| {
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
//}

#[cfg(test)]
mod test {

    use super::*;
    use std::fmt::Debug;

    fn test_and_print<'a, T, V>(parser: Parser<'a, T>, test_data: V) 
        where T: Debug, V: Iterator<Item = &'a str>
    {
        for item in test_data {
            let result = parser(item);
            println!("item: {:_>10}; parsed: {:?}", item, result);
        }
    }

    fn test_and_assert<'a, T, U, V>(parser: Parser<'a, T>, test_data: U, expected_data: V)
        where T: Debug + std::cmp::PartialEq,
              U: Iterator<Item = &'a str>,
              V: Iterator<Item = ParserResult<T>>,
    {
        for item in std::iter::zip(test_data, expected_data) {
            let result = parser(item.0);
            assert_eq!(result, item.1);
        }
    }

    #[test]
    fn space_parser_works() {
        let test_data = [" ", "    ", " some", "foo  ", ""];
        let expected_data: Vec<ParserResult<String>> = vec![
            Some((" ".to_string(), "".to_string())),
            Some(("    ".to_string(), "".to_string())),
            Some((" ".to_string(), "some".to_string())),
            Some(("".to_string(), "foo  ".to_string())),
            Some(("".to_string(), "".to_string())),
        ];
        let spaces = make_parser_span_of(&whitespace_predicate, false);
        test_and_assert(spaces, test_data.into_iter(), expected_data.into_iter());
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

        let natural = make_parser_span_of(&natural_number_predicate, false);
        test_and_assert(natural, test_data.into_iter(), expected_data.into_iter());
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
        test_and_assert(subs, test_data.into_iter(), expected_data.into_iter());
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
            make_parser_span_of(&whitespace_predicate, true),
            make_parser_span_of(&natural_number_predicate, true),
            make_parser_span_of(&whitespace_predicate, false),
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
        test_and_assert(sequence, test_data.into_iter(), expected_data.into_iter());
    }


    fn make_one_of_parser_for_test<'a>() -> Parser<'a, Vec<String>> {
        make_parser_one_of(
            vec![
                make_parser_sequence(
                    vec![
                        make_parser_substring("foo"),
                        make_parser_span_of(&whitespace_predicate, true),
                        make_parser_span_of(&natural_number_predicate, true),
                        make_parser_span_of(&whitespace_predicate, false),
                    ]),
                make_parser_sequence(
                    vec![
                        make_parser_substring("bar"),
                        make_parser_span_of(&whitespace_predicate, true),
                        make_parser_span_of(&natural_number_predicate, true),
                        make_parser_span_of(&whitespace_predicate, false),
                    ]),
                make_parser_sequence(
                    vec![
                        make_parser_substring("baz"),
                        make_parser_span_of(&whitespace_predicate, true),
                        make_parser_span_of(&natural_number_predicate, true),
                        make_parser_span_of(&whitespace_predicate, false),
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
        test_and_assert(parser, test_data.into_iter(), test_results.into_iter());
    }

    fn make_zero_or_many_parser_for_test<'a>() -> Parser<'a, Vec<Vec<String>>> {
        make_parser_zero_or_many(
            make_parser_one_of( // command
                vec![ // commands
                    make_parser_sequence(
                        vec![
                            make_parser_substring("forward"),
                            make_parser_span_of(&whitespace_predicate, true),
                            make_parser_span_of(&natural_number_predicate, true),
                            make_parser_span_of(&whitespace_predicate, false),
                        ]
                    ),
                    make_parser_sequence(
                        vec![
                            make_parser_substring("backward"),
                            make_parser_span_of(&whitespace_predicate, true),
                            make_parser_span_of(&natural_number_predicate, true),
                            make_parser_span_of(&whitespace_predicate, false),
                        ]
                    ),
                    make_parser_sequence(
                        vec![
                            make_parser_substring("rotate"),
                            make_parser_span_of(&whitespace_predicate, true),
                            make_parser_span_of(&natural_number_predicate, true),
                            make_parser_span_of(&whitespace_predicate, true),
                            make_parser_optional(
                                make_parser_one_of(
                                    vec![
                                        make_parser_substring("left"),
                                        make_parser_substring("right"),
                                    ]
                                ),
                                "left".to_string()
                            ),
                            make_parser_span_of(&whitespace_predicate, false)
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
        test_and_assert(parser, test_data.into_iter(), test_results.into_iter());
    }
}
