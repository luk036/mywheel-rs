use crate::dllist::{Dllink, Dllist};

#[doc = svgbobdoc::transform!(
/// The `BPQueue` struct is a bounded priority queue implemented using an array of doubly-linked lists,
/// with integer keys in a specified range.
/// 
/// Bounded Priority Queue with integer keys in [a..b].
/// Implemented by an array (bucket) of doubly-linked lists.
/// Efficient if the keys are bounded by a small integer value.
///
/// Note that this class does not own PQ nodes. This feature
/// allows these nodes sharable in both doubly linked list class and
/// this class. In the FM algorithm, nodes are either attached to
/// the gain buckets (PQ) or to the waitinglist (doubly-linked list),
/// but cannot be in both at the same time.
///
/// Another improvement is to increase the size of the array by one
/// element, i.e. (b - a + 2). The extra dummy array element (called
/// sentinel) is used to reduce the boundary checking during updates.
///
/// All the member functions assume that the keys are inside the bounds.
/// 
/// ```svgbob
///                   ____ bucket
///          +----+  /     
///        b |high| V
///          +----+
///          |    |  
///          +----+    +----+    +----+
///          |max-|--->|{c}-|--->|{c} |
///          +----+    +----+    +----+
///          |    |
///          +----+    +----+    +----+    +----+
///          |   -|--->|{c}-|--->|{c}-|--->|{c} |
///          +----+    +----+    +----+    +----+
///          :    :
///          
///          :    :
///          +----+    +----+    +----+    +----+    +----+
///          |2  -|--->|{c}-|--->|{c}-|--->|{c}-|--->|{c} |
///          +----+    +----+    +----+    +----+    +----+
///        a |1   | 
///          +----+ 
///  sentinel|0   |
///          +----+^
///                 \
///                   always empty
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
/// * `max`: The maximum number of elements that can be stored in the bounded priority queue.
/// * `offset`: The `offset` property represents the lower bound of the integer keys in the bounded
/// priority queue. It is of type `i32`, which means it can hold both positive and negative values. The
/// offset is used to calculate the index of the bucket in the `bucket` array for a given key.
/// * `high`: The `high` property represents the highest priority level in the bounded priority queue.
/// It indicates the index of the last bucket in the `bucket` array.
/// * `sentinel`: A doubly linked list node that serves as a sentinel or dummy node. It is used to
/// reduce boundary checking during updates.
/// * `bucket`: The `bucket` property is a vector of doubly-linked lists. Each doubly-linked list
/// represents a priority level, with the index of the vector representing the priority value. The
/// elements in the doubly-linked lists are tuples containing a priority value and a value of type `T`.
)]
#[derive(Debug)]
pub struct BPQueue<T> {
    max: usize,
    offset: i32,
    high: usize,
    sentinel: Dllink<(usize, T)>,
    pub bucket: Vec<Dllist<(usize, T)>>,
}

impl<T: Default + Clone> BPQueue<T> {
    /// Construct a new BPQueue object
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mywheel_rs::bpqueue::BPQueue;
    /// let bpq = BPQueue::<i32>::new(-3, 3);
    ///
    /// assert!(bpq.is_empty());
    /// ```
    pub fn new(a: i32, b: i32) -> Self {
        assert!(a <= b);
        let mut res = Self {
            max: 0,
            offset: a - 1,
            high: (b - a + 1) as usize,
            sentinel: Dllink::new((1314, T::default())),
            bucket: vec![Dllist::new((5354, T::default())); (b - a + 2) as usize],
        };
        for lst in res.bucket.iter_mut() {
            lst.clear();
        }
        // res.sentinel.clear();
        res.bucket[0].append(&mut res.sentinel);
        res
    }

