use std::collections::HashMap;

use aoc_runner_derive::aoc;

#[derive(Hash, Eq, PartialEq, Debug)]
struct XY {
    x: i32,
    y: i32,
}

#[aoc(day4, part1)]
pub fn xmas(input: &str) -> u64 {
    let mut matrix = HashMap::<XY, char>::new();
    // create matrix
    input.lines().enumerate().for_each(|(row, line)| {
        line.split("").enumerate().for_each(|(col, char)| {
            if let Some(chr) = char.chars().next() {
                let key = XY {
                    x: row as i32,
                    y: col as i32,
                };
                matrix.insert(key, chr);
            }
        });
    });

    let mut res = 0;

    matrix.iter().for_each(|(xy, chr)| {
        if *chr != 'X' {
            return;
        }

        let directions: Vec<(i32, i32)> = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let default = &' ';

        directions.iter().for_each(|direction| {
            let m_pos = &offset(xy, *direction, 1);
            let a_pos = &offset(xy, *direction, 2);
            let s_pos = &offset(xy, *direction, 3);

            if matrix.get(m_pos).unwrap_or(default) == &'M'
                && matrix.get(a_pos).unwrap_or(default) == &'A'
                && matrix.get(s_pos).unwrap_or(default) == &'S'
            {
                res += 1;
            }
        });
    });

    res
}

fn offset(xy: &XY, direction: (i32, i32), mul: i32) -> XY {
    XY {
        x: xy.x + (direction.0 * mul),
        y: xy.y + (direction.1 * mul),
    }
}

#[aoc(day4, part2)]
pub fn x_mas(input: &str) -> u64 {
    let mut matrix = HashMap::<XY, char>::new();
    // create matrix
    input.lines().enumerate().for_each(|(row, line)| {
        line.split("").enumerate().for_each(|(col, char)| {
            if let Some(chr) = char.chars().next() {
                let key = XY {
                    x: row as i32,
                    y: col as i32,
                };
                matrix.insert(key, chr);
            }
        });
    });

    let mut res = 0;

    matrix.iter().for_each(|(xy, chr)| {
        if *chr != 'A' {
            return;
        }

        // clockwise
        let pairs = [(-1, -1), (1, -1), (1, 1), (-1, 1)];

        let phrase: Vec<char> = pairs
            .iter()
            .map(|pair| {
                let m = XY {
                    x: xy.x + pair.0,
                    y: xy.y + pair.1,
                };

                *matrix.get(&m).unwrap_or(&' ')
            })
            .collect();

        let s = String::from_iter(phrase);

        if s == "MMSS" || s == "SMMS" || s == "SSMM" || s == "MSSM" {
            res += 1;
        }
    });

    res
}

#[cfg(test)]
mod tests {
    use crate::day4::{x_mas, xmas};

    #[test]
    fn test_matrix() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        assert_eq!(18, xmas(input));
    }

    #[test]
    fn test_x_mas() {
        let input = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";

        assert_eq!(9, x_mas(input));
    }
}
