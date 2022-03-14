use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use ironside::command::encode_int;
use rand::Rng;

fn klipper_impls(c: &mut Criterion) {
    let ranges = [
        (-32..95i32, 1),
        (-4096..12_287, 2),
        (-524_288..1_572_863, 3),
        (-67_108_864..201_326_591, 4),
        (-2147483648..2147483647, 5),
    ];
    let mut g = c.benchmark_group("encode");
    for (range, len) in ranges {
        g.bench_with_input(
            BenchmarkId::new("by encoded length", len),
            &range,
            |b, range| {
                let range = range.clone();
                let i = rand::thread_rng().gen_range(range);
                b.iter(move || {
                    encode_int(i as u32);
                })
            },
        );
    }
}

criterion_group!(benches, klipper_impls);
criterion_main!(benches);
