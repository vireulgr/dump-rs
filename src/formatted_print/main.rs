use std::fmt;

// rust-by-example/hello/print/print_debug
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

// activity 2 & 1.2.2. Display
// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for Structure {
    // This trait requires `fmt` with this exact signature
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


#[derive(Debug)]
struct Complex {
    re: f64,
    im: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.re.abs() < 1e-6 {
            write!(f, "{:.3}i", self.im)
        }
        else if self.im.abs() < 1e-6 {
            write!(f, "{:.3}", self.re)
        }
        else {
            if self.im > 0.0 {
                write!(f, "{:.3} {} {:.3}i", self.re, '+', self.im)
            }
            else {
                write!(f, "{:.3} {} {:.3}i", self.re, '-', self.im.abs())
            }
        }
    }
}

// rust-by-example/hello/print/print_display/testcase_list
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        let mut vec_iter = self.0.iter().enumerate();
        if let Option::Some(val) = vec_iter.next() {
            write!(f, "{}: {}", val.0, val.1)?;
        }
        for val in vec_iter {
            write!(f, ", {}: {}", val.0, val.1)?;
        }

        write!(f, "]")
    }
}

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

    let some = Deep(Structure(42));
    println!("Deep struct print: {:?}", some);

    println!("This struct `{}` will print...", Structure(3));

    // from rust 1.58
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    // activity 3
    println!("Pi is roughly {:.3}", pi);

    // 1.2.1 debug
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // rust-by-example/hello/print/print_debug
    // use #? format for pretty debug print
    println!("{:#?}", peter);

    // fmt::Display is not implemented for generic containers bc there is no
    // ideal style for all types. fmt::Debug is implemented and must be used.

    let im_unit = Complex { re: 0.0, im: 1.0 };
    let re_unit = Complex { re: 1.0, im: 0.0 };
    let complex1 = Complex { re: 1.414292347, im: 1.4142000438726 };
    let complex2 = Complex { re: 0.781, im: -1.289328 };
    let complex3 = Complex { re: -1.41421739182739, im: 1.41421111 };
    let complex4 = Complex { re: -0.781129694, im: -1.2893238283 };
    println!("im unit:             {} \t\t {:?}", im_unit, im_unit);
    println!("re unit:             {} \t\t {:?}", re_unit, re_unit);
    println!("complex number 1: {} \t {:?}", complex1, complex1);
    println!("complex number 2: {} \t {:?}", complex2, complex2);
    println!("complex number 3: {} \t {:?}", complex3, complex3);
    println!("complex number 4: {} \t {:?}", complex4, complex4);

    // rust-by-example/hello/print/print_display/testcase_list
    let v = List(vec![1, 2, 3, 4]);
    println!("a vector: {}", v);
    let v = List(vec![]);
    println!("a vector: {}", v);
}
