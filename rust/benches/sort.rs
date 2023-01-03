use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use alda::Container;
use alda::{heap::Heap, sort::Sort};

mod data;
use data::DATA;

fn sorting_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Insertion sort");
    for container in [DATA[..].to_vec(), DATA[..DATA.len() / 2].to_vec()]
        .into_iter()
        .map(Container::new)
    {
        group.bench_with_input(
            BenchmarkId::new("CLRS", container.len()),
            &container,
            |b, i| {
                b.iter(|| {
                    let mut container = i.clone();
                    container.naive_insertion_sort(|a, b| a > b);
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("Insertion", container.len()),
            &container,
            |b, i| {
                b.iter(|| {
                    let mut container = i.clone();
                    container.insertion_sort(|a, b| a > b);
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("Selection", container.len()),
            &container,
            |b, i| {
                b.iter(|| {
                    let mut container = i.clone();
                    container.insertion_sort(|a, b| a < b);
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("MergeSort", container.len()),
            &container,
            |b, i| {
                b.iter(|| {
                    let mut container = i.clone();
                    container.merge_sort(0, container.len());
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("RecursiveInsertionSort", container.len()),
            &container,
            |b, i| {
                b.iter(|| {
                    let mut container = i.clone();
                    container.rec_insertion_sort();
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::new("HeapSort", container.len()),
            &container,
            |b, i| {
                b.iter(|| {
                    let mut heap = Heap::new(i.inner().to_owned());
                    heap.sort();
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::new("QuickSort", container.len()),
            &container,
            |b, i| {
                b.iter(|| {
                    let mut container = i.clone();
                    container.quick_sort(0, container.len());
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::new("RandomizedQuickSort", container.len()),
            &container,
            |b, i| {
                b.iter(|| {
                    let mut container = i.clone();
                    container.randomize_quick_sort(0, container.len());
                })
            },
        );
    }

    group.finish();
}

criterion_group!(benches, sorting_benchmark);
criterion_main!(benches);
