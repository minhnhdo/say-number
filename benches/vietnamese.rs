use criterion::{black_box, criterion_group, criterion_main, Criterion};
use say_number::vietnamese::say;

fn say_0(c: &mut Criterion) {
    c.bench_function("say 0 in Vietnamese", |b| b.iter(|| say(black_box(0))));
}

fn say_42(c: &mut Criterion) {
    c.bench_function("say 42 in Vietnamese", |b| b.iter(|| say(black_box(42))));
}

fn say_514(c: &mut Criterion) {
    c.bench_function("say 514 in Vietnamese", |b| b.iter(|| say(black_box(514))));
}

fn say_max_u64(c: &mut Criterion) {
    c.bench_function("say max u64 in Vietnamese", |b| {
        b.iter(|| say(black_box(std::u64::MAX)))
    });
}

criterion_group!(vietnamese, say_max_u64, say_514, say_42, say_0);
criterion_main!(vietnamese);
