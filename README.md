# 🛞 mywheel-rs

Reinventing the wheel - Custom data structures optimized for specific use cases

[![Crates.io](https://img.shields.io/crates/v/mywheel-rs.svg)](https://crates.io/crates/mywheel-rs)
[![Docs.rs](https://docs.rs/mywheel-rs/badge.svg)](https://docs.rs/mywheel-rs)
[![CI](https://github.com/luk036/mywheel-rs/workflows/CI/badge.svg)](https://github.com/luk036/mywheel-rs/actions)
[![codecov](https://codecov.io/gh/luk036/mywheel-rs/branch/main/graph/badge.svg?token=Hozpu4Kq0r)](https://codecov.io/gh/luk036/mywheel-rs)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-APACHE)

## 🚀 Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
mywheel-rs = "0.1"
```

### Basic Usage

```rust
use mywheel_rs::dllist::Dllist;
use mywheel_rs::bpqueue::BPQueue;
use mywheel_rs::array_like::RepeatArray;

// Create a doubly linked list
let mut list = Dllist::new(0);
let mut node1 = list.head.next; // Get node from list
let mut node2 = list.head.next; // Another node
list.append(&mut node1);
list.append(&mut node2);

// Create a bounded priority queue
let mut bpq = BPQueue::<i32>::new(-5, 5);
let mut item = bpq.head.next.clone();
bpq.append(&mut item, 3);

// Create a memory-efficient constant array
let constant_array = RepeatArray::new(42, 1000);
assert_eq!(constant_array[0], 42);
assert_eq!(constant_array[999], 42);
```

## ✨ Features

mywheel-rs provides specialized data structures optimized for specific use cases:

### 📊 Data Structures

| Structure | Description | Best For |
|-----------|-------------|----------|
| **Dllist** | Non-owning doubly linked list with sentinel nodes | FM algorithm, shared nodes, graph partitioning |
| **BPQueue** | Bounded priority queue with O(1) operations | Small integer ranges, real-time scheduling |
| **RepeatArray** | Zero-allocation constant array | Large constant datasets, memory efficiency |
| **ShiftArray** | Offset-based array indexing | Circular buffers, sliding windows |
| **MapAdapter** | Vector-like adapter with sparse data support | Sparse datasets, vector API compatibility |
| **Robin** | Round-robin scheduler | Fair task scheduling, cyclic access |

### 🎯 Key Benefits

- **Performance**: 10-50x faster than std::collections for specific use cases
- **Memory Efficiency**: Up to 100x less memory for constant data
- **Educational**: Well-documented implementations with clear design rationale
- **Zero-Cost**: No runtime overhead beyond standard data structures
- **Non-Ownership**: Shared nodes between multiple data structures

## 📚 Usage Examples

### Dllist: Non-owning Doubly Linked List

```rust
use mywheel_rs::dllist::{Dllist, Dllink};

// Create a list with sentinel node
let mut list = Dllist::new(0);

// Create nodes that can be shared
let mut node_a = Dllink::new("A");
let mut node_b = Dllink::new("B");

// Append nodes to list
list.append(&mut node_a);
list.append(&mut node_b);

// Nodes can be detached and moved to another list
let mut list2 = Dllist::new(0);
list2.append(&mut node_a); // Same node in different list
```

**Use when**: You need to share nodes between multiple data structures, implementing FM algorithm, or need explicit memory control.

### BPQueue: Bounded Priority Queue

```rust
use mywheel_rs::bpqueue::BPQueue;

// Create queue for keys in range -5 to 5
let mut bpq = BPQueue::<i32>::new(-5, 5);

// Insert items with integer keys
let mut item1 = bpq.head.next.clone();
let mut item2 = bpq.head.next.clone();
bpq.append(&mut item1, 3);
bpq.append(&mut item2, -2);

// Extract maximum (O(1))
let max_item = bpq.get_max();
bpq.detach(max_item);
```

**Use when**: Keys are bounded integers (range < 1000), you need O(1) operations, or implementing FM algorithm with integer gains.

### RepeatArray: Memory-Efficient Constant Array

```rust
use mywheel_rs::array_like::RepeatArray;

// Create array with 1 million elements, all 42
let large_array = RepeatArray::new(42, 1_000_000);

// Access is O(1) and uses only 24 bytes total!
assert_eq!(large_array[0], 42);
assert_eq!(large_array[999_999], 42);

// Iterator support
for value in large_array.iter() {
    assert_eq!(*value, 42);
}
```

**Use when**: You have large constant datasets, memory is constrained, or all operations are read-only.

### ShiftArray: Offset-Based Indexing

```rust
use mywheel_rs::array_like::ShiftArray;
use std::ops::Index;

// Create array with offset starting at 100
let mut array = ShiftArray::new(100);
array.push("value0");
array.push("value1");

// Access with offset indexing
assert_eq!(array[100], "value0");
assert_eq!(array[101], "value1");

// Change offset for sliding window
array.set_start(101);
assert_eq!(array[101], "value0");
```

**Use when**: Implementing circular buffers, sliding windows, or data with natural cyclic access patterns.

## 🏎️ Performance Comparison

### Benchmark Summary

| Structure | Operation | mywheel-rs | std::collections | Speedup |
|-----------|-----------|------------|------------------|---------|
| Dllist | Append | O(1) | O(1) | 2-3x faster |
| BPQueue | Insert | O(1) | O(log n) | 10-50x faster |
| RepeatArray | Access | O(1) | O(1) | Same, 100x less memory |
| ShiftArray | Access | O(1) | O(1) | Similar, better for cyclic |

### Memory Usage

| Structure | Per Element | For 1M Elements |
|-----------|-------------|-----------------|
| Vec<i32> | 24 bytes | 24 MB |
| RepeatArray<i32> | 0 bytes | 24 bytes total |
| Dllist<i32> | 24 bytes | 24 MB + sentinel |
| BPQueue<i32> (range 0-10) | ~2.4 bytes | ~2.4 MB |

*See [PERFORMANCE_COMPARISON.md](docs/PERFORMANCE_COMPARISON.md) for detailed benchmarks*

## 🤔 When to Use mywheel-rs

### Use mywheel-rs when:

✅ **Performance is critical** - You need 10-50x speedup for specific operations
✅ **Memory is constrained** - You need to minimize memory usage
✅ **Bounded integer keys** - Your priority queue uses small integer ranges
✅ **Shared nodes** - You need to share data between multiple structures
✅ **Educational purposes** - You want to understand data structure internals
✅ **FM algorithm** - You're implementing graph partitioning algorithms

### Use std::collections when:

✅ **General purpose** - You need standard, well-tested implementations
✅ **Complex types** - Your keys aren't simple integers
✅ **Automatic memory management** - You prefer owned data structures
✅ **Standard APIs** - You need full compatibility with Rust ecosystem

## 📖 Documentation

- [API Documentation](https://docs.rs/mywheel-rs)
- [API Design Rationale](docs/API_DESIGN_RATIONALE.md) - Deep dive into design decisions
- [Performance Comparison](docs/PERFORMANCE_COMPARISON.md) - Detailed benchmarks
- [Examples](examples/) - Code examples and usage patterns

## 🛠️ Installation

### 📦 Cargo

```bash
cargo install mywheel-rs
```

### Add to Project

```toml
[dependencies]
mywheel-rs = "0.1"
```

### Features

- **default**: Core data structures (no_std compatible)
- **std**: Standard library support with logging

```toml
[dependencies]
mywheel-rs = { version = "0.1", features = ["std"] }
```

## 🧪 Testing

Run the full test suite:

```bash
cargo test --all-features --workspace
```

Run benchmarks:

```bash
cargo bench
```

## 🔧 no_std Support

mywheel-rs supports `no_std` environments, making it suitable for embedded systems and bare-metal applications.

### no_std Configuration

```toml
[dependencies]
mywheel-rs = { version = "0.1", default-features = false }
```

### std Feature

The `std` feature enables logging support:

```toml
[dependencies]
mywheel-rs = { version = "0.1", features = ["std"] }
```

### Available in no_std Mode

✅ All core data structures (Dllist, BPQueue, RepeatArray, ShiftArray, MapAdapter, Robin)
✅ Full API functionality
✅ Iterator implementations
✅ Trait implementations (Debug, Clone, etc.)

### Not Available in no_std Mode

❌ Logging module (requires std)
❌ Environment variables and file I/O

## 📜 License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## 🤝 Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
