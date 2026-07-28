fn main() {

    // positional arguments
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // named arguments
    println!("{subject} {verb} {object}",
                object="the lazy dog",
                subject="the quick brown fox",
                verb="jumps over");

    // number formats
    println!("Base 10 {}", 69420);
    println!("Base 2  {:b}", 69420);
    println!("Base 8  {:o}", 69420);
    println!("Base 16 {:x}", 69420);

    // align and filling
    println!("{number:>5}", number=42);
    println!("{number:0>5}", number=42);
    println!("{number:0<5}", number=42);

    println!("{name:<16}{value:>16}", name="foo", value=3.1415926);
    println!("{name:<16}{value:>16}", name="bar", value=2.718281828);
    println!("{name:<16}{value:>16}", name="baz", value=1.4142);

    println!("{name:<16}{value:<16}", name="foo", value=3.1415926);
    println!("{name:<16}{value:<16}", name="bar", value=2.718281828);
    println!("{name:<16}{value:<16}", name="baz", value=1.4142);

    println!("{number:0>width$b}", number=42, width=16);
}
