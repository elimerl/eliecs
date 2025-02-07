use criterion::{black_box, criterion_group, criterion_main, Criterion};
use eliecs::Pool;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("pool 1k dense", |b| {
        let mut pool: Pool<u32> = Pool::with_dense_capacity(1000);

        b.iter(|| {
            pool.clear();
            for i in 0..1000 {
                pool.insert(i, i * i); // index is i * 5 to spread them out
            }

            for i in 0..1000 {
                black_box(pool.get(i));
            }
        })
    });
    c.bench_function("pool 1k sparse", |b| {
        let mut pool: Pool<u32> = Pool::with_dense_capacity(1000);

        b.iter(|| {
            pool.clear();
            for i in 0..1000 {
                pool.insert(i * 5, i * i); // index is i * 5 to spread them out
            }

            for i in 0..1000 {
                black_box(pool.get(i));
            }
        })
    });
    c.bench_function("pool 1M dense", |b| {
        let mut pool: Pool<u32> = Pool::with_dense_capacity(1000000);

        b.iter(|| {
            pool.clear();
            for i in 0..1000000 {
                pool.insert(i, i * i); // index is i * 5 to spread them out
            }

            for i in 0..1000000 {
                black_box(pool.get(i));
            }
        })
    });
    c.bench_function("pool 1M sparse", |b| {
        let mut pool: Pool<u32> = Pool::with_dense_capacity(1000000);

        b.iter(|| {
            pool.clear();
            for i in 0..1000000 {
                pool.insert(i * 5, i * i); // index is i * 5 to spread them out
            }

            for i in 0..1000000 {
                black_box(pool.get(i));
            }
        })
    });
    c.bench_function("pool 1M sparse read only", |b| {
        let mut pool: Pool<u32> = Pool::with_dense_capacity(1000000);
        for i in 0..1000000 {
            pool.insert(i * 5, i * i); // index is i * 5 to spread them out
        }
        b.iter(|| {
            for i in 0..1000000 {
                black_box(pool.get(i));
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
