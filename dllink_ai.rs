 Here's the Rust equivalent of your C++ code. I assume you mean a circular doubly-linked list since the `Dllink` nodes seem to reference each other:

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
struct Dllink<T> {
    data: T,
    next: Option<Rc<RefCell<Dllink<T>>>>,
}

impl<T> Dllink<T> {
    fn new(data: T) -> Self {
        Dllink { data, next: None }
    }

    fn lock(&mut self) {
        self.next = Some(Rc::new(RefCell::new(self)));
    }

    fn is_locked(&self) -> bool {
        self.next.is_some()
    }

    fn detach(&mut self, prev: &mut Option<Rc<RefCell<Dllink<T>>>>) {
        if let Some(next) = self.next.take() {
            match prev {
                Some(p) => p.borrow_mut().next.replace(Some(next)),
                None => {} // this is the head node
            }
            next.borrow_mut().prev.replace(prev);
        }
    }

    fn append_left(&mut self, other: Rc<RefCell<Dllink<T>>>) {
        match self.next.take() {
            Some(n) => n.borrow_mut().append_left(other),
            None => {
                self.next = Some(other);
                other.borrow_mut().prev = Some(Rc::downgrade(&self));
            }
        }
    }

    fn append(&mut self, other: Rc<RefCell<Dllink<T>>>) {
        match self.next.take() {
            Some(n) => n.borrow_mut().append(other),
            None => {} // this is the head node
        }
        self.prev = Some(Rc::downgrade(&other));
        other.borrow_mut().next = Some(self);
    }

    fn pop_left(&mut self) -> Rc<RefCell<Dllink<T>>> {
        if let Some(n) = self.next.take() {
            self.next = n.borrow_mut().next.take();
            n
        } else {
            panic!("Popping an empty list.");
        }
    }

    fn pop(&mut self) -> Rc<RefCell<Dllink<T>>> {
        if let Some(n) = self.prev.take() {
            self.prev = n.borrow_mut().prev.take();
            n
        } else {
            panic!("Popping an empty list.");
        }
    }
}
```

This Rust version of the code defines a generic `Dllink` struct and implements the methods as requested in your C++ code. The struct uses `Option`, `Rc`, and `RefCell` to manage the references between nodes, and manages the circular reference using `Some(Rc::new(RefCell::new(self)))`.

Please note that the Rust version does not include a `Dllist` or `DllIterator` since they were not defined in your original C++ code. To create a list, you can use an `Arc<RefCell<Dllink<T>>>` as the head node and manage the list as needed.

