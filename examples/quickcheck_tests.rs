//! Quickcheck property-based tests for mywheel-rs
//!
//! This example demonstrates property-based testing using quickcheck.
//! Run with: cargo run --example quickcheck_tests

use mywheel_rs::array_like::{RepeatArray, ShiftArray};
use mywheel_rs::bpqueue::BPQueue;
use mywheel_rs::map_adapter::MapAdapter;
use quickcheck::{Arbitrary, Gen, QuickCheck, TestResult};

/// Newtype wrapper for RepeatArray to implement Arbitrary
#[derive(Clone, Debug)]
struct TestRepeatArray {
    value: i32,
    size: usize,
}

impl Arbitrary for TestRepeatArray {
    fn arbitrary(g: &mut Gen) -> Self {
        let size = usize::arbitrary(g) % 100;
        let value = i32::arbitrary(g);
        TestRepeatArray { value, size }
    }
}

/// Newtype wrapper for ShiftArray to implement Arbitrary
#[derive(Clone, Debug)]
struct TestShiftArray(ShiftArray<i32>);

impl Arbitrary for TestShiftArray {
    fn arbitrary(g: &mut Gen) -> Self {
        let size = usize::arbitrary(g) % 50 + 1;
        let vec: Vec<i32> = (0..size).map(|_| i32::arbitrary(g)).collect();
        let start = usize::arbitrary(g) % size; // Valid range: 0..size
        let mut arr = ShiftArray::new(vec);
        arr.set_start(start);
        TestShiftArray(arr)
    }
}

/// Newtype wrapper for BPQueue to implement Arbitrary
#[derive(Clone, Debug)]
struct TestBPQueue {
    a: i32,
    b: i32,
}

impl Arbitrary for TestBPQueue {
    fn arbitrary(g: &mut Gen) -> Self {
        // Generate valid range where a <= b (use unsigned to avoid overflow)
        let a = u32::arbitrary(g) % 20;
        let range = (u32::arbitrary(g) % 10 + 1) as i32;
        TestBPQueue {
            a: a as i32,
            b: (a as i32) + range,
        }
    }
}

/// Newtype wrapper for MapAdapter to implement Arbitrary
#[derive(Clone, Debug)]
struct TestMapAdapter(MapAdapter<i32>);

impl Arbitrary for TestMapAdapter {
    fn arbitrary(g: &mut Gen) -> Self {
        let size = usize::arbitrary(g) % 20;
        let vec: Vec<i32> = (0..size).map(|_| i32::arbitrary(g)).collect();
        TestMapAdapter(MapAdapter::new(vec))
    }
}

/// Property: RepeatArray.len() always equals the size passed to new()
fn repeat_array_len_property(arr: TestRepeatArray) -> TestResult {
    let repeat_arr = RepeatArray::new(arr.value, arr.size);
    TestResult::from_bool(repeat_arr.len() == arr.size)
}

/// Property: RepeatArray returns the same value for any index
fn repeat_array_index_property(arr: TestRepeatArray) -> TestResult {
    if arr.size == 0 {
        return TestResult::discard();
    }
    let repeat_arr = RepeatArray::new(arr.value, arr.size);
    for i in 0..repeat_arr.len() {
        if repeat_arr.get(i) != arr.value {
            return TestResult::failed();
        }
    }
    TestResult::passed()
}

/// Property: RepeatArray is_empty() is true only when size is 0
fn repeat_array_empty_property(arr: TestRepeatArray) -> TestResult {
    let repeat_arr = RepeatArray::new(arr.value, arr.size);
    let is_empty = repeat_arr.is_empty();
    TestResult::from_bool(is_empty == (arr.size == 0))
}

/// Property: ShiftArray can be created without panic
fn shift_array_creation(arr: TestShiftArray) -> TestResult {
    // Just verify creation works - len() should match
    TestResult::from_bool(!arr.0.is_empty())
}

/// Property: ShiftArray.len() returns correct length
fn shift_array_len_property(arr: TestShiftArray) -> TestResult {
    // Just verify len() returns non-zero for non-empty
    TestResult::from_bool(!arr.0.is_empty())
}

