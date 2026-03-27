/// Array-like data structures: RepeatArray and ShiftArray
pub mod array_like;
/// Bounded Priority Queue implementation
pub mod bpqueue;
/// Doubly linked list and node implementations
pub mod dllist;
/// Map adapter for vector-like access
pub mod map_adapter;
/// Round robin scheduling
pub mod robin;

#[cfg(feature = "std")]
/// Logging utilities (requires std feature)
pub mod logging;

// use bpqueue::BPQueue;
// use dllist::{DllIterator, Dllink, Dllist};
