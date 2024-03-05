/// The below code defines a struct called SlNode that represents a node in a singly linked list.
/// 
/// Properties:
/// 
/// * `next`: The `next` property is an `Option` that holds a `Box` containing another `SlNode`. This
/// allows us to create a linked list structure where each node points to the next node in the list. The
/// `Option` type is used to handle the case where there is no next node
/// * `data`: The `data` property is of type `usize`, which represents an unsigned integer. It is used
/// to store the actual data value associated with the `SlNode`.
#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct SlNode {
    next: Option<Box<SlNode>>,
    data: usize,
}

/// The `RobinIterator` struct is a iterator over a singly linked list.
/// 
/// Properties:
/// 
/// * `cur`: A mutable reference to the current node in the iterator.
/// * `stop`: The `stop` property is a reference to the node at which the iteration should stop.
pub struct RobinIterator<'a> {
    cur: &'a mut SlNode,
    stop: &'a SlNode,
}

impl<'a> Iterator for RobinIterator<'a> {
    type Item = usize;

    /// The `next` function returns the next item in a linked list if it exists, otherwise it returns
    /// `None`.
    /// 
    /// Returns:
    /// 
    /// The `next` method returns an `Option<Self::Item>`.
    fn next(&mut self) -> Option<Self::Item> {
        self.cur = self.cur.next.as_mut().unwrap();
        if self.cur as *const SlNode != self.stop as *const SlNode {
            Some(self.cur.data)
        } else {
            None
        }
    }
}

/// The `Robin` struct represents a round robin scheduling algorithm.
/// 
/// Properties:
/// 
/// * `cycle`: A vector of SlNode objects.
pub struct Robin {
    cycle: Vec<SlNode>,
}

impl Robin {
    /// The `new` function creates a cycle of linked nodes with a specified number of parts.
    /// 
    /// Arguments:
    /// 
    /// * `num_parts`: The `num_parts` parameter is the number of parts or nodes in the cycle.
    /// 
    /// Returns:
    /// 
    /// The `new` function is returning an instance of the struct that it is defined in.
    pub fn new(num_parts: usize) -> Self {
        let mut cycle: Vec<_> = (0..num_parts)
            .into_iter()
            .map(|k| SlNode {
                next: None,
                data: k,
            })
            .collect();
        let mut sl2 = &mut cycle[num_parts - 1];
        for sl1 in &cycle {
            sl2.next = Some(Box::new(sl1.clone()));
            sl2 = sl2.next.as_mut().unwrap();
        }
        Self { cycle }
    }

    /// The `exclude` function returns a `RobinIterator` that excludes a specified part of a cycle.
    /// 
    /// Arguments:
    /// 
    /// * `from_part`: The `from_part` parameter is the index of the cycle from which you want to exclude
    /// elements.
    /// 
    /// Returns:
    /// 
    /// The `exclude` method returns a `RobinIterator` object.
    pub fn exclude(&mut self, from_part: usize) -> RobinIterator {
        RobinIterator {
            cur: &mut self.cycle[from_part],
            stop: &self.cycle[from_part],
        }
    }
}

// fn main() {
//     let mut r = Robin::new(5);
//     for k in r.exclude(3) {
//         println!("{}", k);
//     }
// }
