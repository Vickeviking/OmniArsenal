use criterion::{criterion_group, criterion_main, Criterion};
use rand::seq::SliceRandom;
use rand::thread_rng;
use ::omni_arsenal::bubblesort;
use ::omni_arsenal::insertionsort;
use ::omni_arsenal::selectionsort;
use ::omni_arsenal::mergesort;
use ::omni_arsenal::quicksort;
use ::omni_arsenal::heapsort;



fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sorting algorithms");

    for &size in [100, 1000, 10000].iter() {
        let mut rng = thread_rng();
        let mut arr: Vec<_> = (0..size).collect();
        arr.shuffle(&mut rng);

        group.bench_function(format!("Bubble sort {}", size), |b| b.iter(|| {
            let mut arr_clone = arr.clone();
            bubblesort(&mut arr_clone);
        }));

        group.bench_function(format!("Insertion sort {}", size), |b| b.iter(|| {
            let mut arr_clone = arr.clone();
            insertionsort(&mut arr_clone);
        }));

        group.bench_function(format!("Selection sort {}", size), |b| b.iter(|| {
            let mut arr_clone = arr.clone();
            selectionsort(&mut arr_clone);
        }));

        group.bench_function(format!("Merge sort {}", size), |b| b.iter(|| {
            let mut arr_clone = arr.clone();
            mergesort(&mut arr_clone);
        }));

        group.bench_function(format!("Quick sort {}", size), |b| b.iter(|| {
            let mut arr_clone = arr.clone();
            quicksort(&mut arr_clone);
        }));

        group.bench_function(format!("Heap sort {}", size), |b| b.iter(|| {
            let mut arr_clone = arr.clone();
            heapsort(&mut arr_clone);
        }));

    }
    group.finish();
}

//make a heavy test on heap sort 
fn benchmark_heavy(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sorting algorithms");

    for &size in [1000000].iter() {
        let mut rng = thread_rng();
        let mut arr: Vec<_> = (0..size).collect();
        arr.shuffle(&mut rng);

        group.bench_function(format!("Heap sort {}", size), |b| b.iter(|| {
            let mut arr_clone = arr.clone();
            heapsort(&mut arr_clone);
        }));

    }
    group.finish();
}

criterion_group!(benches, benchmark, benchmark_heavy);
criterion_main!(benches);