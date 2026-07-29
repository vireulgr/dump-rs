use std::fmt::{Formatter, Display, Result};
fn main() {

    // positional arguments
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // named arguments
    println!("{subject} {verb} {object}",
                object="the lazy dog",
                subject="the quick brown fox",
                verb="jumps over");

    let pi = 3.14159265358979;
    // number formats
    println!("Base 10 {}", 69420);
    println!("Base 2  {:b}", 69420);
    println!("Base 8  {:o}", 69420);
    println!("Base 16 {:x}", 69420);

    // align and filling
    println!("{number:>5}", number=42);
    println!("{number:0>5}", number=42);
    println!("{number:0<5}", number=42);

    println!("{name:<16}{value:>16}", name="foo", value=pi);
    println!("{name:<16}{value:>16}", name="bar", value=2.718281828);
    println!("{name:<16}{value:>16}", name="baz", value=1.4142);

    println!("{name:<16}{value:<16}", name="foo", value=pi);
    println!("{name:<16}{value:<16}", name="bar", value=2.718281828);
    println!("{name:<16}{value:<16}", name="baz", value=1.4142);

    println!("{number:0>width$b}", number=42, width=16);
    // activity 1
    println!("My name is {0}, {1} {0}", "Bond", "James");

    struct Structure(i32);

    // activity 2
    impl Display for Structure {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "{}", self.0)
        }
    }

    println!("This struct `{}` will print...", Structure(3));

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    // activity 3
    println!("Pi is roughly {:.3}", pi);
}
