# Run
```bash
$ cargo test
```

## With console output

```bash
$ cargo test -- --nocapture
```

# Benchmark
Using the `criterion` crate.

```bash
$ cargo bench
```

# 2024 - Benchmarks

2019 | day 1 | part 1   time:   [148.95 µs 149.20 µs 149.47 µs]
                        change: [+7593.6% +7847.1% +8056.1%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe

2024 | day 1 | part 2   time:   [71.705 µs 71.856 µs 72.089 µs]
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe

2024 | day 2 | part 1   time:   [125.96 µs 126.17 µs 126.41 µs]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

2024 | day 2 | part 2   time:   [365.86 µs 372.80 µs 383.65 µs]
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
