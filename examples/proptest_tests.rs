//! Property-based tests using proptest for mywheel-rs
//!
//! Run with: `cargo test --example proptest_tests`

use proptest::prelude::*;

proptest! {
    #[test]
    fn bpqueue_starts_empty(a in any::<i32>(), range in 1..20i32) {
        let b = a + range;
        let bpqueue: BPQueue<i32> = BPQueue::new(a, b);
        assert!(bpqueue.is_empty());
    }
}

fn main() {
    println!("Run `cargo test --example proptest_tests` to execute proptest tests.");
}
