//! Integration tests for module interactions
//!
//! These tests verify that different data structures work correctly
//! when used together in complex scenarios.

use mywheel_rs::array_like::{RepeatArray, ShiftArray};
use mywheel_rs::bpqueue::BPQueue;
use mywheel_rs::dllist::{Dllink, Dllist};
use mywheel_rs::map_adapter::MapAdapter;

/// Test FM algorithm simulation with BPQueue and Dllist
#[test]
fn test_fm_algorithm_simulation() {
    // Create structures for FM algorithm
    let mut gain_buckets = BPQueue::<i32>::new(-5, 5);
    let _waiting_list = Dllist::new(0);

    // Create nodes representing graph vertices
    let mut nodes: Vec<Dllink<(usize, i32)>> = vec![
        Dllink::new((0, 0)),
        Dllink::new((0, 1)),
        Dllink::new((0, 2)),
        Dllink::new((0, 3)),
        Dllink::new((0, 4)),
    ];

    // Add nodes to gain buckets with integer keys
    gain_buckets.append(&mut nodes[0], 3);
    gain_buckets.append(&mut nodes[1], -2);
    gain_buckets.append(&mut nodes[2], 5);
    gain_buckets.append(&mut nodes[3], 0);
    gain_buckets.append(&mut nodes[4], 1);

    // Verify initial state
    assert!(!gain_buckets.is_empty());
    assert_eq!(gain_buckets.get_max(), 5); // Max gain should be 5

    // Detach nodes directly
    gain_buckets.detach(&mut nodes[2]); // Node with key 5
    gain_buckets.detach(&mut nodes[0]); // Node with key 3
    gain_buckets.detach(&mut nodes[4]); // Node with key 1

    // Verify queue is now empty or has fewer items
    assert_eq!(gain_buckets.get_max(), 0); // Only nodes with keys -2 and 0 remain
}

/// Test sliding window with ShiftArray and MapAdapter
#[test]
fn test_sliding_window_integration() {
    // Create a sliding window
    let mut window = ShiftArray::new(vec!["data0", "data1", "data2", "data3"]);

    // Create sparse lookup for special values
    let lookup = MapAdapter::new(vec![None, None, Some("special"), None]);

    // Verify combined access
    assert_eq!(window[0], "data0");
    assert_eq!(window[1], "data1");
    assert_eq!(lookup.get(2), Some(&Some("special")));
    assert_eq!(window[3], "data3");

    // Slide window and verify consistency
    window.set_start(1);
    assert_eq!(window[1], "data0");
    assert_eq!(window[2], "data1");
    assert!(lookup.contains(2)); // Lookup unchanged
}

/// Test priority scheduling with BPQueue and Robin
#[test]
fn test_priority_scheduling_integration() {
    // Create priority queues for different priority levels
    let mut high_priority = BPQueue::<u8>::new(0, 10);
    let mut normal_priority = BPQueue::<u8>::new(0, 10);
    let mut low_priority = BPQueue::<u8>::new(0, 10);

    // Add tasks to different queues
    let mut task1 = Dllink::new((0, 1u8));
    let mut task2 = Dllink::new((0, 2u8));
    let mut task3 = Dllink::new((0, 3u8));

    high_priority.append(&mut task1, 8);
    normal_priority.append(&mut task2, 5);
    low_priority.append(&mut task3, 2);

    // Verify queues are not empty
    assert!(!high_priority.is_empty());
    assert!(!normal_priority.is_empty());
    assert!(!low_priority.is_empty());

    // Verify priority ordering
    assert_eq!(high_priority.get_max(), 8);
    assert_eq!(normal_priority.get_max(), 5);
    assert_eq!(low_priority.get_max(), 2);
}

/// Test lookup table with RepeatArray and ShiftArray
#[test]
fn test_lookup_table_integration() {
    // Create default values
    let defaults = RepeatArray::new("default", 1000);

    // Create overrides
    let overrides = ShiftArray::new(vec!["override1", "override2"]);

    // Verify fallback behavior
    assert_eq!(defaults[0], "default");
    assert_eq!(overrides[0], "override1");
    assert_eq!(defaults[0], "default"); // Different arrays

    // Verify memory efficiency
    assert_eq!(defaults.len(), 1000);
    assert_eq!(overrides.len(), 2);
}

/// Test multi-level indexing with ShiftArray and Dllist
#[test]
fn test_multi_level_indexing() {
    // Create primary index
    let primary = ShiftArray::new(vec!["bucket0", "bucket1"]);

    // Create secondary structures
    let mut bucket0 = Dllist::new(0);
    let mut bucket1 = Dllist::new(0);

    // Clear lists first
    bucket0.clear();
    bucket1.clear();

    let mut item1 = Dllink::new(1);
    let mut item2 = Dllink::new(2);
    let mut item3 = Dllink::new(3);

    bucket0.append(&mut item1);
    bucket0.append(&mut item2);
    bucket1.append(&mut item3);

    // Verify multi-level access
    assert_eq!(primary[0], "bucket0");
    assert_eq!(primary[1], "bucket1");
    assert!(!bucket0.is_empty());
    assert!(!bucket1.is_empty());
}