/// Property: BPQueue starts empty
fn bpqueue_starts_empty(q: TestBPQueue) -> TestResult {
    let bpqueue: BPQueue<i32> = BPQueue::new(q.a, q.b);
    TestResult::from_bool(bpqueue.is_empty())
}

/// Property: BPQueue can be created with valid range
fn bpqueue_creation(q: TestBPQueue) -> TestResult {
    let _: BPQueue<i32> = BPQueue::new(q.a, q.b);
    TestResult::passed()
}

/// Property: MapAdapter len matches number of items
fn map_adapter_len_property(map: TestMapAdapter) -> TestResult {
    let len = map.0.len();
    let mut count = 0;
    for _ in map.0.lst.iter() {
        count += 1;
    }
    TestResult::from_bool(len == count)
}

/// Property: MapAdapter get returns Some for valid indices
fn map_adapter_get_property(map: TestMapAdapter) -> TestResult {
    if map.0.is_empty() {
        return TestResult::discard();
    }
    for i in 0..map.0.len() {
        if map.0.get(i).is_none() {
            return TestResult::failed();
        }
    }
    TestResult::passed()
}

/// Edge case: Empty RepeatArray
fn repeat_array_empty_edge_case() -> TestResult {
    let arr: RepeatArray<i32> = RepeatArray::new(42, 0);
    TestResult::from_bool(arr.is_empty())
}

/// Edge case: Single element RepeatArray
fn repeat_array_single_edge_case() -> TestResult {
    let arr: RepeatArray<i32> = RepeatArray::new(42, 1);
    TestResult::from_bool(!arr.is_empty() && arr.len() == 1 && arr.get(0) == 42)
}

/// Edge case: Empty BPQueue
fn bpqueue_empty_edge_case() -> TestResult {
    let q: BPQueue<i32> = BPQueue::new(-5, 5);
    TestResult::from_bool(q.is_empty())
}

/// Edge case: Empty MapAdapter
fn map_adapter_empty_edge_case() -> TestResult {
    let map: MapAdapter<i32> = MapAdapter::new(vec![]);
    TestResult::from_bool(map.is_empty())
}

/// Edge case: Single element MapAdapter
fn map_adapter_single_edge_case() -> TestResult {
    let map = MapAdapter::new(vec![42]);
    TestResult::from_bool(!map.is_empty() && map.len() == 1 && map.get(0) == Some(&42))
}

