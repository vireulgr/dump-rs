mod parser_combinator;
use parser_combinator::*;
use std::fmt::Debug;

fn run_and_print<'a, T, V>(parser: Parser<'a, T>, test_data: V) 
    where T: Debug, V: Iterator<Item = &'a str>
{
    for item in test_data {
        let result = parser(item);
        println!("item: {:_>10}; parsed: {:?}", item, result);
    }
}


fn main() {
}
