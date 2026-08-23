
use std::io;

fn main() {
    let mut user_input = String::new();

    let value: u32 = loop {
        println!("Enter positive integer value:");

        user_input.clear();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Error reading user input");
        
        let parse_result: u32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("{}", e);
                println!("{}", user_input);
                continue;
            },
        };

        break parse_result;
    };

    let mut fib_cur = 1;
    let mut fib_prev = 0;

    for _i in 1..=value {
        let fib_new = fib_prev + fib_cur;

        fib_prev = fib_cur;
        fib_cur = fib_new;

    }

    println!("fibonacci number {} is {}", value, fib_cur);
}

