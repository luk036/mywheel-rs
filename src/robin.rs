/// Round-robin scheduler using a circular singly-linked list.
///
/// The cycle is stored as a "next-index" array where `next[i]` is the index of
/// the node after node `i`.  The nodes are linked in a circle: 0 → 1 → 2 → … → n-1 → 0.
///
/// `exclude(from_part)` returns an iterator that visits every node **except**
/// the excluded one, starting from the node immediately after `from_part`.
///
/// # Example
///
/// ```rust
/// use mywheel_rs::robin::Robin;
///
/// let rr = Robin::new(6);
/// let visited: Vec<u8> = rr.exclude(2).collect();
/// assert_eq!(visited, vec![3, 4, 5, 0, 1]);
/// ```
///
/// ```svgbob
/// Round Robin Cycle:
///
///    ┌─────┐
///    │  0  │◄────────────────────────┐
///    └──┬──┘                        │
///       │                           │
///       ▼                           │
///    ┌─────┐                        │
///    │  1  │◄────┐                 │
///    └──┬──┘     │                 │
///       │        │                 │
///       ▼        │                 │
///    ┌─────┐     │                 │
///    │  2  │◄────┼─────────────────┤  <-- exclude(2) skips this
///    └──┬──┘     │                 │
///       │        │                 │
///       ▼        │                 │
///    ┌─────┐     │                 │
///    │  3  │◄────┘                 │
///    └──┬──┘                       │
///       │                          │
///       └──────────────────────────┘
/// ```
#[derive(Debug, Clone)]
pub struct Robin {
    /// `next[i]` stores the index of the node that follows node `i` in the cycle.
    next: Vec<u8>,
}

/// Iterator over a [`Robin`] cycle that visits every node except one.
///
/// Created by [`Robin::exclude`].
#[derive(Debug)]
pub struct RobinIterator<'a> {
    /// The "next-index" array of the parent [`Robin`].
    next: &'a [u8],
    /// Current position (index into `next`).
    curr: u8,
    /// Index of the excluded node – iteration stops when we reach this.
    stop: u8,
}

impl Robin {
    /// Construct a round-robin cycle with `num_parts` nodes.
    ///
    /// Nodes are created with sequential keys 0, 1, …, `num_parts`-1 and
    /// wired into a circular singly-linked list.
    ///
    /// # Panics
    ///
    /// Panics if `num_parts < 2`.
    #[inline]
    pub fn new(num_parts: u8) -> Robin {
        let n = num_parts as usize;
        assert!(
            n >= 2,
            "Robin::new: num_parts must be at least 2, got {num_parts}"
        );
        let next: Vec<u8> = (0..n).map(|i| ((i + 1) % n) as u8).collect();
        Robin { next }
    }

    /// Return an iterator that visits every node **except** `from_part`.
    ///
    /// Iteration begins at the node after `from_part` and stops when it would
    /// circle back to the excluded node.
    #[inline]
    pub fn exclude(&self, from_part: u8) -> RobinIterator<'_> {
        RobinIterator {
            next: &self.next,
            curr: from_part,
            stop: from_part,
        }
    }
}

impl<'a> Iterator for RobinIterator<'a> {
    type Item = u8;

    /// Advance to the next node and yield its key.
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let nxt = self.next[self.curr as usize];
        if nxt == self.stop {
            None
        } else {
            self.curr = nxt;
            Some(self.curr)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_robin_count() {
        let rr = Robin::new(6);
        let mut count = 0;
        for _i in rr.exclude(2) {
            count += 1;
        }
        assert_eq!(count, 5);
    }

    #[test]
    fn test_robin_exclude_values() {
        let rr = Robin::new(6);
        let mut count = 0u32;
        let mut sum = 0u32;
        for i in rr.exclude(2) {
            count += 1;
            sum += i as u32;
        }
        assert_eq!(count, 5);
        // Every value except 2: 0 + 1 + 3 + 4 + 5 = 13
        assert_eq!(sum, 13);
    }

    #[test]
    fn test_robin_exclude_start() {
        let rr = Robin::new(6);
        // exclude(0) should yield: 1, 2, 3, 4, 5
        let visited: Vec<u8> = rr.exclude(0).collect();
        assert_eq!(visited, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_robin_exclude_last() {
        let rr = Robin::new(6);
        // exclude(5) should yield: 0, 1, 2, 3, 4
        let visited: Vec<u8> = rr.exclude(5).collect();
        assert_eq!(visited, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_robin_two_parts() {
        let rr = Robin::new(2);
        // exclude(0): 1
        assert_eq!(rr.exclude(0).collect::<Vec<_>>(), vec![1]);
        // exclude(1): 0
        assert_eq!(rr.exclude(1).collect::<Vec<_>>(), vec![0]);
    }

    #[test]
    fn test_robin_stress() {
        // Mirrors the C++ stress test: random exclusions on a 1000-part cycle
        let num_parts = 250u8;
        let rr = Robin::new(num_parts);
        for excluded in 0..num_parts {
            let expected_len = (num_parts - 1) as usize;
            let visited: Vec<u8> = rr.exclude(excluded).collect();
            assert_eq!(visited.len(), expected_len);
            // All values 0..num_parts except excluded should be present
            let mut sorted = visited.clone();
            sorted.sort();
            let expected: Vec<u8> = (0..num_parts).filter(|&x| x != excluded).collect();
            assert_eq!(sorted, expected);
        }
    }
}
