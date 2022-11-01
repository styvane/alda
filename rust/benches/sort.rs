use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use alda::sort::{Container, Sort};

fn sorting_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Insertion sort");

    for container in [fake::vec![i32; 10000], fake::vec![i32; 100]]
        .into_iter()
        .map(Container::new)
    {
        group.bench_with_input(
            BenchmarkId::new("CLRS", container.len()),
            &container,
            |b, i| {
                b.iter(|| {
                    let mut container = i.clone();
                    container.clrs_insertion(|a, b| a > b);
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("Insertion", container.len()),
            &container,
            |b, i| {
                b.iter(|| {
                    let mut container = i.clone();
                    container.insertion(|a, b| a > b);
                })
            },
        );
    }

    group.finish();
}

criterion_group!(benches, sorting_benchmark);
criterion_main!(benches);
