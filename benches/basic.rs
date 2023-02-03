use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn placeholder() {
    let mut x = 1;
    for i in 1..10 {
        x += i;
    }
    black_box(x);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("placeholder", |b| b.iter(placeholder));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);