use std::fmt;

enum Cons {
    pub Nil,
    pub Next(isize, Box<Cons>),
}

impl fmt::Display for Cons {
    pub fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Nil => return write!(f, "Nil"),
            Next(d, next) => { write!(f, "{}, {}", d, next); },
        }
    }
}

impl Cons {
    pub fn new(&self, data: isize, next: &Option<Cons>) {
        if let Some(val) = data {
            return Box::new(Cons::Next(
        }
    }
}

fn main() {
    let my_list = Cons::Next(-1, Cons::Next(-2, Cons::Next(-3, Cons::Nil)));
    
    let some_int = Box::new(43_323);
    assert_eq!(*some_int, 43_323);
    println!("if no panic then integers are equal!");
}
