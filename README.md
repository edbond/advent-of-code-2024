# Advent Of Code 2024

## How to run

1. Login on advent of code website and get credentials from Cookie `session`
2. Run `cargo aoc credentials <session>` to store session
3. Run `cargo aoc input` to download inputs to `input/` folder
4. Run `cargo aoc` to run solutions

## Benchmarks

Run `cargo aoc bench` to run benchmarks

Running on Macbook Air M1

```shell
Benchmarking Day1 - Part1/(default): Collecting 100 samples in estimated 5.0522 s (374k iteraDay1 - Part1/(default)  time:   [13.656 µs 14.213 µs 15.095 µs]
                        change: [+2.3466% +7.6597% +18.291%] (p = 0.01 < 0.05)
                        Performance has regressed.
Found 17 outliers among 100 measurements (17.00%)
  5 (5.00%) high mild
  12 (12.00%) high severe

Benchmarking Day1 - Part2/(default): Collecting 100 samples in estimated 5.0626 s (146k iteraDay1 - Part2/(default)  time:   [35.805 µs 36.568 µs 37.533 µs]
                        change: [+6.1442% +9.0405% +12.488%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 9 outliers among 100 measurements (9.00%)
  6 (6.00%) high mild
  3 (3.00%) high severe
```
