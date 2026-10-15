   // -------------------------------------------
   // 			Benchmarking using Criterion
   // ------------------------------------------- 

// add the following to cargo.toml file 
/*
[dev-dependencies] 
criterion = "0.4.0"  


[[bench]] 
name = "sorting_benchmark"
harness = false
*/  


// code for sorting_benchmark.rs file

use learning_rust::{sort_algo_1, sort_algo_2};

use criterion::{criterion_group, criterion_main, Criterion};

fn sort_benchmark(c: &mut Criterion) {
    let mut numbers: Vec<i32> = vec![
        1, 2, 3, 6, 5, 4, 8, 52, 2, 1, 5, 4, 4, 5, 8, 54, 2, 0, 55, 5, 2, 0, 5, 5, 5, 21,
    ];

    // This creates a benchmark
    c.bench_function("Sorting Algorithm", |b| {
        b.iter(|| sort_algo_2(&mut numbers))
    });
}

criterion_group!(benches, sort_benchmark);
criterion_main!(benches);


// Code for lib.rs
pub fn sort_algo_1<T: PartialOrd>(arr: &mut Vec<T>) {
    let mut swapped = false;
    for i in 0..(arr.len() - 1) {
      if arr[i] > arr[i + 1] {
        arr.swap(i, i + 1);
        swapped = true;
      }
    }
    if swapped {
      sort_algo_1(arr);
    }
  }



pub fn sort_algo_2<T: Ord>(arr: &mut Vec<T>) {
    let len = arr.len();
    for left in 0..len {
        let mut smallest = left;
        for right in (left + 1)..len {
            if arr[right] < arr[smallest] {
                smallest = right;
            }
        }
        arr.swap(smallest, left);
    }
}





