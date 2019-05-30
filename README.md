```bash
running 9 tests
test bench_default_hasher         ... bench:   1,342,819 ns/iter (+/- 137,410)
test bench_default_hasher_80bytes ... bench:   5,638,427 ns/iter (+/- 536,055)
test bench_default_hasher_hashset ... bench:   4,400,129 ns/iter (+/- 1,080,342)
test bench_fnv                    ... bench:     930,111 ns/iter (+/- 123,109)
test bench_fnv_80bytes            ... bench:   8,996,418 ns/iter (+/- 352,129)
test bench_fnv_hashset            ... bench:   1,969,783 ns/iter (+/- 131,065)
test bench_rustc_hash             ... bench:       8,399 ns/iter (+/- 416)
test bench_rustc_hash_80bytes     ... bench:   1,345,089 ns/iter (+/- 48,735)
test bench_rustc_hash_hashset     ... bench:     962,355 ns/iter (+/- 198,173)

test result: ok. 0 passed; 0 failed; 0 ignored; 9 measured; 0 filtered out
```
