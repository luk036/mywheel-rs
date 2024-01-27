 Rust does not have the same constructs as C++, such as templates or `#pragma once`. However, we can create a similar data structure using Rust's ownership and borrowing system. Here is an equivalent Rust implementation of your C++ code:

```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct Dllink<T> {
    next: Option<Rc<RefCell<Dllink<T>>>>,
    prev: Option<Weak<RefCell<Dllink<T>>>>,
    data: T,
}

impl<T> Clone for Dllink<T> {
    fn clone(&self) -> Self {
        Dllink {
            next: self.next.map(|link| Rc::clone(link)),
            prev: self.prev,
            data: self.data,
        }
    }
}

impl<T> Dllink<T> {
    fn new(data: T) -> Self {
        Dllink { next: None, prev: None, data }
    }

    fn appendleft(&mut self, link: Rc<RefCell<Dllink<T>>>) {
        let prev = Rc::downgrade(&self.prev);
        std::mem::swap(&mut self.next, &mut link.borrow_mut().next);
        std::mem::swap(&mut self.prev, &mut link.borrow_mut().prev);

        if let Some(prev) = prev {
            prev.upgrade_mut().map(|n| n.prev).map(|p| p.appendleft(self.clone()));
        }
    }

    fn append(&mut self, link: Rc<RefCell<Dllink<T>>>) {
        let next = Rc::downgrade(&self.next);
        std::mem::swap(&mut self.prev, &mut link.borrow_mut().next);
        std::mem::swap(&mut self.next, &mut link.borrow_mut().prev);

        if let Some(next) = next {
            next.upgrade_mut().map(|n| n.append(self.clone()));
        }
    }
}

pub struct Dllist<T> {
    head: Rc<RefCell<Dllink<T>>>,
}

impl<T> Clone for Dllist<T> {
    fn clone(&self) -> Self {
        let cloned = self.head.clone();
        Dllist { head: cloned }
    }
}

impl<T> Dllist<T> {
    pub fn new() -> Self {
        Dllist { head: Rc::new(RefCell::new(Dllink::new(T::default()))) }
    }

    pub fn push(&mut self, data: T) {
        let link = Rc::new(RefCell::new(Dllink::new(data)));
        self.head.borrow_mut().appendleft(link);
    }
}
```

This Rust implementation creates a `Dllink` struct that represents an element in the doubly linked list and provides methods for appending links to each other. The `Dllist` struct is used to store the head of the linked list, providing methods for creating a new empty list and appending elements to it. Note that this implementation doesn't include the iterator and begin/end functions since Rust has built-in support for iterators and looping through collections using the `iter()` method.

Keep in mind that Rust uses ownership and borrowing rules, so you need to be careful when manipulating references or cloning data to avoid data races or memory leaks.

