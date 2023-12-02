# day 1

benchmarks:

Ensure that `input.txt` contains many, many copies of the actual input, the actual input is way too small.

`cargo build --release && hyperfine 'target/release/day1 naive' 'target/release/day1 zero_alloc' 'target/release/day1 branchless'`
```
  target/release/day1 branchless ran
    1.52 ± 0.06 times faster than target/release/day1 zero_alloc
    7.74 ± 0.27 times faster than target/release/day1 naive
```
