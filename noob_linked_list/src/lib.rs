mod linked_list {
    pub struct List {
        head: Item,
    }

    struct Node {
        data: i32,
        next: Item,
    }

    type Item = Option<Box<Node>>;

    impl List {
        pub fn new() -> Self {
            List { head: None } 
        }

        pub fn push(&mut self, val: i32) {
            let new_node = Box::new(Node {
                data: val,
                next: self.head.take(),
            });
            self.head = Some(new_node);
        }

        pub fn pop(&mut self) -> Option<i32> {
            self.head.take().map(|first_node| {
                self.head = first_node.next;
                first_node.data
            })
        }
    }

    impl Drop for List {
        fn drop(&mut self) {
            let mut cur_item = self.head.take();
            while let Some(mut ptr) = cur_item {
                cur_item = ptr.next.take();
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::linked_list::List;
    #[test]
    fn it_works() {
        let mut my_list = List::new();
        assert_eq!(my_list.pop(), None);

        my_list.push(1);
        my_list.push(2);
        my_list.push(5);

        assert_eq!(my_list.pop(), Some(5));

        my_list.push(2);
        my_list.push(3);

        assert_eq!(my_list.pop(), Some(3));
        assert_eq!(my_list.pop(), Some(2));
        assert_eq!(my_list.pop(), Some(2));
        assert_eq!(my_list.pop(), Some(1));
        assert_eq!(my_list.pop(), None);
    }
}
