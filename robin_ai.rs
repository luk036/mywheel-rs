 Here is the equivalent Rust code using linked lists and an iterator:

```rust
use std::rc::Rc;
use std::cell::RefCell;
use std::iter::{Iterator, Peekable};

#[derive(Debug)]
struct Node {
    data: i32,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(data: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node { data, next: None }))
    }
}

struct LinkedList {
    head: Option<Rc<RefCell<Node>>>,
}

impl LinkedList {
    fn new() -> Self {
        Self { head: None }
    }

    fn push(&mut self, data: i32) {
        let node = Node::new(data);
        match self.head.take() {
            Some(head) => head.borrow_mut().next = Some(node),
            None => self.head = Some(node),
        }
    }
}

struct RobinIterator<'a> {
    node: Option<&'a Rc<RefCell<Node>>>,
}

impl<'a> Iterator for RobinIterator<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.node.take() {
            Some(node) => {
                let data = node.borrow().data;
                self.node = node.borrow_mut().next.take();
                Some(data)
            }
            None => None,
        }
    }

    fn peek(&mut self) -> Option<&Self::Item> {
        self.node.as_ref().map(|n| &n.borrow().data)
    }
}

impl<'a> Peekable<Self::Iterator> for RobinIterator<'a> {
    fn peekable(self) -> Self {
        Self { node: self.node }
    }
}

struct Robin {
    nodes: LinkedList,
}

impl Robin {
    fn new(num_parts: i32) -> Self {
        let mut list = LinkedList::new();
        for _ in 0..num_parts {
            list.push(Node::new(i32::from(i64::wrapping_add(1, num_parts as i64 * i32::from(i64::wrapping_sub(1, num_parts as i64)))))
        }
        let last = list.nodes.head.as_ref().unwrap().borrow().data;
        for node in &mut list.nodes.nodes {
            match node.borrow_mut().next.take() {
                Some(next) => next.borrow_mut().next = Some(node),
                None => {}
            }
            node.borrow_mut().next = Some(list.nodes.head.clone());
        }
        list.nodes.head.as_ref().unwrap().borrow_mut().next = None;
        Self { nodes: list }
    }

    fn exclude(&self, from_part: i32) -> RobinIterator<'static> {
        let head = self.nodes.head.as_ref().unwrap();
        let node = match self.nodes.head.take() {
            Some(node) => node,
            None => return RobinIterator::new(Box::new(self.nodes.nodes.into_iter())),
        };
        if from_part > 0 && i32::from(i64::wrapping_sub(head.borrow().data as i64, from_part as i64)) < 0 {
            let mut iter = RobinIterator::new(Box::new(self.nodes.nodes.into_iter()));
            iter.next();
            iter
        } else {
            RobinIterator { node: Some(Rc::downgrade(&node)) }
        }
    }
}

fn main() {
    let mut r = Robin::new(5);
    for i in r.exclude(3) {
        println!("{}", i);
    }
}
```

This code uses Rust's standard library for linked lists and iterators. The `LinkedList` struct represents the list, and the `Node` struct represents a single node in the list. The `RobinIterator` struct is used to traverse the list as an iterator. The `Robin` struct creates and manages the linked list. The `exclude` method returns an iterator that excludes a specified part of the cycle. In the main function, we create a `Robin` instance, add some nodes to it, and then iterate over the excluded part using an iterator.

