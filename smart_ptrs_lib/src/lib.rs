pub mod cons_list {
    use std::fmt;

    pub enum List {
        Nil,
        Cons(isize, Box<List>),
    }

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                List::Nil => write!(f, "Nil"),
                List::Cons(d, next) => write!(f, "{}, {}", d, next)
            }
        }
    }

    use std::rc::Rc;
    pub enum SharedList {
        Nil,
        Cons(isize, Rc<SharedList>),
    }

    impl fmt::Display for SharedList {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                SharedList::Nil => write!(f, "Nil"),
                SharedList::Cons(d, next) => write!(f, "{}, {}", d, next)
            }
        }
    }

    use std::cell::RefCell;
    pub enum RefCellList {
        Nil,
        Cons(Rc<RefCell<isize>>, Rc<RefCellList>),
    }

    impl fmt::Display for RefCellList {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                RefCellList::Nil => write!(f, "Nil"),
                RefCellList::Cons(d, next) => write!(f, "{}, {}", d.borrow(), next)
            }
        }
    }

    //impl<T> fmt::Display for T {
    //    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //        match self {
    //            T::Nil => write!(f, "Nil"),
    //            T::Cons(d, next) => write!(f, "{}, {}", d, next)
    //        }
    //    }
    //}
}

#[cfg(test)]
mod test {
    use std::rc::Rc;
    use std::cell::RefCell;
    use crate::cons_list::{List, SharedList, RefCellList};

    #[test]
    fn list_works() {
        let my_list = List::Cons(
            -1, Box::new(
                List::Cons(-2,
                    Box::new(
                        List::Cons(-3,
                            Box::new(List::Nil))))));
        println!("simpliest list: {}", my_list);
    }

    #[test]
    fn shared_list_works() {

        let my_shared_list = SharedList::Cons(
            -4, Rc::new(
                SharedList::Cons(
                    -5, Rc::new(
                        SharedList::Cons(
                            -6, Rc::new(
                                SharedList::Nil))))));

        println!("shared list:\n{}", my_shared_list);
    }

    #[test]
    fn shared_list_shares() {
        // Rc smart pointer allows you to let value have multiple owners by means of reference counting
        // Rc only allows one to have non-mutable references
        let a = Rc::new(SharedList::Cons(
                16, Rc::new(
                    SharedList::Cons(
                        32, Rc::new(SharedList::Nil)))));

        let b = SharedList::Cons(8, Rc::clone(&a));
        let c = SharedList::Cons(4, Rc::clone(&a));

        println!("shared lists:\n{}\n{}", b, c);
    }

    #[test]
    fn ref_cell_list_works() {
        let rc = Rc::new(RefCellList::Cons(
                    Rc::new(RefCell::new(3)), Rc::new(RefCellList::Cons(
                                Rc::new(RefCell::new(5)), Rc::new(RefCellList::Nil)))));
        println!("ref cell list\n{}", rc);
    }

    #[test]
    fn ref_cell_list_change_value() {
        let value = Rc::new(RefCell::new(41_isize));
        let rc = Rc::new(RefCellList::Cons(
                Rc::new(RefCell::new(5)), Rc::new(RefCellList::Cons(
                        Rc::clone(&value), Rc::new(RefCellList::Nil)))));

        // * to get rid of Rc, & to borrow inner type
        if let RefCellList::Cons(_, next_lvl) = &*rc {
             // 1st * for Rc, 2nd * for RefCell, & to borrow inner type
            if let RefCellList::Cons(val, _) = &**next_lvl {            
                println!("second value before change {}", *val.borrow());
                *val.borrow_mut() += 1;
            }
        }
        println!("ref cell list after \n{}", rc);
        assert_eq!(&*value.borrow(), &42);
    }
}
