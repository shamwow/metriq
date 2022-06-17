use metriq;
use metriq::FastDisplay;
use criterion::{criterion_group, criterion_main, Criterion};
use solana_program::{pubkey::Pubkey};

fn benchmark_fast_fmt(c: &mut Criterion) {
    c.bench_function("pubkey", |b| {
        let pubkey1 = Pubkey::new_unique();
        let pubkey2 = Pubkey::new_unique();
        b.iter(|| {
            metriq::fast_fmt!(pubkey1, pubkey2);
        })
    });
    c.bench_function("str", |b| {
        let str1 = "test_string_one";
        let str2 = "test_string_two";
        b.iter(|| {
            metriq::fast_fmt!(str1, str2);
        })
    });
    c.bench_function("integer", |b| {
        let int1 = -5;
        let int2 = -10;
        b.iter(|| {
            metriq::fast_fmt!(int1, int2);
        })
    });
    c.bench_function("unsigned_integer", |b| {
        let int1: u32 = 5;
        let int2: u32 = 10;
        b.iter(|| {
            metriq::fast_fmt!(int1, int2);
        })
    });
    c.bench_function("key_pair", |b| {
        let keypair1 = metriq::kv("key1", "value1");
        let keypair2 = metriq::kv("key2", "value2");
        b.iter(|| {
            metriq::fast_fmt!(keypair1, keypair2);
        })
    });
}

fn benchmark_slow_fmt(c: &mut Criterion) {
    c.bench_function("pubkey_slow", |b| {
        let pubkey1 = Pubkey::new_unique();
        let pubkey2 = Pubkey::new_unique();
        b.iter(|| {
            format!("{}^{}^", pubkey1.to_string(), pubkey2.to_string());
        })
    });
    c.bench_function("str_slow", |b| {
        let str1 = "test_string_one";
        let str2 = "test_string_two";
        b.iter(|| {
            format!("{}^{}^", str1, str2);
        })
    });
    c.bench_function("integer_slow", |b| {
        let int1 = -5;
        let int2 = -10;
        b.iter(|| {
            format!("{}^{}^", int1, int2);
        })
    });
    c.bench_function("unsigned_integer_slow", |b| {
        let int1: u32 = 5;
        let int2: u32 = 10;
        b.iter(|| {
            format!("{}^{}^", int1, int2);
        })
    });
    c.bench_function("key_pair_slow", |b| {
        b.iter(|| {
            format!("{}`{}^{}`{}^", "key1", "value1", "key2", "value2");
        })
    });
}

criterion_group!(benches, benchmark_fast_fmt, benchmark_slow_fmt);
criterion_main!(benches);
