#![feature(test)]

extern crate test;

use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use test::Bencher;

#[inline]
fn bench_hasher<T: Hasher + Default>() -> u64 {
    let mut v: u64 = test::black_box(114514);
    for _ in 0..100000 {
        let mut hasher = T::default();
        hasher.write_u64(v);
        v = hasher.finish();
    }
    v
}

#[inline]
fn bench_hasher_80bytes<T: Hasher + Default>() -> u64 {
    let mut v: u64 = test::black_box(114514);
    for _ in 0..100000 {
        let mut hasher = T::default();
        for _ in 0..10 {
            hasher.write_u64(v);
        }
        v = hasher.finish();
    }
    v
}

#[bench]
fn bench_default_hasher(b: &mut Bencher) {
    b.iter(|| bench_hasher::<DefaultHasher>());
}

#[bench]
fn bench_rustc_hash(b: &mut Bencher) {
    b.iter(|| bench_hasher::<rustc_hash::FxHasher>());
}

#[bench]
fn bench_fnv(b: &mut Bencher) {
    b.iter(|| bench_hasher::<fnv::FnvHasher>());
}

#[bench]
fn bench_default_hasher_80bytes(b: &mut Bencher) {
    b.iter(|| bench_hasher_80bytes::<DefaultHasher>());
}

#[bench]
fn bench_rustc_hash_80bytes(b: &mut Bencher) {
    b.iter(|| bench_hasher_80bytes::<rustc_hash::FxHasher>());
}

#[bench]
fn bench_fnv_80bytes(b: &mut Bencher) {
    b.iter(|| bench_hasher_80bytes::<fnv::FnvHasher>());
}
