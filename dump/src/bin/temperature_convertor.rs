use std::io;

fn c_to_f(c: i32) ->  i32 {
    return c * 9/5 + 32;
}

fn f_to_c(f: i32) -> i32 {
    return (f - 32)*5/9;
}

fn main() {
    let mut user_input = String::new();

    let value: i32 = loop {
        println!("Enter integer value:");

        user_input.clear();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Error reading user input");
        
        let parse_result: i32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("{}", e);
                println!("{}", user_input);
                continue;
            },
        };

        break parse_result;
    };

    let unit_char: char = loop {
        println!("Enter F or C for Fahrengheit or Celsius units of enered value ({}):", value);

        user_input.clear();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Error reading user input");
        
        let parse_result: char = user_input.trim().chars().nth(0).expect("Cannot get 0 char");

        match parse_result {
            'f'| 'F'| 'c'| 'C' => {
                break parse_result;
            },
            _ => continue,
        };

    };

    match unit_char {
        'C'| 'c' => {
            println!("{} celsius is {} fahrengheits", value, c_to_f(value));
        },
        'F'| 'f' => {
            println!("{} fahrengheitsis is {} celsius ", value, f_to_c(value));
        },
        _ => println!("Error!"),
    };
}
