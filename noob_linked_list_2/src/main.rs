struct ListNode {
    data: i32,
    next: ListItem,
}

type ListItem = Option<Box<ListNode>>;

struct List {
    head: ListItem,
}

impl List {
    pub fn new() -> Self { List { head: None } }
    pub fn push(&mut self, value: i32) {
        let new_node = Box::new(ListNode {
            data: value,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        // let first = self.head.take();
        // self.head = if let Some(ptr) = first {
        //     *ptr.next
        // }

        self.head.take().map(|some_node| {
            self.head = (*some_node).next;
            some_node.data
        })
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut list_item: ListItem = self.head.take();
        while let Some(node_ptr) = list_item { // Box<ListNode>
            list_item = (*node_ptr).next;
        }
    }
}


fn main() {
    let wasya: Option<Box<i32>> = Some(Box::new(42));
    let wasya_mapped = wasya.map(|box_item| {
        Box::new(*box_item + 12)
    });

    println!("after {}", *(wasya_mapped.unwrap()));

    let mut my_linked_list = List::new();

    assert_eq!(my_linked_list.pop(), None);

    my_linked_list.push(1);
    my_linked_list.push(2);
    my_linked_list.push(3);

    assert_eq!(my_linked_list.pop(), Some(3));

    my_linked_list.push(4);

    assert_eq!(my_linked_list.pop(), Some(4));
    assert_eq!(my_linked_list.pop(), Some(2));
    assert_eq!(my_linked_list.pop(), Some(1));
    assert_eq!(my_linked_list.pop(), None);
}
