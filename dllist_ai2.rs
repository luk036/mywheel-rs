use std::rc::Rc;
use std::cell::RefCell;

struct Dllink<T> {
    next: Option<Rc<RefCell<Dllink<T>>>>,
    prev: Option<Rc<RefCell<Dllink<T>>>>,
    data: T,
}

impl<T> Dllink<T> {
    fn new(data: T) -> Rc<RefCell<Self>> {
        let node = Rc::new(RefCell::new(Self {
            next: None,
            prev: None,
            data,
        }));
        node.borrow_mut().next = Some(Rc::clone(&node));
        node.borrow_mut().prev = Some(Rc::clone(&node));
        node
    }

    fn is_locked(&self) -> bool {
        self.next == Some(Rc::clone(self))
    }

    fn lock(&mut self) {
        self.next = Some(Rc::clone(self));
    }

    fn appendleft(&mut self, node: Rc<RefCell<Self>>) {
        node.borrow_mut().next = self.next.take();
        node.borrow_mut().prev = Some(Rc::clone(self));
        self.next = Some(Rc::clone(&node));
        if let Some(next) = &node.borrow().next {
            next.borrow_mut().prev = Some(Rc::clone(&node));
        }
    }

    fn append(&mut self, node: Rc<RefCell<Self>>) {
        node.borrow_mut().prev = self.prev.take();
        node.borrow_mut().next = Some(Rc::clone(self));
        self.prev = Some(Rc::clone(&node));
        if let Some(prev) = &node.borrow().prev {
            prev.borrow_mut().next = Some(Rc::clone(&node));
        }
    }

    fn popleft(&mut self) -> Option<Rc<RefCell<Self>>> {
        let next = self.next.take()?;
        let next_next = next.borrow().next.clone();
        self.next = next_next;
        if let Some(next) = &self.next {
            next.borrow_mut().prev = Some(Rc::clone(self));
        }
        Some(next)
    }

    fn pop(&mut self) -> Option<Rc<RefCell<Self>>> {
        let prev = self.prev.take()?;
        let prev_prev = prev.borrow().prev.clone();
        self.prev = prev_prev;
        if let Some(prev) = &self.prev {
            prev.borrow_mut().next = Some(Rc::clone(self));
        }
        Some(prev)
    }

    fn detach(&mut self) {
        if let Some(next) = &self.next {
            next.borrow_mut().prev = self.prev.take();
        }
        if let Some(prev) = &self.prev {
            prev.borrow_mut().next = self.next.take();
        }
    }
}

struct DllIterator<T> {
    link: Rc<RefCell<Dllink<T>>>,
    cur: Option<Rc<RefCell<Dllink<T>>>>,
}

impl<T> Iterator for DllIterator<T> {
    type Item = Rc<RefCell<Dllink<T>>>;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.cur.take()?;
        let next = cur.borrow().next.clone();
        self.cur = next;
        Some(cur)
    }
}

struct Dllist<T> {
    head: Rc<RefCell<Dllink<T>>>,
}

impl<T> Dllist<T> {
    fn new(data: T) -> Self {
        Self {
            head: Dllink::new(data),
        }
    }

    fn is_empty(&self) -> bool {
        self.head.borrow().next == Some(Rc::clone(&self.head))
    }

    fn clear(&mut self) {
        self.head.borrow_mut().next = Some(Rc::clone(&self.head));
        self.head.borrow_mut().prev = Some(Rc::clone(&self.head));
    }

    fn appendleft(&mut self, node: Rc<RefCell<Dllink<T>>>) {
        self.head.borrow_mut().appendleft(node);
    }

    fn append(&mut self, node: Rc<RefCell<Dllink<T>>>) {
        self.head.borrow_mut().append(node);
    }

    fn popleft(&mut self) -> Option<Rc<RefCell<Dllink<T>>>> {
        self.head.borrow_mut().popleft()
    }

    fn pop(&mut self) -> Option<Rc<RefCell<Dllink<T>>>> {
        self.head.borrow_mut().pop()
    }

    fn iter(&self) -> DllIterator<T> {
        DllIterator {
            link: Rc::clone(&self.head),
            cur: self.head.borrow().next.clone(),
        }
    }
}

fn main() {
    let mut list: Dllist<i32> = Dllist::new(3);
    let node = Dllink::new(4);
    list.appendleft(node.clone());
    assert_eq!(list.is_empty(), false);
    let popped = list.popleft().unwrap();
    assert_eq!(popped.borrow().data, 4);
    assert_eq!(list.is_empty(), true);
}