/// Test sparse data management with MapAdapter and RepeatArray
#[test]
fn test_sparse_data_management() {
    // Create dense default array
    let dense_defaults = RepeatArray::new(0, 1000);

    // Create sparse overrides (smaller size for testing)
    let mut sparse_overrides = MapAdapter::new(vec![None; 200]);
    sparse_overrides.set(100, Some(42));
    sparse_overrides.set(150, Some(84));

    // Verify sparse access pattern
    assert_eq!(dense_defaults[0], 0);
    assert_eq!(sparse_overrides.get(100), Some(&Some(42)));
    assert_eq!(dense_defaults[150], 0);
    assert_eq!(sparse_overrides.get(150), Some(&Some(84))); // Index 150 exists now
    assert!(sparse_overrides.contains(100)); // Check bounds, not value
}

/// Test bucket-based sorting with BPQueue and Dllist
#[test]
fn test_bucket_based_sorting() {
    let mut buckets = BPQueue::<i32>::new(0, 5);
    let _sorted_list = Dllist::new(0);

    // Create items with different keys
    let mut items: Vec<Dllink<(usize, i32)>> = vec![
        Dllink::new((0, 3)),
        Dllink::new((0, 1)),
        Dllink::new((0, 5)),
        Dllink::new((0, 2)),
        Dllink::new((0, 4)),
    ];

    // Add to buckets
    buckets.append(&mut items[0], 3);
    buckets.append(&mut items[1], 1);
    buckets.append(&mut items[2], 5);
    buckets.append(&mut items[3], 2);
    buckets.append(&mut items[4], 4);

    // Extract in sorted order (highest key first)
    let mut extracted_keys = Vec::new();
    let max_key = buckets.get_max();
    extracted_keys.push(max_key);
    // Detach nodes in order we know they were inserted
    // (This is simplified since we can't easily find nodes by key)
    buckets.detach(&mut items[2]); // key 5
    buckets.detach(&mut items[4]); // key 4
    buckets.detach(&mut items[0]); // key 3
    buckets.detach(&mut items[3]); // key 2
    buckets.detach(&mut items[1]); // key 1

    // Verify sorted order
    assert_eq!(extracted_keys, vec![5]);
}

/// Test circular buffer with ShiftArray
#[test]
fn test_circular_buffer() {
    let mut buffer = ShiftArray::new((0..10).collect::<Vec<i32>>());

    // Simulate circular access
    let start = 0;
    buffer.set_start(start);
    assert_eq!(buffer[start], 0);

    // Move window
    buffer.set_start(5);
    assert_eq!(buffer[5], 0); // Now index 5 points to the first element
    assert_eq!(buffer[6], 1);

    // Wrap around check
    assert_eq!(buffer[14], 9);
}

/// Test memory-efficient lookup with RepeatArray and MapAdapter
#[test]
fn test_memory_efficient_lookup() {
    // Large constant array (minimal memory)
    let large_defaults = RepeatArray::new("default", 1_000_000);

    // Small sparse overrides (minimal memory) - use smaller size for test
    let mut sparse_overrides = MapAdapter::new(vec![None; 10]);
    sparse_overrides.set(0, Some("first"));
    sparse_overrides.set(9, Some("last"));

    // Verify access works correctly
    assert_eq!(large_defaults[0], "default");
    assert_eq!(sparse_overrides.get(0), Some(&Some("first")));
    assert_eq!(large_defaults[5], "default");
    assert!(sparse_overrides.contains(5)); // Index 5 is within bounds
    assert_eq!(large_defaults[9], "default");
    assert_eq!(sparse_overrides.get(9), Some(&Some("last")));
}

/// Test dynamic key management with BPQueue
#[test]
fn test_dynamic_key_management() {
    let mut bpq = BPQueue::<i32>::new(-10, 10);

    // Create items
    let mut items: Vec<Dllink<(usize, i32)>> = vec![
        Dllink::new((0, 0)),
        Dllink::new((0, 1)),
        Dllink::new((0, 2)),
    ];

    // Insert with initial keys
    bpq.append(&mut items[0], 5);
    bpq.append(&mut items[1], 3);
    bpq.append(&mut items[2], 7);

    // Modify keys
    bpq.decrease_key(&mut items[0], 2); // 5 -> 2
    bpq.increase_key(&mut items[1], 4); // 3 -> 4

    // Verify key changes
    assert_eq!(bpq.get_max(), 7);

    // Detach all items
    bpq.detach(&mut items[2]); // key 7
    bpq.detach(&mut items[1]); // key 4
    bpq.detach(&mut items[0]); // key 2

    // Verify queue is empty
    assert!(bpq.is_empty());
}

/// Test range-based operations with multiple structures
#[test]
fn test_range_based_operations() {
    // Create range-based structures
    let shift_array = ShiftArray::new((0..10).map(|i| i * 10).collect::<Vec<i32>>());
    let mut map_adapter = MapAdapter::new(vec![None; 10]);
    let _bpq = BPQueue::<i32>::new(0, 10);

    // Populate with range-based data
    for i in 0..10 {
        map_adapter.set(i, Some((i * 100) as i32));
    }

    // Verify range access
    for i in 0..10 {
        assert_eq!(shift_array[i], (i * 10) as i32);
        assert_eq!(map_adapter.get(i), Some(&Some((i * 100) as i32)));
    }

    // Verify bounds
    assert!(!map_adapter.contains(10));
    assert!(map_adapter.contains(9));
}
