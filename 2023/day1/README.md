# day 1

benchmarks:

Ensure that `input.txt` contains many, many copies of the actual input, the actual input is way too small.

`cargo build --release && hyperfine 'target/release/day1 naive' 'target/release/day1 zero_alloc' 'target/release/day1 branchless' 'target/release/day1 vectorized'`
```
Benchmark 1: target/release/day1 naive
  Time (mean ± σ):      4.735 s ±  0.061 s    [User: 4.663 s, System: 0.072 s]
  Range (min … max):    4.643 s …  4.798 s    10 runs
 
Benchmark 2: target/release/day1 zero_alloc
  Time (mean ± σ):     880.1 ms ±  10.7 ms    [User: 807.9 ms, System: 72.1 ms]
  Range (min … max):   858.3 ms … 891.4 ms    10 runs
 
Benchmark 3: target/release/day1 branchless
  Time (mean ± σ):     587.1 ms ±   4.4 ms    [User: 515.0 ms, System: 72.1 ms]
  Range (min … max):   578.3 ms … 594.1 ms    10 runs
 
Benchmark 4: target/release/day1 vectorized
  Time (mean ± σ):     394.3 ms ±   5.2 ms    [User: 322.2 ms, System: 71.9 ms]
  Range (min … max):   386.4 ms … 400.0 ms    10 runs
 
Summary
  target/release/day1 vectorized ran
    1.49 ± 0.02 times faster than target/release/day1 branchless
    2.23 ± 0.04 times faster than target/release/day1 zero_alloc
   12.01 ± 0.22 times faster than target/release/day1 naive
```
