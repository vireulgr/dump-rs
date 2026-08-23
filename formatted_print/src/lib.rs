
use std::fmt;

pub const PI: f64 = 3.14159265358979;

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

// rust-by-example/hello/print/fmt
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let some: u32 = 65536 * self.red as u32 + 256 * self.green as u32 + self.blue as u32;
        //write!(f, "RGB ({}, {}, {}) {:0>2x}{:0>2x}{:0>2x}", self.red, self.green, self.blue, self.red, self.green, self.blue);
        write!(f, "RGB ({}, {}, {}) 0x{:0>6x}", self.red, self.green, self.blue, some)
    }
}


pub fn test() {

    let some = Deep(Structure(42));
    println!("Deep struct print: {:?}", some);

    println!("This struct `{}` will print...", Structure(3));

    // from rust 1.58
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    // activity 3
    println!("Pi is roughly {:.3}", PI);

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

    // rust-by-example/hello/print/fmt
    let color = [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ];
    for item in color {
        println!("{}", item);
    }
}
