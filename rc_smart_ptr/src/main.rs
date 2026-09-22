use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Nil,
    Cons(i32, RefCell<Rc<List>>)
}

fn main() {
    let a = Rc::new(List::Cons(42, RefCell::new(Rc::new(List::Nil))));
    let b = Rc::new(List::Cons(67, RefCell::new(Rc::clone(&a))));

    if let List::Cons(_, inner) = &*a {
        *inner.borrow_mut() = Rc::clone(&b);
    }

    println!("a ref count {}", Rc::strong_count(&a));
    println!("b ref count {}", Rc::strong_count(&b));
    //println!("b is {b:?}");
}
