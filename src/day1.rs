use std::collections::{BinaryHeap, HashMap};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
pub struct Day1Input {
    left: Vec<i64>,
    right: Vec<i64>,
}

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Day1Input {
    let mut left = vec![];
    let mut right = vec![];

    input.lines().for_each(|line| {
        let nums: Vec<&str> = line.trim().split_whitespace().collect();

        let left_num: i64 = nums[0]
            .parse()
            .expect(format!("can't parse left number {}", nums[0]).as_str());
        let right_num: i64 = nums[1]
            .parse()
            .expect(format!("can't parse right number {}", nums[1]).as_str());

        left.push(left_num);
        right.push(right_num);
    });

    Day1Input { left, right }
}

#[aoc(day1, part1)]
pub fn part1(input: &Day1Input) -> u32 {
    let copy = input.clone();
    let (mut left, mut right) = (copy.left.clone(), copy.right.clone());

    left.sort_unstable();
    right.sort_unstable();

    let mut sum = 0;

    left.iter().enumerate().for_each(|(i, _val)| {
        sum += left[i].abs_diff(right[i]);
    });

    sum as u32
}

#[aoc(day1, part1, heap)]
pub fn part1_heap(input: &Day1Input) -> u32 {
    let mut left = BinaryHeap::new();
    let mut right = BinaryHeap::new();

    input.left.iter().for_each(|n| left.push(n));
    input.right.iter().for_each(|n| right.push(n));

    let mut sum: u32 = 0;
    while let Some(a) = left.pop() {
        let b = right.pop().expect("have right number");
        sum += a.abs_diff(*b) as u32
    }
    sum
}

#[aoc(day1, part2)]
pub fn part2(input: &Day1Input) -> u32 {
    let (left, right) = (input.left.clone(), input.right.clone());
    let mut freq = HashMap::<i64, i64>::new();

    right.iter().for_each(|key| {
        let _ = *freq.entry(*key).and_modify(|e| *e += 1).or_insert(1);
    });

    let mut similarity = 0;

    left.iter().enumerate().for_each(|(i, val)| {
        if let Some(count) = freq.get(val) {
            similarity += left[i] * count;
        }
    });

    similarity as u32
}
