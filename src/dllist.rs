#[doc = svgbobdoc::transform!(
/// The `Dllink` type represents a doubly linked node with a pointer to the next and previous nodes and
/// a data field of type `T`.
///
/// ```svgbob
///         +--------+
///         | next *-|----->
///         +--------+
///    <----|-* prev |
///         +--------+
///         |  data  |
///         +--------+
/// ```
/// 
/// Properties:
///
/// * `next`: A pointer to the next node in the doubly linked list.
/// * `prev`: A pointer to the previous node in the doubly linked list.
/// * `data`: The `data` property is a generic type `T` that represents the actual data stored in the
/// node. It can be any type that you specify when creating an instance of the `Dllink` struct.
)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Dllink<T> {
    /// pointer to the next node
    pub next: *mut Dllink<T>,
    /// pointer to the previous node
    pub prev: *mut Dllink<T>,
    pub data: T,
}

impl<T: Default> Default for Dllink<T> {
    /// The `default` function constructs a default `Dllink` object with a data field of type
    /// `T::default()`.
    ///
    /// Returns:
    ///
    /// The `default()` function returns a `Dllink` object with default values.
    /// Construct a default Dllink object
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mywheel_rs::dllist::Dllink;
    /// let a = Dllink::<i32>::default();
    ///
    /// assert_eq!(a.data, 0);
    /// ```
    #[inline]
    fn default() -> Self {
        Self {
            next: std::ptr::null_mut(),
            prev: std::ptr::null_mut(),
            data: T::default(),
        }
    }
}

impl<T> Dllink<T> {
    /// Construct a new Dllink object
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mywheel_rs::dllist::Dllink;
    /// let a = Dllink::new(3);
    ///
    /// assert_eq!(a.data, 3);
    /// ```
    #[inline]
    pub fn new(data: T) -> Self {
        Self {
            next: std::ptr::null_mut(),
            prev: std::ptr::null_mut(),
            data,
        }
    }

    /// Reset the list
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mywheel_rs::dllist::Dllink;
    /// let mut a = Dllink::new(3);
    /// a.clear();
    ///
    /// assert!(a.is_locked());
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        self.next = self as *mut Dllink<T>;
        self.prev = self as *mut Dllink<T>;
    }

    /// Lock the node (and don't append it to any list)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mywheel_rs::dllist::Dllink;
    /// let mut a = Dllink::new(3);
    /// a.lock();
    ///
    /// assert!(a.is_locked());
    /// ```
    #[inline]
    pub fn lock(&mut self) {
        // self.next = std::ptr::null_mut();
        self.next = self as *mut Dllink<T>;
    }

    /// whether the node is locked
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mywheel_rs::dllist::Dllink;
    /// let mut a = Dllink::new(3);
    /// a.lock();
    ///
    /// assert!(a.is_locked());
    /// ```
    #[inline]
    pub fn is_locked(&self) -> bool {
        // self.next.is_null()
        std::ptr::eq(self.next, self)
    }

    /// Append the node to the front
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mywheel_rs::dllist::Dllink;
    /// let mut a = Dllink::new(3);
    /// let mut b = Dllink::new(3);
    /// a.clear();
    /// a.attach(&mut b);
    ///
    /// assert!(!a.is_locked());
    /// ```
    #[inline]
    pub fn attach(&mut self, node: &mut Dllink<T>) {
        node.next = self.next;
        unsafe {
            (*self.next).prev = node as *mut Dllink<T>;
        }
        self.next = node as *mut Dllink<T>;
        node.prev = self as *mut Dllink<T>;
    }

    #[doc = svgbobdoc::transform!(
    /// Detach from a list
    ///
    /// ```svgbob
    ///                     .---------------.
    ///         +--------+  |   +--------+  |   +--------+
    ///       ->| {c}  *-|--'   | {c}  *-|- `-->| {c}  *-|-
    ///        -|-*      |<-.  -|-*      |  .---|-*      |<-
    ///         +--------+  |   +--------+  |   +--------+
    ///                     `---------------'
    ///
    /// # Legend:
    /// c = {
    ///     fill: papayawhip;
    /// }
    /// ```
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mywheel_rs::dllist::Dllink;
    /// let mut a = Dllink::new(3);
    /// let mut b = Dllink::new(3);
    /// a.clear();
    /// a.attach(&mut b);
    /// b.detach();
    /// ```
    )]
    #[inline]
    pub fn detach(&mut self) {
        assert!(!self.is_locked());
        let n = self.next;
        let p = self.prev;
        unsafe {
            (*p).next = n;
            (*n).prev = p;
        }
    }
}

