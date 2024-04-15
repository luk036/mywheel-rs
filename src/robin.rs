/// The `Robin` struct represents a round robin scheduling algorithm.
///
/// Properties:
///
/// * `cycle`: A vector of SlNode objects.
pub struct Robin {
    cycle: Vec<u8>,
}

/// The `RobinIterator` struct is a iterator over a singly linked list.
///
/// Properties:
///
/// * `cur`: A mutable reference to the current node in the iterator.
/// * `stop`: The `stop` property is a reference to the node at which the iteration should stop.
pub struct RobinIterator<'a> {
    cycle: &'a [u8],
    cur: u8,
    stop: u8,
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
    #[inline]
    pub fn new(num_parts: u8) -> Robin {
        let mut cycle = Vec::with_capacity(num_parts as usize);
        let mut k = 0;

        for _ in 0..num_parts {
            k += 1;
            cycle.push(k);
        }

        cycle[num_parts as usize - 1] = 0;
        Robin { cycle }
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
    #[inline]
    pub fn exclude(&self, from_part: u8) -> RobinIterator {
        RobinIterator {
            cycle: &self.cycle,
            cur: from_part,
            stop: from_part,
        }
    }
}

impl<'a> Iterator for RobinIterator<'a> {
    type Item = u8;

    /// The `next` function returns the next item in a linked list if it exists, otherwise it returns
    /// `None`.
    ///
    /// Returns:
    ///
    /// The `next` method returns an `Option<Self::Item>`.
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.cycle[self.cur as usize];
        if next == self.stop {
            None
        } else {
            self.cur = next;
            Some(self.cur)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_robin() {
        let rr: Robin = Robin::new(6);
        let mut count = 0;
        for _i in rr.exclude(2) {
            count += 1;
        }
        assert_eq!(count, 5);
    }
}
// fn main() {
//     let mut r = Robin::new(5);
//     for k in r.exclude(3) {
//         println!("{}", k);
//     }
// }
