# API Design Rationale

This document explains the design decisions behind the data structures in mywheel-rs,
providing context for understanding when and why to use each implementation.

## Table of Contents

- [Design Philosophy](#design-philosophy)
- [Dllist Design](#dllist-design)
- [BPQueue Design](#bpqueue-design) 
- [Array Structures Design](#array-design)
- [MapAdapter Design](#map-adapter-design)
- [Robin Design](#robin-design)
- [Performance Considerations](#performance-considerations)
- [Trade-offs and Decisions](#trade-offs-decisions)

## Design Philosophy

The mywheel-rs project follows specific design principles:

### Educational Focus
- **"Reinventing the wheel"** approach to provide deep understanding of fundamental algorithms
- **Clarity over optimization** - Priority on making code readable and well-documented
- **Step-by-step learning** - Each implementation demonstrates specific algorithmic concepts
- **Real-world context** - Examples show practical applications of theoretical structures

### Non-Ownership Model
- **Shared nodes** - Structures don't own data, enabling flexible usage patterns
- **External lifetime management** - Users control node allocation and deallocation
- **Memory efficiency** - Minimize allocations while maintaining safety
- **Interoperability** - Design allows easy combination of different data structures

### API Design Principles
- **Explicit APIs** - Clear separation of concerns between different operations
- **Type safety** - Leverage Rust's type system for compile-time guarantees
- **Zero-cost abstractions** - No hidden allocations or performance penalties
- **Unsafe only when necessary** - Use raw pointers only where they provide clear benefits
- **Extensibility** - Trait-based design allows custom implementations

## Dllist Design

### Core Concepts
The doubly linked list implementation is designed around **non-owning nodes** and **sentinel-based design**.

### Non-Ownership Benefits
```rust
// Nodes can be shared between multiple data structures
let mut shared_node = Dllink::new("shared_data");
list1.append(&mut shared_node);
list2.append(&mut shared_node); // Same node in two lists
```

This approach enables:
- **FM algorithm implementations** where nodes move between priority queues and waiting lists
- **Memory pooling** where expensive nodes are reused across operations
- **Zero-copy transfers** of data between different structures

### Sentinel Node Design
```rust
pub struct Dllist<T> {
    pub head: Dllink<T>, // Sentinel node with specific data
}

impl<T> Dllist<T> {
    pub fn clear(&mut self) {
        self.head.clear(); // Creates self-referential loop
    }
}
```

**Benefits:**
- Eliminates null checks in hot paths
- Simplifies iteration logic (no special case for empty list)
- Reduces branching complexity in algorithms

### Unsafe Usage Rationale
Raw pointers are used strategically:
```rust
// Safe: self.head.next points to valid node (self or other list nodes)
unsafe {
    (*self.head.prev).next = new_node; // Known valid pointer
}
```

**Safety guarantees:**
- All pointer dereferences target known-valid memory
- Lifetime annotations prevent dangling references
- Assertions validate preconditions in debug builds

## BPQueue Design

### Bounded Priority Queue Concept
BPQueue implements a **bucket-based priority queue** optimized for **small integer ranges**.

### Range-Based Optimization
```rust
let mut bpq = BPQueue::<i32>::new(-5, 5); // Range: -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5
```

**Design advantages:**
- **O(1) key operations** - Direct array indexing
- **Cache efficiency** - Bucket array provides excellent locality
- **Predictable performance** - No heap allocations in common operations

### Sentinel Bucket Management
```rust
// Extra bucket at index 0 always empty for boundary management
let mut bpq = BPQueue::<i32>::new(-6, 6); // One extra bucket
bpq.bucket[0].append(&mut sentinel); // Boundary checking elimination
```

**Benefits:**
- **O(1) insert and delete** without searching
- **Memory predictability** - Fixed overhead per key range
- **Real-time performance** - Bounded latency critical for real-time systems

### FM Algorithm Integration
BPQueue is specifically designed for **Fiduccia-Mattheyses partitioning algorithms**:
```rust
// Nodes maintain (gain, data) tuples for FM operations
bpq.modify_key(&mut node, new_gain); // O(log k) priority update
```

**Design alignment:**
- **Shared nodes** allow moving between buckets without data copying
- **Bounded keys** match integer gain values in partitioning
- **Cache-friendly buckets** complement FM algorithm's access patterns

## Array Structures Design

### RepeatArray: Zero-Allocation Abstraction
```rust
pub struct RepeatArray<T: Copy> {
    value: T,
    size: usize,
}

impl<T: Copy> Index<usize> for RepeatArray<T> {
    fn index(&self, _index: usize) -> &Self::Output {
        &self.value // Always returns same reference
    }
}
```

**Memory efficiency:**
- **100x less memory** than `Vec<T>` for constant data
- **Single cache line** for all elements
- **O(1) access** regardless of array size

### ShiftArray: Offset-Based Indexing
```rust
pub struct ShiftArray<T> {
    start: usize,
    lst: Vec<T>,
}

impl<T> Index<usize> for ShiftArray<T> {
    fn index(&self, key: usize) -> &Self::Output {
        &self.lst[key - self.start] // O(1) translation
    }
}
```

**Use cases:**
- **Sliding windows** - O(1) window updates with `start` offset changes
- **Circular buffers** - Wraparound addressing for cyclic data
- **In-place operations** - No allocations for data transformations

## MapAdapter Design

### Vector-Like API Compatibility
```rust
impl<T> Index<usize> for MapAdapter<T> {
    fn index(&self, key: usize) -> &Self::Output {
        &self.lst[key] // Direct Vec access
    }
}
```

**Design advantages:**
- **Drop-in replacement** for Vec in sparse data scenarios
- **Bounds checking** - `get()` returns `Option` for safe access
- **Familiar API** - Mirrors `Vec<T>` interface for easier adoption

### Sparse Data Optimization
```rust
pub fn contains(&self, key: usize) -> bool {
    key < self.lst.len() // O(1) check
}
```

**Memory patterns:**
- **Empty slots** represented by missing values (requires Option handling)
- **Efficient iteration** - No allocations during traversal
- **Cache-conscious access** - Sequential memory access patterns

## Robin Design

### Round-Robin Scheduling Algorithm
```rust
pub struct Robin {
    cycle: Vec<u8>, // Pre-computed cycle
}

impl<'a> Iterator for Robin<'a> {
    fn next(&mut self) -> Option<Self::Item> {
        // O(1) iteration with state machine
    }
}
```

**Design characteristics:**
- **O(1) iteration** with no allocation overhead
- **Fixed memory footprint** - Deterministic size for known participant counts
- **Fair access patterns** - Equal iteration probability for all parts

### Limitations and Extensions
**Current limitations:**
- **u8 part limit** (255 maximum participants)
- **No priority support** - Pure round-robin only
- **Fixed cycle size** - Requires external iteration control for dynamic sizing

**Potential extensions:**
- **Weighted round-robin** - Different time allocations per part
- **Priority-based scheduling** - Integration with priority queue
- **Dynamic participant management** - Add/remove parts during operation

## Performance Considerations

### Cache Optimization Strategies

#### L1 Cache Considerations
- **Sequential access**: BPQueue bucket array provides excellent spatial locality
- **Hot data structures**: Frequently accessed nodes should be in low-index buckets
- **Memory alignment**: 64-byte alignment for optimal cache line usage

#### Cache-Avoiding Patterns
```rust
// Avoid: Pointer chasing in linked lists
for node in list.iter() {
    // Cache-inefficient: random memory access pattern
}

// Prefer: Array-based iteration
for i in 0..array.len() {
    // Cache-friendly: sequential memory access
    process(array[i]);
}
```

### Memory Access Patterns

#### Optimal Patterns
1. **Sequential access** - Best for cache prefetching
2. **Strided access** - Regular stride patterns (matrix operations)
3. **Blocked access** - Process contiguous chunks
4. **Access locality** - Group related operations spatially

#### Suboptimal Patterns
1. **Pointer chasing** - Linked list random access
2. **Random access** - Poor cache hit rates
3. **False sharing** - Multiple threads writing same cache lines

### Algorithmic Complexity Trade-offs

#### Time vs Space Complexity
| Operation | Custom Implementation | Standard Library | When to Prefer Custom |
|------------|-------------------|-----------------|-------------------|-----------------------|
| Access | O(1) | O(1) | Custom when cache-critical |
| Insert | O(1) | O(log n) | Custom for bounded ranges |
| Search | O(k) | O(log n) | Custom for specialized structures |
| Delete | O(1) | O(1) | Custom when non-owning needed |

#### Memory Usage Patterns
| Structure | Memory Per Element | Total Memory | Use Cases |
|-----------|------------------|------------|-----------|---------------|
| Vec<T> | 24 + sizeof(T) | 24n + n*sizeof(T) | General purpose |
| Dllist | 24 (sentinel) + 1n | 24 + 24n | Shared nodes |
| BPQueue | 24 + k + n | 24 + k*n | Bounded keys |
| RepeatArray | 24 + sizeof(T) | 24 + sizeof(T) | Constant data |
| ShiftArray | 24 + sizeof(T) | 24 + n*sizeof(T) | Offset addressing |

## Trade-offs and Decisions

### Binary Heap vs BPQueue

**std::collections::BinaryHeap advantages:**
- Unbounded keys with any orderable type
- Automatic memory management
- Standard API integration

**BPQueue advantages:**
- O(1) operations for bounded integer ranges
- Predictable memory usage
- Better cache locality for small ranges

### Raw Pointers vs Safe Abstractions

**Raw pointers (Dllist):**
- **Pros**: Maximum performance, zero allocation, precise memory layout
- **Cons**: Requires manual memory management, unsafe code, error-prone

**Safe abstractions (Vec):**
- **Pros**: Memory safety, automatic management, Rust idiomatic
- **Cons**: Allocation overhead, less control over memory layout

### Evolution Strategy

The mywheel-rs project demonstrates a **progressive enhancement approach**:

1. **Foundation** - Basic, safe implementations of fundamental algorithms
2. **Performance layering** - Add specialized, optimized versions alongside general ones
3. **Educational layering** - Rich examples and documentation for each design choice
4. **Integration patterns** - Demonstrate how different structures work together

This design philosophy ensures that each data structure serves both **educational purposes** and **practical performance needs**, with clear documentation of when each approach is optimal.