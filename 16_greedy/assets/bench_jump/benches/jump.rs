// benches/jump.rs
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

// Your benchmarked functions
fn jump_to_the_end_for(nums: &[usize]) -> bool {
    if nums.is_empty() {
        return false;
    }
    let mut destination = nums.len() - 1;
    for i in (0..nums.len()).rev() {
        if i + nums[i] >= destination {
            destination = i;
        }
    }
    destination == 0
}

fn jump_to_the_end_fold_range(nums: &[usize]) -> bool {
    if nums.is_empty() {
        return false;
    }
    (0..nums.len()).rev().fold(nums.len() - 1, |destination, i| if i + nums[i] >= destination { i } else { destination }) == 0
}

fn jump_to_the_end_fold_iter(nums: &[usize]) -> bool {
    if nums.is_empty() {
        return false;
    }
    nums.iter()
        .enumerate()
        .rev()
        .fold(nums.len() - 1, |destination, (i, &num)| if i + num >= destination { i } else { destination })
        == 0
}

// Benchmark function
fn benchmark_jump(c: &mut Criterion) {
    let nums: Vec<usize> = (0..10_000).map(|i| if i % 7 == 0 { 5 } else { 1 }).collect();

    c.bench_function("for loop", |b| b.iter(|| jump_to_the_end_for(black_box(&nums))));

    c.bench_function("fold with range", |b| b.iter(|| jump_to_the_end_fold_range(black_box(&nums))));

    c.bench_function("fold with iter enumerate", |b| b.iter(|| jump_to_the_end_fold_iter(black_box(&nums))));
}

// Mandatory macros to register the benchmark
criterion_group!(benches, benchmark_jump);
criterion_main!(benches);