    /// Whether the %BPQueue is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mywheel_rs::bpqueue::BPQueue;
    /// let bpq = BPQueue::<i32>::new(-3, 3);
    ///
    /// assert!(bpq.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.max == 0
    }

    /// Get the max value
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mywheel_rs::bpqueue::BPQueue;
    /// let bpq = BPQueue::<i32>::new(-3, 3);
    ///
    /// assert_eq!(bpq.get_max(), -4);
    /// ```
    pub fn get_max(&self) -> i32 {
        self.offset + self.max as i32
    }

    /// Clear reset the PQ
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mywheel_rs::bpqueue::BPQueue;
    /// let mut bpq = BPQueue::<i32>::new(-3, 3);
    /// bpq.clear();
    ///
    /// assert!(bpq.is_empty());
    /// ```
    pub fn clear(&mut self) {
        while self.max > 0 {
            self.bucket[self.max].clear();
            self.max -= 1;
        }
    }

    /// Set the key object
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mywheel_rs::bpqueue::BPQueue;
    /// let mut bpq = BPQueue::<i32>::new(-3, 3);
    ///
    /// assert!(bpq.is_empty());
    /// ```
    pub fn set_key(&mut self, it: &mut Dllink<(usize, T)>, gain: i32) {
        it.data.0 = (gain - self.offset) as usize;
    }

    /// Append item with external key
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mywheel_rs::bpqueue::BPQueue;
    /// use mywheel_rs::dllist::Dllink;
    ///
    /// let mut bpq = BPQueue::<i32>::new(-3, 3);
    /// let mut a = Dllink::<(usize, i32)>::new((0, 3));
    /// bpq.append(&mut a, 0);
    ///
    /// assert!(!bpq.is_empty());
    /// ```
    pub fn append(&mut self, it: &mut Dllink<(usize, T)>, k: i32) {
        assert!(k > self.offset);
        it.data.0 = (k - self.offset) as usize;
        if self.max < it.data.0 {
            self.max = it.data.0;
        }
        self.bucket[it.data.0].append(it);
    }

    /// Append item with external key
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mywheel_rs::bpqueue::BPQueue;
    /// use mywheel_rs::dllist::Dllink;
    ///
    /// let mut bpq = BPQueue::<i32>::new(-3, 3);
    /// let mut a = Dllink::<(usize, i32)>::new((0, 3));
    /// bpq.appendleft(&mut a, 0);
    ///
    /// assert!(!bpq.is_empty());
    /// ```
    pub fn appendleft(&mut self, it: &mut Dllink<(usize, T)>, k: i32) {
        assert!(k > self.offset);
        it.data.0 = (k - self.offset) as usize;
        if self.max < it.data.0 {
            self.max = it.data.0;
        }
        self.bucket[it.data.0].appendleft(it);
    }

    /// Append item with internal key
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mywheel_rs::bpqueue::BPQueue;
    /// use mywheel_rs::dllist::Dllink;
    ///
    /// let mut bpq = BPQueue::<i32>::new(-3, 3);
    /// let mut a = Dllink::<(usize, i32)>::new((0, 3));
    /// bpq.appendleft_direct(&mut a);
    ///
    /// assert!(!bpq.is_empty());
    /// ```
    pub fn appendleft_direct(&mut self, it: &mut Dllink<(usize, T)>) {
        assert!(it.data.0 as i32 > self.offset);
        self.appendleft(it, it.data.0 as i32);
    }

    /// Pop node with the highest key
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mywheel_rs::bpqueue::BPQueue;
    /// use mywheel_rs::dllist::Dllink;
    ///
    /// let mut bpq = BPQueue::<i32>::new(-3, 3);
    /// let mut a = Dllink::<(usize, i32)>::new((0, 3));
    /// bpq.append(&mut a, 0);
    /// let d = bpq.popleft();
    /// let (key, v) = unsafe { (*d).data.clone() };
    ///
    /// assert_eq!(key, 4);
    /// assert_eq!(v, 3);
    /// ```
    pub fn popleft(&mut self) -> *mut Dllink<(usize, T)> {
        let res = self.bucket[self.max].popleft();
        while self.bucket[self.max].is_empty() {
            self.max -= 1;
        }
        res
    }

    /// Detach the item from BPQueue
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mywheel_rs::bpqueue::BPQueue;
    /// use mywheel_rs::dllist::Dllink;
    ///
    /// let mut bpq = BPQueue::<i32>::new(-3, 3);
    /// let mut a = Dllink::<(usize, i32)>::new((0, 3));
    /// bpq.append(&mut a, 0);
    /// bpq.detach(&mut a);
    ///
    /// assert!(bpq.is_empty());
    /// ```
    pub fn detach(&mut self, it: &mut Dllink<(usize, T)>) {
        // self.bucket[it.data.second].detach(it)
        it.detach();
        while self.bucket[self.max].is_empty() {
            self.max -= 1;
        }
    }

    /// Decrease key by delta
    ///
    /// Note that the order of items with same key will not be preserved.
    /// For the FM algorithm, this is a desired behavior.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mywheel_rs::bpqueue::BPQueue;
    /// use mywheel_rs::dllist::Dllink;
    ///
    /// let mut bpq = BPQueue::<i32>::new(-3, 3);
    /// let mut a = Dllink::<(usize, i32)>::new((0, 3));
    /// bpq.append(&mut a, 0);
    /// bpq.decrease_key(&mut a, 1);
    ///
    /// assert_eq!(bpq.get_max(), -1);
    /// ```
    pub fn decrease_key(&mut self, it: &mut Dllink<(usize, T)>, delta: usize) {
        // self.bucket[it.data.second].detach(it)
        it.detach();
        it.data.0 -= delta;
        assert!(it.data.0 > 0);
        assert!(it.data.0 <= self.high);
        self.bucket[it.data.0].append(it); // FIFO
        if self.max < it.data.0 {
            self.max = it.data.0;
            return;
        }
        while self.bucket[self.max].is_empty() {
            self.max -= 1;
        }
    }

    /// Increase key by delta
    ///
    /// Note that the order of items with same key will not be preserved.
    /// For the FM algorithm, this is a desired behavior.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mywheel_rs::bpqueue::BPQueue;
    /// use mywheel_rs::dllist::Dllink;
    ///
    /// let mut bpq = BPQueue::<i32>::new(-3, 3);
    /// let mut a = Dllink::<(usize, i32)>::new((0, 3));
    /// bpq.append(&mut a, 0);
    /// bpq.increase_key(&mut a, 1);
    ///
    /// assert_eq!(bpq.get_max(), 1);
    /// ```
    pub fn increase_key(&mut self, it: &mut Dllink<(usize, T)>, delta: usize) {
        // self.bucket[it.data.second].detach(it)
        it.detach();
        it.data.0 += delta;
        assert!(it.data.0 > 0);
        assert!(it.data.0 <= self.high);
        self.bucket[it.data.0].appendleft(it); // LIFO
        if self.max < it.data.0 {
            self.max = it.data.0;
        }
    }

    /// Modify key by delta
    ///
    /// Note that the order of items with same key will not be preserved.
    /// For the FM algorithm, this is a desired behavior.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mywheel_rs::bpqueue::BPQueue;
    /// use mywheel_rs::dllist::Dllink;
    ///
    /// let mut bpq = BPQueue::<i32>::new(-3, 3);
    /// let mut a = Dllink::<(usize, i32)>::new((0, 3));
    /// bpq.append(&mut a, 0);
    /// bpq.modify_key(&mut a, -1);
    ///
    /// assert_eq!(bpq.get_max(), -1);
    /// ```
    pub fn modify_key(&mut self, it: &mut Dllink<(usize, T)>, delta: i32) {
        use core::cmp::Ordering;

        if it.is_locked() {
            return;
        }
        match delta.cmp(&0) {
            Ordering::Greater => self.increase_key(it, delta as usize),
            Ordering::Less => self.decrease_key(it, -delta as usize),
            Ordering::Equal => (),
        }
        // if delta > 0 {
        //     self.increase_key(it, delta as usize);
        // } else if delta < 0 {
        //     self.decrease_key(it, -delta as usize);
        // }
    }
}