#[doc = svgbobdoc::transform!(
/// The `Dllist` struct represents a doubly linked list.
/// 
/// A Doubly-linked List class. This class simply contains a link of
/// node's. By adding a "head" node (sentinel), deleting a node is
/// extremely fast (see "Introduction to Algorithm"). This class does
/// not keep the length information as it is not necessary for the FM
/// algorithm. This saves memory and run-time to update the length
/// information. Note that this class does not own the list node. They
/// are supplied by the caller in order to better reuse the nodes.
/// 
/// ```svgbob
///      .----------------------------------------------- - - ------------------------------.
///      |  +--------+      +--------+      +--------+           +--------+      +--------+  )
///      `->| head *-|----->| {c}  *-|----->| {c}  *-|--- - - -->| {c}  *-|----->| {c1} *-|-'
///       .-|-* {a}  |<-----|-*      |<-----|-*      |<-- - - ---|-*      |<-----|-*      |<-.
///      (  +--------+      +--------+      +--------+           +--------+      +--------+   |
///       `---------------------------------------------- - - -------------------------------' 
/// 
/// # Legend:
/// a = {
///     fill: lightblue;
/// }
/// c = {
///     fill: papayawhip;
/// }
/// ```
/// 
/// Properties:
/// 
/// * `head`: The head property is a `Dllink<T>` that represents the first node in the doubly linked list.
///           Doubly linked list
)]
#[derive(Debug, Clone)]
pub struct Dllist<T> {
    pub head: Dllink<T>,
}

impl<T: Default> Default for Dllist<T> {
    /// Construct a default Dllist object
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mywheel_rs::dllist::Dllist;
    /// let a = Dllist::<i32>::default();
    ///
    /// assert_eq!(a.head.data, 0);
    ///  ```
    #[inline]
    fn default() -> Self {
        Self {
            head: Dllink::<T>::default(), // move occurred!
        }
    }
}

impl<T> Dllist<T> {
    /// Construct a new Dllist object
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mywheel_rs::dllist::Dllist;
    /// let mut a = Dllist::new(3);
    /// a.clear();
    ///
    /// assert_eq!(a.head.data, 3);
    /// assert!(a.head.is_locked());
    /// ```
    #[inline]
    pub fn new(data: T) -> Self {
        Self {
            head: Dllink::new(data), // move occurred!
        }
    }

    #[doc = svgbobdoc::transform!(
    /// Whether the list is empty
    ///
    /// ```svgbob
    ///      .-------------.
    ///      |  +--------+  )
    ///      `->| head *-|-'
    ///       .-|-* {a}  |<-.
    ///      (  +--------+  |
    ///       `-------------'
    ///
    /// # Legend:
    /// a = {
    ///     fill: lightblue;
    /// }
    /// ```
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mywheel_rs::dllist::Dllist;
    /// let mut a = Dllist::new(3);
    /// a.clear();
    ///
    /// assert!(a.is_empty());
    /// ```
    )]
    #[inline]
    pub fn is_empty(&mut self) -> bool {
        // self.head.is_empty()
        std::ptr::eq(self.head.next, &mut self.head as *mut Dllink<T>)
    }

    /// Reset the list
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mywheel_rs::dllist::Dllist;
    /// let mut a = Dllist::new(3);
    /// a.clear();
    ///
    /// assert!(a.is_empty());
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        self.head.clear();
    }

    /// Append the node to the front
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mywheel_rs::dllist::{Dllist, Dllink};
    /// let mut a = Dllist::new(3);
    /// a.clear();
    /// let mut b = Dllink::new(3);
    /// a.appendleft(&mut b);
    ///
    /// assert!(!a.is_empty());
    /// ```
    #[inline]
    pub fn appendleft(&mut self, node: &mut Dllink<T>) {
        self.head.attach(node);
    }

    /// Append the node to the back
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mywheel_rs::dllist::{Dllist, Dllink};
    /// let mut a = Dllist::new(3);
    /// a.clear();
    /// let mut b = Dllink::new(3);
    /// a.append(&mut b);
    ///
    /// assert!(!a.is_empty());
    /// ```
    #[inline]
    pub fn append(&mut self, node: &mut Dllink<T>) {
        unsafe {
            (*self.head.prev).attach(node);
        }
    }

    #[doc = svgbobdoc::transform!(
    /// Pop a node from the front
    ///
    /// Precondition: list is not empty
    ///
    /// ```svgbob
    ///                     .---------------.
    ///         +--------+  |   +--------+  |   +--------+           +--------+      +--------+
    ///       ->| head *-|--'   | {c}  *-|- `-->| {c}  *-|--- ... -->| {c}  *-|----->| {c}  *-|-
    ///        -|-* {a}  |<-.  -|-*      |  .---|-*      |<-- ... ---|-*      |<-----|-*      |<-
    ///         +--------+  |   +--------+  |   +--------+           +--------+      +--------+
    ///                     `---------------'
    ///
    /// # Legend:
    /// a = {
    ///     fill: lightblue;
    /// }
    /// c = {
    ///     fill: papayawhip;
    /// }
    /// ```
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mywheel_rs::dllist::{Dllist, Dllink};
    /// let mut a = Dllist::new(0);
    /// let mut b = Dllink::new(3);
    /// a.clear();
    /// assert!(a.is_empty());
    /// a.appendleft(&mut b);
    /// assert!(!a.is_empty());
    /// let d = a.popleft();
    /// assert!(a.is_empty());
    /// assert!(std::ptr::eq(&mut b as *mut Dllink<i32>, d));
    /// ```
    )]
    #[inline]
    pub fn popleft(&mut self) -> *mut Dllink<T> {
        let res = self.head.next;
        unsafe {
            (*res).detach();
        }
        res
    }

    /// Pop a node from the back
    ///
    /// Precondition: list is not empty
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mywheel_rs::dllist::{Dllist, Dllink};
    /// let mut a = Dllist::new(0);
    /// let mut b = Dllink::new(3);
    /// a.clear();
    /// a.append(&mut b);
    /// let d = a.pop();
    /// assert!(std::ptr::eq(&mut b as *mut Dllink<i32>, d));
    /// ```
    #[inline]
    pub fn pop(&mut self) -> *mut Dllink<T> {
        let res = self.head.prev;
        unsafe {
            (*res).detach();
        }
        res
    }
}

