```bash
running 12 tests
test bench_default_hasher         ... bench:   1,291,405 ns/iter (+/- 185,924)
test bench_default_hasher_80bytes ... bench:   5,243,199 ns/iter (+/- 576,191)
test bench_default_hasher_hashset ... bench:   4,328,125 ns/iter (+/- 386,722)
test bench_fnv                    ... bench:     923,496 ns/iter (+/- 38,665)
test bench_fnv_80bytes            ... bench:   9,277,113 ns/iter (+/- 319,083)
test bench_fnv_hashset            ... bench:   1,961,058 ns/iter (+/- 94,254)
test bench_metrohash              ... bench:     940,722 ns/iter (+/- 54,541)
test bench_metrohash_80bytes      ... bench:   4,390,872 ns/iter (+/- 140,878)
test bench_metrohash_hashset      ... bench:   3,347,676 ns/iter (+/- 136,092)
test bench_rustc_hash             ... bench:       8,300 ns/iter (+/- 889)
test bench_rustc_hash_80bytes     ... bench:   1,324,807 ns/iter (+/- 132,410)
test bench_rustc_hash_hashset     ... bench:   1,014,883 ns/iter (+/- 150,598)

test result: ok. 0 passed; 0 failed; 0 ignored; 12 measured; 0 filtered out

```
