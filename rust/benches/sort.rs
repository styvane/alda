use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use alda::sort::Sort;
use alda::Container;

fn sorting_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Insertion sort");

    #[rustfmt::skip]
    let data = [
        1900051505, 382349293, -1506816081, -590240448, 879863158, 1385790297, -459647565, -1413788150,
        -1046888025, -994760075, -1763081902, 0, 2147483647, 1218652326, -2147483648, 857293661, -953380327, 
        -1751933632, -1564223684, -1010527042, -1217998979, -458347561, 1613185258, 1476824899, 696852106, 
        -1442420187, -2147483648, 535791435, -801146902, -1199325717, -2147483648, -408041282, 2049258067, 
        4330442, 814600832, 294389313, -632150607, 61721288, -606708063, -2030087634, 581782196,
        -2086920665, 786182098, 196391289, -289039012, 1584347010, 1296717346, 0, 1993532039, 1947854319,
        -144419522, -2147483648, 1005930716, 1325755950, 86969421, 502163880, 274597710, 998970608,
        -377157898, 2008825421, 353434805, -753526708, 954835216, 0, 1013918934, -785012829, 1219793039,
        202597746, 1454734829, 117565486, -697139625, 1917677201, -2147483648, 1395026255, -1338512191,
        -796438198, -65316533, -1978288093, -395696769, 1025233388, 138650805, 703117888, 1757337917,
    ];
    for container in [data[..].to_vec(), data[..data.len() / 2].to_vec()]
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

        group.bench_with_input(
            BenchmarkId::new("Selection", container.len()),
            &container,
            |b, i| {
                b.iter(|| {
                    let mut container = i.clone();
                    container.insertion(|a, b| a < b);
                })
            },
        );
    }

    group.finish();
}

criterion_group!(benches, sorting_benchmark);
criterion_main!(benches);
