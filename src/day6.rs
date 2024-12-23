use aoc_runner_derive::aoc;
use std::{
    collections::{HashMap, HashSet},
    ops::{Add, AddAssign},
};

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

impl Add<Direction> for Pos {
    type Output = Pos;

    fn add(self, direction: Direction) -> Pos {
        match direction {
            Direction::Up => Pos {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Down => Pos {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Pos {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Pos {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

impl AddAssign<Direction> for Pos {
    fn add_assign(&mut self, direction: Direction) {
        *self = *self + direction;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum SimulationResult {
    Loop,
    Exit,
}

#[derive(Clone)]
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
                _ => panic!("Invalid entity {:?}", c),
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
        } else {
            break;
        }
    }

    result
}

#[aoc(day6, part2)]
pub fn part2(input: &Input) -> u64 {
    let input: &mut Input = &mut Input {
        map: input.map.clone(),
        guard: input.guard,
    };

    let initial_guard = input.guard;

    // direction where guard is moving
    let mut guard_direction = Direction::Up;

    let mut rocks_positions = HashSet::<Pos>::new();

    let mut visited = HashSet::<Pos>::new();
    visited.insert(input.guard);

    // for each position we want to try to put a rock in front of the guard
    // guard will turn to the right and if he will enter a loop we can increase
    // number of rocks
    loop {
        let next_pos = input.guard + guard_direction;

        if let Some(entity) = input.map.get(&next_pos) {
            match entity {
                Entity::Blank => {
                    if visited.contains(&next_pos) {
                        input.guard = next_pos;
                        continue;
                    }

                    // skip already visited positions
                    // we shouldn't put a rock in position where guard has already been
                    visited.insert(next_pos);

                    let mut modified_input = input.clone();
                    modified_input.map.insert(next_pos, Entity::Obstacle);

                    // println!(
                    //     "next pos {:?}, modified input map\n{}",
                    //     next_pos,
                    //     print_map(&modified_input, &HashMap::new())
                    // );

                    // let's see if we can put a rock in front of the guard
                    // simulate until guard enter a loop or exit map
                    match simulate(modified_input, guard_direction.clone()) {
                        SimulationResult::Loop => {
                            if next_pos != initial_guard {
                                rocks_positions.insert(next_pos);
                            }
                        }
                        SimulationResult::Exit => {}
                    }

                    input.guard = next_pos;
                }
                Entity::Obstacle => {
                    guard_direction = match guard_direction {
                        Direction::Up => Direction::Right,
                        Direction::Right => Direction::Down,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Up,
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

    rocks_positions.len() as u64
}

#[aoc(day6, part2, brute_force)]
pub fn part2_bf(input: &Input) -> u64 {
    let input: &mut Input = &mut Input {
        map: input.map.clone(),
        guard: input.guard,
    };

    let initial_guard = input.guard;

    let mut min_x = i64::MAX;
    let mut max_x = i64::MIN;
    let mut min_y = i64::MAX;
    let mut max_y = i64::MIN;

    input.map.iter().for_each(|(pos, _)| {
        min_x = min_x.min(pos.x);
        max_x = max_x.max(pos.x);
        min_y = min_y.min(pos.y);
        max_y = max_y.max(pos.y);
    });

    let mut rocks_positions = HashSet::<Pos>::new();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let pos = Pos { x, y };
            if let Some(entity) = input.map.get(&pos) {
                match entity {
                    Entity::Blank => {
                        let mut modified_input = input.clone();
                        modified_input.map.insert(pos, Entity::Obstacle);
                        match simulate(modified_input, Direction::Up) {
                            SimulationResult::Loop => {
                                if pos != initial_guard {
                                    rocks_positions.insert(pos);
                                }
                            }
                            SimulationResult::Exit => {}
                        }
                    }
                    Entity::Obstacle => {}
                }
            }
        }
    }

    rocks_positions.len() as u64
}

fn simulate(mut input: Input, mut guard_direction: Direction) -> SimulationResult {
    let mut visited = HashSet::<(Pos, Direction)>::new();
    // visited.insert(input.guard, vec![guard_direction]);

    loop {
        let next_pos = input.guard + guard_direction;

        // println!("simulation step");
        // print_map(&input);

        // std::thread::sleep(std::time::Duration::from_millis(10));

        // if we have visited this position with same direction
        // we are in a loop
        if visited.contains(&(input.guard, guard_direction)) {
            return SimulationResult::Loop;
        }

        // add this direction to visited
        visited.insert((input.guard, guard_direction));

        match input.map.get(&next_pos) {
            Some(entity) => {
                match entity {
                    Entity::Blank => {
                        input.guard = next_pos;
                    }
                    Entity::Obstacle => {
                        // turn right
                        guard_direction = match guard_direction {
                            Direction::Up => Direction::Right,
                            Direction::Right => Direction::Down,
                            Direction::Down => Direction::Left,
                            Direction::Left => Direction::Up,
                        };
                    }
                }
            }
            None => return SimulationResult::Exit,
        }
    }
}

fn print_map<'a>(input: &'a Input, visited: &'a HashMap<Pos, Vec<Direction>>) -> String {
    let mut min_x = i64::MAX;
    let mut max_x = i64::MIN;
    let mut min_y = i64::MAX;
    let mut max_y = i64::MIN;

    input.map.iter().for_each(|(pos, _)| {
        min_x = min_x.min(pos.x);
        max_x = max_x.max(pos.x);
        min_y = min_y.min(pos.y);
        max_y = max_y.max(pos.y);
    });

    let mut result = String::new();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let pos = Pos { x, y };
            if pos == input.guard {
                result.push_str("G");
            } else if let Some(entity) = input.map.get(&pos) {
                match entity {
                    Entity::Blank => {
                        if let Some(directions) = visited.get(&pos) {
                            if directions.contains(&Direction::Up) {
                                result.push_str("^");
                            } else if directions.contains(&Direction::Right) {
                                result.push_str(">");
                            } else if directions.contains(&Direction::Down) {
                                result.push_str("v");
                            } else if directions.contains(&Direction::Left) {
                                result.push_str("<");
                            } else {
                                result.push_str("?");
                            }
                        } else {
                            result.push_str(".");
                        }
                        // line.push(Span::styled(".", Style::default().fg(Color::DarkGray)))
                    }
                    Entity::Obstacle => {
                        result.push_str("#");
                    }
                }
            } else {
                result.push_str(" ");
            }
        }
        result.push_str("\n");
    }

    result
}

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

    static LOOP: &str = ".#..
.^.#
#...
..#.";

    #[test]
    fn test_day6_part1() {
        assert_eq!(41, part1(&parse_input(INPUT)));
    }

    #[test]
    fn test_day6_part2() {
        assert_eq!(6, part2(&parse_input(INPUT)));
    }

    #[test]
    fn test_day6_loop() {
        assert_eq!(
            SimulationResult::Loop,
            simulate(parse_input(LOOP), Direction::Up,)
        );
    }
}
