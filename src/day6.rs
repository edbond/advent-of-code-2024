use std::collections::{HashMap, HashSet};

use aoc_runner_derive::aoc;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Entity {
    Blank,
    Obstacle,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Pos {
    x: i64,
    y: i64,
}

pub struct Input {
    map: HashMap<Pos, Entity>,
    guard: Pos,
}

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Input {
    let mut result = Input {
        map: HashMap::new(),
        guard: Pos { x: 0, y: 0 },
    };

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            let pos = Pos {
                x: x as i64,
                y: y as i64,
            };
            let entity = match c {
                '.' => Entity::Blank,
                '#' => Entity::Obstacle,
                '^' => {
                    result.guard = pos;
                    Entity::Blank
                }
                _ => panic!("Invalid entity"),
            };
            result.map.insert(pos, entity);
        });
    });

    result
}

#[aoc(day6, part1)]
pub fn part1(input: &Input) -> u64 {
    let mut result = 1;
    let input: &mut Input = &mut Input {
        map: input.map.clone(),
        guard: input.guard,
    };

    let mut delta = (0, -1);

    let mut visited = HashSet::<Pos>::new();
    visited.insert(input.guard);

    loop {
        let next_pos = Pos {
            x: input.guard.x + delta.0,
            y: input.guard.y + delta.1,
        };

        if let Some(entity) = input.map.get(&next_pos) {
            match entity {
                Entity::Blank => {
                    if !visited.contains(&next_pos) {
                        visited.insert(next_pos);
                        result += 1;
                    }

                    input.guard = next_pos;
                }
                Entity::Obstacle => {
                    delta = match delta {
                        (0, -1) => (1, 0),
                        (1, 0) => (0, 1),
                        (0, 1) => (-1, 0),
                        (-1, 0) => (0, -1),
                        _ => panic!("Invalid delta"),
                    };
                }
            }

            // print_map(&input);

            // wait for keypress
            // let mut _input = String::new();
            // std::io::stdin().read_line(&mut _input).unwrap();
        } else {
            break;
        }
    }

    result
}

fn print_map(input: &Input) {
    let mut min_x = i64::MAX;
    let mut max_x = i64::MIN;
    let mut min_y = i64::MAX;
    let mut max_y = i64::MIN;

    println!("printing map");

    input.map.iter().for_each(|(pos, _)| {
        min_x = min_x.min(pos.x);
        max_x = max_x.max(pos.x);
        min_y = min_y.min(pos.y);
        max_y = max_y.max(pos.y);
    });

    println!("bounds x: {} - {}", min_x, max_x);
    println!("bounds y: {} - {}", min_y, max_y);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let pos = Pos { x, y };
            if pos == input.guard {
                print!("^");
            } else if let Some(entity) = input.map.get(&pos) {
                match entity {
                    Entity::Blank => print!("."),
                    Entity::Obstacle => print!("#"),
                }
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

// #[aoc(day6, part2)]
// fn part2(input: &Input) -> u64 {}

#[cfg(test)]
mod tests {
    use crate::day6::*;

    static INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part1() {
        assert_eq!(41, part1(&parse_input(INPUT)));
    }
}
