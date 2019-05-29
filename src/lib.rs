#![feature(test)]

extern crate test;

use test::Bencher;
use std::hash::Hasher;

#[bench]
fn bench_rustc_hash_1e5(b: &mut Bencher) {
    b.iter(|| {
        let mut v = test::black_box(114514);
        for _ in 0..100000 {
            let mut hasher = rustc_hash::FxHasher::default();
            hasher.write_u64(v);
            v = hasher.finish();
        }
        v
    });
}

#[bench]
fn bench_fnv_1e5(b: &mut Bencher) {
    b.iter(|| {
        let mut v = test::black_box(114514);
        for _ in 0..100000 {
            let mut hasher = fnv::FnvHasher::default();
            hasher.write_u64(v);
            v = hasher.finish();
        }
        v
    });
}
