#![feature(test)]

extern crate test;

use std::collections::hash_map::{DefaultHasher, RandomState};
use std::collections::HashSet;
use std::hash::{BuildHasher, BuildHasherDefault, Hasher};
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

#[inline]
fn bench_hashset<S: BuildHasher + Default>() {
    let mut hashset = HashSet::<u64, S>::default();
    for i in 0..100000 {
        hashset.insert(i);
    }
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
fn bench_metrohash(b: &mut Bencher) {
    b.iter(|| bench_hasher::<metrohash::MetroHash>());
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

#[bench]
fn bench_metrohash_80bytes(b: &mut Bencher) {
    b.iter(|| bench_hasher_80bytes::<metrohash::MetroHash>());
}

#[bench]
fn bench_default_hasher_hashset(b: &mut Bencher) {
    b.iter(|| bench_hashset::<RandomState>())
}

#[bench]
fn bench_rustc_hash_hashset(b: &mut Bencher) {
    b.iter(|| bench_hashset::<BuildHasherDefault<rustc_hash::FxHasher>>())
}

#[bench]
fn bench_fnv_hashset(b: &mut Bencher) {
    b.iter(|| bench_hashset::<fnv::FnvBuildHasher>())
}

#[bench]
fn bench_metrohash_hashset(b: &mut Bencher) {
    b.iter(|| bench_hashset::<metrohash::MetroBuildHasher>())
}
