// Copyright 2021 Twitter, Inc.
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use rand::SeedableRng;
use rand::{Rng, RngCore};

use mixed_ds::{Index, StrValue};
use std::time::Duration;

pub const MB: usize = 1024 * 1024;
use rand::distributions::{Alphanumeric, DistString};
// A very fast PRNG which is appropriate for testing
pub fn rng() -> impl RngCore {
    rand_xoshiro::Xoshiro256PlusPlus::seed_from_u64(0)
}

fn get_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("get");
    group.measurement_time(Duration::from_secs(30));
    group.throughput(Throughput::Elements(1));

    for key_size in [1, 255].iter() {
        let (keys, _values) = key_values(*key_size, 1_000_000, 0, 0);

        // launch the server
        let mut cache = Index::new();

        let mut key = 0;

        group.bench_function(&format!("{key_size}b/0b"), |b| {
            b.iter(|| {
                cache.get(&keys[key]);
                key += 1;
                if key >= keys.len() {
                    key = 0;
                }
            })
        });
    }
}

fn key_values(
    key_size: usize,
    key_count: usize,
    value_size: usize,
    value_count: usize,
) -> (Vec<String>, Vec<String>) {
    let mut keys = Vec::new();
    let mut rng = rng();
    for i in 0..key_count {
        let key = Alphanumeric.sample_string(&mut rng, key_size);

        keys.push(key);
    }

    let mut values = Vec::new();
    for i in 0..value_count {
        let value = Alphanumeric.sample_string(&mut rng, value_size);

        values.push(value);
    }

    (keys, values)
}

fn set_benchmark(c: &mut Criterion) {
    let ttl = Duration::ZERO;
    let mut group = c.benchmark_group("set");
    group.measurement_time(Duration::from_secs(30));
    group.throughput(Throughput::Elements(1));

    for key_size in [1, 255].iter() {
        for value_size in [1, 64, 1024, 16384].iter() {
            let (keys, values) = key_values(*key_size, 1_000_000, *value_size, 10_000);

            // launch the server
            let mut cache = Index::new();

            let mut key = 0;
            let mut value = 0;

            group.bench_function(&format!("{key_size}b/{value_size}b"), |b| {
                let str_value = StrValue::StringValue(values[value].clone());
                b.iter(|| {
                    let _ = cache.set(keys[key].clone(), &str_value);
                    key += 1;
                    if key >= keys.len() {
                        key = 0;
                    }
                    value += 1;
                    if value >= values.len() {
                        value = 0;
                    }
                })
            });
        }
    }
}

criterion_group!(benches, get_benchmark, set_benchmark);
criterion_main!(benches);