/// BPQueue iterator
///
/// Traverse the list from the first item. Usually it is safe
/// to attach/detach list items during the iterator is active.
#[derive(Debug)]
pub struct BPQueueIterator<'a, T> {
    pub bpq: &'a mut BPQueue<T>,
    pub curkey: usize,
}

impl<'a, T: Default> BPQueueIterator<'a, T> {
    /// Construct a new DllIterator object
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mywheel_rs::bpqueue::{BPQueue, BPQueueIterator};
    /// let mut b = BPQueue::<i32>::new(-3, 3);
    /// let it = BPQueueIterator::new(&mut b);
    /// ```
    #[inline]
    pub fn new(bpq: &'a mut BPQueue<T>) -> Self {
        let curkey = bpq.max;
        // let curitem = (*bpq).bucket[bpq.max].iter_mut();
        Self { bpq, curkey }
    }
}

impl<T: Default> BPQueue<T> {
    /// Return a new DllIterator object
    pub fn iter_mut(&mut self) -> BPQueueIterator<T> {
        BPQueueIterator::new(self)
    }
}

// impl<'a, T> Iterator for BPQueueIterator<'a, T> {
//     type Item = &'a mut Dllink<T>;
//
//     /// Return a next item
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.cur as *const Dllink<T> != self.link as *const Dllink<T> {
//             let res = self.cur;
//             unsafe {
//                 self.cur = (*self.cur).next;
//                 return Some(&mut *res);
//             }
//         }
//         None
//     }
// }
//


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bpqueue1() {
        let mut bpq = BPQueue::<i32>::new(-3, 3);
        let mut a = Dllink::<(usize, i32)>::new((0, 3));
        bpq.append(&mut a, 0);
        assert_eq!(bpq.get_max(), 0);
        assert_eq!(bpq.is_empty(), false);
        bpq.set_key(&mut a, 0);
        assert_eq!(a.data.0, 4);
        bpq.popleft();
        assert_eq!(bpq.is_empty(), true);
        assert_eq!(bpq.get_max(), -4);
    }

    #[test]
    fn test_bpqueue2() {
        let mut bpq = BPQueue::<i32>::new(-3, 3);
        let mut a = Dllink::<(usize, i32)>::new((0, 3));
        bpq.appendleft_direct(&mut a);
        assert_eq!(bpq.get_max(), 0);
        bpq.increase_key(&mut a, 1);
        assert_eq!(bpq.get_max(), 1);
        bpq.decrease_key(&mut a, 1);
        assert_eq!(bpq.get_max(), 0);

        bpq.decrease_key(&mut a, 1);
        bpq.increase_key(&mut a, 1);
        bpq.modify_key(&mut a, 1);
        bpq.detach(&mut a);
        assert_eq!(bpq.get_max(), -4);
        bpq.clear();
        assert_eq!(bpq.get_max(), -4);
    
        let mut c = Dllink::<(usize, i32)>::new((3, 2));
        let mut waiting_list = Dllist::<(usize, i32)>::new((99, 98));
        waiting_list.clear();
        waiting_list.append(&mut c);  // will unlock c
        bpq.modify_key(&mut c, -1);  // c is not yet in bpq
        assert_eq!(bpq.is_empty(), false);
        assert_eq!(bpq.get_max(), -2);
        assert_eq!(waiting_list.is_empty(), true);
    }

    #[test]
    fn test_bpqueue3() {
        // assert!(BPQueue::<i32>::new(-10.4, 10.4).is_err());
    
        let mut bpq1 = BPQueue::<i32>::new(-10, 10);
        let mut bpq2 = BPQueue::<i32>::new(-10, 10);
    
        assert_eq!(bpq1.get_max(), -11);
    
        let mut d = Dllink::<(usize, i32)>::new((0, 0));
        let mut e = Dllink::<(usize, i32)>::new((0, 1));
        let mut f = Dllink::<(usize, i32)>::new((0, 2));
    
        assert_eq!(d.data.0, 0);
    
        bpq1.append(&mut e, 3);
        bpq1.append(&mut f, -10);
        bpq1.append(&mut d, 5);
    
        unsafe {
            bpq2.append(&mut *bpq1.popleft(), -6);  // d
            bpq2.append(&mut *bpq1.popleft(), 3);
            bpq2.append(&mut *bpq1.popleft(), 0);
        }

        bpq2.modify_key(&mut d, 15);
        bpq2.modify_key(&mut d, -3);
        bpq2.detach(&mut f);
        // assert_eq!(bpq1._max, 0);
        assert_eq!(bpq2.get_max(), 6);
        bpq1.clear();
    }
    
    #[test]
    fn test_bpqueue4() {
        let mut bpq = BPQueue::<i32>::new(-3, 3);
        let mut a = Dllink::<(usize, i32)>::new((0, 3));
        bpq.append(&mut a, 0);
        bpq.modify_key(&mut a, 0);  // unchange
        assert_eq!(bpq.get_max(), 0);
    
        bpq.modify_key(&mut a, -1);
        assert_eq!(bpq.get_max(), -1);
    
        a.lock();
        bpq.modify_key(&mut a, 1);  // unchange because it is locked
        assert_eq!(bpq.get_max(), -1);
    
        let mut b = Dllink::<(usize, i32)>::new((0, 8));
        bpq.append(&mut b, -3);
        bpq.modify_key(&mut b, 1);
        assert_eq!(bpq.get_max(), -1);
    }
}
