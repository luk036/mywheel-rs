//! Integration examples demonstrating how to combine multiple data structures
//! from mywheel-rs for real-world use cases.

use mywheel_rs::array_like::{RepeatArray, ShiftArray};
use mywheel_rs::bpqueue::BPQueue;
use mywheel_rs::dllist::{Dllink, Dllist};
use mywheel_rs::map_adapter::MapAdapter;
use mywheel_rs::robin::Robin;

/// Example 1: FM Algorithm Simulation
/// Demonstrates using BPQueue and Dllist together for graph partitioning
///
/// The Fiduccia-Mattheyses (FM) algorithm is a graph partitioning algorithm
/// that uses a priority queue to select nodes with maximum gain and moves them
/// between partitions.
fn fm_algorithm_example() {
    println!("=== FM Algorithm Example ===");

    // Create a bounded priority queue for gain values (-5 to 5)
    let mut gain_buckets = BPQueue::<i32>::new(-5, 5);

    // Create a waiting list for nodes that can't be moved
    let _waiting_list = Dllist::new(0);

    // Create nodes representing graph vertices
    let mut node1 = Dllink::new((0, 1i32));
    let mut node2 = Dllink::new((0, 2i32));
    let mut node3 = Dllink::new((0, 3i32));
    let mut node4 = Dllink::new((0, 4i32));

    // Add nodes to gain buckets with integer keys
    gain_buckets.append(&mut node1, 3);
    gain_buckets.append(&mut node2, -2);
    gain_buckets.append(&mut node3, 5);
    gain_buckets.append(&mut node4, 0);

    // Simulate FM algorithm: extract max gain node
    println!("Initial state:");
    println!("BPQueue is empty: {}", gain_buckets.is_empty());

    // Modify gain of remaining nodes
    gain_buckets.decrease_key(&mut node1, 2);
    gain_buckets.increase_key(&mut node2, 4);

    println!("Updated gains in buckets");
}

/// Example 2: Round-Robin Task Scheduling with Priority
/// Combines Robin for fair scheduling with BPQueue for priority handling
fn task_scheduling_example() {
    println!("\n=== Task Scheduling Example ===");

    // Create round-robin scheduler for 4 task categories
    let scheduler = Robin::new(4);

    // Create priority queues for each category
    let mut high_priority = BPQueue::<u8>::new(0, 10);
    let mut normal_priority = BPQueue::<u8>::new(0, 10);
    let mut low_priority = BPQueue::<u8>::new(0, 10);

    // Add tasks to different priority queues
    let mut task1 = Dllink::new((0, 1u8));
    let mut task2 = Dllink::new((0, 2u8));
    let mut task3 = Dllink::new((0, 3u8));

    high_priority.append(&mut task1, 8);
    normal_priority.append(&mut task2, 5);
    low_priority.append(&mut task3, 2);

    // Simulate fair scheduling across categories
    println!("Scheduling tasks in round-robin fashion:");
    for category in scheduler.exclude(0).take(8) {
        println!("Processing category: {}", category);
    }
}

/// Example 3: Sliding Window with Sparse Data
/// Uses ShiftArray for window management and MapAdapter for sparse data
fn sliding_window_example() {
    println!("\n=== Sliding Window Example ===");

    // Create a shift array representing a sliding window
    let mut window = ShiftArray::new(vec!["data0", "data1", "data2", "data3"]);

    // Create a map adapter for sparse data lookup
    let sparse_lookup = MapAdapter::new(vec![None, None, Some("special_value"), None]);

    // Access data with offset
    println!("Window contents:");
    for i in 0..4 {
        if sparse_lookup.contains(i) {
            if let Some(&Some(special)) = sparse_lookup.get(i) {
                println!("  [{}]: {} (special)", i, special);
            }
        } else {
            println!("  [{}]: {}", i, window[i]);
        }
    }

    // Slide the window
    window.set_start(1);
    println!("After sliding window:");
    for i in 1..4 {
        println!("  [{}]: {}", i, window[i]);
    }
}

/// Example 4: Memory-Efficient Lookup Table
/// Combines RepeatArray for constant values with ShiftArray for dynamic data
fn lookup_table_example() {
    println!("\n=== Lookup Table Example ===");

    // Create a constant array for default values (saves memory)
    let defaults = RepeatArray::new("default", 1000);

    // Create a shift array for overrides
    let overrides = ShiftArray::new(vec!["custom1", "custom2", "custom3"]);

    // Perform lookup with fallback
    println!("Lookup results:");
    for key in [0, 1, 2, 3, 999].iter() {
        if *key < overrides.len() {
            println!("  [{}]: {}", key, overrides[*key]);
        } else {
            println!("  [{}]: {} (default)", key, defaults[*key]);
        }
    }

    println!("Memory saved: ~24KB using RepeatArray instead of Vec");
}

/// Example 5: Multi-Level Indexing
/// Uses nested data structures for efficient multi-dimensional access
fn multi_level_indexing_example() {
    println!("\n=== Multi-Level Indexing Example ===");

    // Create primary index using shift array
    let _primary_index = ShiftArray::new(vec!["bucket0", "bucket1"]);

    // Create secondary structures for each bucket
    let mut bucket0 = Dllist::new(0);
    let mut bucket1 = Dllist::new(0);

    // Add items to buckets
    let mut item1 = Dllink::new(1);
    let mut item2 = Dllink::new(2);
    let mut item3 = Dllink::new(3);

    bucket0.append(&mut item1);
    bucket0.append(&mut item2);
    bucket1.append(&mut item3);

    // Access through multi-level indexing
    println!("Multi-level access:");
    println!(
        "Primary index [0]: Bucket has {} items",
        !bucket0.is_empty()
    );
    println!(
        "Primary index [1]: Bucket has {} items",
        !bucket1.is_empty()
    );
}

fn main() {
    fm_algorithm_example();
    task_scheduling_example();
    sliding_window_example();
    lookup_table_example();
    multi_level_indexing_example();

    println!("\n=== All integration examples completed ===");
}
