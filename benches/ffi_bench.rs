use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use ironside::proto::KlipperBytes;
use rand::Rng;

fn klipper_impls(c: &mut Criterion) {
    let ranges = [
        (-32..95i32, 1),
        (-4096..12287, 2),
        (-524288..1572863, 3),
        (-67108864..201326591, 4),
        (-2147483648..2147483647, 5),
    ];
    let mut g = c.benchmark_group("encode");
    for (range, len) in ranges {
        // Generate a random int in the given range and encode it
        // ensuring its length matches the documentation
        g.bench_with_input(
            BenchmarkId::new("by encoded length", len),
            &range,
            |b, range| {
                let range = range.clone();
                let i = rand::thread_rng().gen_range(range);
                b.iter(move || {
                    (i as u32).to_klipper_bytes();
                })
            },
        );
    }
}

criterion_group!(benches, klipper_impls);
criterion_main!(benches);
