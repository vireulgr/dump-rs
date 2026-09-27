use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
enum List {
    Nil,
    Cons(i32, RefCell<Rc<List>>)
}

impl List {
    pub fn get_value(&self) -> &i32 {
        match &self {
            List::Cons(number, _) => &number,
            List::Nil => &0,
        }
    }
}

struct TreeNode {
    value: i32,
    parent: RefCell<Weak<TreeNode>>,
    children: RefCell<Vec<Rc<TreeNode>>>,
}


fn main() {
    let leaf = Rc::new(TreeNode {
        value: 17,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(TreeNode {
        value: 51,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
        parent: RefCell::new(Weak::new()),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    let parent = leaf.parent.borrow().upgrade().unwrap();
    println!("leaf parent's value {}", parent.value);

// following code creates circular references thus creating conditions for memory leak
    let a = Rc::new(List::Cons(42, RefCell::new(Rc::new(List::Nil))));
    let b = Rc::new(List::Cons(67, RefCell::new(Rc::clone(&a))));

    if let List::Cons(_, inner) = &*a {
        *inner.borrow_mut() = Rc::clone(&b);
    }

    let value_in_a = *a.get_value();
    let value_in_b = *b.get_value();

    println!("values: {} {}", value_in_a, value_in_b);
    println!("a ref count {}", Rc::strong_count(&a));
    println!("b ref count {}", Rc::strong_count(&b));
    //println!("b is {b:?}");
}