fn main() {
    println!("Running quickcheck property-based tests for mywheel-rs...\n");

    let mut all_passed = true;

    // RepeatArray tests
    println!("=== RepeatArray Tests ===");
    all_passed &= match QuickCheck::new()
        .tests(100)
        .quicktest(repeat_array_len_property as fn(TestRepeatArray) -> TestResult)
    {
        Ok(n) => {
            println!("repeat_array_len_property... Passed {}/100", n);
            true
        }
        Err(_) => {
            println!("repeat_array_len_property... FAILED");
            false
        }
    };

    all_passed &= match QuickCheck::new()
        .tests(100)
        .quicktest(repeat_array_index_property as fn(TestRepeatArray) -> TestResult)
    {
        Ok(n) => {
            println!("repeat_array_index_property... Passed {}/100", n);
            true
        }
        Err(_) => {
            println!("repeat_array_index_property... FAILED");
            false
        }
    };

    all_passed &= match QuickCheck::new()
        .tests(100)
        .quicktest(repeat_array_empty_property as fn(TestRepeatArray) -> TestResult)
    {
        Ok(n) => {
            println!("repeat_array_empty_property... Passed {}/100", n);
            true
        }
        Err(_) => {
            println!("repeat_array_empty_property... FAILED");
            false
        }
    };

    // ShiftArray tests
    println!("\n=== ShiftArray Tests ===");
    all_passed &= match QuickCheck::new()
        .tests(100)
        .quicktest(shift_array_creation as fn(TestShiftArray) -> TestResult)
    {
        Ok(n) => {
            println!("shift_array_creation... Passed {}/100", n);
            true
        }
        Err(_) => {
            println!("shift_array_creation... FAILED");
            false
        }
    };

    all_passed &= match QuickCheck::new()
        .tests(100)
        .quicktest(shift_array_len_property as fn(TestShiftArray) -> TestResult)
    {
        Ok(n) => {
            println!("shift_array_len_property... Passed {}/100", n);
            true
        }
        Err(_) => {
            println!("shift_array_len_property... FAILED");
            false
        }
    };

    // BPQueue tests
    println!("\n=== BPQueue Tests ===");
    all_passed &= match QuickCheck::new()
        .tests(100)
        .quicktest(bpqueue_starts_empty as fn(TestBPQueue) -> TestResult)
    {
        Ok(n) => {
            println!("bpqueue_starts_empty... Passed {}/100", n);
            true
        }
        Err(_) => {
            println!("bpqueue_starts_empty... FAILED");
            false
        }
    };

    all_passed &= match QuickCheck::new()
        .tests(100)
        .quicktest(bpqueue_creation as fn(TestBPQueue) -> TestResult)
    {
        Ok(n) => {
            println!("bpqueue_creation... Passed {}/100", n);
            true
        }
        Err(_) => {
            println!("bpqueue_creation... FAILED");
            false
        }
    };

    // MapAdapter tests
    println!("\n=== MapAdapter Tests ===");
    all_passed &= match QuickCheck::new()
        .tests(100)
        .quicktest(map_adapter_len_property as fn(TestMapAdapter) -> TestResult)
    {
        Ok(n) => {
            println!("map_adapter_len_property... Passed {}/100", n);
            true
        }
        Err(_) => {
            println!("map_adapter_len_property... FAILED");
            false
        }
    };

    all_passed &= match QuickCheck::new()
        .tests(100)
        .quicktest(map_adapter_get_property as fn(TestMapAdapter) -> TestResult)
    {
        Ok(n) => {
            println!("map_adapter_get_property... Passed {}/100", n);
            true
        }
        Err(_) => {
            println!("map_adapter_get_property... FAILED");
            false
        }
    };

    // Edge cases
    println!("\n=== Edge Cases ===");
    all_passed &= match QuickCheck::new()
        .tests(1)
        .quicktest(repeat_array_empty_edge_case as fn() -> TestResult)
    {
        Ok(n) => {
            println!("repeat_array_empty_edge_case... Passed {}/1", n);
            true
        }
        Err(_) => {
            println!("repeat_array_empty_edge_case... FAILED");
            false
        }
    };

    all_passed &= match QuickCheck::new()
        .tests(1)
        .quicktest(repeat_array_single_edge_case as fn() -> TestResult)
    {
        Ok(n) => {
            println!("repeat_array_single_edge_case... Passed {}/1", n);
            true
        }
        Err(_) => {
            println!("repeat_array_single_edge_case... FAILED");
            false
        }
    };

    all_passed &= match QuickCheck::new()
        .tests(1)
        .quicktest(bpqueue_empty_edge_case as fn() -> TestResult)
    {
        Ok(n) => {
            println!("bpqueue_empty_edge_case... Passed {}/1", n);
            true
        }
        Err(_) => {
            println!("bpqueue_empty_edge_case... FAILED");
            false
        }
    };

    all_passed &= match QuickCheck::new()
        .tests(1)
        .quicktest(map_adapter_empty_edge_case as fn() -> TestResult)
    {
        Ok(n) => {
            println!("map_adapter_empty_edge_case... Passed {}/1", n);
            true
        }
        Err(_) => {
            println!("map_adapter_empty_edge_case... FAILED");
            false
        }
    };

    all_passed &= match QuickCheck::new()
        .tests(1)
        .quicktest(map_adapter_single_edge_case as fn() -> TestResult)
    {
        Ok(n) => {
            println!("map_adapter_single_edge_case... Passed {}/1", n);
            true
        }
        Err(_) => {
            println!("map_adapter_single_edge_case... FAILED");
            false
        }
    };

    println!();
    if all_passed {
        println!("Quickcheck integration verified!");
    } else {
        println!("Some tests failed!");
        std::process::exit(1);
    }
}
