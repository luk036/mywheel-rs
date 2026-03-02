//! Comprehensive benchmark suite for mywheel-rs data structures
//!
//! This benchmark suite compares mywheel-rs implementations against
//! std::collections equivalents across various workloads.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use mywheel_rs::array_like::{RepeatArray, ShiftArray};
use mywheel_rs::bpqueue::BPQueue;
use mywheel_rs::dllist::{Dllink, Dllist};
use mywheel_rs::map_adapter::MapAdapter;
use mywheel_rs::robin::Robin;
use std::collections::{BinaryHeap, LinkedList};

/// Benchmark RepeatArray vs Vec for constant data access
fn bench_repeat_array_access(c: &mut Criterion) {
    let mut group = c.benchmark_group("repeat_array_access");

    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("repeat_array", size), size, |b, &size| {
            let array = RepeatArray::new(42, size);
            b.iter(|| (0..size).map(|i| array[i]).sum::<i32>());
        });

        group.bench_with_input(BenchmarkId::new("std_vec", size), size, |b, &size| {
            let vec: Vec<i32> = vec![42; size];
            b.iter(|| (0..size).map(|i| vec[i]).sum::<i32>());
        });
    }

    group.finish();
}

/// Benchmark ShiftArray vs Vec for sliding window access
fn bench_shift_array_access(c: &mut Criterion) {
    let mut group = c.benchmark_group("shift_array_access");

    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("shift_array", size), size, |b, &size| {
            let mut array = ShiftArray::new(vec![1i32; size]);
            array.set_start(size / 2);
            b.iter(|| ((size / 2)..size).map(|i| array[i]).sum::<i32>());
        });

        group.bench_with_input(BenchmarkId::new("std_vec", size), size, |b, &size| {
            let vec: Vec<i32> = vec![1; size];
            let offset = size / 2;
            b.iter(|| (offset..size).map(|i| vec[i - offset]).sum::<i32>());
        });
    }

    group.finish();
}

/// Benchmark Dllist vs std::LinkedList append operations
fn bench_dllist_append(c: &mut Criterion) {
    let mut group = c.benchmark_group("dllist_append");

    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(
            BenchmarkId::new("mywheel_dllist", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let mut list = Dllist::new(0);
                    let mut nodes: Vec<Dllink<i32>> = (0..size).map(Dllink::new).collect();
                    for node in nodes.iter_mut() {
                        list.append(node);
                    }
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("std_linkedlist", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let mut list: LinkedList<i32> = LinkedList::new();
                    for i in 0..size {
                        list.push_back(i);
                    }
                });
            },
        );
    }

    group.finish();
}

/// Benchmark BPQueue vs std::BinaryHeap insert operations
fn bench_bpqueue_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("bpqueue_insert");

    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(
            BenchmarkId::new("mywheel_bpqueue", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let mut bpq = BPQueue::<i32>::new(0, 100);
                    let mut nodes: Vec<Dllink<(usize, i32)>> =
                        (0..size).map(|i| Dllink::new((0, i % 100))).collect();
                    for (i, node) in nodes.iter_mut().enumerate() {
                        bpq.append(node, (i % 100) as i32);
                    }
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("std_binaryheap", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let mut heap: BinaryHeap<i32> = BinaryHeap::new();
                    for i in 0..size {
                        heap.push(i % 100);
                    }
                });
            },
        );
    }

    group.finish();
}

/// Benchmark MapAdapter vs Vec for random access
fn bench_map_adapter_access(c: &mut Criterion) {
    let mut group = c.benchmark_group("map_adapter_access");

    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("map_adapter", size), size, |b, &size| {
            let adapter = MapAdapter::new(vec![1i32; size]);
            b.iter(|| (0..size).filter_map(|i| adapter.get(i)).sum::<i32>());
        });

        group.bench_with_input(BenchmarkId::new("std_vec", size), size, |b, &size| {
            let vec: Vec<i32> = vec![1; size];
            b.iter(|| (0..size).map(|i| vec[i]).sum::<i32>());
        });
    }

    group.finish();
}

/// Benchmark Robin iteration
fn bench_robin_iteration(c: &mut Criterion) {
    let mut group = c.benchmark_group("robin_iteration");

    for parts in [10, 50, 100].iter() {
        group.bench_with_input(BenchmarkId::new("robin", parts), parts, |b, &parts| {
            let robin = Robin::new(parts as u8);
            b.iter(|| {
                let mut count = 0;
                for _ in robin.exclude(0) {
                    count += 1;
                }
                count
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_repeat_array_access,
    bench_shift_array_access,
    bench_dllist_append,
    bench_bpqueue_insert,
    bench_map_adapter_access,
    bench_robin_iteration
);
criterion_main!(benches);
