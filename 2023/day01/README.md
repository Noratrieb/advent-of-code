# day 1

benchmarks:

Ensure that `input.txt` contains many, many copies of the actual input, the actual input is way too small.

`cargo build --release && hyperfine 'target/release/day01 part2 naive' 'target/release/day01 part2 zero_alloc' 'target/release/day01 part2 branchless' 'target/release/day01 part2 vectorized'`
```
Benchmark 1: target/release/day01 part2 naive
  Time (mean ± σ):      1.066 s ±  0.017 s    [User: 1.048 s, System: 0.018 s]
  Range (min … max):    1.049 s …  1.099 s    10 runs
 
Benchmark 2: target/release/day01 part2 zero_alloc
  Time (mean ± σ):     212.7 ms ±   3.5 ms    [User: 195.0 ms, System: 17.6 ms]
  Range (min … max):   206.9 ms … 219.0 ms    14 runs
 
Benchmark 3: target/release/day01 part2 branchless
  Time (mean ± σ):     137.2 ms ±   1.8 ms    [User: 117.5 ms, System: 19.6 ms]
  Range (min … max):   133.5 ms … 142.2 ms    21 runs
 
Benchmark 4: target/release/day01 part2 vectorized
  Time (mean ± σ):      87.9 ms ±   0.9 ms    [User: 68.6 ms, System: 19.1 ms]
  Range (min … max):    86.5 ms …  90.6 ms    33 runs
 
Summary
  target/release/day01 part2 vectorized ran
    1.56 ± 0.03 times faster than target/release/day01 part2 branchless
    2.42 ± 0.05 times faster than target/release/day01 part2 zero_alloc
   12.13 ± 0.22 times faster than target/release/day01 part2 naive
```
