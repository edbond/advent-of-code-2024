# Advent Of Code 2024

## How to run

1. Login on advent of code website and get credentials from Cookie `session`
2. Run `cargo aoc credentials <session>` to store session
3. Run `cargo aoc input` to download inputs to `input/` folder
4. Run `cargo aoc` to run solutions

```shell
AOC 2024
Day 1 - Part 1 : <answer>
	generator: 137.208µs,
	runner: 29.959µs

Day 1 - Part 1 - heap : <answer>
	generator: 132.334µs,
	runner: 79.916µs

Day 1 - Part 2 : <answer>
	generator: 130.625µs,
	runner: 48.5µs
```

## Benchmarks

Run `cargo aoc bench` to run benchmarks

Running on Macbook Air M1 2020

```shell
Benchmarking Day1 - Part1/(default): Collecting 100 samples in estimated 5.0488 s (429k iteraDay1 - Part1/(default)  time:   [11.754 µs 11.775 µs 11.800 µs]
                        change: [-11.893% -11.662% -11.417%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
Benchmarking Day1 - Part1/heap: Collecting 100 samples in estimated 5.1119 s (126k iterationsDay1 - Part1/heap       time:   [40.928 µs 41.437 µs 41.971 µs]
                        change: [-0.6124% +1.0622% +2.8083%] (p = 0.22 > 0.05)
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  6 (6.00%) high mild

Benchmarking Day1 - Part2/(default): Collecting 100 samples in estimated 5.1297 s (152k iteraDay1 - Part2/(default)  time:   [34.012 µs 34.098 µs 34.200 µs]
                        change: [-1.0123% -0.1619% +0.4262%] (p = 0.72 > 0.05)
                        No change in performance detected.
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) high mild
  5 (5.00%) high severe
```