/// List iterator
///
/// Traverse the list from the first item. Usually it is safe
/// to attach/detach list items during the iterator is active.
#[derive(Debug, PartialEq, Eq)]
pub struct DllIterator<'a, T> {
    cur: *mut Dllink<T>,
    link: &'a mut Dllink<T>,
}

impl<'a, T> DllIterator<'a, T> {
    /// Construct a new DllIterator object
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mywheel_rs::dllist::{Dllink, DllIterator};
    /// let mut b = Dllink::new(3);
    /// b.clear();
    /// let it = DllIterator::new(&mut b);
    /// ```
    #[inline]
    pub fn new(link: &'a mut Dllink<T>) -> Self {
        Self {
            cur: link.next,
            link,
        }
    }
}

impl<T> Dllist<T> {
    /// Return a new DllIterator object
    pub fn iter_mut(&mut self) -> DllIterator<T> {
        DllIterator::new(&mut self.head)
    }
}

impl<'a, T> Iterator for DllIterator<'a, T> {
    type Item = &'a mut Dllink<T>;

    /// Return a next item
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur as *const Dllink<T> != self.link as *const Dllink<T> {
            let res = self.cur;
            unsafe {
                self.cur = (*self.cur).next;
                return Some(&mut *res);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dllink() {
        let mut a = Dllink::new(3);
        let mut b = Dllink::<i32>::default();
        a.clear();
        b.clear();
        assert!(a.is_locked());
        assert!(b.is_locked());
        assert_ne!(a, b);

        assert_eq!(a.data, 3);
        assert_eq!(b.data, 0);
        a.data = 5;
        b.data = 5;
        assert_ne!(a, b);
        assert_eq!(a.next, a.prev);
        assert!(std::ptr::eq(a.next, &a));
        assert!(a.is_locked());
        a.lock();
        assert!(a.is_locked());
    }

    #[test]
    fn test_dllist() {
        let mut a = Dllist::new(3);
        a.clear();
        assert!(a.is_empty());
        assert_eq!(a.head.data, 3);
        assert_eq!(a.head.next, a.head.prev);

        let mut b = Dllink::new(4);
        a.append(&mut b);
        let c = a.pop();
        assert_eq!(c, &mut b as *mut Dllink<i32>);
        assert!(a.is_empty());
        a.appendleft(&mut b);
        assert!(!a.is_empty());
        a.clear();
        assert!(a.is_empty());
    }

    #[test]
    fn test_dllist2() {
        let mut l1 = Dllist::new(99);
        let mut l2 = Dllist::new(99);

        let mut d = Dllink::new(1);
        let mut e = Dllink::new(2);
        let mut f = Dllink::new(3);

        l1.clear();
        assert!(l1.is_empty());

        l1.appendleft(&mut e);
        assert!(!l1.is_empty());

        l1.appendleft(&mut f);
        assert!(!l1.is_empty());
        l1.append(&mut d);

        l2.clear();
        unsafe {
            l2.append(&mut *l1.pop());
        }
        unsafe {
            l2.append(&mut *l1.popleft());
        }
        assert!(!l1.is_empty());
        e.detach();
        assert!(l1.is_empty());
    }

    #[test]
    fn test_dllist3() {
        let mut l1 = Dllist::new(99);
        let mut d = Dllink::new(1);
        let mut e = Dllink::new(2);
        let mut f = Dllink::new(3);

        l1.clear(); // Yes, it has to be called every time!
        l1.append(&mut d);
        l1.append(&mut e);
        l1.append(&mut f);

        let mut count = 0;
        for _n in l1.iter_mut() {
            count += 1;
        }
        assert_eq!(count, 3);
    }
}
